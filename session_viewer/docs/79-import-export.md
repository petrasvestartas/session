# 79 Import / export — OBJ in, OBJ out (and the STEP honesty)

> **Big picture.** *Phase 14.* The plan review's most embarrassing find: the kernel has shipped an
> OBJ codec since long before this course — minitested in all three languages — and the viewer only
> ever spoke `.pb`/`.json`. This lesson wires it through: fetch-by-URL, a real **Open… file dialog**,
> and `export obj` via 39's download path. It also triggered a kernel fix: the codec was
> **path-based only** (`read_file_obj(filepath)` — dead on wasm), so the kernel gained the
> `_from_str`/`_to_string` pair (the same `_loads`/`_dumps` split the Session codecs always had),
> ×3 languages + a String Roundtrip minitest.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="obj text enters via url fetch or a file dialog, parses through the kernel string codec into a mesh, joins the session; export runs the writer and downloads" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="22" width="120" height="26" fill="none" stroke="#6fb3ff"/>
  <text x="70" y="39" fill="#d7dae0" text-anchor="middle">fetch url (34a)</text>
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
  <text x="340" y="104" fill="#888" text-anchor="middle">export = write_file_obj_to_string → 39's Blob download · STEP: C++-only today (kernel-gap #11)</text>
  <defs><marker id="ah79" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
    <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/persistence.rs   # .obj arm in session_from_bytes; export_obj_bytes
src/app/commands.rs      # `open` (file dialog) + `export obj` verbs
Cargo.toml               # web-sys: HtmlInputElement, File, FileList (the dialog)
```

## Step 1 — import by extension: `src/app/persistence.rs`

34a's dispatch grows one arm. An OBJ is a bare mesh, not a Session — wrap it:

```rust
use session_rust::file_obj::{read_file_obj_from_str, write_file_obj_to_string};

pub fn session_from_bytes(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        Session::file_json_loads(&String::from_utf8_lossy(bytes))
    } else if url.ends_with(".obj") {
        // an OBJ is one mesh, not a document — wrap it in a fresh Session
        let mesh = read_file_obj_from_str(&String::from_utf8_lossy(bytes));
        let mut s = Session::default();
        s.add_mesh(mesh, None);
        s
    } else {
        Session::pb_loads(bytes).unwrap_or_default()
    }
}
```

That alone makes `DEMO_SESSION_URL = "session_data/bunny.obj"` work — and, better, it makes the
**watch loop (40) and reconcile (38b) work on OBJ files too**, because everything downstream only
ever sees a `Session`. Zero extra wiring; the funnel design pays again.

## Step 2 — the Open… dialog: `src/app/persistence.rs`

The user-driven path 34a deferred. A hidden `<input type=file>`, clicked programmatically; the
picked `File` is read as bytes and fed to the same funnel.

First, add the three features to the `web-sys` `features = [...]` list in `Cargo.toml`
(next to the ones 34a added):

```toml
    "HtmlInputElement",
    "File",
    "FileList",
```

Add these two imports at the top of `persistence.rs` (alongside Step 1's): `JsCast`
powers the `.dyn_into()` / `.unchecked_ref()` casts, and `Rc` is how the `done`
callback survives being handed to *two* closures:

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

The `open` verb calls it; `done` routes to `session_from_bytes(name, bytes)` → 38b's
`apply_session` (reconcile — so opening a *changed* copy of the current file diffs instead of
rebuilding, for free). Same borrow rule as 40: the async part only *fetches*; the sync frame drains.

## Step 3 — export: `src/app/commands.rs`

The mirror, riding 39's download machinery:

```rust
        "export" => match parts.next() {
            Some("obj") => {
                // one mesh per file; the selection's first mesh (or the only mesh in the scene)
                let Some(m) = state.scene.first_selected_mesh() else {
                    return Dispatch::Instant("select a mesh to export".into());
                };
                let s = write_file_obj_to_string(m);
                let _ = crate::app::persistence::download_bytes("export.obj", s.as_bytes());
                Dispatch::Instant("exported export.obj".into())
            }
            Some("json") => {                                          // whole doc, 39's machinery
                let bytes = crate::app::persistence::session_to_bytes("session.json",
                                                                      &state.scene.session);
                let _ = crate::app::persistence::download_bytes("session.json", &bytes);
                Dispatch::Instant("exported session.json".into())
            }
            Some("pb") | None => {
                let bytes = crate::app::persistence::session_to_bytes("session.pb",
                                                                      &state.scene.session);
                let _ = crate::app::persistence::download_bytes("session.pb", &bytes);
                Dispatch::Instant("exported session.pb".into())
            }
            _ => Dispatch::Instant("export obj|pb|json".into()),
        }
```

`first_selected_mesh` is a five-liner on `impl Scene`:

```rust
    /// First selected Mesh, else the scene's only mesh — `export obj`'s subject.
    pub fn first_selected_mesh(&self) -> Option<&Mesh> {
        self.selected.iter()
            .filter_map(|g| match self.session.lookup.get(g) {
                Some(Geometry::Mesh(m)) => Some(m), _ => None })
            .next()
            .or_else(|| self.session.objects.meshes.first())
    }
```

(The kernel writer takes ONE mesh — multi-object OBJ (`o name` groups) is a small kernel extension,
noted in `_KERNEL_GAPS.md`. Curves/BReps export via their tessellation — `b.mesh()` — losing
exactness by format; OBJ *is* a mesh format, that's honest.)

## The STEP honesty

`file_step` exists **only in C++** today — there is no `session_rust::file_step`, so the wasm viewer
cannot parse STEP no matter what we wire. That's now **kernel-gap #11**: port the STEP codec
C++ → Rust/Python (C++ is ground truth; the reader/writer logic is substantial — this is a real
project, not an afternoon). The dispatch arm is written to fail loudly rather than silently.

In `session_from_bytes` (Step 1), insert this arm **between the `.obj` arm and the final
`else`** — it slots right in as one more `else if`, closing with the trailing `}` of the
existing fallback:

```rust
    } else if url.ends_with(".step") || url.ends_with(".stp") {
        log::warn!("STEP import needs the C++->Rust codec port (kernel-gap #11)");
        Session::default()
    } else {
        Session::pb_loads(bytes).unwrap_or_default()
    }
```

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- `open`, pick any `.obj` (the Stanford bunny from the kernel's own test data works) → it appears,
  shaded, edged, pickable, with a fit-able bound — a first-class citizen, because it entered as a
  `Session` like everything else.
- Draw a box, select it, `export obj` → the download opens in Blender/Rhino/FreeCAD with the same
  dimensions (the kernel's String Roundtrip minitest guarantees the text is faithful; this checks
  the *browser* half).
- Kernel side, already green: `FileObj::String Roundtrip` passes 3/3 languages.

## Recap

```
Ch 78: sections.
Ch 79: FILES. Import: session_from_bytes grows a `.obj` arm — kernel read_file_obj_from_str (the
       path-based codec gained a string pair ×3 languages for exactly this lesson) wraps the mesh in
       a fresh Session, so watch/reconcile/save all work on OBJ for free. Open… dialog: hidden
       <input type=file> + File.array_buffer → the same funnel → apply_session (a changed copy DIFFS
       in). Export: write_file_obj_to_string → 39's download (one mesh/file; multi-object OBJ =
       kernel extension, noted). STEP: C++-only — gap #11, the dispatch arm warns instead of lying.
```

Edited: `app/persistence.rs` (`.obj` arm, `open_file_dialog`, STEP warning arm), `app/commands.rs`
(`open`, `export`), `Cargo.toml` (3 web-sys features). Kernel (done with this lesson):
`file_obj` string pair ×3 + String Roundtrip minitest ×3.

## Next

`80-copy-array.md` — duplication: `copy` between two points, Alt+gumball-drag-a-copy, and `array` —
nearly free because clone → `refresh_guid` → `AddGeometry` rides three existing rails.
