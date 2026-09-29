// Background service worker. Receives assignments from the content script and
// POSTs them to Zelinx Study Planner's local receiver (the app listens on 127.0.0.1).
// Nothing goes to the internet — only to the app on this same computer.
const APP_URL = "http://127.0.0.1:47814/teams";

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (!msg || msg.type !== "g9-assignments") return;
  fetch(APP_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ items: msg.items || [], diag: msg.diag || null, syncedAt: new Date().toISOString() })
  })
    .then((r) => sendResponse({ ok: r.ok }))
    .catch(() => sendResponse({ ok: false }));
  return true; // keep the message channel open for the async response
});
