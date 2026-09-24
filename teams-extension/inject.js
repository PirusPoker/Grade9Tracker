// Runs in the page's own world (MAIN) at document_start so it can wrap fetch/XHR
// before Teams uses them. It watches for the one response that carries your
// assignments — assignments.edu.cloud.microsoft/api/v1.0/edu/me/work — parses out
// the fields we need, and hands them to the isolated content script. It only
// reads that response; it never changes requests or touches anything else.
(function () {
  "use strict";
  const WORK_RE = /assignments\.edu\.cloud\.microsoft\/api\/v[\d.]+\/edu\/me\/work/i;

  function pick(a) {
    const sub = (a.submissions && a.submissions[0]) || null;
    return {
      id: a.id,
      title: (a.displayName != null ? String(a.displayName) : "").trim() || "(untitled assignment)",
      due: a.dueDateTime || null,
      assigned: a.assignedDateTime || a.createdDateTime || null,
      close: a.closeDateTime || null,
      status: a.status || null,                 // "assigned", "returned", ...
      completed: !!a.isCompleted,
      turnedIn: !!a.allTurnedIn,
      submission: sub ? sub.status : null,       // "working", "submitted", "returned", ...
      classId: a.classId || null,
      webUrl: a.webUrl || null,
      instructions: a.instructions || null,
      maxPoints: (a.grading && a.grading.maxPoints != null) ? a.grading.maxPoints : null
    };
  }

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

  function handle(url, text) {
    try {
      if (!WORK_RE.test(url) || !text) return;
      const j = JSON.parse(text);
      const arr = Array.isArray(j.value) ? j.value : (Array.isArray(j) ? j : null);
      if (!arr) return;                       // not the assignment feed after all
      const items = arr.filter((a) => a && a.id).map(pick).filter(forthcoming);
      // Sent even when empty, so the app drops assignments that are no longer due.
      window.postMessage({ __g9teams: 1, kind: "assignments", items: items }, "*");
    } catch (e) {}
  }

  const _fetch = window.fetch;
  if (_fetch) {
    window.fetch = function () {
      const p = _fetch.apply(this, arguments);
      try {
        const a0 = arguments[0];
        const url = typeof a0 === "string" ? a0 : (a0 && a0.url) || "";
        if (WORK_RE.test(url)) p.then((r) => r.clone().text().then((t) => handle(url, t)).catch(() => {})).catch(() => {});
      } catch (e) {}
      return p;
    };
  }

  const _open = XMLHttpRequest.prototype.open;
  const _send = XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.open = function (m, u) { this.__g9url = u; return _open.apply(this, arguments); };
  XMLHttpRequest.prototype.send = function () {
    try {
      if (WORK_RE.test(this.__g9url || "")) this.addEventListener("load", () => { try { handle(this.__g9url, this.responseText); } catch (e) {} });
    } catch (e) {}
    return _send.apply(this, arguments);
  };
})();
