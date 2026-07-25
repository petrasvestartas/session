# 40 Watch — external edits flow back in

> **Big picture.** *Phase 6 closes.* Load (34), save (39) — the last direction is external edits
> flowing in *while the viewer runs*: a script regenerates the `.pb`, a teammate exports, another
> tool writes — and the viewer just updates. That's what makes the file a live document shared with
> other tools instead of an import. The reaction is already built (38b's reconcile); what's new here
> is the transport and one classic trap — not reacting to your *own* saves.

Three sync directions close the loop: 34 reads a file in, 39 writes it out, and this lesson watches it
— when *something else* edits the file (another tool, a script, a teammate's export), the viewer picks
up the change and reconciles it, without a manual reload. It's the last piece that makes the `.pb` a
live source of truth instead of a one-shot import.

Two problems to solve. First, **the browser can't watch a filesystem** — there's no inotify in a tab.
Second, **your own saves change the file too** (39), so a naive watcher would see its own write, reload,
and potentially loop. Both have clean answers, and the *reaction* to a change is already built: it's
38's `reconcile`.

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="every N frames the viewer re-fetches the file, hashes the bytes, and if the hash changed AND is not its own last save it runs 38 reconcile; otherwise it skips" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="120" height="34" fill="none" stroke="#6fb3ff"/><text x="70" y="51" fill="#d7dae0" text-anchor="middle">poll: fetch bytes</text>
  <text x="70" y="80" fill="#666" text-anchor="middle" font-size="9">every N frames (34)</text>
  <rect x="168" y="30" width="120" height="34" fill="none" stroke="#6fb3ff"/><text x="228" y="51" fill="#d7dae0" text-anchor="middle">hash bytes</text>
  <path d="M326,47 l-30,-14 l0,28 z" fill="none" stroke="#6fb3ff"/><text x="360" y="34" fill="#888">same?</text>
  <rect x="404" y="12" width="150" height="24" fill="none" stroke="#3a3a3a"/><text x="479" y="28" fill="#888" text-anchor="middle">unchanged → skip</text>
  <rect x="404" y="44" width="150" height="24" fill="none" stroke="#3a3a3a"/><text x="479" y="60" fill="#888" text-anchor="middle">== our last save → skip</text>
  <rect x="404" y="76" width="150" height="24" fill="none" stroke="#6fb3ff"/><text x="479" y="92" fill="#d7dae0" text-anchor="middle">changed → reconcile (38)</text>
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
src/state.rs             # poll every N frames; self-write guard; apply_reload factored out of 39/38
```

## Step 1 — factor the apply path out of reload: `src/state.rs`

38's `reload` fetched *and* applied. Watch needs the apply half on its own (it already has the bytes), so
split it:

```rust
    /// Diff `new` against the loaded scene and push only the changes to the GPU — the body of 38's
    /// reload, minus the fetch. Both manual reload and the watcher call this.
    fn apply_session(&mut self, new: Session) {
        let diff = self.scene.reconcile(&new);
        let unchanged = self.scene.order.len() - diff.changed.len() - diff.removed.len();
        log::info!("sync: {} added, {} changed, {} removed, {} unchanged",
            diff.added.len(), diff.changed.len(), diff.removed.len(), unchanged);
        for g in &diff.removed {
            let row = self.scene.guid_to_row[g];
            self.gpu.remove_object(g); self.gpu.hide_row(row); self.scene.free_row(g);
        }
        for g in &diff.changed {
            let row = self.scene.guid_to_row[g];
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], row);
        }
        for g in &diff.added {
            let row = self.scene.assign_row(g);
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], row);
        }
        self.scene.commit(new);
    }
```

38's `reload` becomes: fetch, then `self.apply_session(new)`.

## Step 2 — one fingerprint for the whole file: `src/app/persistence.rs`

Object-level `content_hash` (38) answers "which objects changed"; the watcher first needs a cheaper
"did the *file* change at all", and the same number doubles as the self-write guard. Hash the raw bytes:

```rust
pub fn file_hash(bytes: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut h);
    h.finish()
}
```

## Step 3 — poll, guard, reconcile: `src/state.rs`

The watcher is another frame-counter job (like 39's debounce): every `WATCH_POLL_FRAMES`, re-fetch and
compare. `last_file_hash` remembers what we last *saw*; `self_write_hash` remembers what we last *wrote*
(set by 39's save). A change is real only if it differs from both.

```rust
    // add to State: last_file_hash: u64, self_write_hash: Option<u64>, watch_url: String,
    //               watch_inbox: Rc<RefCell<Option<(u64, Vec<u8>)>>>   (used by Step 4's queue)
    // and at the top of state.rs: use std::rc::Rc; use std::cell::RefCell;
    // initialize all four in State::new (watch_inbox: Rc::new(RefCell::new(None))).
    const WATCH_POLL_FRAMES: u64 = 60;   // ~1 s

    /// Called once per frame from render(). Cheap when nothing changed: one fetch + one hash.
    async fn poll_watch(&mut self) {
        if self.frame % WATCH_POLL_FRAMES != 0 { return; }
        let bytes = match crate::app::persistence::fetch_bytes(&self.watch_url).await {
            Ok(b) if !b.is_empty() => b,
            // fetch failed / empty — leave the scene as-is
            _ => return,
        };
        let h = crate::app::persistence::file_hash(&bytes);
        // file byte-identical to last poll → nothing to do
        if h == self.last_file_hash { return; }
        self.last_file_hash = h;
        // ← self-write guard: this is our OWN 39 save
        if Some(h) == self.self_write_hash {
            log::info!("watch: ignoring our own write");
            return;
        }
        log::info!("watch: external change detected");
        let new = crate::app::persistence::session_from_bytes(&self.watch_url, &bytes);
        // 38's incremental diff — only what moved
        self.apply_session(new);
    }
```

**Close the loop with 39's save.** When Step 4 of lesson 39 writes bytes, stamp their hash so the very
next poll recognizes them as ours:

```rust
    // in 39's save trigger, right after building `bytes` and before/after download_bytes:
    self.self_write_hash = Some(crate::app::persistence::file_hash(&bytes));
```

> **Why the guard is a hash, not a flag.** A boolean "I just saved, ignore the next change" is fragile:
> a *concurrent* external edit between your save and the next poll would be swallowed by the flag. Keying
> on the actual bytes means only the file that *equals your write* is ignored — any genuinely different
> content, even one that lands in the same poll window, still reconciles. (If the file's `lastModified`
> is available via the File System Access upgrade, guard on that too — belt and suspenders.)

## Step 4 — drive the poll: `src/state.rs`

`poll_watch` above is the guard logic in one place, but it can't be called directly: its `.await`
would hold `&mut self` across the fetch, and `render` is synchronous. Split it the way the note below
describes — a tiny async task owns only the URL and drops `(hash, bytes)` into `watch_inbox`; the
**synchronous** top of the next `render` drains the inbox and runs the same guard + `apply_session`.
Both halves go inside `render()`:

```rust
    // TOP of render(), synchronous — drain last frame's fetch, then guard + apply.
    // take() empties the RefCell and drops the borrow on this line, so `self` is free below.
    let inbox_msg = self.watch_inbox.borrow_mut().take();   // Option<(u64, Vec<u8>)>
    if let Some((h, bytes)) = inbox_msg {
        if h != self.last_file_hash {
            self.last_file_hash = h;
            if Some(h) == self.self_write_hash {
                log::info!("watch: ignoring our own write");     // ← self-write guard
            } else {
                log::info!("watch: external change detected");
                let new = crate::app::persistence::session_from_bytes(&self.watch_url, &bytes);
                self.apply_session(new);                          // 38's incremental diff
            }
        }
    }

    // later in render(), once per WATCH_POLL_FRAMES — spawn the fetch; it never touches `self`:
    if self.frame % WATCH_POLL_FRAMES == 0 {
        let url = self.watch_url.clone();
        let inbox = self.watch_inbox.clone();                     // clone the Rc, not the data
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(b) = crate::app::persistence::fetch_bytes(&url).await {
                if !b.is_empty() {
                    let h = crate::app::persistence::file_hash(&b);
                    *inbox.borrow_mut() = Some((h, b));
                }
            }
        });
    }
```

The drain repeats `poll_watch`'s guard because that logic now lives on the sync side; keep `poll_watch`
as the readable one-place reference (the Recap points back to it) — it just isn't the thing wired to run.

> **Borrow reality.** `apply_session` needs `&mut self`, but a spawned future can't hold that across an
> `await`. The clean shape is the classic split: the async task only *fetches* (owns just the URL), drops
> its result into a shared queue, and the **synchronous** top of the next `render` drains the queue and
> calls `apply_session`. Same logic as `poll_watch` above, just with the `await` moved off the borrow.
> Keep the queue a `Rc<RefCell<Option<(u64, Vec<u8>)>>>` — single-threaded wasm, no `Mutex` needed.

`session_data/floor_model.pb` is served static, so for a first cut you can even skip the async dance:
poll it with a blocking-style fetch behind a dev flag and confirm the reconcile fires; wire the
channel once it works.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770 — serving session_data/ too
```

- **External edit flows in.** With the viewer open on `floor_model.pb`, edit that file from another
  process — e.g. load it in `session_rust`, move one object, `pb_dump` it back over the same path — and
  within ~1 s the console logs `watch: external change detected` then `sync: 0 added, 1 changed, …`, and
  that one object updates. Nothing else redraws (38's diff).
- **Own save doesn't loop.** Trigger a 39 save (Ctrl+S). The file changes, the next poll sees it — and
  logs `watch: ignoring our own write`, no reconcile. Without the guard, this is exactly where a
  save→watch→save loop would start; watch it *not* happen.
- **Idle is quiet.** Leave it untouched: every poll hashes identical bytes and returns early — no log
  spam, no GPU work.

## Recap

```
Ch 39: save — write the file out, gated by dirty + debounce + hash.
Ch 40: WATCH — external edits flow back IN, closing the 3-way sync. Browser can't watch a
       filesystem, so POLL the same URL (34's fetch) every ~1 s and hash the bytes (file_hash
       over the whole Vec<u8>). Unchanged hash → skip. Changed → session_from_bytes → 38's
       reconcile → apply_session (the apply half of 38's reload, factored so manual reload and
       the watcher share it) → only what moved hits the GPU. The SELF-WRITE GUARD
       (self_write_hash, stamped by 39's save) drops any change whose bytes equal our own last
       write — that's what stops save→watch→reload→save from looping; keying on the hash (not
       a bool flag) means a concurrent external edit in the same window still reconciles.
       Transport upgrades noted, not built: File System Access handle (lastModified, unstable
       web-sys) and a watcher→WebSocket push; both reuse the same reconcile. Async fetch is
       kept off the &mut self borrow via a next-frame queue.
```

Edited: `app/persistence.rs` (`file_hash` — whole-file fingerprint), `state.rs` (`apply_session` factored
from 38's reload, `poll_watch` + self-write guard, per-frame non-blocking poll, `self_write_hash` stamped
by 39's save).

## Next

`41-screen-to-ray.md` — Phase 7 opens: picking. The mouse is a 2D point; everything selectable is 3D. The
next lesson unprojects the cursor through the inverse `view_proj` into a world-space ray — with the
`ndc_z = 0.5` far-point trick that dodges a real precision bug — the ray every later pick (mesh, edge,
vertex, line) is cast from.
