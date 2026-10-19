# 20.4.1 · Start with current browser display defaults

**Chapter 20 · Shared GPU rendering · Section 20.4 · Browser display defaults** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Start the browser with the current viewer’s nearly solid desktop faces and visible construction grid.

## Type

Continue from [Retain RGBA in prepared displays](96zze-retain-rgba-in-prepared-displays.md). [Save or recover your work](recovery.md).

### 1. `src/browser_defaults.rs`

Reuse existing opacity and grid state; detect pointer capability and read URL defaults once.

Create the file and type:

```rust
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-1.rs"
```

### 2. `src/lib.rs`

Register the startup settings without a second renderer settings owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod packed_vertex;
```

</details>

After that block, type:

```rust
pub mod browser_defaults;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-2.rs"
```

</details>

### 3. `Cargo.toml`

Enable the browser media-query return type in the existing pinned web-sys dependency.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "MediaQueryList", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-3.rs"
```

</details>

### 4. `src/browser.rs`

Initialize a new browser document once before creating its renderer.

<details>
<summary>Locate the existing block</summary>

```rust
pub async fn run() -> Result<(), JsValue> { run_with(crate::drawing_document::new()).await }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub async fn run() -> Result<(), JsValue> {
    let shared=crate::drawing_document::new();
    crate::browser_defaults::initialize(&mut shared.borrow_mut());
    run_with(shared).await
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-4.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/browser_defaults_tests.rs`

Supply pointer-policy, finite override, clamping and existing-owner fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-5.rs"
```

</details>

### 6. `src/lib.rs`

Register the supplied startup policy fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod browser_defaults;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod browser_defaults_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-6.rs"
```

</details>

### 7. `src/browser_defaults.rs`

Use one query reader for startup defaults and viewport scale. Cap requested pixels using the current finite f32 override policy.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Settings { pub opacity:f32, pub grid:bool }

impl Settings {
    pub fn new(coarse:bool,opacity:Option<&str>,nogrid:Option<&str>)->Self {
        let default=if coarse {1.} else {0.95};
        let opacity=opacity.and_then(|s|s.parse::<f32>().ok())
            .filter(|v|v.is_finite()).unwrap_or(default).clamp(0.,1.);
        Self {opacity,grid:nogrid.is_none()}
    }

    pub fn apply(&self,editor:&mut crate::editor::Editor) {
        editor.opacity=self.opacity;
        editor.camera.grid=self.grid.then(Default::default);
    }
}

#[cfg(target_arch="wasm32")]
pub fn initialize(editor:&mut crate::editor::Editor) {
    let Some(window)=web_sys::window() else {return};
    let coarse=window.match_media("(pointer: coarse)").ok().flatten()
        .is_some_and(|media|media.matches());
    let search=window.location().search().unwrap_or_default();
    let query=web_sys::UrlSearchParams::new_with_str(&search).ok();
    let opacity=query.as_ref().and_then(|q|q.get("opacity"));
    let nogrid=query.as_ref().and_then(|q|q.get("nogrid"));
    Settings::new(coarse,opacity.as_deref(),nogrid.as_deref()).apply(editor);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Settings { pub opacity:f32, pub grid:bool }

impl Settings {
    pub fn new(coarse:bool,opacity:Option<&str>,nogrid:Option<&str>)->Self {
        let default=if coarse {1.} else {0.95};
        let opacity=opacity.and_then(|s|s.parse::<f32>().ok())
            .filter(|v|v.is_finite()).unwrap_or(default).clamp(0.,1.);
        Self {opacity,grid:nogrid.is_none()}
    }

    pub fn apply(&self,editor:&mut crate::editor::Editor) {
        editor.opacity=self.opacity;
        editor.camera.grid=self.grid.then(Default::default);
    }
}

pub fn density(ratio:f64,override_value:Option<&str>)->f64 {
    let ratio=if ratio>0. {ratio} else {1.};
    let cap=override_value.and_then(|s|s.parse::<f32>().ok())
        .filter(|v|v.is_finite()&&*v>=0.5).map(f64::from).unwrap_or(2.);
    ratio.min(cap)
}

#[cfg(target_arch="wasm32")]
fn query()->Option<web_sys::UrlSearchParams>{
    web_sys::UrlSearchParams::new_with_str(&web_sys::window()?.location().search().ok()?).ok()
}

#[cfg(target_arch="wasm32")]
pub fn scale(ratio:f64)->f64 {
    let value=query().and_then(|q|q.get("dpr"));
    density(ratio,value.as_deref())
}

#[cfg(target_arch="wasm32")]
pub fn initialize(editor:&mut crate::editor::Editor) {
    let Some(window)=web_sys::window() else {return};
    let coarse=window.match_media("(pointer: coarse)").ok().flatten()
        .is_some_and(|media|media.matches());
    let query=query();
    let opacity=query.as_ref().and_then(|q|q.get("opacity"));
    let nogrid=query.as_ref().and_then(|q|q.get("nogrid"));
    Settings::new(coarse,opacity.as_deref(),nogrid.as_deref()).apply(editor);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzg-cap-browser-device-scale-1.rs"
```

```text
--8<-- "journey/code/96zzg-cap-browser-device-scale-2.rs"
```

</details>

### 8. `src/browser.rs`

Give viewport images and CSS ink the same capped scale before recovery reduction.

<details>
<summary>Locate the existing block</summary>

```rust
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let density = crate::browser_recovery::density(crate::browser_defaults::scale(window.device_pixel_ratio()));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzg-cap-browser-device-scale-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/browser_defaults_tests.rs`

Supply high-density, finite override, lower-bound and fallback policy cases.

<details>
<summary>Locate the existing block</summary>

```text
use crate::browser_defaults::Settings;
#[test]
fn pointer_defaults_and_url_overrides_share_existing_editor_state(){
 let mut editor=crate::editor::Editor::default();
 for (coarse,raw,nogrid,opacity,grid) in [
 (false,None,None,0.95,true),(true,None,None,1.,true),
 (true,Some("0.5"),Some("1"),0.5,false),
 (false,Some("NaN"),None,0.95,true),(true,Some("inf"),None,1.,true),
 (false,Some("2"),None,1.,true),(false,Some("-1"),None,0.,true)]{
  Settings::new(coarse,raw,nogrid).apply(&mut editor);
  assert_eq!(editor.opacity,opacity);assert_eq!(editor.camera.grid.is_some(),grid);
 }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzg-cap-browser-device-scale-4.rs" skip=584
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.4.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Desktop faces start at opacity 0.95, touch faces at 1, and the grid is visible. Ordinary pixel density caps at two; finite URL overrides and the reduced-recovery cap remain effective.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for start with current browser display defaults.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use a coarse-pointer media query to keep touch-first devices solid. Read opacity and grid overrides once before renderer creation. Cap ordinary pixel density at two and honor a valid dpr override through the existing recovery policy. Later commands change the editor; a page reload reads startup defaults again.

Follow pointer capability and URL overrides into opacity, the grid owner and capped viewport pixels..



How much more image work does density three require than density two?

For the same CSS viewport, the pixel count grows by 3²/2² = 2.25. The cap reduces every full-screen image pass together.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Reload with ?opacity=0.5&nogrid=1, then without either override. Compare a closed CAD surface and the grid. Emulate a coarse primary pointer and reload: faces start solid. At browser density three, compare canvas dimensions without a dpr override, with ?dpr=1 and with ?dpr=3. Change Opacity and Grid, then orbit and resize: both choices persist until a page reload.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.4.1
npm --prefix ../session_tests run course -- save 20.4.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This connects current desktop/coarse-pointer opacity, the startup grid, nogrid/opacity overrides and DEFAULT_MAX_DPR=2 to the existing renderer. A valid dpr override controls viewport and ink scale together. Ordinary background remains white. Arctic background matching, multisampling and measured adaptive quality still require connection.

Supplied policy fixtures check coarse-pointer defaults, finite URL overrides and device-scale caps. Chrome verifies startup settings, actual GPU texture sizes and view behavior.

Actual browser CAD surface after desktop/coarse-pointer startup and high-density device-scale checks.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.4.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
