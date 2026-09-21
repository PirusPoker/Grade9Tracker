// Offline cache for the browser edition. The build stamps VERSION with the
// commit, so every deploy is a new cache: pages are fetched network-first and
// fall back to the cache, so an update shows up on the next open with a
// connection and the app still works without one.
const VERSION = "__VERSION__";
const CACHE = "g9-" + VERSION;
const SHELL = ["./", "./index.html", "./web.js", "./g9.js", "./g9_bg.wasm", "./manifest.webmanifest",
  "./katex/katex.min.css", "./katex/katex.min.js", "./icon-192.png", "./icon-512.png"];

self.addEventListener("install", (e) => {
  e.waitUntil(caches.open(CACHE).then((c) => c.addAll(SHELL)).then(() => self.skipWaiting()));
});

self.addEventListener("activate", (e) => {
  e.waitUntil(caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)))).then(() => self.clients.claim()));
});

self.addEventListener("fetch", (e) => {
  const url = new URL(e.request.url);
  if (e.request.method !== "GET" || url.origin !== location.origin) return;   // YouTube thumbnails etc. go straight through
  e.respondWith(
    fetch(e.request).then((res) => {
      if (res.ok) { const copy = res.clone(); caches.open(CACHE).then((c) => c.put(e.request, copy)); }
      return res;
    }).catch(() => caches.match(e.request).then((hit) => hit || (e.request.mode === "navigate" ? caches.match("./index.html") : Response.error())))
  );
});
