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
const NO_ANSWER: &str = "Outlook didn't respond in time — it's usually showing a dialog box (a prompt, or a message it's waiting on). Bring Outlook to the front, clear whatever box is open, then press Refresh.";

/// Outlook answers COM calls only while it has no dialog box open; a security
/// prompt or an error box it is showing would otherwise hang this forever.
const TIMEOUT: Duration = Duration::from_secs(60);

/// Ask Outlook for everything. Blocking — a second or so.
///
/// The COM work runs on its own apartment-threaded thread and we wait for it
/// with a timeout. A call into an Outlook that is showing a modal dialog
/// blocks until the dialog closes, and an in-process call cannot be killed
/// the way the old PowerShell child could, so on timeout the thread is left
/// to finish on its own once Outlook is free again, and its answer is
/// dropped.
pub fn fetch() -> Result<Dump, String> {
    #[cfg(not(windows))]
    {
        return Err("Reading Outlook only works on Windows with classic Outlook installed.".into());
    }
    #[cfg(windows)]
    {
        use std::sync::mpsc;
        use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new()
            .name("outlook-com".into())
            .spawn(move || {
                let init = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
                if init.is_err() {
                    let _ = tx.send(Err(com::Failure::Other(format!("COM would not start ({:#010x})", init.0 as u32))));
                    return;
                }
                let result = com::read(); // every COM object is released inside
                unsafe { CoUninitialize() };
                let _ = tx.send(result);
            })
            .map_err(|e| format!("Outlook did not answer: {e}"))?;
        match rx.recv_timeout(TIMEOUT) {
            Ok(Ok(dump)) => Ok(dump),
            Ok(Err(com::Failure::NotRunning)) => Err(NOT_RUNNING.into()),
            Ok(Err(com::Failure::Other(why))) => Err(format!("Outlook did not answer: {why}")),
            Err(mpsc::RecvTimeoutError::Timeout) => Err(NO_ANSWER.into()),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err("Outlook did not answer: the reader stopped unexpectedly.".into()),
        }
    }
}
