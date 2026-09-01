# 102 Import / export — OBJ in, OBJ out (and the STEP honesty)

> **Big picture.** *Phase 14.* The plan review's most embarrassing find: the kernel has shipped an
> OBJ codec since long before this course — minitested in all three languages — and the viewer only
> ever spoke `.pb`/`.json`. This lesson wires it through: an `.obj` as a **manifest item**, a real
> **Open… file dialog**, and `export obj` via 50's download path. It also triggered a kernel fix:
> the codec was **path-based only** (`read_file_obj(filepath)` — dead on wasm), so the kernel gained
> the `_from_str`/`_to_string` pair (the same `_loads`/`_dumps` split the Session codecs always had),
> ×3 languages + a String Roundtrip minitest.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="obj text enters via a manifest item or a file dialog, parses through the kernel string codec into a mesh, joins the session; export runs the writer and downloads" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="22" width="120" height="26" fill="none" stroke="#6fb3ff"/>
  <text x="70" y="39" fill="#d7dae0" text-anchor="middle">manifest item (35)</text>
  <rect x="10" y="58" width="120" height="26" fill="none" stroke="#6fb3ff"/>
  <text x="70" y="75" fill="#d7dae0" text-anchor="middle">open… dialog</text>
  <g stroke="#6fb3ff" stroke-width="1.2">
    <line x1="130" y1="35" x2="200" y2="48" marker-end="url(#ah79)"/>
    <line x1="130" y1="71" x2="200" y2="58" marker-end="url(#ah79)"/>
  </g>
  <rect x="204" y="40" width="200" height="26" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <text x="304" y="57" fill="#d7dae0" text-anchor="middle">read_file_obj_from_str (kernel)</text>
  <line x1="404" y1="53" x2="450" y2="53" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah79)"/>
  <rect x="454" y="40" width="120" height="26" fill="none" stroke="#6fb3ff"/>
  <text x="514" y="57" fill="#d7dae0" text-anchor="middle">Mesh → Session</text>
  <text x="340" y="104" fill="#888" text-anchor="middle">export = write_file_obj_to_string → 50's Blob download · STEP: C++-only today (kernel-gap #11)</text>
  <defs><marker id="ah79" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
    <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/persistence.rs   # .obj + STEP arms in session_from_bytes_chunked; open_file_dialog
src/app/scene.rs         # first_selected_mesh — export obj's subject
src/app/commands.rs      # `open` (file dialog) + `export obj|pb|json` verbs
src/state.rs, src/lib.rs # State carries a proxy clone — the dialog delivers files as Msg::File
Cargo.toml               # web-sys: HtmlInputElement, File, FileList (the dialog)
```

## Step 1 — import by extension: `src/app/persistence.rs`

35's `session_from_bytes_chunked` dispatches on extension with early returns — the `.json` arm is
the pattern. An OBJ is a bare mesh, not a Session — wrap it. Find:

```rust
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
```

and insert directly below it (plus
`use session_rust::file_obj::read_file_obj_from_str;` with the file's other imports):

```rust
    if url.ends_with(".obj") {
        // an OBJ is one mesh, not a document — wrap it in a fresh Session.
        let mut s = Session::default();
        s.add_mesh(read_file_obj_from_str(&String::from_utf8_lossy(bytes)), None);
        return s;
    }
```

One honesty fix to the comment that used to live in that arm — *"OBJ files are small text, the arm
stays synchronous"* is wrong often enough to matter: 100 MB OBJs exist (scans, exported meshes), and
a synchronous multi-second parse pins the one wasm thread — no preemption, the tab freezes (28's
budget rule). The kernel reader is a line loop, so when a big OBJ shows up, slice the parse exactly
like 35's `.pb` converter (N lines, `yield_now().await`, repeat). Until then the arm is honest for
the megabyte-scale OBJs this course ships.

That alone makes an `.obj` a **manifest citizen** — placement included:

```json
{ "items": [ { "file": "bunny.obj", "name": "bunny", "at": [3400, 0, 0] } ] }
```

The loader (35's lib.rs loop) fetches it, the new arm parses it, `Msg::File` appends it through
`scene.add_file` + `gpu.set_scene(&scene.tables)` — and, better, **the watch loop (51) and
reconcile (49) work on OBJ files too**, because everything downstream only ever sees a `Session`.
Zero extra wiring; the funnel design pays again.

## Step 2 — the Open… dialog: `src/app/persistence.rs`

The user-driven path 34a deferred. A hidden `<input type=file>`, clicked programmatically; the
picked `File` is read as bytes and handed to a callback.

First, add the three features to the `web-sys` `features = [...]` list in `Cargo.toml`
(next to the ones 34a added):

```toml
    "HtmlInputElement",
    "File",
    "FileList",
```

Add these two imports at the top of `persistence.rs` (`Rc` is already imported for the chunked
parse — skip it if so): `JsCast` powers the `.dyn_into()` / `.unchecked_ref()` casts, and `Rc` is
how the `done` callback survives being handed to *two* closures:

```rust
use std::rc::Rc;
use wasm_bindgen::JsCast;
```

`done` is `impl Fn` — **not** `Clone`. It gets captured into the `onchange` closure
*and*, from there, into the `spawn_local` future, so it has to be shared. Wrap it in an
`Rc` once at the top, then clone the handle inside:

```rust
/// Pop the browser's file dialog; on pick, hand (name, bytes) to `done`.
/// New ground like 34a's fetch: verified against web-sys 0.3, wants a click-through.
pub fn open_file_dialog(done: impl Fn(String, Vec<u8>) + 'static) {
    let done = Rc::new(done);
    // DOM boundary — no unwraps (ARCHITECTURE §3): any failure here logs and no-ops
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        log::warn!("open_file_dialog: no window/document"); return;
    };
    let Some(input) = document.create_element("input").ok()
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok()) else {
        log::warn!("open_file_dialog: <input> unavailable"); return;
    };
    input.set_type("file");
    input.set_accept(".pb,.json,.obj");
    if let Some(body) = document.body() { let _ = body.append_child(&input); }
    let input2 = input.clone();
    let onchange = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        if let Some(file) = input2.files().and_then(|l| l.get(0)) {
            let name = file.name();
            let done = done.clone();        // clone the Rc handle for the async frame
            wasm_bindgen_futures::spawn_local(async move {
                match wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await {
                    Ok(buf) => done(name, js_sys::Uint8Array::new(&buf).to_vec()),
                    Err(e) => log::warn!("open: read failed: {e:?}"),   // loud, not unwrap
                }
            });
        }
        input2.remove();   // one open = one element; don't leak a DOM node per click
    }) as Box<dyn Fn()>);
    input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
    onchange.forget();                      // the closure lives as long as the input does
    input.click();
}
```

(The element now gets appended so the callback's `remove()` is real — an un-appended input would
make `remove()` a no-op and the leak cosmetic-only. `forget()` still leaks one *closure* per
`open` — a few dozen bytes per click; the zero-leak shape is one lazily-created static input reused
across opens. Noted, not built.)

**Delivery is a `Msg`, like every other file.** The parse is *async* now (35's sliced converter),
so `done` cannot parse inline — it spawns the parse and sends the result to the event loop, exactly
the shape the progressive loader already receives. Commands need a sender for that: give `State` a
proxy. In `state.rs`, add the field + parameter —

```rust
    pub proxy: winit::event_loop::EventLoopProxy<crate::Msg>,   // ← ADD to `struct State`
```

`State::new(window: Arc<Window>, scene: Scene)` grows a third parameter
`proxy: winit::event_loop::EventLoopProxy<crate::Msg>` (store it in the struct literal:
`proxy,`), and in lib.rs's loader, find
`let state = State::new(window.clone(), scene).await.expect("State init failed");` and pass
`proxy.clone()` as the third argument.

The `open` verb (`src/app/commands.rs`, one more arm in `dispatch`; add `"open"` to `VERBS`):

```rust
        "open" => {
            let proxy = state.proxy.clone();
            crate::app::persistence::open_file_dialog(move |name, bytes| {
                let proxy = proxy.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let session = crate::app::persistence::session_from_bytes_chunked(
                        &name, &bytes).await;
                    // a refusing arm (STEP, Step 4) comes back EMPTY — don't append an empty
                    // doc: it would sit in the tree (82) looking like a successful load
                    if session.lookup.is_empty() {
                        log::warn!("open: nothing loaded from {name}");
                        return;
                    }
                    let _ = proxy.send_event(crate::Msg::File(
                        name, session, session_rust::Xform::identity()));
                });
            });
            Dispatch::Instant("open: pick a file".into())
        }
```

An opened file is an **append** — a new `Doc` at identity placement (type
`move 0,0,0`-style placement later, or gumball-drag the whole doc's objects). Re-opening a
*changed copy* of an already-loaded doc is a different operation: that's 46's reconcile — match
`name` against `scene.docs` and route the parsed session to `reconcile` instead of `Msg::File`
when you want the diff instead of a second copy.

## Step 3 — export: `src/app/commands.rs`

The mirror, riding 50's download machinery. There is no "the session" anymore — a scene is
*several* docs, so `export pb|json` writes the **active doc** (70's `active_doc`;
`export pb <name>` picking a doc by its manifest name is a two-line extension):

```rust
        "export" => match parts.next() {
            Some("obj") => {
                // one mesh per file; the selection's first mesh — and ONLY the selection's:
                // no silent fallback to the scene's first mesh (exporting an object the user
                // didn't choose is worse than refusing)
                let Some(m) = state.scene.first_selected_mesh() else {
                    return Dispatch::Instant("select a mesh to export".into());
                };
                let s = write_file_obj_to_string(&m);   // Rc<Mesh> derefs to &Mesh
                let _ = crate::app::persistence::download_bytes("export.obj", s.as_bytes());
                Dispatch::Instant("exported export.obj".into())
            }
            Some("json") => {                                          // ONE doc, 50's machinery
                let d = state.scene.active_doc;
                let bytes = crate::app::persistence::session_to_bytes("session.json",
                                                                      &state.scene.docs[d].session);
                let _ = crate::app::persistence::download_bytes("session.json", &bytes);
                Dispatch::Instant("exported session.json".into())
            }
            Some("pb") | None => {
                let d = state.scene.active_doc;
                let bytes = crate::app::persistence::session_to_bytes("session.pb",
                                                                      &state.scene.docs[d].session);
                let _ = crate::app::persistence::download_bytes("session.pb", &bytes);
                Dispatch::Instant("exported session.pb".into())
            }
            _ => Dispatch::Instant("export obj|pb|json".into()),
        }
```

(`use session_rust::file_obj::write_file_obj_to_string;` at the top of `commands.rs`.)

`first_selected_mesh` goes on `impl Scene` (`src/app/scene.rs`) — doc-aware, and it returns the
`Rc` handle, not a borrow (the caller holds `&mut State`, so a `&Mesh` borrowed out of `self.scene`
would lock the whole struct; cloning the `Rc` is a refcount bump):

```rust
    /// First selected Mesh — `export obj`'s subject. Returns the Rc HANDLE (cheap clone) so the
    /// caller isn't borrowing the Scene. No selection → None: the caller REFUSES, it doesn't go
    /// fishing for an arbitrary mesh.
    pub fn first_selected_mesh(&self) -> Option<Rc<Mesh>> {
        self.selected.iter()
            .filter_map(|&row| {
                let g = &self.order[row as usize];          // selection is row-keyed (58)
                match self.docs[self.doc_of_row(row)].session.lookup.get(g) {
                    Some(Geometry::Mesh(m)) => Some(Rc::clone(m)),
                    _ => None,
                }
            })
            .next()
    }
```

(`use std::rc::Rc;` joins scene.rs's imports if 35 didn't already bring it. The kernel writer takes
ONE mesh — multi-object OBJ (`o name` groups) is a small kernel extension, noted in
`_KERNEL_GAPS.md`. Curves/BReps export via their tessellation — `b.mesh()` — losing exactness by
format; OBJ *is* a mesh format, that's honest.)

## Where the importers actually live — and the STEP honesty

The viewer is the *last* stop of an import pipeline that runs offline, and it's worth one honest
map:

- **PDF** — `session_rust/src/pdf.rs` (`import_pdf()`) + the `src/bin/pdf_import.rs` binary,
  behind the **optional `pdf` feature** (`session_data/import_drawings.sh` builds
  `--features pdf --bin pdf_import`). MuPDF compiles minutes of C through bindgen, so it's off by
  default — and its deps are declared only under `cfg(not(target_arch = "wasm32"))`, so a wasm
  build **cannot** pull MuPDF even if something enables the feature: the viewer is structurally
  safe, not flag-disciplined. The viewer only ever sees the `.pb` the importer wrote.
- **OBJ** — `session_rust/src/file_obj.rs` (`read_file_obj_from_str` / `write_file_obj_to_string`,
  this lesson's pair) — plain Rust, wasm-fine, so it runs *in* the viewer.
- **XYZ point clouds** — `session_rust/src/io.rs` (`read_xyz_from_str` / `write_xyz_to_string`,
  the same string-pair shape) — a third dispatch arm whenever a cloud needs the door.
- **STEP** — exists **only in C++** today (`session_cpp/src/file_step.h`); there is no
  `session_rust::file_step`, so the wasm viewer cannot parse STEP no matter what we wire. That's
  **kernel-gap #11**: port the codec C++ → Rust/Python (C++ is ground truth; the reader/writer
  logic is substantial — a real project, not an afternoon). The dispatch arm fails loudly rather
  than silently — the log line says why, and the empty Session it returns is exactly what Step 2's
  guard drops instead of appending (no phantom empty doc in the tree; 35's manifest loader should
  skip an empty session at its `add_file` call site the same way). In
  `session_from_bytes_chunked`, insert below Step 1's `.obj` arm:

```rust
    if url.ends_with(".step") || url.ends_with(".stp") {
        log::warn!("STEP import needs the C++->Rust codec port (kernel-gap #11)");
        return Session::default();
    }
```

One free improvement while we're here: every kernel type's `to_proto()`/`from_proto()` is `pub`,
so an import can be regression-tested **headlessly** — `.obj` text in, `to_proto` bytes out,
compare — no browser, no GPU, just `cargo test`.

## Verify

```bash
cd session_viewer && trunk serve   # http://127.0.0.1:8770
```

- Add `{ "file": "bunny.obj", "at": [3400, 0, 0] }` to the manifest — the Stanford bunny ships as
  the kernel's own fixture (`session_rust/session_data/bunny.obj`), but manifest paths resolve
  against the **assets root**, so copy it there first (`cp` it to `assets/bunny.obj`; the browser
  can't reach the source tree — 88's native selftest can). It streams in placed, shaded,
  edged, pickable — a first-class citizen, because it entered as a `Session` like everything else.
- `open`, pick any `.obj` → same result, appended as a new doc, log `loaded …`.
- Draw a box, select it, `export obj` → the download opens in Blender/Rhino/FreeCAD with the same
  dimensions (the kernel's String Roundtrip minitest guarantees the text is faithful; this checks
  the *browser* half). `export pb` → the ACTIVE doc only.
- Kernel side, already green: `FileObj::String Roundtrip` passes 3/3 languages.

## Recap

```
Ch 83: sections.
Ch 84: FILES. Import: session_from_bytes_chunked grows a `.obj` early-return arm (the .json arm is
       the pattern) — kernel read_file_obj_from_str (the path-based codec gained a string pair ×3
       languages for exactly this lesson) wraps the mesh in a fresh Session, so the manifest,
       watch, and reconcile all speak OBJ for free. Open… dialog: hidden <input type=file> +
       File.array_buffer → spawn_local parses (ASYNC — the chunked converter) → proxy.send_event
       (Msg::File — an APPEND; reconcile is for changed COPIES of a loaded doc). Export: a scene is
       DOCS now — export pb/json writes the ACTIVE doc; export obj takes first_selected_mesh
       (Option<Rc<Mesh>> — clone the handle, don't borrow the Scene). Importer map: pdf.rs behind
       the target-gated `pdf` feature (MuPDF never in wasm), file_obj.rs + io.rs in-viewer, STEP
       C++-only — gap #11, the dispatch arm warns instead of lying.
```

Edited: `app/persistence.rs` (`.obj` + STEP arms, `open_file_dialog`), `app/scene.rs`
(`first_selected_mesh`), `app/commands.rs` (`open`, `export`), `state.rs`/`lib.rs` (proxy on
`State`), `Cargo.toml` (3 web-sys features). Kernel (done with this lesson): `file_obj` string
pair ×3 + String Roundtrip minitest ×3.

## Next

`103-copy-array.md` — duplication: `copy` between two points, Alt+gumball-drag-a-copy, and `array` —
nearly free because `duplicate()` → one `AddGeometry` batch → `apply_world_delta` rides three
existing rails.
