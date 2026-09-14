//! Reading the calendar and inbox out of classic Outlook.
//!
//! The school tenant will not grant a Graph API app permission to read mail,
//! so nothing here goes near Microsoft's cloud. Classic Outlook has already
//! synced everything to this machine and exposes it over COM; a short
//! PowerShell script (run hidden, script fed over stdin so there is nothing to
//! quote) asks Outlook for the next week of appointments, the last fortnight of
//! mail, anything flagged and any open tasks, and prints them as one JSON blob.
//!
//! Only classic Outlook exposes COM. The new Outlook does not, and neither does
//! a machine without Office — both come back as a plain error the UI can show.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

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

/// The PowerShell that does the talking. Placeholders are filled in by
/// [`script`] so the limits live in one place, up top.
const SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
try { $ol = New-Object -ComObject Outlook.Application } catch { Write-Error 'NO_OUTLOOK'; exit 2 }
$ns = $ol.GetNamespace('MAPI')
function Clip([string]$s, [int]$n) { if ($null -eq $s) { return '' }; if ($s.Length -gt $n) { return $s.Substring(0, $n) }; return $s }
function Day([object]$d) { if ($null -eq $d -or $d.Year -ge 4000) { return $null }; return $d.ToString('yyyy-MM-dd') }

# Calendar. Sort, then IncludeRecurrences, then Restrict — Outlook insists on
# that order, and iterating recurrences without a Restrict never ends.
$events = New-Object System.Collections.Generic.List[object]
$from = (Get-Date).Date
$to = $from.AddDays(__CAL_DAYS__)
$cal = $ns.GetDefaultFolder(9).Items
$cal.Sort('[Start]')
$cal.IncludeRecurrences = $true
$cf = "[Start] < '" + $to.ToString('g') + "' AND [End] > '" + $from.ToString('g') + "'"
foreach ($e in $cal.Restrict($cf)) {
  try {
    $events.Add([pscustomobject]@{
      id = $e.EntryID + '|' + $e.Start.ToString('s'); subject = '' + $e.Subject
      start = $e.Start.ToString('s'); end = $e.End.ToString('s'); allDay = [bool]$e.AllDayEvent
      location = '' + $e.Location; body = Clip $e.Body __EVENT_BODY__ })
  } catch {}
  if ($events.Count -ge 300) { break }
}

# Inbox, newest first. Class 43 is a plain mail item.
$mails = New-Object System.Collections.Generic.List[object]
$inbox = $ns.GetDefaultFolder(6).Items
$inbox.Sort('[ReceivedTime]', $true)
$since = (Get-Date).AddDays(-__MAIL_DAYS__)
$mf = "[ReceivedTime] >= '" + $since.ToString('g') + "'"
foreach ($m in $inbox.Restrict($mf)) {
  if ($m.Class -ne 43) { continue }
  try {
    $due = $null; try { $due = Day $m.TaskDueDate } catch {}
    $full = ''; try { $full = '' + $m.Body } catch {}
    $links = ([regex]::Matches($full, 'https?://')).Count
    $bulk = [bool]($full -match '(?i)unsubscribe|manage (your )?preferences|email preferences')
    $mails.Add([pscustomobject]@{
      id = $m.EntryID; subject = '' + $m.Subject; sender = '' + $m.SenderName
      received = $m.ReceivedTime.ToString('s'); unread = [bool]$m.UnRead
      flagged = ($m.FlagStatus -eq 2); due = $due; body = (Clip $full __MAIL_BODY__)
      links = $links; bulk = $bulk })
  } catch {}
  if ($mails.Count -ge __MAX_MAILS__) { break }
}

# Open tasks, if the folder exists.
$tasks = New-Object System.Collections.Generic.List[object]
try {
  foreach ($t in $ns.GetDefaultFolder(13).Items.Restrict('[Complete] = False')) {
    try { $tasks.Add([pscustomobject]@{ id = $t.EntryID; subject = '' + $t.Subject; due = Day $t.DueDate }) } catch {}
  }
} catch {}

[pscustomobject]@{ events = $events; mails = $mails; tasks = $tasks } | ConvertTo-Json -Depth 4 -Compress
"#;

fn script() -> String {
    SCRIPT
        .replace("__CAL_DAYS__", &CALENDAR_DAYS.to_string())
        .replace("__MAIL_DAYS__", &MAIL_DAYS.to_string())
        .replace("__MAX_MAILS__", &MAX_MAILS.to_string())
        .replace("__MAIL_BODY__", &MAIL_BODY_CHARS.to_string())
        .replace("__EVENT_BODY__", &EVENT_BODY_CHARS.to_string())
}

/// Windows PowerShell 5.1 serialises a one-item list as a bare object rather
/// than a one-element array. Accept either.
fn as_list(v: Option<&Value>) -> Vec<Value> {
    match v {
        Some(Value::Array(a)) => a.clone(),
        Some(Value::Object(_)) => vec![v.unwrap().clone()],
        _ => Vec::new(),
    }
}

fn parse_dump(json: &str) -> Result<Dump, String> {
    let v: Value = serde_json::from_str(json.trim()).map_err(|e| format!("Outlook returned something that was not JSON: {e}"))?;
    fn take<T: serde::de::DeserializeOwned>(v: &Value, key: &str) -> Vec<T> {
        as_list(v.get(key)).into_iter().filter_map(|x| serde_json::from_value(x).ok()).collect()
    }
    Ok(Dump { events: take(&v, "events"), mails: take(&v, "mails"), tasks: take(&v, "tasks") })
}

/// Outlook answers COM calls only while it has no dialog box open; a security
/// prompt or an error box it is showing would otherwise hang this forever.
const TIMEOUT: Duration = Duration::from_secs(60);

/// Ask Outlook for everything. Blocking — takes a second or two, longer if
/// Outlook itself has to start up in the background.
pub fn fetch() -> Result<Dump, String> {
    #[cfg(not(windows))]
    {
        return Err("Reading Outlook only works on Windows with classic Outlook installed.".into());
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let mut child = Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", "-"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Could not start PowerShell: {e}"))?;
        child.stdin.take().ok_or("no stdin")?.write_all(script().as_bytes()).map_err(|e| e.to_string())?;
        // Drain both pipes on threads: the dump is bigger than a pipe buffer,
        // so reading it only after exit would deadlock.
        fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> std::thread::JoinHandle<Vec<u8>> {
            std::thread::spawn(move || {
                let mut buf = Vec::new();
                if let Some(mut p) = pipe { let _ = p.read_to_end(&mut buf); }
                buf
            })
        }
        let out_t = drain(child.stdout.take());
        let err_t = drain(child.stderr.take());
        let started = Instant::now();
        let status = loop {
            if let Some(s) = child.try_wait().map_err(|e| e.to_string())? {
                break s;
            }
            if started.elapsed() > TIMEOUT {
                let _ = child.kill();
                let _ = child.wait();
                return Err("Outlook did not answer within a minute. It is probably showing a box saying \"A program is trying to access email address information\" — that is this app reading your inbox. Choose Allow, or stop it asking: File → Options → Trust Center → Trust Center Settings → Programmatic Access → Never warn me.".into());
            }
            std::thread::sleep(Duration::from_millis(100));
        };
        let stdout = String::from_utf8_lossy(&out_t.join().unwrap_or_default()).into_owned();
        let stderr = String::from_utf8_lossy(&err_t.join().unwrap_or_default()).into_owned();
        if stderr.contains("NO_OUTLOOK") || stderr.contains("80040154") {
            return Err("Classic Outlook is not installed, so there is nothing to read. (The new Outlook does not let other programs see your mail.)".into());
        }
        if !status.success() && stdout.trim().is_empty() {
            let first = stderr.lines().find(|l| !l.trim().is_empty()).unwrap_or("unknown error");
            return Err(format!("Outlook did not answer: {}", first.trim()));
        }
        parse_dump(&stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_item_lists_are_still_lists() {
        let d = parse_dump(r#"{"events":{"id":"a","subject":"Maths","start":"2026-09-14T09:00:00","end":"2026-09-14T10:00:00","allDay":false},"mails":[],"tasks":null}"#).unwrap();
        assert_eq!(d.events.len(), 1);
        assert!(d.mails.is_empty() && d.tasks.is_empty());
    }

    #[test]
    fn script_has_no_placeholders_left() {
        assert!(!script().contains("__"));
    }
}
