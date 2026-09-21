// The browser edition's stand-in for the Tauri bridge.
//
// The interface (ui/index.html) talks to its backend through
// window.__TAURI__.core.invoke(cmd, args). On the desktop those commands are
// Rust; here the pure ones (plan, config, lessons, papers) come from the same
// Rust compiled to WebAssembly (g9.js / g9_bg.wasm), and everything that lived
// in files - progress, plan edits, settings, profiles - lives in this browser's
// localStorage instead. Machine-only features (Outlook, Teams, the neural
// narrator, the updater) answer honestly so the interface falls back or hides.
(function () {
  const NS = "g9web:";
  const ls = {
    get(k, d) { try { const v = localStorage.getItem(NS + k); return v == null ? d : JSON.parse(v); } catch (e) { return d; } },
    set(k, v) { try { localStorage.setItem(NS + k, JSON.stringify(v)); } catch (e) { } },
    del(k) { try { localStorage.removeItem(NS + k); } catch (e) { } },
  };

  // ---- the WebAssembly core ----
  let core = null;
  const ready = import("./g9.js").then(async (m) => { await m.default(); core = m; return m; });

  // ---- profiles (a registry in localStorage; PINs stored as SHA-256) ----
  function registry() {
    let r = ls.get("profiles", null);
    if (!r || !Array.isArray(r.profiles) || !r.profiles.length) {
      r = { current: "me", profiles: [{ id: "me", name: "Me", pinHash: "", createdAt: new Date().toISOString(), lastUsed: "" }] };
      ls.set("profiles", r);
    }
    if (!r.profiles.some(p => p.id === r.current)) r.current = r.profiles[0].id;
    return r;
  }
  function view(r) {
    return { current: r.current, profiles: r.profiles.map(p => ({ id: p.id, name: p.name, hasPin: !!p.pinHash, createdAt: p.createdAt || "", lastUsed: p.lastUsed || "" })) };
  }
  async function hashPin(pin) {
    const data = new TextEncoder().encode("g9web-pin:" + pin);
    if (crypto && crypto.subtle) {
      const buf = await crypto.subtle.digest("SHA-256", data);
      return Array.from(new Uint8Array(buf)).map(b => b.toString(16).padStart(2, "0")).join("");
    }
    return "plain:" + pin;   // very old browsers only
  }
  function slug(name) {
    const base = String(name || "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "") || "profile";
    const r = registry(); let id = base, n = 2;
    while (r.profiles.some(p => p.id === id)) id = base + "-" + n++;
    return id;
  }
  const cur = () => registry().current;

  // ---- config + plan ----
  async function config() {
    await ready;
    const saved = ls.get("config:" + cur(), null);
    const json = saved ? JSON.stringify(saved) : core.default_config();
    try { return JSON.parse(core.sanitise_config(json)); } catch (e) { return JSON.parse(core.default_config()); }
  }
  async function plan(cfg, catchUp) {
    await ready;
    return JSON.parse(core.build_plan(JSON.stringify(cfg), catchUp ? JSON.stringify(catchUp) : null));
  }

  function download(name, text) {
    const a = document.createElement("a");
    a.href = URL.createObjectURL(new Blob([text], { type: "application/json" }));
    a.download = name; document.body.appendChild(a); a.click();
    setTimeout(() => { URL.revokeObjectURL(a.href); a.remove(); }, 1000);
  }
  const stamp = () => new Date().toISOString().slice(0, 10);

  async function invoke(cmd, args) {
    args = args || {};
    switch (cmd) {
      // plan + config
      case "get_plan": return plan(await config(), args.catchUp || null);
      case "get_config": return config();
      case "save_config": { await ready; const cfg = JSON.parse(core.sanitise_config(JSON.stringify(args.config))); ls.set("config:" + cur(), cfg); return plan(cfg, null); }
      case "reset_config": { ls.del("config:" + cur()); return plan(await config(), null); }
      // built-in content
      case "get_lesson": { await ready; const t = core.lesson(args.topicId); if (t == null) throw new Error("No lesson written for " + args.topicId + " yet"); return t; }
      case "list_papers": { await ready; return JSON.parse(core.list_papers()); }
      case "get_paper": { await ready; const t = core.paper(args.id); if (t == null) throw new Error("No paper called " + args.id); return t; }
      // progress
      case "load_state": return ls.get("state:" + cur(), null);
      case "save_state": ls.set("state:" + cur(), args.state); return null;
      case "state_path": return "this browser's storage on this device";
      case "backup": { download(`grade9-tracker-backup-${stamp()}.json`, JSON.stringify({ profile: cur(), state: ls.get("state:" + cur(), null), config: ls.get("config:" + cur(), null) }, null, 2)); return "Downloaded a backup file"; }
      case "get_settings": return ls.get("settings", {});
      case "set_settings": ls.set("settings", args.settings || {}); return null;
      // profiles
      case "list_profiles": return view(registry());
      case "switch_profile": {
        const r = registry(); const p = r.profiles.find(p => p.id === args.id); if (!p) throw new Error("No such profile.");
        if (p.pinHash) { if (!args.pin) throw new Error("PIN needed."); if ((await hashPin(args.pin)) !== p.pinHash) throw new Error("Wrong PIN."); }
        r.current = p.id; p.lastUsed = new Date().toISOString(); ls.set("profiles", r); return null;
      }
      case "create_profile": {
        const name = String(args.name || "").trim(); if (!name) throw new Error("Give the profile a name.");
        const r = registry(); const id = slug(name);
        r.profiles.push({ id, name, pinHash: args.pin ? await hashPin(args.pin) : "", createdAt: new Date().toISOString(), lastUsed: "" });
        r.current = id; ls.set("profiles", r); return id;
      }
      case "rename_profile": { const r = registry(); const p = r.profiles.find(p => p.id === args.id); if (p) { p.name = String(args.name || p.name).trim() || p.name; ls.set("profiles", r); } return null; }
      case "set_pin": { const r = registry(); const p = r.profiles.find(p => p.id === args.id); if (p) { p.pinHash = args.pin ? await hashPin(args.pin) : ""; ls.set("profiles", r); } return null; }
      case "delete_profile": {
        const r = registry(); if (r.profiles.length <= 1) throw new Error("Keep at least one profile.");
        r.profiles = r.profiles.filter(p => p.id !== args.id); if (r.current === args.id) r.current = r.profiles[0].id;
        ls.set("profiles", r); ls.del("state:" + args.id); ls.del("config:" + args.id);
        try { localStorage.removeItem("grade9-tracker-v2:" + args.id); } catch (e) { }
        return null;
      }
      case "export_profile": {
        const r = registry(); const p = r.profiles.find(p => p.id === (args.id || r.current)) || r.profiles[0];
        const local = (() => { try { return JSON.parse(localStorage.getItem("grade9-tracker-v2:" + p.id) || "null"); } catch (e) { return null; } })();
        download(`${p.name.replace(/[^\w-]+/g, "_")}-${stamp()}.g9profile.json`, JSON.stringify({ format: "g9profile", profile: { id: p.id, name: p.name }, state: local || ls.get("state:" + p.id, null), config: ls.get("config:" + p.id, null) }, null, 2));
        return "downloaded";
      }
      case "import_profile": {
        const b = args.bundle || {}; const name = (b.profile && b.profile.name) || "Imported";
        const r = registry(); const id = slug(name);
        r.profiles.push({ id, name, pinHash: "", createdAt: new Date().toISOString(), lastUsed: "" }); ls.set("profiles", r);
        if (b.state) { ls.set("state:" + id, b.state); try { localStorage.setItem("grade9-tracker-v2:" + id, JSON.stringify(b.state)); } catch (e) { } }
        if (b.config) ls.set("config:" + id, b.config);
        return id;
      }
      // desktop-only: answer so the interface falls back or hides
      case "day_context": throw new Error("Your day comes from Outlook on the desktop app; it is not available in the browser.");
      case "organise_day": throw new Error("desktop only");
      case "teams_assignments": return { items: [] };
      case "open_teams": window.open("https://teams.microsoft.com/", "_blank", "noopener"); return null;
      case "check_update": return null;             // the browser edition updates itself when the site does
      case "install_update": return null;
      case "voices": return [];                       // no neural engine here: the interface uses the device's voices
      case "download_voice": throw new Error("desktop only");
      case "narrate": throw new Error("desktop only"); // speakNow() falls back to speechSynthesis
      case "draft_subject": throw new Error("Drafting a subject from a specification needs the desktop app.");
      default: throw new Error("Not available in the browser edition: " + cmd);
    }
  }

  window.__TAURI__ = {
    core: { invoke },
    event: { listen: async () => () => { } },
    opener: { openUrl: (u) => { window.open(u, "_blank", "noopener"); } },
  };
  window.G9_WEB = true;

  // Hide the machine-only panels rather than show their errors.
  const style = document.createElement("style");
  style.textContent = ".dayplan,#openteams,#openteams2,#upd,.card:has(#autoupdon),.card:has(#aion){display:none!important}"
    // keep the bars clear of the notch and the home indicator when installed on a phone
    + ".top{padding-top:env(safe-area-inset-top)}.nav{padding-bottom:env(safe-area-inset-bottom)}"
    + "body{padding-bottom:calc(var(--navh) + 24px + env(safe-area-inset-bottom))}"
    + ".lplayer{bottom:calc(var(--navh) + 14px + env(safe-area-inset-bottom))}";
  document.head.appendChild(style);

  // Offline after the first visit, and a proper home-screen app on phones.
  if ("serviceWorker" in navigator) {
    window.addEventListener("load", () => { navigator.serviceWorker.register("./sw.js").catch(() => { }); });
  }
})();
