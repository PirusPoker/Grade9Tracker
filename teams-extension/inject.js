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

  // Which feed request this was (path + query), so a sync can say what Teams asked for.
  function queryOf(url) {
    try {
      const u = new URL(url, location.href);
      return (u.pathname.replace(/^.*\/edu\//, "edu/") + decodeURIComponent(u.search)).slice(0, 200);
    } catch (e) { return ""; }
  }

  function handle(url, text) {
    try {
      if (!WORK_RE.test(url) || !text) return;
      const j = JSON.parse(text);
      const arr = Array.isArray(j.value) ? j.value : (Array.isArray(j) ? j : null);
      if (!arr) return;                       // not the assignment feed after all
      // Everything in this response goes up unfiltered: Teams asks for the feed
      // more than once (different views, pages, frames), and only the top frame
      // sees all of them, so it merges and decides what's forthcoming.
      const items = arr.filter((a) => a && a.id).map(pick);
      window.postMessage({ __g9teams: 1, kind: "assignments", items: items, query: queryOf(url) }, "*");
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
