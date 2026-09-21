//! The browser edition's backend: the desktop app's pure modules, included by
//! path, behind a handful of string-in/string-out functions that web.js calls
//! in place of Tauri commands. Everything that needs a machine - files,
//! Outlook, Teams, the narrator engine, the updater - stays in the desktop
//! app; web.js keeps state in the browser instead.

#[path = "../../src-tauri/src/config.rs"]
pub mod config;
#[path = "../../src-tauri/src/course.rs"]
pub mod course;
#[path = "../../src-tauri/src/lessons.rs"]
pub mod lessons;
#[path = "../../src-tauri/src/papers.rs"]
pub mod papers;
#[path = "../../src-tauri/src/plan.rs"]
pub mod plan;
#[path = "../../src-tauri/src/statements.rs"]
pub mod statements;
#[path = "../../src-tauri/src/videos.rs"]
pub mod videos;

use config::PlanConfig;
use wasm_bindgen::prelude::*;

fn js_err(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}

/// The built-in plan configuration, as JSON.
#[wasm_bindgen]
pub fn default_config() -> String {
    serde_json::to_string(&PlanConfig::default()).unwrap_or_else(|_| "{}".into())
}

/// Tidy a configuration the way the desktop app does before saving it.
#[wasm_bindgen]
pub fn sanitise_config(config_json: &str) -> Result<String, JsValue> {
    let mut cfg: PlanConfig = serde_json::from_str(config_json).map_err(js_err)?;
    cfg.sanitise();
    serde_json::to_string(&cfg).map_err(js_err)
}

/// The plan for a configuration, optionally re-poured from a catch-up
/// snapshot (the same JSON the desktop `get_plan` takes).
#[wasm_bindgen]
pub fn build_plan(config_json: &str, catch_up_json: Option<String>) -> Result<String, JsValue> {
    let mut cfg: PlanConfig = serde_json::from_str(config_json).map_err(js_err)?;
    cfg.sanitise();
    let catch_up: Option<plan::CatchUp> = match catch_up_json {
        Some(s) if !s.trim().is_empty() && s.trim() != "null" => Some(serde_json::from_str(&s).map_err(js_err)?),
        _ => None,
    };
    serde_json::to_string(&plan::build_with(&cfg, catch_up.as_ref())).map_err(js_err)
}

/// The built-in lesson for a topic id such as `chem:3b`, or null.
#[wasm_bindgen]
pub fn lesson(topic_id: &str) -> Option<String> {
    lessons::lesson(topic_id).map(str::to_string)
}

/// Every mock paper's header entry, as JSON.
#[wasm_bindgen]
pub fn list_papers() -> String {
    serde_json::to_string(&papers::all()).unwrap_or_else(|_| "[]".into())
}

/// One mock paper's text, or null.
#[wasm_bindgen]
pub fn paper(id: &str) -> Option<String> {
    papers::paper(id).map(str::to_string)
}
