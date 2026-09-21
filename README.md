# Grade 9 Tracker (Tauri + Rust)

Two-year GCSE and IGCSE study plan. It works out what you should do each week for the next two years, puts one session a day in front of you with the objectives for it, and teaches every topic with its own lesson, worked examples and end-of-topic test. Runs offline, starts instantly.

## Install it (Windows)

**Download page:** https://piruspoker.github.io/Grade9Tracker/

Or grab the installer directly — this link always gives the newest version:
https://github.com/PirusPoker/Grade9Tracker/releases/latest/download/Grade9Tracker-Setup.exe

1. Run `Grade9Tracker-Setup.exe`. Windows shows "Windows protected your PC" because the
   app is not from the Microsoft Store: click **More info**, then **Run anyway** (once).
2. Open Grade 9 Tracker from the Start menu, add yourself as a profile, tick your subjects.

That is the last download you make: the app checks GitHub while it runs and installs
new releases itself (Settings lets you switch that to "ask me first"). Your progress,
plan and notes stay on your own PC.
The narrator fetches its voice (about 60 MB) the first time you press Listen; pick a
different voice in the lesson's voice menu and it downloads that one too.

Everything below is for people who want to change the app itself.

- `src-tauri/src/plan.rs` — the subjects, topics, calendar and scheduler (Rust). Change term dates or hours here.
- `src-tauri/src/config.rs` — the editable plan: subjects, topics, hours, term dates and links. Changed from inside the app, saved to `config.json`.
- `src-tauri/src/course.rs` — the written course: objectives and common mistakes per topic, plus each specification's official content references so coverage can be checked.
- `src-tauri/src/draft.rs` — drafting a new subject from a public spec page (optional, needs an API key).
- `src-tauri/src/lib.rs` — building the plan, saving progress, backups.
- `ui/index.html` — the interface (plain HTML/CSS/JS, no framework, no build step).

## Set up once (Windows)

1. Install Rust: https://rustup.rs (accept the defaults).
2. Install the Visual Studio Build Tools with the **Desktop development with C++** workload: https://aka.ms/vs/17/release/vs_BuildTools.exe
3. Install the Tauri CLI: `cargo install tauri-cli --locked --version "^2.0"`

WebView2 is already part of Windows 10/11, so there is nothing else to install. Node.js is **not** needed — the interface is plain HTML with no build step.

Windows **Smart App Control** must be off, or it blocks Rust from running the build scripts it compiles (`os error 4551`). Windows Security → App & browser control → Smart App Control settings. Note that turning it off is permanent.

## Run it

```
cargo tauri dev
```

## Make the installer

```
cargo tauri build
```

The installer appears in `src-tauri\target\release\bundle\nsis\Grade 9 Tracker_<version>_x64-setup.exe`. Run it once; the app then lives in your Start menu like any other program.

## Ship an update

Installed copies check GitHub on launch and offer any newer release with one
button, so nobody re-downloads by hand. To publish one:

1. Bump `version` in `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` and `package.json` (keep them the same).
2. Commit, then tag and push:

```
git tag v1.2.0
git push origin master --tags
```

GitHub Actions (`.github/workflows/release.yml`) builds, signs and publishes
the release, including the `latest.json` the app reads. Signing uses the
keypair in `~/.tauri/grade9tracker.key` (private, uploaded once as the
`TAURI_SIGNING_PRIVATE_KEY` repository secret) and the public key in
`tauri.conf.json`. **Back up the private key** - without it, installed
copies will refuse any future update.

## Where your progress lives

`%APPDATA%\uk.alastair.grade9tracker\` on Windows:

- `state.json` — what you have ticked off, what is on the fix list, your streak
- `config.json` — your edited plan, written whenever you save in the **Plan** tab
- `backups\` — dated copies, written by **Guide → Back up now**

The Guide tab shows the exact path.

## Save My Exams

Every session links into the right section of [Save My Exams](https://www.savemyexams.com) for that subject, and each topic can pin its own exact page. The app stores links only, never any of their content — so it needs your own Save My Exams account, and anyone else using this needs theirs.

Edit the links per subject in **Plan**, or pin a topic's exact page from its session card.

## Tests

```
cd src-tauri && cargo test
```

Checks that every topic is scheduled, that content finishes before Year 11, that a stripped-down or broken configuration still produces a usable plan, that every subject ships with working links, and — for all five subjects — that every content reference in the official specification is covered by a topic, that no topic invents a reference the specification does not contain, and that every topic is written up with objectives.
