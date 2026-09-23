//! A tiny localhost receiver for Microsoft Teams assignments.
//!
//! The browser extension reads your assignments straight off the Teams page you
//! open in your own browser and POSTs them here. We bind 127.0.0.1 only, so
//! nothing is reachable from the network, and store what arrives to teams.json
//! for the "Your day" panel to show. Nothing here touches the internet.

use std::path::PathBuf;
use tiny_http::{Header, Method, Response, Server};

/// Fixed port the extension also targets (see teams-extension/background.js).
const PORT: u16 = 47814;

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("valid header")
}

/// Start the receiver on a background thread. If the port is already taken
/// (another window of the app is already listening) it just exits quietly.
pub fn start(store_path: PathBuf) {
    std::thread::spawn(move || {
        let server = match Server::http(("127.0.0.1", PORT)) {
            Ok(s) => s,
            Err(_) => return,
        };
        for mut req in server.incoming_requests() {
            let method = req.method().clone();
            let url = req.url().to_string();

            let mut resp = if method == Method::Options {
                Response::from_string("").with_status_code(204)
            } else if method == Method::Post && url.starts_with("/teams") {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                let ok = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .filter(|v| v.get("items").map(|i| i.is_array()).unwrap_or(false))
                    .map(|_| std::fs::write(&store_path, body.as_bytes()).is_ok())
                    .unwrap_or(false);
                Response::from_string(if ok { "ok" } else { "bad request" })
                    .with_status_code(if ok { 200u16 } else { 400u16 })
            } else if method == Method::Get && url.starts_with("/ping") {
                Response::from_string("grade9tracker")
            } else {
                Response::from_string("").with_status_code(404)
            };

            resp.add_header(header("Access-Control-Allow-Origin", "*"));
            resp.add_header(header("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            resp.add_header(header("Access-Control-Allow-Headers", "Content-Type"));
            let _ = req.respond(resp);
        }
    });
}
