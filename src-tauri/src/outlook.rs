//! Reading the calendar and inbox out of classic Outlook.
//!
//! The school tenant will not grant a Graph API app permission to read mail,
//! so nothing here goes near Microsoft's cloud. Classic Outlook has already
//! synced everything to this machine and exposes it over COM; we attach to the
//! running Outlook and ask it for the next week of appointments, the last
//! fortnight of mail, anything flagged and any open tasks (see outlook/com.rs).
//!
//! Only classic Outlook exposes COM. The new Outlook does not, and neither does
//! a machine without Office — both come back as a plain error the UI can show.

#[cfg(windows)]
mod com;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// How far ahead to read the calendar. A week covers "what's on tomorrow" and
/// still keeps recurring lessons from flooding the dump.
pub const CALENDAR_DAYS: i64 = 7;
/// How far back to read the inbox. Homework set a fortnight ago can still be
/// due this week.
pub const MAIL_DAYS: i64 = 14;
const MAX_MAILS: usize = 150;
/// Enough of a message to find "due Friday" in, without shipping whole threads.
const MAIL_BODY_CHARS: usize = 2500;
const EVENT_BODY_CHARS: usize = 400;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub subject: String,
    /// Local time, `YYYY-MM-DDTHH:MM:SS`.
    pub start: String,
    pub end: String,
    pub all_day: bool,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mail {
    pub id: String,
    pub subject: String,
    #[serde(default)]
    pub sender: String,
    pub received: String,
    #[serde(default)]
    pub unread: bool,
    #[serde(default)]
    pub flagged: bool,
    /// Follow-up flag due date, `YYYY-MM-DD`, when the user set one.
    #[serde(default)]
    pub due: Option<String>,
    #[serde(default)]
    pub body: String,
    /// Web links in the whole message, before clipping. Teachers send none or
    /// one; newsletters send a dozen.
    #[serde(default)]
    pub links: u32,
    /// Has an unsubscribe footer.
    #[serde(default)]
    pub bulk: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub subject: String,
    #[serde(default)]
    pub due: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Dump {
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub mails: Vec<Mail>,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

/// The UI splits an error at its first ". " into a headline and a "how to fix
/// it" block, so both of these keep that shape.
const NOT_RUNNING: &str = "Classic Outlook isn't open. Open Outlook — the classic desktop app, not the new one or the web version — and let it finish signing in, then press Refresh. The planner reads your day straight from the running app, so nothing leaves your computer and it never has to start Outlook itself.";
/// When the app itself is running as administrator, Outlook is open but hidden
/// from us (an elevated process can't see a normal one's COM objects), so "isn't
/// open" would be wrong. Say what's actually going on.
const ELEVATED: &str = "The app is running as administrator, which hides Outlook from it. Use Restart normally on the Today page, then try again.";

fn not_running(normal: &str) -> String {
    if crate::elevation::is_elevated() { ELEVATED.to_string() } else { normal.to_string() }
}

const NO_ANSWER: &str = "Outlook didn't respond in time — it's usually showing a dialog box (a prompt, or a message it's waiting on). Bring Outlook to the front, clear whatever box is open, then press Refresh.";

/// Outlook answers COM calls only while it has no dialog box open; a security
/// prompt or an error box it is showing would otherwise hang this forever.
const TIMEOUT: Duration = Duration::from_secs(60);

/// Ask Outlook for everything. Blocking — a second or so.
///
/// The COM work runs on its own apartment-threaded thread and we wait for it
/// with a timeout. A call into an Outlook that is showing a modal dialog
/// blocks until the dialog closes, and an in-process call cannot be killed
/// the way the old PowerShell child could - so reads are single-flight: while
/// one is out, later callers wait on that same read instead of starting
/// another. Without that, every Refresh during a stuck dialog would park one
/// more thread behind it, and they would all fire at once when it closed.
/// Two genuinely overlapping reads (app start plus a Refresh) now share one
/// trip to Outlook as well.
pub fn fetch() -> Result<Dump, String> {
    #[cfg(not(windows))]
    {
        return Err("Reading Outlook only works on Windows with classic Outlook installed.".into());
    }
    #[cfg(windows)]
    {
        use std::sync::{Arc, Condvar, Mutex};
        use std::time::Instant;
        use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

        type Slot = Arc<(Mutex<Option<Result<Dump, String>>>, Condvar)>;
        /// The read that is out, if any, and when it left.
        static FLIGHT: Mutex<Option<(Instant, Slot)>> = Mutex::new(None);
        fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
            m.lock().unwrap_or_else(|e| e.into_inner()) // never panic: release aborts on panic
        }

        let (started, slot) = {
            let mut flight = lock(&FLIGHT);
            if let Some((t, slot)) = flight.as_ref() {
                (*t, slot.clone()) // join the read already out
            } else {
                let slot: Slot = Arc::new((Mutex::new(None), Condvar::new()));
                let mine = slot.clone();
                std::thread::Builder::new()
                    .name("outlook-com".into())
                    .spawn(move || {
                        let init = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
                        let result = if init.is_err() {
                            Err(format!("Outlook did not answer: COM would not start ({:#010x})", init.0 as u32))
                        } else {
                            let r = com::read(); // every COM object is released inside
                            unsafe { CoUninitialize() };
                            r.map_err(|f| match f {
                                com::Failure::NotRunning => not_running(NOT_RUNNING),
                                com::Failure::Other(why) => format!("Outlook did not answer: {why}"),
                            })
                        };
                        let (cell, cv) = &*mine;
                        *lock(cell) = Some(result);
                        cv.notify_all();
                        *lock(&FLIGHT) = None; // the next caller starts a fresh read
                    })
                    .map_err(|e| format!("Outlook did not answer: {e}"))?;
                let now = Instant::now();
                *flight = Some((now, slot.clone()));
                (now, slot)
            }
        };

        // Wait for the answer, but never past the read's own deadline: a
        // caller who joins a read that has been stuck for a minute is told so
        // at once rather than made to wait another minute.
        let deadline = started + TIMEOUT;
        let (cell, cv) = &*slot;
        let mut answer = lock(cell);
        loop {
            if let Some(r) = answer.as_ref() {
                return r.clone();
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(NO_ANSWER.into());
            }
            answer = cv.wait_timeout(answer, deadline - now).map(|(g, _)| g).unwrap_or_else(|e| e.into_inner().0);
        }
    }
}

/// Open one email in Outlook's own window, by its EntryID.
///
/// A click, not a read, so it doesn't share `fetch`'s single-flight: it gets
/// its own short-lived COM thread and a short wait. A Display call behind an
/// open Outlook dialog blocks just like a read, so at most one open is ever out
/// - clicking again while one hangs is told Outlook is busy instead of parking
/// another thread. The flag is cleared by a guard in the thread, so no error
/// path can leave it stuck.
pub fn open_email(entry_id: String) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = entry_id;
        return Err("Opening emails needs classic Outlook on Windows.".into());
    }
    #[cfg(windows)]
    {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::mpsc;
        use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

        static OPENING: AtomicBool = AtomicBool::new(false);
        struct Clear;
        impl Drop for Clear {
            fn drop(&mut self) {
                OPENING.store(false, Ordering::SeqCst);
            }
        }

        if OPENING.swap(true, Ordering::SeqCst) {
            return Err("Outlook is busy — clear any box that's open in Outlook, then try again.".into());
        }
        // Windows won't let a background process steal focus, so Outlook's own
        // Activate usually just flashes its taskbar button. We are the foreground
        // process at the moment of the click, so we may hand that right on.
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::{AllowSetForegroundWindow, ASFW_ANY};
            let _ = AllowSetForegroundWindow(ASFW_ANY);
        }
        let (tx, rx) = mpsc::channel();
        let spawned = std::thread::Builder::new().name("outlook-open".into()).spawn(move || {
            let _clear = Clear;
            let init = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
            let result = if init.is_err() {
                Err(com::Failure::Other(format!("COM would not start ({:#010x})", init.0 as u32)))
            } else {
                let r = com::open(&entry_id); // every COM object is released inside
                unsafe { CoUninitialize() };
                r
            };
            let _ = tx.send(result);
        });
        if let Err(e) = spawned {
            OPENING.store(false, Ordering::SeqCst);
            return Err(format!("Couldn't open the email: {e}"));
        }
        match rx.recv_timeout(Duration::from_secs(15)) {
            Ok(Ok(())) => Ok(()),
            Ok(Err(com::Failure::NotRunning)) => Err(not_running("Classic Outlook isn't open. Open it, let it finish signing in, then try again.")),
            Ok(Err(com::Failure::Other(why))) => Err(format!("Outlook couldn't open that email: {why}")),
            Err(_) => Err(NO_ANSWER.into()),
        }
    }
}
