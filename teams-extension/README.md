# Grade 9 Tracker — Teams assignment sync (Chrome / Edge extension)

Reads your Microsoft Teams assignments from the Teams page in your browser and
syncs them into Grade 9 Tracker on this computer. Nothing leaves your machine —
the extension only talks to the app on `127.0.0.1`.

## Install (one time)

1. Open **`chrome://extensions`** (or `edge://extensions`).
2. Turn on **Developer mode** (top-right).
3. Click **Load unpacked** and select this folder:
   `C:\Users\HP\Documents\Grade9Tracker\teams-extension`

## Use it

1. Keep **Grade 9 Tracker** open (it runs the local receiver the extension sends to).
2. In the browser, go to **https://teams.microsoft.com** and sign in.
3. Open **Assignments** (a class → **Assignments**, or the Assignments app in the
   left rail). As the list loads, the extension reads it and syncs.
4. A small green toast confirms "synced N assignments". They appear in the app
   under **Your day → Assignments**, tagged **Teams**.

Tip: the app's **Teams ↗** button opens Teams in your browser, which is also when
a sync happens.

## Notes

- If you see "Grade 9 Tracker not running", open the app and reload the Teams tab.
- Assignments you've already turned in drop off the list automatically.
- How it works: it reads Teams' own data feed
  (`assignments.edu.cloud.microsoft/api/v1.0/edu/me/work`) — the same data Teams
  shows — so it's accurate, not screen-scraping. If Microsoft ever changes that
  feed, the extension needs an update.
