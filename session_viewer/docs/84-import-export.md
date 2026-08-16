# 84 Import / export — OBJ in, OBJ out (and the STEP honesty)

> **Big picture.** *Phase 14.* The plan review's most embarrassing find: the kernel has shipped an
> OBJ codec since long before this course — minitested in all three languages — and the viewer only
> ever spoke `.pb`/`.json`. This lesson wires it through: an `.obj` as a **manifest item**, a real
> **Open… file dialog**, and `export obj` via 44's download path. It also triggered a kernel fix:
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
  <text x="340" y="104" fill="#888" text-anchor="middle">export = write_file_obj_to_string → 44's Blob download · STEP: C++-only today (kernel-gap #11)</text>
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
        // an OBJ is one mesh, not a document — wrap it in a fresh Session. OBJ files are
        // small text; the arm stays synchronous, like .json (the 25k slicing is for .pb).
        let mut s = Session::default();
        s.add_mesh(read_file_obj_from_str(&String::from_utf8_lossy(bytes)), None);
        return s;
    }
```

That alone makes an `.obj` a **manifest citizen** — placement included:

```json
{ "items": [ { "file": "bunny.obj", "name": "bunny", "at": [3400, 0, 0] } ] }
```

The loader (35's lib.rs loop) fetches it, the new arm parses it, `Msg::File` appends it through
`scene.add_file` + `gpu.set_scene(&scene.tables)` — and, better, **the watch loop (45) and
reconcile (43b) work on OBJ files too**, because everything downstream only ever sees a `Session`.
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
    let document = web_sys::window().unwrap().document().unwrap();
    let input: web_sys::HtmlInputElement =
        document.create_element("input").unwrap().dyn_into().unwrap();
    input.set_type("file");
    input.set_accept(".pb,.json,.obj");
    let input2 = input.clone();
    let onchange = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        if let Some(file) = input2.files().and_then(|l| l.get(0)) {
            let name = file.name();
            let done = done.clone();        // clone the Rc handle for the async frame
            wasm_bindgen_futures::spawn_local(async move {
                let buf = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await.unwrap();
                done(name, js_sys::Uint8Array::new(&buf).to_vec());
            });
        }
    }) as Box<dyn Fn()>);
    input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
    onchange.forget();                      // the closure lives as long as the input does
    input.click();
}
```

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
                    let _ = proxy.send_event(crate::Msg::File(
                        name, session, session_rust::Xform::identity()));
                });
            });
            Dispatch::Instant("open: pick a file".into())
        }
```

An opened file is an **append** — a new `Doc` at identity placement (type
`move 0,0,0`-style placement later, or gumball-drag the whole doc's objects). Re-opening a
*changed copy* of an already-loaded doc is a different operation: that's 43b's reconcile — match
`name` against `scene.docs` and route the parsed session to `reconcile` instead of `Msg::File`
when you want the diff instead of a second copy.

## Step 3 — export: `src/app/commands.rs`

The mirror, riding 44's download machinery. There is no "the session" anymore — a scene is
*several* docs, so `export pb|json` writes the **active doc** (62's `active_doc`;
`export pb <name>` picking a doc by its manifest name is a two-line extension):

```rust
        "export" => match parts.next() {
            Some("obj") => {
                // one mesh per file; the selection's first mesh (or the scene's first mesh)
                let Some(m) = state.scene.first_selected_mesh() else {
                    return Dispatch::Instant("select a mesh to export".into());
                };
                let s = write_file_obj_to_string(&m);   // Rc<Mesh> derefs to &Mesh
                let _ = crate::app::persistence::download_bytes("export.obj", s.as_bytes());
                Dispatch::Instant("exported export.obj".into())
            }
            Some("json") => {                                          // ONE doc, 44's machinery
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
    /// First selected Mesh, else the scene's first mesh — `export obj`'s subject.
    /// Returns the Rc HANDLE (cheap clone) so the caller isn't borrowing the Scene.
    pub fn first_selected_mesh(&self) -> Option<Rc<Mesh>> {
        self.selected.iter()
            .filter_map(|g| {
                let &row = self.guid_to_row.get(g)?;
                match self.docs[self.doc_of_row(row)].session.lookup.get(g) {
                    Some(Geometry::Mesh(m)) => Some(Rc::clone(m)),
                    _ => None,
                }
            })
            .next()
            .or_else(|| self.docs.iter()
                .find_map(|d| d.session.objects.meshes.first().map(Rc::clone)))
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
  than silently: in `session_from_bytes_chunked`, insert below Step 1's `.obj` arm:

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

- Add `{ "file": "bunny.obj", "at": [3400, 0, 0] }` to the manifest (the Stanford bunny ships as
  the kernel's own fixture, `session_rust/session_data/bunny.obj`) → it streams in placed, shaded,
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

`85-copy-array.md` — duplication: `copy` between two points, Alt+gumball-drag-a-copy, and `array` —
nearly free because `duplicate()` → one `AddGeometry` batch → `apply_world_delta` rides three
existing rails.
