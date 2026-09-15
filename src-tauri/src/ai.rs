//! Optional AI pass over the day, run by a small local model through Ollama.
//!
//! The rules in `today.rs` do the reliable, free extraction: which emails
//! matter, their dates, their senders. This layer sits on top only to *word*
//! them nicely and *triage* them into "for today" and "later".
//!
//! Crucially, the split into now/later is pure date arithmetic done here in
//! code — a small model is not trusted to compare dates, and it never touches
//! placement. All the model does is write a one-line summary of each item; the
//! list is built straight from the real items, so the model can neither invent
//! a task nor lose one. If the model is off or unreachable, the plain titles
//! stand and the triage still works.
//!
//! The model runs locally, so nothing leaves the machine, and `keep_alive` is
//! short so Ollama frees the VRAM straight after a refresh — it never competes
//! with a game.

use crate::today::Context;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::Duration as StdDuration;

/// A dated item is "for today" only up to this many days ahead; further off it
/// waits in "later" and resurfaces on a refresh as the date nears. 1 = due
/// today or tomorrow (and anything overdue) counts as now.
const NOW_WINDOW_DAYS: i64 = 1;
/// An undated request ("bring your kit", "return the form") is "for today"
/// while it is this fresh; after that it becomes an older nudge that can wait.
const FRESH_DAYS: i64 = 2;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AiItem {
    /// The id of the underlying deadline/action, so ticking it off still works
    /// and nothing is invented.
    pub id: String,
    pub line: String,
    /// Due date `YYYY-MM-DD` when the item has one, else empty (an undated
    /// request). The UI turns it into a "Today / Tomorrow / …" label.
    #[serde(default)]
    pub when: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiPlan {
    /// Things that need you today — due now or very soon, or a fresh request.
    pub now: Vec<AiItem>,
    /// Things to be aware of but not act on yet — a deadline still days off, or
    /// an older request you have already seen.
    pub later: Vec<AiItem>,
}

const SYSTEM: &str = r#"You write a school student's to-do list.
For each item you are given (id, from, text), write `line`: a short instruction of about 6 words, action first, in your own words. Do NOT copy the email text, and do NOT put a date in it.
Example item: {"id":"x","text":"History essay on the Cold War, submit on Teams by Thursday"}
Example reply: {"id":"x","line":"Finish History Cold War essay"}
Reply with ONLY this JSON: {"items":[{"id":"<id>","line":"<summary>"}]}. Include every id exactly once."#;

/// Triage the day into now/later and, best-effort, word each item with the
/// local model. Never errors on the model's account: if it is off or slow, the
/// items come back with their plain titles and the triage is unchanged.
pub async fn organise(ctx: &Context, today: NaiveDate, base_url: &str, model: &str) -> Result<AiPlan, String> {
    if ctx.deadlines.is_empty() && ctx.actions.is_empty() {
        return Ok(AiPlan::default());
    }
    // Model failure is not our failure — fall back to titles.
    let lines = summarise(ctx, base_url, model).await.unwrap_or_default();
    Ok(build_plan(ctx, today, &lines))
}

/// The instant, model-free triage: the same now/later split as [`organise`],
/// but every line is its plain title. `day_context` returns this straight away
/// so the day is never held up; the UI then calls [`organise`] in the
/// background to upgrade the wording. The dates — and so the split itself —
/// are identical either way, because they are decided here, not by the model.
pub fn split(ctx: &Context, today: NaiveDate) -> AiPlan {
    build_plan(ctx, today, &BTreeMap::new())
}

fn join(title: &str, extra: &str) -> String {
    if extra.trim().is_empty() { title.to_string() } else { format!("{title} — {extra}") }
}

fn tidy(s: &str) -> String {
    s.trim().trim_end_matches('.').trim().to_string()
}

/// The model's summary for an id if it gave a usable one, else the plain title.
fn line_for(id: &str, title: &str, lines: &BTreeMap<String, String>) -> String {
    match lines.get(id).map(|s| tidy(s)) {
        Some(t) if !t.is_empty() => t,
        _ => title.to_string(),
    }
}

/// Build the two buckets from the real items. This owns every date decision;
/// `lines` only supplies wording. Every deadline and action appears exactly
/// once, so nothing can be dropped or invented.
fn build_plan(ctx: &Context, today: NaiveDate, lines: &BTreeMap<String, String>) -> AiPlan {
    // Undated items sort after dated ones within a bucket.
    let far = NaiveDate::MAX;
    let mut now: Vec<(NaiveDate, AiItem)> = Vec::new();
    let mut later: Vec<(NaiveDate, AiItem)> = Vec::new();

    for d in &ctx.deadlines {
        let due = NaiveDate::parse_from_str(&d.date, "%Y-%m-%d").ok();
        let is_now = due.is_none_or(|dt| dt <= today + Duration::days(NOW_WINDOW_DAYS));
        let item = AiItem { id: d.id.clone(), line: line_for(&d.id, &d.title, lines), when: d.date.clone() };
        let bucket = if is_now { &mut now } else { &mut later };
        bucket.push((due.unwrap_or(far), item));
    }

    for a in &ctx.actions {
        let recv = a.received.get(..10).and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
        let is_now = a.unread || recv.is_some_and(|r| r >= today - Duration::days(FRESH_DAYS));
        let item = AiItem { id: a.id.clone(), line: line_for(&a.id, &a.title, lines), when: String::new() };
        let bucket = if is_now { &mut now } else { &mut later };
        bucket.push((far, item));
    }

    // Stable sort by date keeps today.rs's title order for same-day ties.
    now.sort_by(|x, y| x.0.cmp(&y.0));
    later.sort_by(|x, y| x.0.cmp(&y.0));
    AiPlan {
        now: now.into_iter().map(|(_, i)| i).collect(),
        later: later.into_iter().map(|(_, i)| i).collect(),
    }
}

/// Ask the local model for a one-line summary of each item. Returns a map of
/// id -> summary; an `Err` (model off, timed out, bad JSON) is handled by the
/// caller falling back to titles.
async fn summarise(ctx: &Context, base_url: &str, model: &str) -> Result<BTreeMap<String, String>, String> {
    // The real ids are 100-character Outlook EntryIDs; a 3B model cannot echo
    // one back verbatim, so it would tag every summary with a mangled id and
    // none would match. Hand it a short index instead and map it back here.
    let mut items: Vec<Value> = Vec::new();
    let mut real: Vec<&str> = Vec::new();
    for d in &ctx.deadlines {
        items.push(json!({ "id": real.len().to_string(), "from": d.from, "text": join(&d.title, &d.snippet) }));
        real.push(&d.id);
    }
    for a in &ctx.actions {
        items.push(json!({ "id": real.len().to_string(), "from": a.from, "text": join(&a.title, &a.snippet) }));
        real.push(&a.id);
    }
    let user = json!({ "items": items });
    let body = json!({
        "model": model,
        "stream": false,
        "format": "json",
        "keep_alive": "20s",
        "options": { "temperature": 0.1 },
        "messages": [
            { "role": "system", "content": SYSTEM },
            { "role": "user", "content": user.to_string() },
        ],
    });

    let url = format!("{}/api/chat", base_url.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(StdDuration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("status {}", resp.status()));
    }
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    let content = v.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()).unwrap_or("");
    let parsed: Value = serde_json::from_str(content.trim()).map_err(|e| e.to_string())?;
    Ok(remap(harvest(&parsed), &real))
}

/// Turn the model's short index ids ("0", "1", …) back into the real ids, so
/// `build_plan` can attach each summary to its item. Anything that isn't a
/// valid index is dropped (the item then keeps its plain title).
fn remap(short: BTreeMap<String, String>, real: &[&str]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for (k, line) in short {
        if let Some(id) = k.parse::<usize>().ok().and_then(|i| real.get(i)) {
            out.insert((*id).to_string(), line);
        }
    }
    out
}

/// Pull `{id, line}` pairs out of the model's reply, tolerating either an
/// `items` array or a bare top-level array.
fn harvest(parsed: &Value) -> BTreeMap<String, String> {
    let arr = parsed.get("items").and_then(|i| i.as_array()).or_else(|| parsed.as_array());
    let mut out = BTreeMap::new();
    if let Some(arr) = arr {
        for it in arr {
            if let (Some(id), Some(line)) = (
                it.get("id").and_then(|x| x.as_str()),
                it.get("line").and_then(|x| x.as_str()),
            ) {
                out.insert(id.to_string(), line.to_string());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::today::{Action, Deadline};

    fn dl(id: &str, date: &str, title: &str) -> Deadline {
        Deadline { id: id.into(), title: title.into(), date: date.into(), source: "email".into(), from: "Mr X".into(), snippet: String::new() }
    }
    fn act(id: &str, received: &str, title: &str, unread: bool) -> Action {
        Action { id: id.into(), title: title.into(), from: "Mr X".into(), received: received.into(), snippet: String::new(), unread }
    }
    fn ctx(deadlines: Vec<Deadline>, actions: Vec<Action>) -> Context {
        Context { deadlines, actions, ..Default::default() }
    }
    fn ids(items: &[AiItem]) -> Vec<&str> {
        items.iter().map(|i| i.id.as_str()).collect()
    }

    #[test]
    fn dates_decide_the_bucket_not_the_model() {
        let today = NaiveDate::from_ymd_opt(2026, 9, 15).unwrap();
        let c = ctx(
            vec![
                dl("d:today", "2026-09-15", "Essay due"),
                dl("d:tom", "2026-09-16", "Quiz"),
                dl("d:soon", "2026-09-20", "TEDx registration"),
                dl("d:over", "2026-09-13", "Late form"),
            ],
            vec![],
        );
        let plan = build_plan(&c, today, &BTreeMap::new());
        // due today, tomorrow and overdue are "now"; four days out is "later".
        assert_eq!(ids(&plan.now), vec!["d:over", "d:today", "d:tom"]);
        assert_eq!(ids(&plan.later), vec!["d:soon"]);
        // dates are carried through for the UI's label.
        assert_eq!(plan.now[0].when, "2026-09-13");
        assert_eq!(plan.later[0].when, "2026-09-20");
    }

    #[test]
    fn requests_split_on_freshness() {
        let today = NaiveDate::from_ymd_opt(2026, 9, 15).unwrap();
        let c = ctx(
            vec![],
            vec![
                act("a:unread", "2026-09-01T08:00:00", "Old but unread", true),
                act("a:fresh", "2026-09-14T08:00:00", "Seen yesterday", false),
                act("a:stale", "2026-09-05T08:00:00", "Seen last week", false),
            ],
        );
        let plan = build_plan(&c, today, &BTreeMap::new());
        assert!(ids(&plan.now).contains(&"a:unread") && ids(&plan.now).contains(&"a:fresh"));
        assert_eq!(ids(&plan.later), vec!["a:stale"]);
        // undated requests carry no date label.
        assert!(plan.now.iter().all(|i| i.when.is_empty()));
    }

    #[test]
    fn model_wording_is_used_when_present_else_the_title() {
        let today = NaiveDate::from_ymd_opt(2026, 9, 15).unwrap();
        let c = ctx(vec![dl("d:1", "2026-09-15", "History essay on the Cold War, hand in")], vec![]);
        let mut lines = BTreeMap::new();
        lines.insert("d:1".to_string(), "Finish Cold War essay.".to_string());
        let plan = build_plan(&c, today, &lines);
        assert_eq!(plan.now[0].line, "Finish Cold War essay"); // trailing dot tidied
        // no summary -> plain title stands
        let plan2 = build_plan(&c, today, &BTreeMap::new());
        assert_eq!(plan2.now[0].line, "History essay on the Cold War, hand in");
        // empty summary -> plain title stands
        let mut blank = BTreeMap::new();
        blank.insert("d:1".to_string(), "   ".to_string());
        assert_eq!(build_plan(&c, today, &blank).now[0].line, "History essay on the Cold War, hand in");
    }

    #[test]
    fn every_item_appears_exactly_once() {
        let today = NaiveDate::from_ymd_opt(2026, 9, 15).unwrap();
        let c = ctx(
            vec![dl("d:1", "2026-09-15", "A"), dl("d:2", "2026-09-30", "B")],
            vec![act("a:1", "2026-09-15T08:00:00", "C", true), act("a:2", "2026-08-01T08:00:00", "D", false)],
        );
        let plan = build_plan(&c, today, &BTreeMap::new());
        let mut all: Vec<&str> = ids(&plan.now).into_iter().chain(ids(&plan.later)).collect();
        all.sort();
        assert_eq!(all, vec!["a:1", "a:2", "d:1", "d:2"]);
    }

    #[test]
    fn harvest_reads_items_or_a_bare_array() {
        let a = harvest(&json!({"items": [{"id": "x", "line": "do x"}, {"id": "y", "line": "do y"}]}));
        assert_eq!(a.get("x").map(|s| s.as_str()), Some("do x"));
        let b = harvest(&json!([{"id": "z", "line": "do z"}]));
        assert_eq!(b.get("z").map(|s| s.as_str()), Some("do z"));
        assert!(harvest(&json!({"nonsense": true})).is_empty());
    }

    #[test]
    fn remap_swaps_short_ids_for_real_ones_and_drops_junk() {
        let real = vec!["mail:AAAA", "task:BBBB"];
        let mut short = BTreeMap::new();
        short.insert("0".to_string(), "Do the first".to_string());
        short.insert("1".to_string(), "Do the second".to_string());
        short.insert("9".to_string(), "out of range".to_string());
        short.insert("x".to_string(), "not a number".to_string());
        let out = remap(short, &real);
        assert_eq!(out.get("mail:AAAA").map(|s| s.as_str()), Some("Do the first"));
        assert_eq!(out.get("task:BBBB").map(|s| s.as_str()), Some("Do the second"));
        assert_eq!(out.len(), 2);
    }
}
