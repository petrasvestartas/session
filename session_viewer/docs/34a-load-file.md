# 34a Load a Session I — fetch the file

> **Big picture.** Everything on screen so far came from Rust code written by hand. A CAD viewer's
> whole reason to exist is *other people's files* — and `session_rust::Session` is already the
> kernel's file format, byte-identical across C++/Python/Rust (`pb_dumps`/`pb_loads`, round-tripped
> by thousands of CI minitests). This lesson does the first half of the swap: get real `.pb` bytes
> into the browser and parse them into a `Session`, proving it with a console count. 34b then walks
> that `Session` into the GPU tables 30–32 built.

<svg viewBox="0 0 680 90" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="bytes fetched over HTTP become a Session via pb_loads; this lesson stops at the parsed Session" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <rect x="10"  y="24" width="90"  height="34"/>
    <rect x="150" y="24" width="230" height="34"/>
    <rect x="430" y="24" width="230" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="55"  y="45">bytes</text>
    <text x="265" y="45">Session::pb_loads / jsonload</text>
    <text x="545" y="45">lookup{ guid → Geometry }</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.5">
    <line x1="100" y1="41" x2="146" y2="41" marker-end="url(#ah34a)"/>
    <line x1="380" y1="41" x2="426" y2="41" marker-end="url(#ah34a)"/>
  </g>
  <text x="120" y="34" fill="#666" font-size="9">fetch</text>
  <text x="545" y="78" fill="#888" text-anchor="middle">this lesson: log the counts · 34b: draw it</text>
  <defs>
    <marker id="ah34a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
      <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/>
    </marker>
  </defs>
</svg>

## Files we touch

```
Cargo.toml               # web-sys: Request, RequestInit, RequestMode, Response (fetch API)
index.html               # Trunk copy-file — fixture .pb bytes served next to the wasm
src/app/persistence.rs   # NEW — fetch_bytes (fetch API) + session_from_bytes (.pb/.json dispatch)
src/lib.rs               # mod app; — the first app-layer file
src/state.rs             # fetch + parse before Gpu::new; log what loaded
```

`app/` is new territory: `engine/mod.rs`'s own doc comment says app-specific code lives in `app/`,
"never here" — this is the first lesson that needs it.

## Step 1 — fetch bytes: wasm has no `std::fs` — `src/app/persistence.rs`

**1a. Add the fetch-API features to `Cargo.toml`**, in the existing `web-sys` feature list (find the
`features = [` array ending in `"Performance"]`):

```toml
web-sys = { version = "0.3", features = [
    "Document",
    "Window",
    "Element",
    "HtmlCanvasElement",
    "EventTarget",
    "Event",
    "CanvasRenderingContext2d",
    "ImageData",
    "Location",
    "Performance",
    "Request",
    "RequestInit",
    "RequestMode",
    "Response"] }
```

**1b. Create `src/app/persistence.rs`.** Two functions: get bytes, then hand them to `Session`'s
own loaders — the SAME `pb_loads`/`file_json_loads` every other language's minitest already proves
round-trip correctly, just fed bytes/a string instead of a filepath:

```rust
//! Session loading — the kernel file format arrives. wasm32 has no filesystem, so the fetch API
//! is the only way to reach a `.pb`/`.json` file (std::fs is not an option here).

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use session_rust::Session;

/// GET `url` (Trunk-served, same origin as the page) and return the raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// `.pb` → prost, `.json` → serde — dispatched on `url`'s extension. Both loaders already exist
/// on `Session` (used by every language's minitest); a failed/empty fetch degrades to
/// `Session::default()` — an empty scene, not a panic.
pub fn session_from_bytes(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        Session::file_json_loads(&String::from_utf8_lossy(bytes))
    } else {
        Session::pb_loads(bytes).unwrap_or_default()
    }
}
```

> `Session::pb_load(path)`/`file_json_load(path)` (no trailing `s`) read from `std::fs` — those
> panic on wasm32. The `_loads` pair (bytes/string in, no path) is the browser-safe half of the same
> API.
>
> **New ground.** Neither the archive nor today's viewer has ever fetched anything — the archive's
> `ARCHITECTURE.md` lists file loading as documented-but-unbuilt "Phase 1". The signatures above were
> confirmed against `web-sys 0.3.99` in this crate's own `Cargo.lock`, but the *flow* (async fetch →
> bytes → `Session`) is new and wants a real browser click-through before you trust it. The
> user-driven alternative — `<input type="file">` + `FileReader` feeding the same `session_from_bytes`
> — is left for the file-menu lesson.

## Step 2 — serve the fixtures: `index.html`

Trunk only ships what `index.html` tells it to. **Add two `copy-file` links** next to the existing
`data-trunk rel="rust"` link — `session_data/` is a sibling of `session_viewer/`, and Trunk resolves
`href` relative to `index.html` at build time (a `..` is fine; it never reaches the browser):

```html
  <link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="0"/>
  <link data-trunk rel="copy-file" href="../session_data/floor_model.pb" data-target-path="session_data"/>
  <link data-trunk rel="copy-file" href="../session_data/30700_querschnitt_gg.pb" data-target-path="session_data"/>
  <canvas id="canvas"></canvas>
```

With `Trunk.toml`'s `public_url = "./"`, these land at `dist/session_data/*.pb` and are reachable
at runtime as `session_data/floor_model.pb` — the URL Step 3 fetches.

## Step 3 — wire the load and prove it: `src/lib.rs` + `src/state.rs`

**3a. In `lib.rs`, declare the new module** next to the existing three:

```rust
mod engine;
mod state;
mod camera;
mod app;   // ← ADD — the first app-layer file (engine/mod.rs said this was coming)
```

**3b. In `state.rs`, fetch before building the GPU state.** `State::new` has been `async` since the
very first window chapter, so awaiting the fetch is free. `Gpu::new` doesn't take the session yet —
that's 34b's change; today we just prove the parse:

```rust
use crate::app::persistence;

const DEMO_SESSION_URL: &str = "session_data/floor_model.pb";

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let bytes = persistence::fetch_bytes(DEMO_SESSION_URL).await.unwrap_or_default();
        let session = persistence::session_from_bytes(DEMO_SESSION_URL, &bytes);
        log::info!("loaded '{}': {} objects, {} bytes", session.name, session.lookup.len(), bytes.len());
        let gpu = Gpu::new(window.clone()).await?;   // unchanged — 34b threads `session` through
        Ok(Self { window, gpu, camera: Camera::new() })
    }
}
```

A failed fetch (offline, 404) degrades to `bytes = vec![]` → `pb_loads` errors → `Session::default()`
— an empty scene logs 0 objects; nothing panics.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The old five-mesh demo still draws (nothing feeds the GPU yet), but the console (F12) proves the
pipeline:

```
loaded 'floor_model': 491 objects, 3070848 bytes
```

Swap `DEMO_SESSION_URL` to `"session_data/30700_querschnitt_gg.pb"` → `42232 objects`. Break the URL
→ `0 objects`, no panic. That's the whole contract of this half: bytes reliably become a `Session`.

> `session_tests/` (the roadmap's original pointer) holds the Vue viewer's test-report JSON, not
> scene fixtures — real loadable dumps live in `session_data/`. `closest_point.pb` was tried and
> rejected: it pre-dates a proto schema change and fails to decode — worth knowing before picking a
> fixture blind.

## Recap

```
Ch 33: camera-relative — precision groundwork done; the viewer is ready for real coordinates.
Ch 34a: FETCH THE FILE. app/persistence.rs: fetch_bytes (web-sys Request/Response — wasm has no
        std::fs) + session_from_bytes (pb_loads for .pb, file_json_loads for .json — the browser-safe
        `_loads` half of the API every minitest already round-trips; failures degrade to
        Session::default(), never panic). index.html copy-file ships the fixtures next to the wasm.
        State::new fetches, parses, and LOGS the object count — 491 for floor_model.pb. The GPU still
        draws the hand-made demo; feeding it is 34b.
```

Edited: `Cargo.toml` (fetch-API web-sys features), `index.html` (Trunk `copy-file` fixtures),
`src/app/persistence.rs` (NEW — `fetch_bytes` + `session_from_bytes`), `src/lib.rs` (`mod app;`),
`src/state.rs` (fetch + parse + log before `Gpu::new`).

## Next

`34b-session-walk.md` — the parsed `Session` replaces the five hardcoded meshes: a `match` over every
`Geometry` variant walks meshes, lines, and points into the arena/segment/glyph tables, `F` learns the
real scene bounds, and the stress gate — a 42k-object PDF drawing — proves one draw call survives it.
