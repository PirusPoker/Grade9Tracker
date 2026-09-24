// Isolated world. Receives the parsed assignments from inject.js and hands them
// to the background service worker, which sends them to Grade 9 Tracker running
// on this computer. Shows a small confirmation toast.
(function () {
  "use strict";
  const IN_TOP = window.top === window;

  window.addEventListener("message", (e) => {
    const d = e.data;
    if (!d || d.__g9teams !== 1 || d.kind !== "assignments") return;
    // Sub-frames relay up so the top frame does the sending and shows one toast.
    if (!IN_TOP) { try { window.top.postMessage(d, "*"); } catch (x) {} return; }
    try {
      chrome.runtime.sendMessage({ type: "g9-assignments", items: d.items }, (resp) => {
        void chrome.runtime.lastError; // ignore "no receiver" if the worker is asleep
        toast(d.items.length, resp && resp.ok);
      });
    } catch (x) { toast(d.items.length, false); }
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
