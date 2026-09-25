// Isolated world. Receives the parsed assignments from inject.js and hands them
// to the background service worker, which sends them to Grade 9 Tracker running
// on this computer. Shows a small confirmation toast.
(function () {
  "use strict";
  const IN_TOP = window.top === window;
  const SEEN = new Map(), CALLS = [];

  // Only forthcoming work is worth syncing. Teams keeps anything never formally
  // handed in on the list forever, so the feed is mostly old term-time leftovers.
  // Keep: not handed in, and due from a few days ago onwards (so a deadline you
  // only just missed doesn't vanish). Undated: only if set in the last fortnight.
  const DAY_MS = 864e5, GRACE_DAYS = 3, UNDATED_DAYS = 14;
  function forthcoming(a) {
    if (a.completed || a.turnedIn || ["submitted", "returned", "excused"].includes(a.submission)) return false;
    const now = Date.now();
    if (a.due) {
      const d = Date.parse(a.due);
      return isNaN(d) || d >= now - GRACE_DAYS * DAY_MS;
    }
    const set = Date.parse(a.assigned || "");
    return !isNaN(set) && set >= now - UNDATED_DAYS * DAY_MS;
  }

  window.addEventListener("message", (e) => {
    const d = e.data;
    if (!d || d.__g9teams !== 1 || d.kind !== "assignments") return;
    // Sub-frames relay up so the top frame does the sending and shows one toast.
    if (!IN_TOP) { try { window.top.postMessage(d, "*"); } catch (x) {} return; }
    // Merge: every assignment seen on this page, by id, latest copy winning. A
    // later response that only covers some of them (another view, a page of
    // results, the "completed" tab) must not wipe out the ones it doesn't list.
    (d.items || []).forEach((a) => { if (a && a.id) SEEN.set(a.id, a); });
    CALLS.push({ q: d.query || "", n: (d.items || []).length, at: new Date().toISOString() });
    if (CALLS.length > 15) CALLS.shift();
    const all = [...SEEN.values()];
    const items = all.filter(forthcoming);
    // Dates and statuses only - never titles - so a sync that looks wrong can be read back.
    const diag = { calls: CALLS.slice(), seen: all.map((a) => ({ due: a.due, assigned: a.assigned, status: a.status, submission: a.submission, completed: a.completed, turnedIn: a.turnedIn, kept: forthcoming(a) })) };
    try {
      chrome.runtime.sendMessage({ type: "g9-assignments", items: items, diag: diag }, (resp) => {
        void chrome.runtime.lastError; // ignore "no receiver" if the worker is asleep
        toast(items.length, resp && resp.ok);
      });
    } catch (x) { toast(items.length, false); }
  });
  if (!IN_TOP) return;

  let el;
  function toast(n, ok) {
    if (!el) {
      el = document.createElement("div");
      el.style.cssText = "position:fixed;right:16px;bottom:16px;z-index:2147483647;padding:8px 12px;border-radius:8px;color:#fff;font:600 12px system-ui,sans-serif;box-shadow:0 6px 20px rgba(0,0,0,.3);transition:opacity .5s";
      document.documentElement.appendChild(el);
    }
    if (ok === false) {
      el.style.background = "#b3261e";
      el.textContent = "Grade 9 Tracker not running — open the app, then reload Teams";
    } else {
      el.style.background = "#1a7f37";
      el.textContent = n ? "Grade 9 Tracker · synced " + n + " upcoming assignment" + (n === 1 ? "" : "s")
                         : "Grade 9 Tracker · no upcoming assignments";
    }
    el.style.opacity = "1";
    clearTimeout(el.__t);
    el.__t = setTimeout(() => { el.style.opacity = "0"; }, 3500);
  }
})();
