//! Drafting a new subject from a public specification page.
//!
//! Fetches a syllabus page (or takes pasted text), hands it to Claude, and gets
//! back a topic list with hours, objectives and the marks people drop — the same
//! shape the plan already uses. It only ever drafts: nothing is saved until the
//! user reviews it and presses Save in the Plan tab.
//!
//! This is for *public* spec pages. Anything behind a login — Save My Exams
//! included — will return a sign-in page, not a syllabus.

use serde::{Deserialize, Serialize};
use serde_json::Value;

const MODEL: &str = "claude-opus-5";
/// Enough of a spec to work from without sending a whole textbook.
const MAX_PAGE_CHARS: usize = 60_000;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub api_key: String,
    /// Read the day's events and deadlines out of classic Outlook (see
    /// `outlook.rs`). On by default; off means Outlook is never started.
    #[serde(default = "yes")]
    pub outlook: bool,
}

fn yes() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Settings { api_key: String::new(), outlook: true }
    }
}

/// Only public http(s) hosts. Keeps the app from being talked into fetching
/// something on the user's own machine or network.
fn check_public_url(raw: &str) -> Result<(), String> {
    let u = raw.trim();
    if !(u.starts_with("https://") || u.starts_with("http://")) {
        return Err("That does not look like a web address — it needs to start with https://".into());
    }
    let rest = u.split_once("://").map(|(_, r)| r).unwrap_or(u);
    let host = rest.split(['/', '?', '#']).next().unwrap_or("").split('@').next_back().unwrap_or("");
    let host = host.split(':').next().unwrap_or("").to_ascii_lowercase();
    let private = host == "localhost"
        || host.ends_with(".localhost")
        || host == "::1"
        || host.starts_with("127.")
        || host.starts_with("10.")
        || host.starts_with("192.168.")
        || host.starts_with("169.254.")
        || host.starts_with("0.")
        || (host.starts_with("172.")
            && host.split('.').nth(1).and_then(|o| o.parse::<u8>().ok()).is_some_and(|o| (16..=31).contains(&o)))
        || !host.contains('.');
    if private {
        return Err("That address is on this machine or your local network, so there is nothing public to read there.".into());
    }
    Ok(())
}

/// Crude tag stripping — good enough to hand a syllabus page to a model, and it
/// avoids pulling in an HTML parser for one feature.
fn html_to_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len() / 2);
    let mut in_tag = false;
    let mut skipping: Option<&str> = None;
    let lower = html.to_ascii_lowercase();
    let bytes: Vec<char> = html.chars().collect();
    let lower_chars: Vec<char> = lower.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(end) = skipping {
            // inside <script>/<style>: run on until the closing tag
            let tail: String = lower_chars[i..(i + end.len()).min(lower_chars.len())].iter().collect();
            if tail == end {
                i += end.len();
                skipping = None;
            } else {
                i += 1;
            }
            continue;
        }
        let c = bytes[i];
        if c == '<' {
            let tail: String = lower_chars[i..(i + 8).min(lower_chars.len())].iter().collect();
            if tail.starts_with("<script") {
                skipping = Some("</script>");
                i += 7;
                continue;
            }
            if tail.starts_with("<style") {
                skipping = Some("</style>");
                i += 6;
                continue;
            }
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
            out.push(' ');
        } else if !in_tag {
            out.push(c);
        }
        i += 1;
    }
    // collapse whitespace so the model is not paying for blank lines
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn prompt_for(name: &str, source: &str) -> String {
    format!(
        "You are laying out a two-year GCSE/IGCSE study plan for a student aiming for a grade 9.\n\n\
         Draft the subject \"{name}\" from the specification below.\n\n\
         Break the course into the topics they should study, in a sensible teaching order — earlier topics first, \
         topics that depend on others after them. For each topic give:\n\
         - code: the specification reference, exactly as the spec writes it\n\
         - title: what the topic covers, in plain English\n\
         - hours: how many hours of study it is worth (0.5 to 5, whole or half numbers). Weight it by difficulty and exam marks.\n\
         - objectives: 3 to 5 specific things they must be able to DO by the end. Start each with a verb. Be concrete about methods and techniques, not vague.\n\
         - watch: one sentence naming the mark students most often drop on this topic, and what to do instead.\n\n\
         Also give the subject's full qualification title and a list of the broad spec sections, for past-paper practice later.\n\n\
         Return ONLY a JSON object, no prose before or after, in exactly this shape:\n\
         {{\"name\":\"short name\",\"full\":\"full qualification title\",\"sections\":[\"...\"],\"topics\":[{{\"code\":\"1.1\",\"title\":\"...\",\"hours\":2,\"objectives\":[\"...\"],\"watch\":\"...\"}}]}}\n\n\
         SPECIFICATION:\n{source}"
    )
}

/// Ask Claude to draft the subject. Returns the parsed JSON object.
pub async fn draft(api_key: &str, name: &str, url: &str, pasted: &str) -> Result<Value, String> {
    let key = api_key.trim();
    if key.is_empty() {
        return Err("no-key".into());
    }
    if name.trim().is_empty() {
        return Err("Give the subject a name first.".into());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
        .build()
        .map_err(|e| e.to_string())?;

    // Where the spec text comes from: a pasted block wins, otherwise fetch.
    let mut source = pasted.trim().to_string();
    if source.is_empty() {
        check_public_url(url)?;
        let res = client
            .get(url.trim())
            .header("user-agent", "Grade9Tracker/1.0")
            .send()
            .await
            .map_err(|e| format!("Could not open that page: {e}"))?;
        if !res.status().is_success() {
            return Err(format!("That page returned {}. If it needs a login, paste the text instead.", res.status().as_u16()));
        }
        let body = res.text().await.map_err(|e| format!("Could not read that page: {e}"))?;
        source = html_to_text(&body);
        if source.chars().count() < 400 {
            return Err("There was almost no readable text on that page. If it is a PDF or needs a login, copy the spec text and paste it in instead.".into());
        }
    }
    let source: String = source.chars().take(MAX_PAGE_CHARS).collect();

    let res = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "model": MODEL,
            "max_tokens": 16000,
            "messages": [{"role": "user", "content": prompt_for(name.trim(), &source)}]
        }))
        .send()
        .await
        .map_err(|e| format!("Could not reach the Anthropic API: {e}"))?;

    let status = res.status();
    if !status.is_success() {
        return Err(match status.as_u16() {
            401 => "That API key was rejected. Check it in Guide → Settings.".into(),
            429 => "Rate limited by the API. Wait a minute and try again.".into(),
            code => format!("The API returned {code}."),
        });
    }

    let body: Value = res.json().await.map_err(|e| e.to_string())?;
    if body["stop_reason"] == "refusal" {
        return Err("The model declined that request. Try a different page.".into());
    }
    // Thinking blocks carry no "text", so this picks out the answer itself.
    let text: String = body["content"]
        .as_array()
        .map(|parts| parts.iter().filter_map(|p| p["text"].as_str()).collect::<Vec<_>>().join(""))
        .unwrap_or_default();
    let start = text.find('{').ok_or("The reply had no JSON in it. Try again.")?;
    let end = text.rfind('}').ok_or("The reply had no JSON in it. Try again.")?;
    let mut draft: Value = serde_json::from_str(&text[start..=end]).map_err(|e| format!("The reply was not valid JSON: {e}"))?;

    if draft["topics"].as_array().is_none_or(|t| t.is_empty()) {
        return Err("No topics came back. Try a page that actually lists the syllabus content.".into());
    }
    draft["name"] = Value::String(if draft["name"].as_str().unwrap_or("").trim().is_empty() {
        name.trim().to_string()
    } else {
        draft["name"].as_str().unwrap_or(name).to_string()
    });
    Ok(draft)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_public_web_addresses_are_fetched() {
        assert!(check_public_url("https://qualifications.pearson.com/spec.html").is_ok());
        for bad in [
            "http://localhost:8080/x",
            "https://127.0.0.1/admin",
            "https://10.0.0.5/",
            "https://192.168.1.1/",
            "https://172.20.0.1/",
            "https://169.254.169.254/latest/meta-data/",
            "file:///C:/Windows/win.ini",
            "https://intranet/",
        ] {
            assert!(check_public_url(bad).is_err(), "{bad} should be refused");
        }
        // 172.32 is public, unlike 172.16–172.31
        assert!(check_public_url("https://172.32.0.1/").is_ok());
    }

    #[test]
    fn scripts_and_styles_are_stripped() {
        let html = "<html><head><style>body{color:red}</style><script>alert('x')</script></head>\
                    <body><h1>Topic 1</h1><p>Number   and   algebra</p></body></html>";
        let text = html_to_text(html);
        assert!(text.contains("Topic 1"), "{text}");
        assert!(text.contains("Number and algebra"), "whitespace should collapse: {text}");
        assert!(!text.contains("alert"), "script contents must go: {text}");
        assert!(!text.contains("color:red"), "style contents must go: {text}");
    }
}
