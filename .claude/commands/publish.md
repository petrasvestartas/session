Publish viewer geometry to the Cloudflare R2 bucket `session-viewer-data`.

Usage: /publish <what> — e.g. "the new lion cloud", "out/scan.pb as the live scene", "scenes/view_pointclouds.toml"

R2 is the viewer's ONLY storage. The `session_viewer_data` git branch that used to hold this data
was **deleted on 2026-09-03** — do not look for it, do not recreate it, and do not add geometry to
git. Public base: `https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev`

## Naming: everything starts with `view_`

    view_live.toml              the scene the deployed page shows
    view_readme.md              the bucket's own instructions
    pb/view_live.pb             the slot every solver run overwrites
    pb/view_lines_*.pb          drawings: sheets, sections, plans
    pb/view_mesh_*.pb           meshes
    pb/view_pointcloud_*.pb     point clouds
    pb/view_mixed_*.pb          several geometry types in one file
    scenes/view_*.toml          named scenes, opened with ?scene=scenes/view_<name>.toml

A `.pb` says its TYPE before its subject, so a listing sorts into lanes and a name alone says
which renderer it exercises. `view_live.pb` is the one exception - a slot, not a type. When you
add a file, pick its type prefix; do not invent a fifth one without asking.

A key without the prefix did not come from these scripts. `view_put.sh` adds it when the basename
lacks one, so never hand-write an unprefixed key.

## The two scripts — do not hand-roll `aws` commands

| Goal | Command |
|---|---|
| Put a file in the bucket | `./bash/view_put.sh <file> [key]` (key defaults to `pb/view_<basename>`) |
| Change what the deployed page shows | `./bash/view_live.sh <scene.toml> [file.pb]` |

`view_live.sh` picks the destination from the extension — `.pb` → `pb/view_live.pb` (the fixed slot
every solver run overwrites), `.toml`/`.json` → `view_live.toml` (the manifest the page reads).
Given both, it uploads the geometry first so the manifest never names bytes that are not there
yet, refuses to publish a manifest whose `file =` entries are missing from the bucket, verifies
each upload against the public URL, and pings the relay so an open page reloads in ~1 s.

Shared settings (bucket, endpoint, profile, relay) live in `bash/lib/view.sh` — one place.

## Workflow

1. Confirm the file exists and is what the user meant. A `.pb` is written by
   `Session.pb_dump(path)` (Python), `session.pb_dump(path)` (Rust), `Session::pb_dump` (C++).
2. Choose the script from the table above. Changing the live scene is `view_live.sh`; adding an
   asset a manifest will name later is `view_put.sh`.
3. Run it and report what it printed — the `verified:` line is the proof the bytes are being
   SERVED, not merely accepted by `aws`.
4. To check the result, load the DEPLOYED page and read the console for `live: loaded '<name>'`.
   A bare `http://localhost:8770/` will NOT show it: a dev server shows the local scene and does
   not watch the bucket at all. Locally, check with `?live=<the manifest url>`.

## The local scene is not yours to publish

`session_viewer/assets/view_local.toml` and its `pb/view_local_*.pb` are a working copy that
lives only in the repo. A bare `http://localhost:8770/` shows them; nothing uploads them. If asked
to "publish the local scene", upload the .pb under a NAMED key and write a `scenes/view_<name>.toml`
for it — never overwrite `view_local.*` in the bucket, and never add it there.

## Renaming or removing a key

Both manifests and keys have to move together, and a manifest naming a missing key leaves the
page silently showing the previous scene. After any rename, sweep every manifest in the bucket
and confirm each `file =` entry answers 200 before calling it done.

## What to tell the user before overwriting

**There is no history.** The bucket keeps no versions, so a publish replaces bytes that are then
gone. If they may want the old ones back, say so and offer to save a copy first:
`curl -sS <public url>/<key> -o <local>`.

## Do not

- Do not use a hook for this. Hooks fire on harness events (a tool ran, the session ended);
  publishing is deliberate, and an auto-upload would push half-written `.pb` files to a public URL.
- Do not add cache-busting query strings. The r2.dev URL is uncached, and the viewer already
  sends `cache: no-store` with conditional `If-None-Match` reads.
- Do not narrow the bucket CORS policy. Large clouds are read by HTTP range and need
  `Content-Range` in the expose list, or streaming breaks with no visible error.
- Do not use `awk 'BEGIN{IGNORECASE=1}'` when parsing curl headers — Ubuntu's `awk` is mawk,
  which ignores it, and the check then silently matches nothing. Use `tolower($1)=="header:"`.
