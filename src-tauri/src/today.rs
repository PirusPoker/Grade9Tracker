//! Turning a raw Outlook dump into the day: what is on, what is due, what
//! someone has asked for.
//!
//! No model involved — school mail is formulaic enough that a handful of
//! patterns ("due Friday", "hand in by 18th September", "test tomorrow") plus
//! the follow-up flags and tasks Outlook already keeps catch nearly all of it,
//! and it runs offline in a millisecond. Anything cleverer can sit on top of
//! this output later.
//!
//! Free time between events is worked out in the UI, because "free from now"
//! moves while the app is open and this output is cached.

use crate::outlook::{Dump, Event, Mail};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Deadlines further out than this are almost always footer boilerplate
/// ("term ends 17 July") rather than something to plan around.
const HORIZON_DAYS: i64 = 90;
/// Keep a missed deadline visible for a week so it can be ticked off, then let
/// it go rather than shame forever.
const KEEP_OVERDUE_DAYS: i64 = 7;
const MAX_ACTIONS: usize = 12;
/// A message with this many links is a newsletter, whatever it says about
/// Friday. Teachers' emails have none or one.
const BULK_LINKS: u32 = 6;
const SNIPPET_CHARS: usize = 110;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Deadline {
    /// Stable across refreshes, so a tick in the UI survives.
    pub id: String,
    pub title: String,
    /// `YYYY-MM-DD`.
    pub date: String,
    /// `flag` (a follow-up flag with a date), `email` (found in the text),
    /// `calendar` (a test or hand-in on the calendar) or `task`.
    pub source: String,
    #[serde(default)]
    pub from: String,
    /// The sentence the date came from, so a wrong guess is obvious.
    #[serde(default)]
    pub snippet: String,
}

/// A request with no date on it — "please return the form", "bring your PE
/// kit" — worth a glance rather than a slot in the day.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub from: String,
    pub received: String,
    #[serde(default)]
    pub snippet: String,
    #[serde(default)]
    pub unread: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    /// Local time the dump was taken, `YYYY-MM-DDTHH:MM:SS`.
    pub fetched_at: String,
    pub events: Vec<Event>,
    pub deadlines: Vec<Deadline>,
    pub actions: Vec<Action>,
    pub mails_scanned: usize,
    /// Set when this is a stale cached copy because a fresh read failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ---------- Patterns ----------

struct Patterns {
    /// "18th September", "18 Sept 2026", "18th of September"
    day_month: Regex,
    /// "September 18", "Sept 18th, 2026"
    month_day: Regex,
    /// "18/09", "18.9.26", "18/09/2026" — UK order, day first
    numeric: Regex,
    /// "Friday", "this Friday", "next Friday", "Fri" (abbreviations capitalised
    /// only, or "we sat the test" becomes Saturday)
    weekday: Regex,
    relative: Regex,
    deadline_word: Regex,
    deadline_subject: Regex,
    action_word: Regex,
    calendar_deadline: Regex,
    skip_subject: Regex,
    reply_prefix: Regex,
    time_after: Regex,
    sentence_end: Regex,
    ws: Regex,
}

fn pats() -> &'static Patterns {
    static P: OnceLock<Patterns> = OnceLock::new();
    P.get_or_init(|| {
        let months = r"(jan|feb|mar|apr|may|jun|jul|aug|sep|sept|oct|nov|dec)[a-z]*\.?";
        Patterns {
            day_month: Regex::new(&format!(r"(?i)\b(\d{{1,2}})(?:st|nd|rd|th)?\s+(?:of\s+)?{months}(?:,?\s+(\d{{4}}))?\b")).unwrap(),
            month_day: Regex::new(&format!(r"(?i)\b{months}\s+(\d{{1,2}})(?:st|nd|rd|th)?\b(?:,?\s+(\d{{4}}))?")).unwrap(),
            numeric: Regex::new(r"\b(\d{1,2})[/.](\d{1,2})(?:[/.](\d{2}|\d{4}))?\b").unwrap(),
            weekday: Regex::new(r"(?:\b(?i:(next|this))\s+)?\b(?:(?i:(monday|tuesday|wednesday|thursday|friday|saturday|sunday))|(Mon|Tues?|Wed|Thur?s?|Fri|Sat|Sun))\b").unwrap(),
            relative: Regex::new(r"(?i)\b(tomorrow|today|tonight|end of (?:the |this )?week)\b").unwrap(),
            deadline_word: Regex::new(r"(?i)\b(due|deadline|by|before|until|submit|submitted|submission|hand(?:ed)?\s*in|return|bring|complete|completed|finish|test|exam|assessment|quiz|mock|homework|assignment|coursework|closes?|expires?)\b").unwrap(),
            deadline_subject: Regex::new(r"(?i)\b(due|deadline|homework|assignment|coursework|test|exam|assessment|quiz|mock|hand\s*in|submit|reminder|overdue)\b").unwrap(),
            action_word: Regex::new(r"(?i)\b(please (?:reply|respond|confirm|sign|return|bring|complete|fill|let me know|ensure|make sure|remember|can you|could you)|rsvp|reminder|remember to|don'?t forget|permission|consent|action required|overdue|outstanding|missing work|detention|bring your|you (?:need|must|should) (?:to )?)\b").unwrap(),
            calendar_deadline: Regex::new(r"(?i)\b(test|exam|assessment|mock|deadline|due|hand\s*-?\s*in|submission|quiz|controlled assessment)\b").unwrap(),
            skip_subject: Regex::new(r"(?i)^\s*(accepted|declined|tentative|canceled|cancelled|automatic reply|out of office|undeliverable)\b").unwrap(),
            reply_prefix: Regex::new(r"(?i)^\s*(?:(?:re|fw|fwd|aw|wg)\s*:\s*)+").unwrap(),
            time_after: Regex::new(r"(?i)^\s*(?:am|pm|:\d)").unwrap(),
            sentence_end: Regex::new(r"[.!?]\s|\n").unwrap(),
            ws: Regex::new(r"[\s\u{200B}\u{200C}\u{200D}\u{FEFF}]+").unwrap(),
        }
    })
}

fn month_no(s: &str) -> Option<u32> {
    let m = s.to_ascii_lowercase();
    Some(match &m[..3] {
        "jan" => 1, "feb" => 2, "mar" => 3, "apr" => 4, "may" => 5, "jun" => 6,
        "jul" => 7, "aug" => 8, "sep" => 9, "oct" => 10, "nov" => 11, "dec" => 12,
        _ => return None,
    })
}

fn weekday_no(s: &str) -> Option<Weekday> {
    Some(match &s.to_ascii_lowercase()[..3] {
        "mon" => Weekday::Mon, "tue" => Weekday::Tue, "wed" => Weekday::Wed, "thu" => Weekday::Thu,
        "fri" => Weekday::Fri, "sat" => Weekday::Sat, "sun" => Weekday::Sun,
        _ => return None,
    })
}

/// A date with no year is this year — unless that puts it well behind the
/// message, in which case it was a December email talking about January.
fn with_year(day: u32, month: u32, year: Option<u32>, received: NaiveDate) -> Option<NaiveDate> {
    let y = match year {
        Some(y) if y < 100 => 2000 + y as i32,
        Some(y) => y as i32,
        None => received.year(),
    };
    let d = NaiveDate::from_ymd_opt(y, month, day)?;
    if year.is_none() && d < received - Duration::days(30) {
        NaiveDate::from_ymd_opt(y + 1, month, day)
    } else {
        Some(d)
    }
}

/// The next occurrence of a weekday on or after `from`. "next Friday" means
/// the one in the following week, as it usually does in Britain.
fn next_weekday(from: NaiveDate, wd: Weekday, next: bool) -> NaiveDate {
    let ahead = (wd.num_days_from_monday() as i64 + 7 - from.weekday().num_days_from_monday() as i64) % 7;
    let mut d = from + Duration::days(ahead);
    if next {
        let end_of_week = from + Duration::days(6 - from.weekday().num_days_from_monday() as i64);
        if d <= end_of_week {
            d += Duration::days(7);
        }
    }
    d
}

struct Found {
    date: NaiveDate,
    /// Byte offset of the match, for the snippet and the keyword check.
    at: usize,
    end: usize,
}

/// Every date mentioned in the text, relative to the day it was received.
fn find_dates(text: &str, received: NaiveDate) -> Vec<Found> {
    let p = pats();
    let mut out = Vec::new();
    for c in p.day_month.captures_iter(text) {
        let m = c.get(0).unwrap();
        let day: u32 = c[1].parse().unwrap_or(0);
        if let (Some(month), true) = (month_no(&c[2]), (1..=31).contains(&day)) {
            let year = c.get(3).and_then(|y| y.as_str().parse().ok());
            if let Some(date) = with_year(day, month, year, received) {
                out.push(Found { date, at: m.start(), end: m.end() });
            }
        }
    }
    for c in p.month_day.captures_iter(text) {
        let m = c.get(0).unwrap();
        if out.iter().any(|f| f.at < m.end() && m.start() < f.end) {
            continue; // "18 Sept" already took it; "Sept 18" can't overlap it
        }
        let day: u32 = c[2].parse().unwrap_or(0);
        if let (Some(month), true) = (month_no(&c[1]), (1..=31).contains(&day)) {
            let year = c.get(3).and_then(|y| y.as_str().parse().ok());
            if let Some(date) = with_year(day, month, year, received) {
                out.push(Found { date, at: m.start(), end: m.end() });
            }
        }
    }
    for c in p.numeric.captures_iter(text) {
        let m = c.get(0).unwrap();
        // "9.30am" and "10:15" are times, not dates
        if p.time_after.is_match(&text[m.end()..]) {
            continue;
        }
        let day: u32 = c[1].parse().unwrap_or(0);
        let month: u32 = c[2].parse().unwrap_or(0);
        let year = c.get(3).and_then(|y| y.as_str().parse().ok());
        if (1..=31).contains(&day) && (1..=12).contains(&month) {
            if let Some(date) = with_year(day, month, year, received) {
                out.push(Found { date, at: m.start(), end: m.end() });
            }
        }
    }
    for c in p.weekday.captures_iter(text) {
        let m = c.get(0).unwrap();
        // "Wednesday 23rd September": the weekday is part of the date, not a
        // second one
        let rest = &text[m.end()..];
        let next_at = m.end() + (rest.len() - rest.trim_start_matches([' ', ',']).len());
        if out.iter().any(|f| f.at == next_at) {
            continue;
        }
        let name = c.get(2).or_else(|| c.get(3)).map(|x| x.as_str()).unwrap_or("");
        if let Some(wd) = weekday_no(name) {
            let next = c.get(1).is_some_and(|x| x.as_str().eq_ignore_ascii_case("next"));
            out.push(Found { date: next_weekday(received, wd, next), at: m.start(), end: m.end() });
        }
    }
    for c in p.relative.captures_iter(text) {
        let m = c.get(0).unwrap();
        let word = c[1].to_ascii_lowercase();
        let date = if word == "tomorrow" {
            received + Duration::days(1)
        } else if word.starts_with("end of") {
            let fri = next_weekday(received, Weekday::Fri, false);
            if received.weekday() == Weekday::Sat || received.weekday() == Weekday::Sun { fri + Duration::days(7) } else { fri }
        } else {
            received
        };
        out.push(Found { date, at: m.start(), end: m.end() });
    }
    out
}

fn snippet(text: &str, at: usize, end: usize) -> String {
    let sent = |c: char| matches!(c, '.' | '!' | '?');
    let half = SNIPPET_CHARS / 2;
    let mut s = at.saturating_sub(half);
    let mut e = (end + half).min(text.len());
    while s > 0 && !text.is_char_boundary(s) { s -= 1; }
    while e < text.len() && !text.is_char_boundary(e) { e += 1; }
    // Start on a word boundary, so it never opens mid-word ("…er that" → "…that").
    if s > 0 {
        if let Some(sp) = text[s..at].find(' ') { s += sp + 1; }
    }
    // End at the sentence's full stop if one is near, otherwise a word boundary —
    // so it doesn't trail into the next sentence or cut a word in half.
    if e < text.len() {
        if let Some(dot) = text[end..e].find(sent) { e = end + dot + 1; }
        else if let Some(sp) = text[end..e].rfind(' ') { e = end + sp; }
    }
    let lead = s > 0;
    let trail = e < text.len() && !text[..e].trim_end().ends_with(sent);
    let mut out = pats().ws.replace_all(text[s..e].trim(), " ").into_owned();
    if lead { out.insert(0, '…'); }
    if trail { out.push('…'); }
    out
}

/// Whether a deadline-ish word sits just before the date, in the same
/// sentence ("due on Friday", "hand in by 18th"). A date with no such word
/// nearby is probably a trip or a meeting, not something to do.
fn has_deadline_word_before(text: &str, at: usize) -> bool {
    let mut s = at.saturating_sub(60);
    while s > 0 && !text.is_char_boundary(s) { s -= 1; }
    let window = &text[s..at];
    let from = pats().sentence_end.find_iter(window).last().map(|m| m.end()).unwrap_or(0);
    pats().deadline_word.is_match(&window[from..])
}

fn is_bulk(m: &Mail) -> bool {
    m.bulk || m.links >= BULK_LINKS
}

fn clean_subject(s: &str) -> String {
    let t = pats().reply_prefix.replace(s, "").trim().to_string();
    if t.is_empty() { "(no subject)".into() } else { t }
}

fn date_of(stamp: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(stamp.get(..10)?, "%Y-%m-%d").ok()
}

/// The single most likely deadline in a message, if any.
fn mail_deadline(m: &Mail, today: NaiveDate) -> Option<(NaiveDate, String)> {
    let received = date_of(&m.received).unwrap_or(today);
    let text = format!("{}\n{}", m.subject, m.body);
    let subject_asks = pats().deadline_subject.is_match(&m.subject);
    let earliest = received - Duration::days(1);
    let latest = received + Duration::days(HORIZON_DAYS);
    let mut best: Option<(bool, NaiveDate, usize, usize)> = None;
    for f in find_dates(&text, received) {
        if f.date < earliest || f.date > latest {
            continue;
        }
        let strong = has_deadline_word_before(&text, f.at);
        if !strong && !subject_asks {
            continue;
        }
        // strong beats weak; among equals, the soonest date wins
        let better = match best {
            None => true,
            Some((bs, bd, _, _)) => (strong && !bs) || (strong == bs && f.date < bd),
        };
        if better {
            best = Some((strong, f.date, f.at, f.end));
        }
    }
    best.map(|(_, date, at, end)| (date, snippet(&text, at, end)))
}

fn key(s: &str) -> String {
    s.to_ascii_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
}

/// Build the day from a dump. `today` is passed in so this stays testable.
pub fn build(dump: &Dump, today: NaiveDate, fetched_at: String) -> Context {
    let p = pats();
    let oldest = today - Duration::days(KEEP_OVERDUE_DAYS);
    let mut deadlines: Vec<Deadline> = Vec::new();
    let mut actions: Vec<Action> = Vec::new();
    let mut seen: Vec<String> = Vec::new();

    let mut push = |d: Deadline, seen: &mut Vec<String>| {
        let k = format!("{}|{}", key(&d.title), d.date);
        if date_of(&d.date).is_some_and(|x| x >= oldest) && !seen.contains(&k) {
            seen.push(k);
            deadlines.push(d);
        }
    };

    // Newest mail first, so a chased-up reminder wins over the original.
    let mut mails: Vec<&Mail> = dump.mails.iter().filter(|m| !p.skip_subject.is_match(&m.subject) && !is_bulk(m)).collect();
    mails.sort_by(|a, b| b.received.cmp(&a.received));
    for m in &mails {
        let title = clean_subject(&m.subject);
        if let Some(due) = m.due.as_deref().and_then(date_of) {
            push(Deadline { id: format!("flag:{}", m.id), title: title.clone(), date: due.to_string(), source: "flag".into(), from: m.sender.clone(), snippet: String::new() }, &mut seen);
            continue;
        }
        if let Some((date, snip)) = mail_deadline(m, today) {
            push(Deadline { id: format!("mail:{}", m.id), title: title.clone(), date: date.to_string(), source: "email".into(), from: m.sender.clone(), snippet: snip }, &mut seen);
            continue;
        }
        let text = format!("{}\n{}", m.subject, m.body);
        let recent = date_of(&m.received).is_some_and(|d| d >= today - Duration::days(7));
        if (m.unread || m.flagged || recent) && actions.len() < MAX_ACTIONS {
            if let Some(hit) = p.action_word.find(&text) {
                // a request in the subject line: quote the start of the body
                // rather than repeating the subject
                let snip = if hit.start() < m.subject.len() { snippet(&m.body, 0, 0) } else { snippet(&text, hit.start(), hit.end()) };
                actions.push(Action { id: format!("mail:{}", m.id), title, from: m.sender.clone(), received: m.received.clone(), snippet: snip, unread: m.unread });
            }
        }
    }

    for e in &dump.events {
        if p.calendar_deadline.is_match(&e.subject) {
            if let Some(d) = date_of(&e.start) {
                push(Deadline { id: format!("cal:{}", e.id), title: e.subject.clone(), date: d.to_string(), source: "calendar".into(), from: e.location.clone(), snippet: String::new() }, &mut seen);
            }
        }
    }

    for t in &dump.tasks {
        match t.due.as_deref().and_then(date_of) {
            Some(d) => push(Deadline { id: format!("task:{}", t.id), title: t.subject.clone(), date: d.to_string(), source: "task".into(), from: "Outlook task".into(), snippet: String::new() }, &mut seen),
            None if actions.len() < MAX_ACTIONS => actions.push(Action { id: format!("task:{}", t.id), title: t.subject.clone(), from: "Outlook task".into(), received: String::new(), snippet: String::new(), unread: false }),
            None => {}
        }
    }

    deadlines.sort_by(|a, b| a.date.cmp(&b.date).then_with(|| a.title.cmp(&b.title)));

    // The timeline needs times and places, not bodies.
    let mut events: Vec<Event> = dump.events.iter().map(|e| Event { body: String::new(), ..e.clone() }).collect();
    events.sort_by(|a, b| a.start.cmp(&b.start));

    Context { fetched_at, events, deadlines, actions, mails_scanned: dump.mails.len(), error: None }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }
    // A Monday.
    const RECV: &str = "2026-09-14";

    fn dates(text: &str) -> Vec<NaiveDate> {
        find_dates(text, d(RECV)).into_iter().map(|f| f.date).collect()
    }

    #[test]
    fn written_dates() {
        assert_eq!(dates("hand in by 18th September please"), vec![d("2026-09-18")]);
        assert_eq!(dates("due 2 Oct"), vec![d("2026-10-02")]);
        assert_eq!(dates("due on the 3rd of November 2026"), vec![d("2026-11-03")]);
        assert_eq!(dates("Test: October 5th"), vec![d("2026-10-05")]);
        assert_eq!(dates("deadline 25/09"), vec![d("2026-09-25")]);
        assert_eq!(dates("deadline 25/09/26"), vec![d("2026-09-25")]);
    }

    #[test]
    fn times_are_not_dates() {
        assert!(dates("meet at 9.30am in the hall").is_empty());
        assert!(dates("starts 10:15").is_empty());
        assert!(dates("May 2026 newsletter").is_empty());
    }

    #[test]
    fn weekdays_count_forward_from_receipt() {
        assert_eq!(dates("due Friday"), vec![d("2026-09-18")]);
        assert_eq!(dates("due Monday"), vec![d("2026-09-14")]); // same day
        assert_eq!(dates("due next Friday"), vec![d("2026-09-25")]);
        assert_eq!(dates("due Fri"), vec![d("2026-09-18")]);
        assert!(dates("we sat the mock and it went fine").is_empty());
    }

    #[test]
    fn relative_words() {
        assert_eq!(dates("test tomorrow"), vec![d("2026-09-15")]);
        assert_eq!(dates("bring it in today"), vec![d("2026-09-14")]);
        assert_eq!(dates("by the end of the week"), vec![d("2026-09-18")]);
    }

    #[test]
    fn december_mail_about_january_rolls_the_year() {
        let found = find_dates("due 5 Jan", d("2026-12-15"));
        assert_eq!(found[0].date, d("2027-01-05"));
    }

    fn mail(subject: &str, body: &str) -> Mail {
        Mail { id: "x".into(), subject: subject.into(), sender: "Mr Jones".into(), received: format!("{RECV}T08:00:00"), body: body.into(), ..Default::default() }
    }

    #[test]
    fn picks_the_date_with_a_deadline_word_before_it() {
        let m = mail("Physics", "We did this on Monday. Please complete questions 1-10, due Thursday.");
        let (date, snip) = mail_deadline(&m, d(RECV)).unwrap();
        assert_eq!(date, d("2026-09-17"));
        assert!(snip.contains("due Thursday"));
    }

    #[test]
    fn a_trip_date_is_not_a_deadline() {
        let m = mail("Year 10 theatre trip", "The coach leaves at 8am on Wednesday 23rd September.");
        assert!(mail_deadline(&m, d(RECV)).is_none());
    }

    #[test]
    fn the_deadline_word_must_be_in_the_same_sentence() {
        let m = mail("Maths", "This is a hard deadline. See you all on Friday.");
        assert!(mail_deadline(&m, d(RECV)).is_none());
        let m = mail("Maths", "Hand in by Friday. See you then.");
        assert_eq!(mail_deadline(&m, d(RECV)).unwrap().0, d("2026-09-18"));
    }

    #[test]
    fn snippets_are_clean_and_do_not_repeat_the_subject() {
        let dump = Dump { mails: vec![mail("Please fill in the form", "Dear all,\u{200B}\u{200B} the form is on the portal. Mr X")], ..Default::default() };
        let c = build(&dump, d(RECV), "now".into());
        assert_eq!(c.actions[0].snippet, "Dear all, the form is on the portal. Mr X");
    }

    #[test]
    fn newsletters_are_ignored_whatever_they_say() {
        let dump = Dump { mails: vec![
            Mail { links: 12, ..mail("5 things before Monday", "Deadline: reply by Friday!") },
            Mail { bulk: true, ..mail("Offers", "Reminder: sale ends Friday") },
        ], ..Default::default() };
        let c = build(&dump, d(RECV), "now".into());
        assert!(c.deadlines.is_empty() && c.actions.is_empty());
    }

    #[test]
    fn subject_can_vouch_for_a_bare_date() {
        let m = mail("Homework", "Questions 4-9 for Wednesday.");
        assert_eq!(mail_deadline(&m, d(RECV)).unwrap().0, d("2026-09-16"));
    }

    /// Reads your actual Outlook and prints what the rules made of it:
    /// `cargo test live_outlook -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn live_outlook() {
        let dump = crate::outlook::fetch().expect("Outlook");
        let today = chrono::Local::now().date_naive();
        let c = build(&dump, today, chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string());
        std::fs::write(std::env::temp_dir().join("g9-day.json"), serde_json::to_string(&c).unwrap()).unwrap();
        println!("{} events, {} mails scanned, {} tasks", c.events.len(), c.mails_scanned, dump.tasks.len());
        println!("
DEADLINES");
        for d in &c.deadlines {
            println!("  {} [{}] {} — {}
      {}", d.date, d.source, d.title, d.from, d.snippet);
        }
        println!("
ACTIONS");
        for a in &c.actions {
            println!("  {} {} — {}
      {}", &a.received[..10.min(a.received.len())], a.title, a.from, a.snippet);
        }
        println!("
SKIPPED (no deadline or action found)");
        let used: Vec<&str> = c.deadlines.iter().map(|d| d.id.as_str()).chain(c.actions.iter().map(|a| a.id.as_str())).collect();
        for m in &dump.mails {
            if !used.iter().any(|u| u.ends_with(&m.id)) {
                println!("  {} {} — {}", &m.received[..10], m.subject, m.sender);
            }
        }
    }

    #[test]
    fn build_dedupes_flags_reminders_and_calendar() {
        let dump = Dump {
            events: vec![Event { id: "e1".into(), subject: "Maths test".into(), start: "2026-09-18T09:00:00".into(), end: "2026-09-18T10:00:00".into(), all_day: false, location: "M4".into(), body: "long".into() }],
            mails: vec![
                mail("Maths homework", "Due Friday."),
                Mail { received: "2026-09-13T08:00:00".into(), ..mail("RE: Maths homework", "Reminder: due Friday 18th September.") },
                Mail { due: Some("2026-09-20".into()), ..mail("Sign the trip form", "Attached.") },
                mail("PE kit", "Please remember to bring your PE kit this term."),
                mail("Accepted: Parents evening", "due Friday"),
            ],
            tasks: vec![],
        };
        let c = build(&dump, d(RECV), "now".into());
        let titles: Vec<_> = c.deadlines.iter().map(|x| (x.title.as_str(), x.date.as_str(), x.source.as_str())).collect();
        assert_eq!(titles, vec![
            ("Maths homework", "2026-09-18", "email"),
            ("Maths test", "2026-09-18", "calendar"),
            ("Sign the trip form", "2026-09-20", "flag"),
        ]);
        assert_eq!(c.actions.len(), 1);
        assert_eq!(c.actions[0].title, "PE kit");
        assert!(c.events[0].body.is_empty());
    }
}
