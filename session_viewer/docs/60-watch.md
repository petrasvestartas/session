# 60 Watch — external edits flow back in

> **Big picture.** *Phase 6 closes.* Load (34), save (50) — the last direction is external edits
> flowing in *while the viewer runs*: a script regenerates the `.pb`, a teammate exports, another
> tool writes — and the viewer just updates. That's what makes the file a live document shared with
> other tools instead of an import. The reaction is already built (46's reconcile); what's new here
> is the transport and one classic trap — not reacting to your *own* saves.

Three sync directions close the loop: 34 reads a file in, 44 writes it out, and this lesson watches it
— when *something else* edits the file (another tool, a script, a teammate's export), the viewer picks
up the change and reconciles it, without a manual reload. It's the last piece that makes the `.pb` a
live source of truth instead of a one-shot import.

Two problems to solve. First, **the browser can't watch a filesystem** — there's no inotify in a tab.
Second, **your own saves change the file too** (50), so a naive watcher would see its own write, reload,
and potentially loop. Both have clean answers, and the *reaction* to a change is already built: it's
46's `reconcile`.

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="every N frames the viewer re-fetches the file, hashes the bytes, and if the hash changed AND is not its own last save it runs 46 reconcile; otherwise it skips" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="120" height="34" fill="none" stroke="#6fb3ff"/><text x="70" y="51" fill="#d7dae0" text-anchor="middle">poll: fetch bytes</text>
  <text x="70" y="80" fill="#666" text-anchor="middle" font-size="9">every N frames (34)</text>
  <rect x="168" y="30" width="120" height="34" fill="none" stroke="#6fb3ff"/><text x="228" y="51" fill="#d7dae0" text-anchor="middle">hash bytes</text>
  <path d="M326,47 l-30,-14 l0,28 z" fill="none" stroke="#6fb3ff"/><text x="360" y="34" fill="#888">same?</text>
  <rect x="404" y="12" width="150" height="24" fill="none" stroke="#3a3a3a"/><text x="479" y="28" fill="#888" text-anchor="middle">unchanged → skip</text>
  <rect x="404" y="44" width="150" height="24" fill="none" stroke="#3a3a3a"/><text x="479" y="60" fill="#888" text-anchor="middle">== our last save → skip</text>
  <rect x="404" y="76" width="150" height="24" fill="none" stroke="#6fb3ff"/><text x="479" y="92" fill="#d7dae0" text-anchor="middle">changed → reconcile (49)</text>
  <line x1="130" y1="47" x2="166" y2="47" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah40)"/>
  <line x1="288" y1="47" x2="294" y2="47" stroke="#6fb3ff" stroke-width="1.4"/>
  <line x1="326" y1="40" x2="402" y2="24" stroke="#3a3a3a" stroke-width="1.1" marker-end="url(#ah40)"/>
  <line x1="326" y1="47" x2="402" y2="56" stroke="#3a3a3a" stroke-width="1.1" marker-end="url(#ah40)"/>
  <line x1="326" y1="54" x2="402" y2="88" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah40)"/>
  <line x1="554" y1="88" x2="600" y2="88" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah40)"/>
  <text x="636" y="92" fill="#d7dae0" text-anchor="middle">apply diff</text>
  <text x="340" y="135" fill="#888" text-anchor="middle">the self-write guard (== our last save) is what stops save → watch → reload → save looping</text>
  <defs><marker id="ah40" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## The transport: poll a URL (verified) vs. a file handle (nicer)

The viewer already fetches its file over HTTP (34). The simplest watcher that works in *every* browser
is to **re-fetch the same URL** every second and compare — no new API, no permissions, and the dev just
edits the file their static server serves. That's the path this lesson builds. Two upgrades, noted but
not built here:

- **File System Access API** (`showOpenFilePicker` → a `FileSystemFileHandle`, poll `getFile().lastModified`):
  true local-file watching, no server — but it needs a user-gesture grant and its `web-sys` bindings are
  behind unstable flags. A UX upgrade, same reconcile underneath.
- **Watcher → WebSocket push**: a tiny dev-server process watches the real file and pushes on change —
  zero polling latency. Best for a desktop-style setup; the browser side still calls the same reconcile.

## Files we touch

```
src/app/persistence.rs   # file_hash(bytes) — one fingerprint for the WHOLE file
                         # (change + self-write guard)
src/state.rs             # poll every N frames; self-write guard; apply_session factored out of 46's reload
src/lib.rs               # Msg::Watched(url, Session) + its user_event arm — the loader's channel, reused
```

## Step 1 — factor the apply path out of reload: `src/state.rs`

46's `reload` fetches, parses, *and* applies. Watch needs the apply half on its own (its poll task
delivers an already-parsed `Session`), so split it — `pub`, because the message arm in lib.rs calls it:

```rust
    /// Diff `new` against the loaded scene and push only the changes to the GPU — the body of 46's
    /// reload, minus the fetch+parse. Both manual reload and the watcher call this.
    pub fn apply_session(&mut self, new: Session) {
        let diff = self.scene.reconcile(&new);
        // guid_to_row spans ALL docs (35) — fine for this log line in a one-doc scene; a real
        // per-doc count needs the doc's own rows. saturating_sub: never underflow a log line.
        let unchanged = self.scene.guid_to_row.len()
            .saturating_sub(diff.changed.len() + diff.removed.len());
        log::info!("sync: {} added, {} changed, {} removed, {} unchanged",
            diff.added.len(), diff.changed.len(), diff.removed.len(), unchanged);
        for g in &diff.removed {
            let row = self.scene.guid_to_row[g];
            self.gpu.remove_object(g); self.gpu.hide_row(row); self.scene.free_row(g);
        }
        // The row's full world frame, composed exactly as 46's reload composes it (manifest
        // place × session world xform) — apply_object takes it as a parameter.
        let world = new.world_xforms();
        let place = self.scene.docs.first()
            .map(|d| d.place.duplicate()).unwrap_or_else(session_rust::Xform::identity);
        let placed = |g: &String| &place * &world.get(g).cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        for g in &diff.changed {
            let row = self.scene.guid_to_row[g];
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], placed(g), row);
        }
        for g in &diff.added {
            let row = self.scene.assign_row(g);
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], placed(g), row);
        }
        self.scene.commit(new, &diff);
    }
```

46's `reload` becomes: fetch + parse in its task, `apply_session(new)` when its message lands —
watch rides the exact same channel below.

> **Which doc is the watcher's?** 46's reconcile diffs against the scene as loaded, but `Doc` stores
> `name`/`place`/`session`
> — *not* the url it was fetched from, so a url arriving from a poll can't be traced to a doc through
> the scene alone. The watcher has to carry the mapping itself: key the watch by the **manifest item**
> (its `file` is the url, its `name` names the doc), or — once 46's reload lands a source url on `Doc`
> — read it from there. This lesson polls one url and reconciles the doc it loaded as; the honest
> "watch every doc" is one poll task per manifest item, same guard each.
>
> **And with more than one doc loaded, `docs.first()` is WRONG, not merely incomplete.** `hashes`
> spans every doc (35's global rows), so a watched session is diffed against the *whole scene*:
> the other docs' objects bucket as `removed` and get freed off the GPU, and `commit` overwrites
> `docs[0]`'s session even if the watched url loaded as doc 3. The code in this lesson is correct
> for the one-doc scene it demos; a multi-doc watch needs per-doc `hashes`/`reconcile` (diff the
> incoming session against one doc's guids) before any of it is safe.

## Step 2 — one fingerprint for the whole file: `src/app/persistence.rs`

Object-level `content_hash` (49) answers "which objects changed"; the watcher first needs a cheaper
"did the *file* change at all", and the same number doubles as the self-write guard. Hash the raw bytes:

```rust
pub fn file_hash(bytes: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut h);
    h.finish()
}
```

## Step 3 — poll, guard, parse — off the borrow: `src/state.rs`

The watcher is another frame-counter job (like 44's debounce): every `WATCH_POLL_FRAMES`, re-fetch and
compare. `watch_seen` remembers what a poll last *saw*; `self_write_hash` remembers what we last *wrote*
(set by 44's save). A change is real only if it differs from both.

The whole poll runs in a spawned task, because both halves of its work are `async`: the fetch (34) and
the parse — `session_from_bytes_chunked` is the *only* parse the viewer has, and it awaits between
chunks. The task owns just a url, the two guard values, and a proxy; it never touches `&mut self`. What
it delivers is a **ready `Session`**, on the same channel the loader already uses (Step 4).

One plumbing move first: the task must send events, and lib.rs's loader currently `take()`s the proxy.
Give `State` a clone — add a `pub proxy: winit::event_loop::EventLoopProxy<crate::Msg>` field, and in
lib.rs's loader find `State::new(window.clone(), Scene::new()).await` (42's empty boot) →
`State::new(window.clone(), Scene::new(), proxy.clone()).await` (and thread the parameter through `State::new` into the struct).

```rust
    // add to struct State (init in State::new):
    //   pub watch_url: String,                          // the ONE url this lesson polls (see the doc note above)
    //   pub self_write_hash: Option<u64>,               // hash of OUR last 44 save
    //   watch_seen: std::rc::Rc<std::cell::Cell<u64>>,  // last hash a poll saw — watch_seen: Rc::new(Cell::new(0))
    //   watch_in_flight: std::rc::Rc<std::cell::Cell<bool>>,  // a slow poll must not overlap the next spawn
    //   pub proxy: winit::event_loop::EventLoopProxy<crate::Msg>,   // the loader's channel, cloned in
    const WATCH_POLL_FRAMES: u64 = 60;   // ~1 s

    // add inside render(), after 44's `self.frame += 1;` — the poll: spawn a task that fetches,
    // guards, parses, and only on a REAL external change sends a Msg::Watched. The in-flight flag
    // stops a fetch+parse that takes LONGER than the poll interval from stacking a second task on
    // top of the first (two parses, and seen.set racing itself).
    if self.frame % WATCH_POLL_FRAMES == 0 && !self.watch_in_flight.get() {
        let url = self.watch_url.clone();
        let seen = self.watch_seen.clone();   // clone the Rc, not the data
        let own = self.self_write_hash;       // captured by copy — fresh at every spawn
        let proxy = self.proxy.clone();
        let in_flight = self.watch_in_flight.clone();
        in_flight.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            // cleared on EVERY exit, early return included — single-threaded wasm, no Mutex
            struct ClearOnDrop(std::rc::Rc<std::cell::Cell<bool>>);
            impl Drop for ClearOnDrop { fn drop(&mut self) { self.0.set(false); } }
            let _clear = ClearOnDrop(in_flight);

            let bytes = match crate::app::persistence::fetch_bytes(&url).await {
                Ok(b) if !b.is_empty() => b,
                // fetch failed / empty — leave the scene as-is
                _ => return,
            };
            let h = crate::app::persistence::file_hash(&bytes);
            // file byte-identical to last poll → nothing to do
            if h == seen.get() { return; }
            seen.set(h);
            // ← self-write guard: this is our OWN 44 save
            if Some(h) == own {
                log::info!("watch: ignoring our own write");
                return;
            }
            log::info!("watch: external change detected");
            // parse OFF the borrow — the task owns url/bytes only; the async chunked parse (34)
            let new = crate::app::persistence::session_from_bytes_chunked(&url, &bytes).await;
            // deliver a READY Session on the loader's channel; Step 4 applies it
            let _ = proxy.send_event(crate::Msg::Watched(url, new));
        });
    }
```

> **Honesty check: with 44's download-shelf save, the self-write guard is currently dead code.** A
> browser download never writes the polled URL — the served file only changes when *something else*
> writes it, which is exactly what we want to reconcile. `Some(h) == own` can fire only if the user
> manually drops the downloaded file back onto the server's path. Keep the guard — it earns its
> keep the moment the File System Access upgrade saves in place — but know which save path it
> belongs to, and don't read its silence as proof it's working.

**Close the loop with 59's save.** When Step 4 of lesson 59 writes bytes, stamp their hash so the very
next poll recognizes them as ours:

```rust
    // in 44's save trigger, right after building `bytes` and before/after download_bytes:
    self.self_write_hash = Some(crate::app::persistence::file_hash(&bytes));
```

> **Why the guard is a hash, not a flag.** A boolean "I just saved, ignore the next change" is fragile:
> a *concurrent* external edit between your save and the next poll would be swallowed by the flag. Keying
> on the actual bytes means only the file that *equals your write* is ignored — any genuinely different
> content, even one that lands in the same poll window, still reconciles. (If the file's `lastModified`
> is available via the File System Access upgrade, guard on that too — belt and suspenders.)

## Step 3b — idle polls should cost ~0 bytes: conditional fetch

The poll above downloads the **whole file every second** just to hash it and throw it away — fine
for a 2 MB `.pb` on localhost, rude for anything bigger or remote. HTTP already has the answer:
conditional requests. Remember the response's `ETag` (or `Last-Modified`) and send it back as
`If-None-Match` / `If-Modified-Since`; an unchanged file answers `304 Not Modified` with an **empty
body**, and the poll becomes ~0 bytes when idle. Clone 34's `fetch_bytes` into a
`fetch_conditional` — one request header in, two response fields out:

```rust
// persistence.rs — same fetch as 34, plus:
//   request:  header `If-None-Match: <last etag>` when we have one
//   response: (status, etag: Option<String>, bytes) — status 304 ⇒ bytes empty, STOP (skip hash,
//             skip parse); 200 ⇒ stash etag, fall through to the hash + guard as before
```

Every static dev server (trunk, `python -m http.server`, nginx) answers conditional GETs out of the
box. Keep the body hash as the second line of defense: a tool that *rewrites identical bytes*
(re-export, `touch`) bumps `Last-Modified` but not the content — the 304 misses it, the hash catches
it. Two layers, each cheap where the other isn't.

> **This poll, too, rides the frame clock — and 71 stops it.** Like 44's debounce, `frame %
> WATCH_POLL_FRAMES` only ticks while `render` runs. Under 71's render-on-demand an *idle* app draws
> no frames — which is exactly when an external edit would arrive. From that lesson on, drive the
> poll off a real timer (a `setInterval`-backed future), not the frame counter.

> **Collision policy: last writer wins, and the loser is YOU.** An incoming change is applied
> wholesale — reconcile overwrites any in-viewer edit to an object the external writer also
> touched, so unsaved local work silently loses. That is a defensible default (the file is the
> source of truth — this phase's whole premise), but it should be a *stated* default: a shipping
> app at minimum logs a warning when the diff touches guids in 44's `dirty` set, and optionally
> refuses to apply while unsaved edits exist. Noted, not built.

## Step 4 — deliver on the loader's channel: `src/lib.rs`

The task can't call `apply_session` itself (that needs `&mut self`), but the viewer already has the
machinery for exactly this shape: the loader's `Msg` events. `Msg::Ready` delivers a built `State`,
`Msg::File` a parsed document — `Msg::Watched` is their sibling, a parsed *replacement*:

```rust
    // find, in lib.rs's `pub enum Msg`, and add under the File variant:
    Watched(String, session_rust::Session),   // (url, freshly parsed) — an external change to watch_url
```

and the arm that applies it, in `user_event`'s `match msg`, right next to the `Msg::File` arm:

```rust
    Msg::Watched(url, new) => {
        let Some(state) = &mut self.state else { return };
        state.apply_session(new);   // 46's incremental diff — only what moved
        log::info!("watched '{}' applied", url);
        state.window.request_redraw();
    }
```

> **Borrow reality.** `apply_session` needs `&mut self`, but a spawned future can't hold that across an
> `await`. The clean shape is the one the loader already proved in 34–35: the task owns only what it
> fetches, and the finished value arrives as a user event — `Msg::Watched` next to `Msg::File`, no
> hand-rolled queue to drain. The one crumb the polls keep *between* tasks — the last hash seen — rides
> an `Rc<Cell<u64>>`: single-threaded wasm, no `Mutex`; the tasks are its only writer, and a `Cell` of
> one `u64` is a cursor, not an inbox.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770 — serving session_data/ too
```

- **External edit flows in.** With the viewer open on `floor_model.pb`, edit that file from another
  process — e.g. load it in `session_rust`, move one object, `pb_dump` it back over the same path — and
  within ~1 s the console logs `watch: external change detected` then `sync: 0 added, 1 changed, …`, and
  that one object updates. Nothing else redraws (46's diff).
- **Own save doesn't loop.** Trigger a 44 save (Ctrl+S), then copy the downloaded file back over the
  served path (a browser download never touches the polled URL itself — Step 3's honesty check — so
  the manual copy is what exercises this). The next poll sees matching bytes and
  logs `watch: ignoring our own write`, no reconcile. Without the guard, this is exactly where a
  save→watch→save loop would start once saves write in place; watch it *not* happen.
- **Idle is quiet.** Leave it untouched: every poll hashes identical bytes and returns early — no log
  spam, no GPU work.

## Recap

```
Ch 44: save — write the file out, gated by dirty + debounce + hash.
Ch 45: WATCH — external edits flow back IN, closing the 3-way sync. Browser can't watch a
       filesystem, so POLL the same URL (34's fetch) every ~1 s and hash the bytes (file_hash
       over the whole Vec<u8>). Unchanged hash → skip. Changed → session_from_bytes_chunked
       (fetched AND parsed in the poll task) → Msg::Watched on the loader's channel → 46's
       reconcile → apply_session (the apply half of 46's reload, factored so manual reload and
       the watcher share it) → only what moved hits the GPU. The SELF-WRITE GUARD
       (self_write_hash, stamped by 44's save) drops any change whose bytes equal our own last
       write — that's what stops save→watch→reload→save from looping; keying on the hash (not
       a bool flag) means a concurrent external edit in the same window still reconciles.
       (Dead code while saves are browser downloads — they never touch the polled URL; it wakes
       with in-place File System Access saves.) A conditional fetch (If-None-Match → 304) makes
       idle polls ~0 bytes; an in-flight flag keeps a slow poll from overlapping the next spawn.
       Multi-doc warning: hashes/reconcile are scene-global, so docs.first() is WRONG with >1 doc
       (other docs diff as "removed") — per-doc hashes first. Poll and debounce both ride the
       frame clock, which 71's render-on-demand stops — timer-driven from there. Collision policy:
       last writer wins, stated.
       Transport upgrades noted, not built: File System Access handle (lastModified, unstable
       web-sys) and a watcher→WebSocket push; both reuse the same reconcile. Fetch + parse stay
       off the &mut self borrow by riding the loader's proxy (Msg::Watched next to Msg::File);
       the last-seen hash is an Rc<Cell<u64>> cursor the poll tasks own.
```

Edited: `app/persistence.rs` (`file_hash` — whole-file fingerprint; `fetch_conditional` sketch),
`state.rs` (`apply_session` factored from 46's reload, the spawned poll task — fetch, guard, chunked
parse, `watch_in_flight` overlap guard — plus `self_write_hash` stamped by 44's save), `lib.rs`
(`Msg::Watched` + its `user_event` arm; proxy clone handed to `State`).

## Next

`63-screen-to-ray.md` — Phase 7 opens: picking. The mouse is a 2D point; everything selectable is 3D. The
next lesson unprojects the cursor through the inverse `view_proj` into a world-space ray — with the
`ndc_z = 0.5` far-point trick that dodges a real precision bug — the ray every later pick (mesh, edge,
vertex, line) is cast from.
