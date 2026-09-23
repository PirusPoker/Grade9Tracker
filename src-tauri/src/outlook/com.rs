//! Outlook over COM, directly from Rust.
//!
//! This used to be a PowerShell script run in a hidden child process. It
//! worked, but a hidden PowerShell that reads someone's mail is, byte for
//! byte, what antivirus heuristics are trained to catch - Defender
//! quarantined the app over it on 22/09/2026. Talking to Outlook ourselves
//! removes the child process entirely, and it is quicker.
//!
//! Everything is late-bound IDispatch: ask Outlook for a member's DISPID by
//! name, then Invoke it. That is exactly what PowerShell was doing under the
//! hood, so the object model calls - and their order, which Outlook is fussy
//! about - carry over one for one from the old script.
//!
//! Must run on a thread that has called CoInitializeEx; `super::fetch` owns
//! that thread and the timeout. Nothing in here may panic: the release build
//! aborts on panic, and a bad mail item must never take the app down.

use super::{Dump, Event, Mail, Task, CALENDAR_DAYS, EVENT_BODY_CHARS, MAIL_BODY_CHARS, MAIL_DAYS, MAX_MAILS};
use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeDelta, Timelike};
use regex::Regex;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::OnceLock;
use windows::core::{Interface, BSTR, GUID, IUnknown, PCWSTR, w};
use windows::Win32::Foundation::SYSTEMTIME;
use windows::Win32::Globalization::{GetDateFormatEx, GetLocaleInfoEx, GetTimeFormatEx, DATE_SHORTDATE, LOCALE_SSHORTTIME, TIME_FORMAT_FLAGS, TIME_NOSECONDS};
use windows::Win32::System::Com::{
    CLSIDFromProgID, IDispatch, DISPATCH_FLAGS, DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPATCH_PROPERTYPUT, DISPPARAMS, EXCEPINFO,
};
use windows::Win32::System::Ole::{GetActiveObject, DISPID_PROPERTYPUT};
use windows::Win32::System::Variant::{
    VariantChangeType, VariantClear, VAR_CHANGE_FLAGS, VARENUM, VARIANT, VT_BOOL, VT_BSTR, VT_DATE, VT_DISPATCH, VT_EMPTY, VT_I4, VT_NULL,
};

/// Why a read failed. `NotRunning` gets the "open classic Outlook" message.
pub enum Failure {
    NotRunning,
    Other(String),
}

// HRESULTs we act on, as unsigned so they read like the documentation.
const DISP_E_MEMBERNOTFOUND: u32 = 0x8002_0003;
const DISP_E_UNKNOWNNAME: u32 = 0x8002_0006;
const DISP_E_EXCEPTION: u32 = 0x8002_0009;
const RPC_E_CALL_REJECTED: u32 = 0x8001_0001;
const RPC_E_SERVERCALL_RETRYLATER: u32 = 0x8001_010A;
const LOCALE_USER_DEFAULT: u32 = 0x0400;

// Outlook's own numbers: default folders, and the class of a plain mail.
const OL_FOLDER_INBOX: i32 = 6;
const OL_FOLDER_CALENDAR: i32 = 9;
const OL_FOLDER_TASKS: i32 = 13;
const OL_MAIL: i32 = 43;
const OL_FLAG_MARKED: i32 = 2;
const MAX_EVENTS: usize = 300;

type ComResult<T> = windows::core::Result<T>;

// ---------- VARIANT in and out ----------

/// An owned VARIANT, cleared on drop (which releases any string or object in it).
struct Var(VARIANT);

impl Drop for Var {
    fn drop(&mut self) {
        unsafe {
            let _ = VariantClear(&mut self.0);
        }
    }
}

fn vt(v: &VARIANT) -> VARENUM {
    unsafe { v.Anonymous.Anonymous.vt }
}

fn arg_str(s: &str) -> VARIANT {
    let mut v = VARIANT::default();
    unsafe {
        (*v.Anonymous.Anonymous).vt = VT_BSTR;
        (*v.Anonymous.Anonymous).Anonymous.bstrVal = ManuallyDrop::new(BSTR::from(s));
    }
    v
}

fn arg_i4(n: i32) -> VARIANT {
    let mut v = VARIANT::default();
    unsafe {
        (*v.Anonymous.Anonymous).vt = VT_I4;
        (*v.Anonymous.Anonymous).Anonymous.lVal = n;
    }
    v
}

fn arg_bool(b: bool) -> VARIANT {
    let mut v = VARIANT::default();
    unsafe {
        (*v.Anonymous.Anonymous).vt = VT_BOOL;
        // VARIANT_TRUE is -1, not 1.
        (*v.Anonymous.Anonymous).Anonymous.boolVal = windows::Win32::Foundation::VARIANT_BOOL(if b { -1 } else { 0 });
    }
    v
}

/// Arguments for one Invoke, cleared however the call ends. An early `?`
/// used to skip the clearing and leak every string passed to Sort and Restrict.
struct Args(Vec<VARIANT>);

impl Drop for Args {
    fn drop(&mut self) {
        for a in self.0.iter_mut() {
            unsafe {
                let _ = VariantClear(a);
            }
        }
    }
}

impl Var {
    /// The object in it, if it holds one. Nothing at all (the end of an
    /// Items list) comes back as None.
    fn object(&self) -> Option<IDispatch> {
        if vt(&self.0) != VT_DISPATCH {
            return None;
        }
        unsafe { (*self.0.Anonymous.Anonymous.Anonymous.pdispVal).clone() }
    }

    /// As text, UTF-16, the way PowerShell's `'' + $x` would give it: empty
    /// for nothing, the string for a string, anything else converted.
    fn text16(&self) -> Vec<u16> {
        match vt(&self.0) {
            VT_EMPTY | VT_NULL => Vec::new(),
            VT_BSTR => unsafe { self.0.Anonymous.Anonymous.Anonymous.bstrVal[..].to_vec() },
            _ => {
                let mut out = Var(VARIANT::default());
                if unsafe { VariantChangeType(&mut out.0, &self.0, VAR_CHANGE_FLAGS(0), VT_BSTR) }.is_ok() && vt(&out.0) == VT_BSTR {
                    unsafe { out.0.Anonymous.Anonymous.Anonymous.bstrVal[..].to_vec() }
                } else {
                    Vec::new()
                }
            }
        }
    }

    fn text(&self) -> String {
        String::from_utf16_lossy(&self.text16())
    }

    fn int(&self) -> Option<i32> {
        match vt(&self.0) {
            VT_I4 => Some(unsafe { self.0.Anonymous.Anonymous.Anonymous.lVal }),
            _ => {
                let mut out = Var(VARIANT::default());
                if unsafe { VariantChangeType(&mut out.0, &self.0, VAR_CHANGE_FLAGS(0), VT_I4) }.is_ok() && vt(&out.0) == VT_I4 {
                    Some(unsafe { out.0.Anonymous.Anonymous.Anonymous.lVal })
                } else {
                    None
                }
            }
        }
    }

    fn truthy(&self) -> bool {
        match vt(&self.0) {
            VT_BOOL => unsafe { self.0.Anonymous.Anonymous.Anonymous.boolVal.0 != 0 },
            _ => self.int().unwrap_or(0) != 0,
        }
    }

    fn date(&self) -> Option<f64> {
        (vt(&self.0) == VT_DATE).then(|| unsafe { self.0.Anonymous.Anonymous.Anonymous.date })
    }
}

// ---------- calling members by name ----------

/// Which sort of object a call is on. DISPIDs are looked up once per sort and
/// member, not once per call: that halves the round trips to Outlook.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Kind {
    App,
    Namespace,
    Folder,
    Items,
    Appt,
    Mail,
    Task,
}

#[derive(Default)]
struct Com {
    ids: HashMap<(Kind, &'static str), i32>,
}

impl Com {
    fn lookup(obj: &IDispatch, name: &str) -> ComResult<i32> {
        let wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        let names = [PCWSTR(wide.as_ptr())];
        let mut id = 0i32;
        unsafe { obj.GetIDsOfNames(&GUID::zeroed(), names.as_ptr(), 1, LOCALE_USER_DEFAULT, &mut id)? };
        Ok(id)
    }

    /// Invoke `name` on `obj`. Arguments are given in their natural order.
    /// A cached DISPID that the object does not recognise is looked up again
    /// once; Outlook being busy is waited out for up to about three seconds.
    fn invoke(&mut self, obj: &IDispatch, kind: Kind, name: &'static str, flags: DISPATCH_FLAGS, args: Vec<VARIANT>) -> ComResult<Var> {
        let mut args = Args(args);
        args.0.reverse(); // IDispatch takes them right to left
        let put = flags == DISPATCH_PROPERTYPUT;
        let mut named = DISPID_PROPERTYPUT;
        let params = DISPPARAMS {
            rgvarg: if args.0.is_empty() { std::ptr::null_mut() } else { args.0.as_mut_ptr() },
            rgdispidNamedArgs: if put { &mut named } else { std::ptr::null_mut() },
            cArgs: args.0.len() as u32,
            cNamedArgs: u32::from(put),
        };
        let mut fresh = false;
        let mut busy = 0;
        loop {
            let id = match self.ids.get(&(kind, name)) {
                Some(&id) => id,
                None => {
                    let id = Self::lookup(obj, name)?;
                    self.ids.insert((kind, name), id);
                    fresh = true;
                    id
                }
            };
            let mut out = Var(VARIANT::default());
            let mut excep = EXCEPINFO::default();
            let r = unsafe {
                obj.Invoke(id, &GUID::zeroed(), LOCALE_USER_DEFAULT, flags, &params, if put { None } else { Some(&mut out.0) }, Some(&mut excep), None)
            };
            match r {
                Ok(()) => break Ok(out),
                Err(e) => {
                    let code = e.code().0 as u32;
                    if (code == DISP_E_MEMBERNOTFOUND || code == DISP_E_UNKNOWNNAME) && !fresh {
                        self.ids.remove(&(kind, name));
                        continue;
                    }
                    if (code == RPC_E_SERVERCALL_RETRYLATER || code == RPC_E_CALL_REJECTED) && busy < 60 {
                        busy += 1;
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        continue;
                    }
                    if code == DISP_E_EXCEPTION && !excep.bstrDescription.is_empty() {
                        break Err(windows::core::Error::new(e.code(), excep.bstrDescription.to_string()));
                    }
                    break Err(e);
                }
            }
        }
    }

    fn get(&mut self, obj: &IDispatch, kind: Kind, name: &'static str) -> ComResult<Var> {
        self.invoke(obj, kind, name, DISPATCH_PROPERTYGET | DISPATCH_METHOD, Vec::new())
    }

    fn call(&mut self, obj: &IDispatch, kind: Kind, name: &'static str, args: Vec<VARIANT>) -> ComResult<Var> {
        self.invoke(obj, kind, name, DISPATCH_METHOD | DISPATCH_PROPERTYGET, args)
    }

    fn put(&mut self, obj: &IDispatch, kind: Kind, name: &'static str, value: VARIANT) -> ComResult<()> {
        self.invoke(obj, kind, name, DISPATCH_PROPERTYPUT, vec![value]).map(|_| ())
    }

    /// A member that has to give back an object.
    fn object(&mut self, obj: &IDispatch, kind: Kind, name: &'static str, args: Vec<VARIANT>) -> ComResult<IDispatch> {
        self.call(obj, kind, name, args)?
            .object()
            .ok_or_else(|| windows::core::Error::new(windows::core::HRESULT(0x8000_4005u32 as i32), format!("{name} returned nothing")))
    }

    /// Walk an Items collection with GetFirst/GetNext, which is what Outlook
    /// documents for restricted, recurrence-expanded lists. `each` returns
    /// false to stop early. A failure part-way through ends the walk and
    /// keeps what was read, rather than throwing the lot away.
    fn walk(&mut self, items: &IDispatch, mut each: impl FnMut(&mut Com, &IDispatch) -> bool) {
        let mut next = self.call(items, Kind::Items, "GetFirst", Vec::new());
        while let Ok(v) = next {
            let Some(item) = v.object() else { break };
            if !each(self, &item) {
                break;
            }
            next = self.call(items, Kind::Items, "GetNext", Vec::new());
        }
    }
}

fn com_err(e: windows::core::Error) -> Failure {
    let msg = e.message();
    Failure::Other(if msg.trim().is_empty() { format!("{:#010x}", e.code().0 as u32) } else { msg.trim().to_string() })
}

// ---------- dates, the way .NET printed them ----------

/// An OLE Automation date as local `yyyy-MM-ddTHH:mm:ss`, reproducing
/// `DateTime.FromOADate(d).ToString('s')` exactly: round to the nearest
/// millisecond, then drop the milliseconds. Getting this wrong is not
/// cosmetic - event ids embed the start time and are stored as dismissal
/// keys, so 09:05 must not come out as 09:04:59 because 09:05 is not exact
/// in binary floating point.
fn oa_to_local(d: f64) -> Option<NaiveDateTime> {
    if !d.is_finite() || !(-657_435.0..2_958_466.0).contains(&d) {
        return None;
    }
    const MS_PER_DAY: i64 = 86_400_000;
    let mut millis = (d * MS_PER_DAY as f64 + if d >= 0.0 { 0.5 } else { -0.5 }) as i64;
    if millis < 0 {
        millis -= (millis % MS_PER_DAY) * 2; // negative dates count the time forwards
    }
    NaiveDate::from_ymd_opt(1899, 12, 30)?.and_hms_opt(0, 0, 0)?.checked_add_signed(TimeDelta::milliseconds(millis))
}

fn stamp(d: f64) -> Option<String> {
    oa_to_local(d).map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string())
}

/// `yyyy-MM-dd`, or None for Outlook's "no date" (4501-01-01).
fn day(d: Option<f64>) -> Option<String> {
    let t = oa_to_local(d?)?;
    (t.year() < 4000).then(|| t.format("%Y-%m-%d").to_string())
}

/// A local time the way .NET's `ToString('g')` writes it - the user's short
/// date pattern, a space, their short time pattern - because Restrict parses
/// filter dates in the user's own locale. An en-GB machine wants
/// `30/09/2026 00:00`, and a hard-coded US order would silently match the
/// wrong fortnight.
///
/// The time half deliberately uses LOCALE_SSHORTTIME, which is what .NET
/// reads for 'g'. TIME_NOSECONDS instead strips the seconds off the *long*
/// time pattern: the same thing on a default install, but not once someone
/// has customised the short time in Region settings.
fn restrict_date(t: NaiveDateTime) -> String {
    let st = SYSTEMTIME {
        wYear: t.year() as u16,
        wMonth: t.month() as u16,
        wDayOfWeek: t.weekday().num_days_from_sunday() as u16,
        wDay: t.day() as u16,
        wHour: t.hour() as u16,
        wMinute: t.minute() as u16,
        wSecond: 0,
        wMilliseconds: 0,
    };
    let mut d = [0u16; 128];
    let mut h = [0u16; 128];
    let dn = unsafe { GetDateFormatEx(PCWSTR::null(), DATE_SHORTDATE, Some(&st), PCWSTR::null(), Some(&mut d), PCWSTR::null()) };
    let mut pattern = [0u16; 80];
    let pn = unsafe { GetLocaleInfoEx(PCWSTR::null(), LOCALE_SSHORTTIME, Some(&mut pattern)) };
    let hn = if pn > 1 {
        // pattern still ends in its null, so it is a valid PCWSTR as it stands
        unsafe { GetTimeFormatEx(PCWSTR::null(), TIME_FORMAT_FLAGS(0), Some(&st), PCWSTR(pattern.as_ptr()), Some(&mut h)) }
    } else {
        unsafe { GetTimeFormatEx(PCWSTR::null(), TIME_NOSECONDS, Some(&st), PCWSTR::null(), Some(&mut h)) }
    };
    // Both counts include the terminating null.
    let date = String::from_utf16_lossy(&d[..(dn.max(1) as usize - 1)]);
    let time = String::from_utf16_lossy(&h[..(hn.max(1) as usize - 1)]);
    format!("{date} {time}")
}

// ---------- text ----------

/// The first `n` UTF-16 units, as PowerShell's Substring counted them, but
/// never cutting an emoji in half: a lone surrogate used to make the whole
/// dump unparseable.
fn clip(s: &[u16], n: usize) -> String {
    if s.len() <= n {
        return String::from_utf16_lossy(s);
    }
    let mut end = n;
    if end > 0 && (0xD800..0xDC00).contains(&s[end - 1]) {
        end -= 1;
    }
    String::from_utf16_lossy(&s[..end])
}

fn links_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"https?://").expect("static regex"))
}

fn bulk_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?i)unsubscribe|manage (your )?preferences|email preferences").expect("static regex"))
}

// ---------- the read ----------

/// Attach to the Outlook the user already has open. Never start one: a
/// cold-started Outlook throws up sign-in and access prompts, and reading
/// an already-running, already-authenticated one is silent.
fn attach() -> Result<IDispatch, Failure> {
    let clsid = unsafe { CLSIDFromProgID(w!("Outlook.Application")) }.map_err(|_| Failure::NotRunning)?;
    let mut unk: Option<IUnknown> = None;
    unsafe { GetActiveObject(&clsid, None, &mut unk) }.map_err(|_| Failure::NotRunning)?;
    unk.ok_or(Failure::NotRunning)?.cast::<IDispatch>().map_err(com_err)
}

fn folder_items(c: &mut Com, ns: &IDispatch, which: i32) -> ComResult<IDispatch> {
    let folder = c.object(ns, Kind::Namespace, "GetDefaultFolder", vec![arg_i4(which)])?;
    c.object(&folder, Kind::Folder, "Items", Vec::new())
}

fn calendar(c: &mut Com, ns: &IDispatch, now: NaiveDateTime) -> ComResult<Vec<Event>> {
    let from = now.date().and_hms_opt(0, 0, 0).unwrap_or(now);
    let to = from + TimeDelta::days(CALENDAR_DAYS);
    let items = folder_items(c, ns, OL_FOLDER_CALENDAR)?;
    // Sort, then IncludeRecurrences, then Restrict - Outlook insists on that
    // order, and walking recurrences without a Restrict never ends.
    c.call(&items, Kind::Items, "Sort", vec![arg_str("[Start]")])?;
    c.put(&items, Kind::Items, "IncludeRecurrences", arg_bool(true))?;
    let filter = format!("[Start] < '{}' AND [End] > '{}'", restrict_date(to), restrict_date(from));
    let found = c.object(&items, Kind::Items, "Restrict", vec![arg_str(&filter)])?;
    let mut out = Vec::new();
    c.walk(&found, |c, e| {
        if let Ok(ev) = event(c, e) {
            out.push(ev);
        }
        out.len() < MAX_EVENTS
    });
    Ok(out)
}

fn event(c: &mut Com, e: &IDispatch) -> ComResult<Event> {
    let bad = || windows::core::Error::new(windows::core::HRESULT(0x8000_4005u32 as i32), "no start or end");
    let start = c.get(e, Kind::Appt, "Start")?.date().and_then(stamp).ok_or_else(bad)?;
    let end = c.get(e, Kind::Appt, "End")?.date().and_then(stamp).ok_or_else(bad)?;
    Ok(Event {
        // Recurrences share one EntryID, so the start time is part of the id.
        // Stored as a dismissal key: it must match what the old script made.
        id: format!("{}|{}", c.get(e, Kind::Appt, "EntryID")?.text(), start),
        subject: c.get(e, Kind::Appt, "Subject")?.text(),
        start,
        end,
        all_day: c.get(e, Kind::Appt, "AllDayEvent")?.truthy(),
        location: c.get(e, Kind::Appt, "Location")?.text(),
        body: clip(&c.get(e, Kind::Appt, "Body")?.text16(), EVENT_BODY_CHARS),
    })
}

fn inbox(c: &mut Com, ns: &IDispatch, now: NaiveDateTime) -> ComResult<Vec<Mail>> {
    let since = now - TimeDelta::days(MAIL_DAYS);
    let items = folder_items(c, ns, OL_FOLDER_INBOX)?;
    c.call(&items, Kind::Items, "Sort", vec![arg_str("[ReceivedTime]"), arg_bool(true)])?; // newest first
    let filter = format!("[ReceivedTime] >= '{}'", restrict_date(since));
    let found = c.object(&items, Kind::Items, "Restrict", vec![arg_str(&filter)])?;
    let mut out = Vec::new();
    c.walk(&found, |c, m| {
        // Only plain mail: meeting requests, receipts and reports have other classes.
        if c.get(m, Kind::Mail, "Class").ok().and_then(|v| v.int()) == Some(OL_MAIL) {
            if let Ok(mail) = mail(c, m) {
                out.push(mail);
            }
        }
        out.len() < MAX_MAILS
    });
    Ok(out)
}

fn mail(c: &mut Com, m: &IDispatch) -> ComResult<Mail> {
    // Links and the unsubscribe test read the whole body, before clipping:
    // today.rs uses them to tell a teacher's note from a newsletter.
    let full = c.get(m, Kind::Mail, "Body").map(|v| v.text16()).unwrap_or_default();
    let full_text = String::from_utf16_lossy(&full);
    let received = c
        .get(m, Kind::Mail, "ReceivedTime")?
        .date()
        .and_then(stamp)
        .ok_or_else(|| windows::core::Error::new(windows::core::HRESULT(0x8000_4005u32 as i32), "no received time"))?;
    Ok(Mail {
        id: c.get(m, Kind::Mail, "EntryID")?.text(),
        subject: c.get(m, Kind::Mail, "Subject")?.text(),
        sender: c.get(m, Kind::Mail, "SenderName")?.text(),
        received,
        unread: c.get(m, Kind::Mail, "UnRead")?.truthy(),
        flagged: c.get(m, Kind::Mail, "FlagStatus")?.int() == Some(OL_FLAG_MARKED),
        due: c.get(m, Kind::Mail, "TaskDueDate").ok().and_then(|v| day(v.date())),
        body: clip(&full, MAIL_BODY_CHARS),
        links: links_re().find_iter(&full_text).count() as u32,
        bulk: bulk_re().is_match(&full_text),
    })
}

/// Open tasks. Optional - plenty of people have no tasks folder in use.
fn tasks(c: &mut Com, ns: &IDispatch) -> ComResult<Vec<Task>> {
    let items = folder_items(c, ns, OL_FOLDER_TASKS)?;
    let found = c.object(&items, Kind::Items, "Restrict", vec![arg_str("[Complete] = False")])?;
    let mut out = Vec::new();
    c.walk(&found, |c, t| {
        let read = |c: &mut Com| -> ComResult<Task> {
            Ok(Task {
                id: c.get(t, Kind::Task, "EntryID")?.text(),
                subject: c.get(t, Kind::Task, "Subject")?.text(),
                due: day(c.get(t, Kind::Task, "DueDate")?.date()),
            })
        };
        if let Ok(task) = read(c) {
            out.push(task);
        }
        true
    });
    Ok(out)
}

/// Read the calendar, inbox and tasks. The calling thread must have COM
/// initialised; every object is released before this returns.
pub fn read() -> Result<Dump, Failure> {
    let now = chrono::Local::now().naive_local();
    let mut c = Com::default();
    let ol = attach()?;
    let ns = c.object(&ol, Kind::App, "GetNamespace", vec![arg_str("MAPI")]).map_err(com_err)?;
    let events = calendar(&mut c, &ns, now).map_err(com_err)?;
    let mails = inbox(&mut c, &ns, now).map_err(com_err)?;
    let tasks = tasks(&mut c, &ns).unwrap_or_default();
    Ok(Dump { events, mails, tasks })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oa(y: i32, mo: u32, d: u32, h: u32, mi: u32) -> f64 {
        // How Outlook stores it: days since 1899-12-30 as a double.
        let base = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();
        let date = NaiveDate::from_ymd_opt(y, mo, d).unwrap();
        (date - base).num_days() as f64 + (h as f64 * 60.0 + mi as f64) / 1440.0
    }

    /// The whole point of the rounding: a start at 09:05 must print 09:05:00,
    /// never 09:04:59, or every dismissed recurring lesson comes back.
    #[test]
    fn minutes_that_are_not_exact_in_binary_still_print_exactly() {
        for (h, m) in [(9, 5), (13, 35), (15, 20), (8, 45), (23, 59), (0, 1)] {
            let s = stamp(oa(2026, 9, 24, h, m)).unwrap();
            assert_eq!(s, format!("2026-09-24T{h:02}:{m:02}:00"));
        }
    }

    #[test]
    fn outlooks_no_date_is_none() {
        assert_eq!(day(Some(oa(4501, 1, 1, 0, 0))), None);
        assert_eq!(day(None), None);
        assert_eq!(day(Some(oa(2026, 10, 2, 0, 0))).as_deref(), Some("2026-10-02"));
    }

    #[test]
    fn clipping_never_splits_an_emoji() {
        let s: Vec<u16> = "ab😀cd".encode_utf16().collect(); // a b [hi lo] c d
        assert_eq!(clip(&s, 3), "ab"); // would have cut the pair
        assert_eq!(clip(&s, 4), "ab😀");
        assert_eq!(clip(&s, 99), "ab😀cd");
    }

    #[test]
    fn restrict_dates_have_no_seconds() {
        let t = NaiveDate::from_ymd_opt(2026, 9, 30).unwrap().and_hms_opt(14, 5, 37).unwrap();
        let s = restrict_date(t);
        assert!(s.contains("2026") && !s.contains(":37"), "{s}");
    }
}
