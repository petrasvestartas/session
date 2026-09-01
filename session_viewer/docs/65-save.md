# 65 Save — write the file back, only when something changed

> **Big picture.** *Phase 6 — the file is the source of truth.* 49 reads changes IN; this lesson
> writes changes OUT — quietly. Real CAD apps never write on every edit: **dirty-flag → wait for
> edits to settle → write only if something truly changed** is the standard pattern, and 49's
> content hash is exactly the "truly changed" test, reused.

Lesson 34 read a `Session` in; this one writes it back out. But a CAD app doesn't save on every
keystroke — it saves when edits *settle*, and it doesn't rewrite a file that didn't actually change.
So save is three gates in front of one `pb_dumps`: a **dirty flag** (did anything get touched?), a
**debounce** (have edits stopped for ~1 s?), and a **hash check** (did the touched objects *really*
change, or get nudged and reverted?). Only past all three does a byte hit the disk.

`wasm32` has no filesystem, so "the disk" is a browser download (a `Blob` + a synthetic
`<a download>` click) — the mirror of 34's fetch. The dirty + content-hash plumbing is exactly what 46
already built for reconcile, now run in reverse.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="an edit marks the scene dirty; a debounce waits for edits to settle; a hash check skips unchanged objects; only then does pb_dumps produce bytes for a Blob download" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="40" width="96" height="34"/>
    <rect x="132" y="40" width="110" height="34"/>
    <rect x="270" y="40" width="120" height="34"/>
    <rect x="418" y="40" width="110" height="34"/>
    <rect x="556" y="40" width="112" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="56" y="61">edit → dirty</text>
    <text x="187" y="57">debounce</text><text x="187" y="69" fill="#666" font-size="9">~1 s settled</text>
    <text x="330" y="57">hash check</text><text x="330" y="69" fill="#666" font-size="9">really changed?</text>
    <text x="473" y="57">pb_dumps</text><text x="473" y="69" fill="#666" font-size="9">Vec&lt;u8&gt;</text>
    <text x="612" y="57">Blob download</text><text x="612" y="69" fill="#666" font-size="9">&lt;a download&gt;</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.4">
    <line x1="104" y1="57" x2="130" y2="57" marker-end="url(#ah39)"/>
    <line x1="242" y1="57" x2="268" y2="57" marker-end="url(#ah39)"/>
    <line x1="390" y1="57" x2="416" y2="57" marker-end="url(#ah39)"/>
    <line x1="528" y1="57" x2="554" y2="57" marker-end="url(#ah39)"/>
  </g>
  <text x="330" y="104" fill="#888" text-anchor="middle">nothing dirty, or edits reverted → hash unchanged → ZERO writes</text>
  <text x="330" y="124" fill="#666" text-anchor="middle">the three gates are why save is quiet, not chatty</text>
  <defs><marker id="ah39" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
# web-sys: Blob, BlobPropertyBag, Url, HtmlAnchorElement (the download path)
Cargo.toml
# save half: session_to_bytes (pb_dumps) + download_bytes (Blob → <a download>)
src/app/persistence.rs
# dirty: HashSet<guid>; mark_dirty; save_if_changed → Option<Vec<u8>> (hash gate)
src/app/scene.rs
src/state.rs             # frame-count debounce + Ctrl+S; fire the download when edits settle
```

## Step 1 — Session → bytes: `src/app/persistence.rs`

The exact inverse of 34's `session_from_bytes_chunked` (minus the chunking — dumping is one
synchronous pass), and the same entry points every language's minitest round-trips — just the dump
side. Add next to the load functions (`Session` is already imported at the top of the file):

```rust
/// `.pb` → prost bytes, `.json` → serde string-as-bytes; dispatched on the target
/// filename's extension. Both dumpers already exist on `Session` (every minitest
/// round-trips them); here we feed the bytes to a browser download instead of `std::fs`.
pub fn session_to_bytes(filename: &str, session: &Session) -> Vec<u8> {
    if filename.ends_with(".json") {
        session.file_json_dumps().into_bytes()
    } else {
        session.pb_dumps()
    }
}
```

Placements ride the dump too: `Session.xforms` (proto tag 7) carries every OBJECT placement, so an
in-viewer move must go through `session.set_xform(guid, …)` for `pb_dumps` to write it. (The doc's
manifest `place` is viewer bookkeeping — it never enters the `Session`, so a save cannot bake the
sheet placement into the file.) And `pb_dumps` on a freshly mutated session is safe as-is — the
kernel syncs its object table inside the dump itself (P2 of the datastructure plan), no manual
refresh step.

## Step 2 — bytes → a download: `src/app/persistence.rs` (new ground)

`wasm32` can't write a path, so hand the bytes to the browser as a `Blob` and trigger a synthetic
download — the standard web pattern, and the mirror image of 34's fetch (`JsCast` and `JsValue`
are already imported at the top of this file — the fetch half uses both):

```rust
/// Save `bytes` as `filename` via a browser download: wrap in a Blob, mint an object URL, click a
/// hidden `<a download>`, then revoke the URL — on the NEXT task: revoking synchronously races
/// the click in Safari, which re-reads the URL after the handler returns and cancels the
/// download if it's already dead.
/// There is no silent filesystem write on the web — the
/// user always sees (and confirms) the download, by design.
pub fn download_bytes(filename: &str, bytes: &[u8]) -> Result<(), JsValue> {
    let array = js_sys::Array::new();
    array.push(&js_sys::Uint8Array::from(bytes));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&array)?;
    let url = web_sys::Url::create_object_url_with_blob(&blob)?;

    let document = web_sys::window().ok_or("no window")?.document().ok_or("no document")?;
    let a: web_sys::HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
    a.set_href(&url);
    a.set_download(filename);
    a.click();
    // Defer the revoke (the Safari race above). once_into_js hands the closure to JS — no
    // forget() leak.
    let revoke = wasm_bindgen::closure::Closure::once_into_js(move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    });
    web_sys::window().ok_or("no window")?
        .set_timeout_with_callback_and_timeout_and_arguments_0(revoke.unchecked_ref(), 0)?;
    Ok(())
}
```

> **A save is a download — the user SEES every one.** Each trigger drops an entry onto the
> browser's download shelf (and Chrome interposes a "this site is downloading multiple files"
> prompt if saves come fast). For an app that saves every settled edit burst that shelf is noise —
> the File System Access API below (writes in place, no download at all) is the real fix, not a
> nicety. Until then: keep the debounce honest, and never wire `touch` to anything chattier than
> an edit.

Add the features to `Cargo.toml`'s `web-sys` list (beside 34's `Request`/`Response`):

```toml
    "Blob",
    "BlobPropertyBag",
    "Url",
    "HtmlAnchorElement",
```

> **New ground, checked against the crate.** Neither the archive nor today's viewer ships a save path —
> like 34's fetch, this is a documented-but-unbuilt "Phase 1" feature. `Blob::new_with_u8_array_sequence`,
> `Url::create_object_url_with_blob` / `revoke_object_url`, and `HtmlAnchorElement::{set_href,
> set_download, click}` are all `web-sys 0.3` (confirmed in `Cargo.lock`); the *flow* is new and wants a
> real browser click-through before you trust it. The heavier alternative — the **File System Access
> API** (`showSaveFilePicker`, writes in place, no re-download) — is a later refinement; the Blob path
> needs no permission prompt and works in every browser the viewer targets.

## Step 3 — the hash gate: only real changes count: `src/app/scene.rs`

46 built `content_hash` + `Scene.hashes` (the last-saved fingerprints). Save reuses them: an object is
only *really* changed if its current hash differs from the stored one. One subtlety the Xform refactor
forces: a pure MOVE never touches the geometry's bytes — placements live in `Session.xforms`, not on
the object — so the fingerprint must fold the placement in with the object's sorted-JSON bytes.
The fold lives in 49's `content_hash(geom, &Xform)`, and the frame MUST match 49's baselines
exactly: the composed `world_xforms()[guid]` (one downward pass), not the object's own
`session.xform(guid)` — for a parented object the two differ, and a gate hashing a different
frame than the stored baseline would flag it dirty forever. Reconcile needs the same fold for
the same reason: a reload that only *moves* an object must still bucket as changed — without
it, a moved-but-otherwise-untouched object would read as unchanged.

Two 35 realities as well: a `Scene` is now *several* docs, and a save targets ONE of them —
`save_if_changed` takes the doc index (the trigger below saves `docs[0]`; a shipping Ctrl+S saves the
*active* doc, or loops over all docs — pick one policy, and split `dirty` per doc when you do: as
written, a guid dirtied in another doc reads as "removed" from the doc being saved). Add a
dirty set and the gate (the field goes in `struct Scene`, its init in `Scene::new`'s `Self { … }`, the
two methods in `impl Scene`):

```rust
    // add to struct Scene:
    // guids touched since the last save (editing lessons fill this)
    pub dirty: std::collections::HashSet<String>,

    // add to Scene::new's Self { … } initializer:
    //   dirty: std::collections::HashSet::new(),

    // add to impl Scene:
    pub fn mark_dirty(&mut self, guid: &str) { self.dirty.insert(guid.to_string()); }

    /// Bytes to write for ONE doc, or None if nothing actually changed. Re-hashes each
    /// dirty object against its stored fingerprint: a nudge that got reverted hashes back
    /// to the same value → not a real change. On a real change, refreshes `hashes`
    /// so the NEXT save's gate starts clean, and clears `dirty`.
    pub fn save_if_changed(&mut self, doc: usize, filename: &str) -> Option<Vec<u8>> {
        let session = &self.docs.get(doc)?.session;   // per-DOC: which loaded file to dump
        // SAME frame as 49's baselines: composed world placements, one downward pass.
        let world = session.world_xforms();
        let ident = Xform::identity();
        let placed = |g: &str| world.get(g).unwrap_or(&ident);
        let real: Vec<String> = self.dirty.iter()
            .filter(|g| session.lookup.get(*g)
                // removed or hash≠ (content_hash folds the placement — see above)
                .map_or(true, |geom| self.hashes.get(*g)
                    != Some(&content_hash(geom, placed(g)))))
            .cloned().collect();
        self.dirty.clear();
        // debounce fired, but nothing truly moved
        if real.is_empty() { return None; }

        for g in &real {
            match session.lookup.get(g) {
                Some(geom) => {
                    self.hashes.insert(g.clone(), content_hash(geom, placed(g)));
                }
                None => { self.hashes.remove(g); }
            }
        }
        Some(crate::app::persistence::session_to_bytes(filename, session))
    }
```

## Step 4 — debounce + trigger: `src/state.rs`

The viewer already runs a per-frame loop, so the debounce is a frame counter — no timer API. An edit
stamps `dirty_since` with the current frame; `render` checks whether edits have been quiet long
enough, then runs the gate:

Add the two fields to `struct State` — and initialize both in `State::new`'s `Ok(Self { … })`,
or the struct won't compile:

```rust
    // add to struct State:
    pub dirty_since: Option<u64>,   // frame of the most recent edit, None once settled
    pub frame: u64,                 // monotonic frame counter — nothing else advances it
```
```rust
    // add to State::new's Ok(Self { … }) initializer:
    dirty_since: None,
    frame: 0,
```

`frame` is the debounce clock, so *something* must advance it. Nothing does yet — add the increment
as the first line of `render()`, right beside the `request_redraw()` treadmill:

```rust
    // find, at the top of State::render():
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        self.frame += 1;   // WITHOUT this the debounce below is dead (frame - since is always 0)
        // …existing render body…
```

Now the module-level constants, the `touch` entry point, and the debounce gate itself:

```rust
    // add near the top of state.rs (module scope):
    const SAVE_DEBOUNCE_FRAMES: u64 = 60;   // ~1 s at 60 fps
    const SAVE_FILENAME: &str = "session.pb";

    // add to impl State:
    /// Editing code calls this on every mutation (gumball drag, delete, …).
    /// Cheap: just stamps a frame.
    pub fn touch(&mut self, guid: &str) {
        self.scene.mark_dirty(guid);
        self.dirty_since = Some(self.frame);
    }
```

```rust
    // add inside render(), after `self.frame += 1;`, before the clear/draw returns:
    if let Some(since) = self.dirty_since {
        if self.frame - since >= SAVE_DEBOUNCE_FRAMES {
            self.dirty_since = None;                               // one save per settled burst
            // doc 0 — see Step 3's policy note
            if let Some(bytes) = self.scene.save_if_changed(0, SAVE_FILENAME) {
                let _ = crate::app::persistence::download_bytes(SAVE_FILENAME, &bytes);
                log::info!("saved {} bytes", bytes.len());
            } else {
                log::info!("save skipped — nothing changed");
            }
        }
    }
```

> **Why a frame counter, not `setTimeout`.** The render loop is already ticking, so "has it been 60
> frames since the last edit?" needs no JS timer, no channel, no `gloo` dependency — and it naturally
> coalesces a *burst* of edits into one save: each edit pushes `dirty_since` forward, so the 60-frame
> countdown only completes once edits actually stop. A wall-clock debounce (`performance.now()`) is the
> swap-in if you ever decouple saves from the frame rate; the logic is identical.
>
> **That swap becomes mandatory at 78.** Today `render` runs every frame, so `frame` always advances.
> 71's render-on-demand ticks `render` only when something pokes a redraw — an idle app produces no
> frames, the 60-frame countdown after the *last* edit of a burst never completes, and the save
> silently never fires. From that lesson on, drive the debounce off a wall-clock timer (or have
> `touch` schedule a wake-up), not the frame counter.

New objects created in-viewer (later lessons) need a `guid` before they can be saved — the kernel mints
one lazily (`Geometry::guid()` fills its `OnceLock` on first read), so a freshly-built object already
has a stable id by the time `pb_dumps` walks it. Nothing to do here; just worth knowing the id exists.

## Step 4b — streamed clouds cannot be saved

`pb_dumps` serializes kernel sessions; a `CloudSlot` (42) has none — its points exist
only in GPU memory, and pulling 165 MB back through a mapped readback to re-encode what
is already bit-identical in the source file would be absurd. So save walks `docs` only,
and the dirty flag never fires for cloud streaming. If a scene is "two streamed scans +
one edited sheet", saving writes the sheet's file; the scans' files were never touched.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

There's no in-viewer editing yet (that's Phase 7+), so drive the gates manually — wire **Ctrl+S** to
`touch` a known guid (and, for the negative test, a key that calls `mark_dirty` on an object *without*
changing it). Add two arms to the keyboard match in `src/lib.rs` (`self.ctrl` is already tracked by the
`ModifiersChanged` handler; `state.scene` exposes a guid to poke — use whatever's first in the first
doc's `lookup`, the same `docs[0]` the save targets):

```rust
    // find, in lib.rs's `match event.logical_key.as_ref()`, beside the "f"/"F" arm:
    Key::Character("s" | "S") if self.ctrl => {
        // swallow the browser's own Save-Page dialog — winit's web extension trait:
        // `use winit::platform::web::KeyEventExtWebSys;` at the top of lib.rs
        event.prevent_default();
        if let Some(g) = state.scene.docs.first()
            .and_then(|d| d.session.lookup.keys().next().cloned()) {
            state.touch(&g);   // real edit path: mark dirty + stamp dirty_since
        }
    }
    Key::Character("d" | "D") => {
        if let Some(g) = state.scene.docs.first()
            .and_then(|d| d.session.lookup.keys().next().cloned()) {
            state.scene.mark_dirty(&g);          // dirty WITHOUT changing the hash…
            state.dirty_since = Some(state.frame); // …but still arm the debounce (the negative test)
        }
    }
```

- **Edit an object, wait ~1 s** → exactly one download fires, console `saved N bytes`. Edit three
  objects in quick succession → still **one** save after they settle, not three (the debounce coalesced
  them).
- **Mark-dirty without changing anything, wait ~1 s** → console `save skipped — nothing changed`, **no
  download**. This is the hash gate earning its place: a dirty flag alone would have written a
  byte-identical file.
- Re-open the downloaded `.pb` — the viewer loads a *manifest* (`DEMO_SCENE_URL` names it, not a
  session), so drop the saved file where the server can see it and point a manifest item's `file` at
  it (keep the item's `at`/`xform`), then reload → the doc round-trips: what you saved is what you
  get back.

A `#[cfg(test)]` covers the gate without a browser: mark an object dirty but leave it untouched →
`save_if_changed` returns `None`; actually change its hash → returns `Some`, and a second immediate call
returns `None` again (the first refreshed `hashes`).

## Recap

```
Ch 43: reconcile — read a file in, diff by guid, touch only what changed.
Ch 44: SAVE — write ONE doc's file out (docs[0] here; active-vs-all is policy), THREE gates before
       one pb_dumps. (1) dirty: a HashSet<guid>
       editing code fills via touch(); (2) debounce: a frame counter (edit stamps dirty_since;
       save fires only after SAVE_DEBOUNCE_FRAMES of quiet — coalesces a burst into one save,
       no JS timer); (3) hash gate: save_if_changed re-hashes each dirty object against 49's
       stored fingerprint (geometry bytes + composed world xform, so a pure MOVE counts)
       and drops the ones that reverted to their old value — nothing truly
       changed → Option::None → ZERO writes. Past all three, session_to_bytes (pb_dumps /
       file_json_dumps, the dump side of 34) → download_bytes wraps the Vec<u8> in a Blob and
       clicks a synthetic <a download> (wasm has no fs; new ground vs web-sys, like 34's
       fetch). hashes refreshes on a real save so the next gate starts clean. New objects
       already carry a lazily-minted guid, so pb_dumps just works.
```

Edited: `Cargo.toml` (web-sys `Blob`/`Url`/`HtmlAnchorElement`), `app/persistence.rs` (`session_to_bytes`,
`download_bytes` — the save half), `app/scene.rs` (`dirty` set, `mark_dirty`, `save_if_changed` hash gate),
`state.rs` (`touch`, frame-count debounce in `render`, Ctrl+S).

## Next

`66-watch.md` — the third sync direction: an *external* edit to the file flows back in. The browser can't
watch a filesystem, so a File System Access handle polls `lastModified` (or a watcher→WebSocket bridge
pushes), and on change we run 49's `reconcile`. The catch is the loop: our own Step-4 save changes the
file too, so the watcher needs a **self-write guard** — ignore any change whose hash matches the bytes we
just wrote.
