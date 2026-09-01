# 53 The shell, the loader and the gestures

> `lib.rs` is the first file anyone opens, and it held three unrelated jobs in 523 lines. After
> this lesson it holds one, in 310.
> Nothing you can see changes.

## 1. Why this seam

`lib.rs` is the crate root: it declares the modules, and it is where a reader starts. It was also
doing three separate jobs.

- **Loading.** `scene_url`, `Msg`, `reload_scene`, `load_manifest` — how a URL becomes a stream of
  documents. About 115 lines that never touch the GPU, the camera or the window.
- **The shell.** `App`, `resumed`, `user_event` — create the canvas, build `State` asynchronously,
  pump messages back into it.
- **Gestures.** `window_event` — 116 lines routing seven kinds of event into camera moves and
  knob flips.

The give-away is `App`'s own fields. Four of them — `orbiting`, `panning`, `last_cursor`, `ctrl` —
are read and written by `window_event` and by nothing else; `fitted`, `proxy` and `state` are
touched only by the shell. Two disjoint sets of fields on one struct, each used by one half of the
file, is the shape of two structs.

A gesture is a state machine: a drag is only a drag because a button went down earlier and has not
come up yet. Those four bits ARE that machine, and they had been sitting beside a window handle.

## 2. Where the code lives after this lesson

```
src/lib.rs             310  the crate root: modules, App, resumed, user_event, run_web
src/app/loader.rs      126  URL -> Msg. Produces messages; touches no GPU, camera or window
src/app/input.rs       163  Input { orbiting, panning, last_cursor, ctrl } + on_window_event
```

`App` keeps `state`, `proxy` and `fitted`, and gains one named field in place of the four loose
booleans — step 3e.

## 3. The steps

**3a.** The loader first. It is a whole file rather than a move, because the pieces change as they
go: `scene_url` and `load_manifest` become `pub(crate)`, and `RELOAD_PROXY` gains a function so the
static itself can stay private — a caller that can reach it can also replace it mid-load.

**Create `src/app/loader.rs`**:

```rust
//! `loader.rs` — how a URL becomes a scene, one message at a time.
//!
//! The viewer never blocks on a load. `load_manifest` fetches the manifest, then each `.pb` it
//! names, and emits a `Msg` per document as it lands — so the first file draws while the fifth is
//! still on the wire. `Msg` is the seam between "something arrived" and "the app does something
//! with it": everything here PRODUCES messages, and nothing here touches the GPU, the camera or
//! the window.
//!
//! `RELOAD_PROXY` is the one piece of state in the file, and it exists because `resumed` takes
//! `self.proxy`: without a copy kept past start-up there is no way to post a file into a running
//! event loop, which is exactly what `reload_scene` does.

use wasm_bindgen::prelude::*;

use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest};
use crate::State;

// The scene: which sheets, and where each one sits.
// Fetched at runtime, so re-arringing the scene is a text edit in assets/scenes, not rebuild (app/scene.rs)
const DEMO_SCENE_URL: &str = "scenes/bunny_drawings.toml";
// const DEMO_SCENE_URL: &str = "scenes/cloud_mix.toml"; // was bunny_drawings.json

/// The manifest to load: `?scene=<path under assets/>` when the page supplies one, else
/// [`DEMO_SCENE_URL`]. One build can therefore serve many scenes - the docs embed a single
/// 7.7 MB wasm in an iframe per example and vary only the query string.
///
/// The value is a path under `assets/`, exactly like a manifest's own `file` entries. It is
/// rejected unless it stays inside that tree: an absolute URL, a scheme, or any `..` segment
/// would let a page point the viewer at another origin.
pub(crate) fn scene_url() -> String {
    fn from_query() -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        let raw = search.strip_prefix('?')?;
        let value = raw
            .split('&')
            .find_map(|pair| pair.strip_prefix("scene="))?;
        let decoded = js_sys::decode_uri_component(value).ok()?.as_string()?;
        let safe = !decoded.is_empty()
            && !decoded.starts_with('/')
            && !decoded.contains("//")
            && !decoded.contains(':')
            && !decoded.split('/').any(|seg| seg == "..");
        safe.then_some(decoded)
    }
    from_query().unwrap_or_else(|| DEMO_SCENE_URL.to_string())
}

/// Async init - event-loop messages.
/// `Ready` carries the State built around the first file
/// pixes in 2s, each file is one more parsed document appended live.
pub enum Msg {
    Ready(Box<State>),
    CloudBegin(String, session_rust::Xform, u32, f32),
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    CloudEnd([f32; 3], [f32; 3]),
    File(String, session_rust::Session, session_rust::Xform, f32, bool),
    /// Drop the current documents, keeping `State` - see [`reload_scene`].
    Clear,
}

thread_local! {
    /// A proxy kept past start-up so [`reload_scene`] can post files into the
    /// running event loop. `resumed` takes `self.proxy`, so without this copy
    /// there is no way back into the app once it is going.
    static RELOAD_PROXY: std::cell::RefCell<Option<winit::event_loop::EventLoopProxy<Msg>>> =
        const { std::cell::RefCell::new(None) };
}

/// Hand the loader a proxy it can keep. `resumed` TAKES `App::proxy`, so this copy is the only
/// way back into a running event loop - and the static stays private, because a caller that can
/// reach it can also replace it mid-load.
pub(crate) fn remember_proxy(proxy: winit::event_loop::EventLoopProxy<Msg>) {
    RELOAD_PROXY.with(|slot| *slot.borrow_mut() = Some(proxy));
}

/// Reload the scene in place: same canvas, same camera, new geometry.
///
/// The page calls this after rewriting a `.pb` (see the docs' Thebe cells) so an
/// edit redraws the MODEL instead of restarting the viewer - reloading the
/// iframe would rebuild the WebGPU device and throw away the view you had
/// framed. `url` is a manifest path under `assets/`, as with `?scene=`.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let proxy = RELOAD_PROXY.with(|slot| slot.borrow().clone());
    let Some(proxy) = proxy else {
        log::warn!("reload_scene: viewer is not running yet");
        return;
    };
    let url = url.unwrap_or_else(scene_url);
    wasm_bindgen_futures::spawn_local(async move {
        let _ = proxy.send_event(Msg::Clear);
        load_manifest(url, move |name, session, place, px, only| {
            let _ = proxy.send_event(Msg::File(name, session, place, px, only));
        })
        .await;
    });
}

/// Fetch a manifest and hand every parsed file to `emit`, in manifest order.
///
/// Shared by start-up and [`reload_scene`]; the only difference between them is
/// that start-up builds `State` around the first file, so it cannot use this
/// directly for that one.
pub(crate) async fn load_manifest<F>(url: String, mut emit: F)
where
    F: FnMut(String, session_rust::Session, session_rust::Xform, f32, bool),
{
    let manifest_bytes = persistence::fetch_bytes(&url).await.unwrap_or_default();
    let Some(manifest) = Manifest::parse(&manifest_bytes) else {
        log::error!("cannot read the scene manifest at {url}");
        return;
    };
    let count = manifest.items.len();
    for (i, item) in manifest.items.iter().enumerate() {
        let bytes = persistence::fetch_bytes(&item.file).await.unwrap_or_default();
        let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
        if session.lookup.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
        emit(name, session, place, item.point_size as f32, item.display_only);
    }
}
```

**3b.** Now the gesture machine. Create it with the handler's body left out, then move the body in;
those 110 lines are not retyped.

**Create `src/app/input.rs`**:

```rust
//! `input.rs` — the pointer and keyboard gestures, and the four bits of state they need.
//!
//! A gesture is a state machine: a drag is only a drag because a button went down earlier and has
//! not come up yet. Those four bits — two buttons, the modifier, the last cursor position — sat
//! as loose fields on `App`, beside the window handle and the event-loop proxy, which are not
//! about input at all. They are one value here, and `App` holds it the way it holds anything
//! else: as a named field.
//!
//! Nothing in this file draws. It reads events, moves the camera or flips a knob on `State`, and
//! asks for a redraw; what that redraw does is `engine/gpu/render.rs`'s business.

// Everything below `Input` runs only under winit's web backend, so its imports are gated with
// the handler. `Input` itself is not: `App` holds one on every target.
#[cfg(target_arch = "wasm32")]
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
#[cfg(target_arch = "wasm32")]
use winit::event_loop::ActiveEventLoop;
#[cfg(target_arch = "wasm32")]
use winit::keyboard::{Key, NamedKey};

#[cfg(target_arch = "wasm32")]
use crate::camera::View;
#[cfg(target_arch = "wasm32")]
use crate::{desired_canvas_size, State};

/// What a gesture has to remember between two events.
#[derive(Default)]
pub struct Input {
    /// Left button held: the pointer is orbiting the camera.
    pub orbiting: bool,
    /// Middle button, or shift-left: the pointer is panning it.
    pub panning: bool,
    /// Where the pointer was last seen, so a move becomes a delta.
    pub last_cursor: (f64, f64),
    /// Ctrl held, which re-aims the wheel from zoom to pen thickness.
    pub ctrl: bool,
}

/// The gestures themselves are the browser's: `window_event` only ever runs under winit's web
/// backend, so the handler is gated the way `impl ApplicationHandler for App` is. `Input` itself
/// is not - `App` holds one on every target.
#[cfg(target_arch = "wasm32")]
impl Input {
    /// One window event. `state` is `None` until `resumed` has built it, and an event that
    /// arrives before then has nothing to act on.
    pub fn on_window_event(
        &mut self,
        state: &mut Option<State>,
        event_loop: &ActiveEventLoop,
        event: WindowEvent,
    ) {
        let state = match state { Some(s) => s, None => return };
    }
}

```

**Move** from `src/lib.rs` to `src/app/input.rs`, **up to** the handler's closing brace, **after**
the line that unwraps the state option:

```rust
        match event {
```

```rust
    }
```

```rust
        let state = match state { Some(s) => s, None => return };
```

**3c.** Both modules get declared.

**Find** in `src/app/mod.rs`:

```rust
pub mod knobs;
```

**Add above it:**

```rust
pub mod input;
pub mod loader;
```

**3d.** What is left of `lib.rs`. The import block goes first: everything from `App` down is
browser-only, and after this lesson so are the imports that serve it — otherwise each one warns on
the native selftest build, and a warning count nobody can drive to zero is a gate nobody reads.
`View` and `load_manifest` are gone from the list entirely: the named views went with the number
keys, and `resumed` does its own fetching.

**Find** in `src/lib.rs`:

```rust
use crate::camera::View;
use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest, Scene};

// The scene: which sheets, and where each one sits.
// Fetched at runtime, so re-arringing the scene is a text edit in assets/scenes, not rebuild (app/scene.rs)
const DEMO_SCENE_URL: &str = "scenes/bunny_drawings.toml";
// const DEMO_SCENE_URL: &str = "scenes/cloud_mix.toml"; // was bunny_drawings.json

/// The manifest to load: `?scene=<path under assets/>` when the page supplies one, else
/// [`DEMO_SCENE_URL`]. One build can therefore serve many scenes - the docs embed a single
/// 7.7 MB wasm in an iframe per example and vary only the query string.
///
/// The value is a path under `assets/`, exactly like a manifest's own `file` entries. It is
/// rejected unless it stays inside that tree: an absolute URL, a scheme, or any `..` segment
/// would let a page point the viewer at another origin.
fn scene_url() -> String {
    fn from_query() -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        let raw = search.strip_prefix('?')?;
        let value = raw
            .split('&')
            .find_map(|pair| pair.strip_prefix("scene="))?;
        let decoded = js_sys::decode_uri_component(value).ok()?.as_string()?;
        let safe = !decoded.is_empty()
            && !decoded.starts_with('/')
            && !decoded.contains("//")
            && !decoded.contains(':')
            && !decoded.split('/').any(|seg| seg == "..");
        safe.then_some(decoded)
    }
    from_query().unwrap_or_else(|| DEMO_SCENE_URL.to_string())
}

/// Async init - event-loop messages.
/// `Ready` carries the State built around the first file
/// pixes in 2s, each file is one more parsed document appended live.
pub enum Msg {
    Ready(Box<State>),
    CloudBegin(String, session_rust::Xform, u32, f32),
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    CloudEnd([f32; 3], [f32; 3]),
    File(String, session_rust::Session, session_rust::Xform, f32, bool),
    /// Drop the current documents, keeping `State` - see [`reload_scene`].
    Clear,
}

thread_local! {
    /// A proxy kept past start-up so [`reload_scene`] can post files into the
    /// running event loop. `resumed` takes `self.proxy`, so without this copy
    /// there is no way back into the app once it is going.
    static RELOAD_PROXY: std::cell::RefCell<Option<winit::event_loop::EventLoopProxy<Msg>>> =
        const { std::cell::RefCell::new(None) };
}

/// Reload the scene in place: same canvas, same camera, new geometry.
///
/// The page calls this after rewriting a `.pb` (see the docs' Thebe cells) so an
/// edit redraws the MODEL instead of restarting the viewer - reloading the
/// iframe would rebuild the WebGPU device and throw away the view you had
/// framed. `url` is a manifest path under `assets/`, as with `?scene=`.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let proxy = RELOAD_PROXY.with(|slot| slot.borrow().clone());
    let Some(proxy) = proxy else {
        log::warn!("reload_scene: viewer is not running yet");
        return;
    };
    let url = url.unwrap_or_else(scene_url);
    wasm_bindgen_futures::spawn_local(async move {
        let _ = proxy.send_event(Msg::Clear);
        load_manifest(url, move |name, session, place, px, only| {
            let _ = proxy.send_event(Msg::File(name, session, place, px, only));
        })
        .await;
    });
}

/// Fetch a manifest and hand every parsed file to `emit`, in manifest order.
///
/// Shared by start-up and [`reload_scene`]; the only difference between them is
/// that start-up builds `State` around the first file, so it cannot use this
/// directly for that one.
async fn load_manifest<F>(url: String, mut emit: F)
where
    F: FnMut(String, session_rust::Session, session_rust::Xform, f32, bool),
{
    let manifest_bytes = persistence::fetch_bytes(&url).await.unwrap_or_default();
    let Some(manifest) = Manifest::parse(&manifest_bytes) else {
        log::error!("cannot read the scene manifest at {url}");
        return;
    };
    let count = manifest.items.len();
    for (i, item) in manifest.items.iter().enumerate() {
        let bytes = persistence::fetch_bytes(&item.file).await.unwrap_or_default();
        let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
        if session.lookup.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
        emit(name, session, place, item.point_size as f32, item.display_only);
    }
}

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent, MouseButton};
use winit::keyboard::{Key, NamedKey};
```

**Replace with:**

```rust
use crate::app::persistence;
use crate::app::loader::{scene_url, Msg};
use crate::app::scene::{auto_grid, Manifest, Scene};

use std::sync::Arc;
// `#[wasm_bindgen]` went to `loader.rs` with `reload_scene`, and `WindowEvent` is now only
// the handler's parameter type — both serve browser-only code, so they say so.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
#[cfg(target_arch = "wasm32")]
use winit::event::WindowEvent;
```

**3e.** The four gesture fields become one.

**Find** in `src/lib.rs`:

```rust
    orbiting: bool,
    panning: bool,
    last_cursor: (f64, f64),
    ctrl: bool,
```

**Replace with:**

```rust
    /// The gesture machine (`app/input.rs`): what a drag remembers between two events.
    input: crate::app::input::Input,
```

**Find** in `src/lib.rs`:

```rust
            orbiting: false,
            panning: false,
            last_cursor: (0.0, 0.0),
            ctrl: false,
```

**Replace with:**

```rust
            input: Default::default(),
```

**3f.** The proxy is handed over by name instead of by reaching into the static.

**Find** in `src/lib.rs`:

```rust
            RELOAD_PROXY.with(|slot| *slot.borrow_mut() = Some(proxy.clone()));
```

**Replace with:**

```rust
            crate::app::loader::remember_proxy(proxy.clone());
```

**3g.** And the handler delegates. The body is gone, so what is left is the signature and
the line that unwrapped the state option — both of which the new method does for itself.

**Find** in `src/lib.rs`:

```rust
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state { Some(s) => s, None => return };
    }
```

**Replace with:**

```rust
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.input.on_window_event(&mut self.state, event_loop, event);
    }
```

**3h.** `input.rs` measures the canvas on a redraw, so the helper it calls stops being private.

**Find** in `src/lib.rs`:

```rust
fn desired_canvas_size() -> Option<(u32, u32)> {
```

**Replace with:**

```rust
pub(crate) fn desired_canvas_size() -> Option<(u32, u32)> {
```

## 4. Proving nothing changed

Both targets, because the native build compiles most of this file out — the first pass of this
split was clean on native and broken on wasm:

```
cargo check --target x86_64-unknown-linux-gnu --all-targets    15 warnings, as before
cargo check --target wasm32-unknown-unknown                     3 warnings, as before
./docs/_gate.sh                                                 gate OK
```

## 5. What is deliberately not here

`resumed` is still 128 lines and `user_event` 83. They are one job — bring the app up, then feed it
— and splitting them would put half a sequence in another file. `State` is still built in `resumed`
rather than by the loader, because until it exists there is nothing for a message to arrive at.

## Recap

`lib.rs` 523 -> 310, and the file a newcomer opens first now declares the modules and runs the
shell, nothing else. The gesture machine is a value with a name, not four loose booleans.

## Next

Lesson [55](55-nurbscurve.md) — NurbsCurve.
