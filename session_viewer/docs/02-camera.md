# 02 · Camera

The triangle orbits, pans and zooms toward the cursor.

![Orbit turns the orientation about the target, pan slides the target across the camera's own plane, and the wheel scales the distance; the view-projection is rebuilt from those three every frame.](illustrations/camera-basis.svg)

## Step 1 · src/camera.rs

New file: the camera with orbit, pan, zoom and its view matrix.

`lessons/02/src/camera.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/02/src/camera.rs:step-1a"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1b"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1c"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1d"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1e"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1f"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1g"
```

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1h"
```

Copy this part from the lesson folder to the path shown.

`lessons/02/src/camera.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:step-1i"
```

Run `cargo check` in `lessons/02/`.

## Step 2 · src/lib.rs

Wire the camera into the struct, the gestures and the uniform upload.

`lessons/02/src/lib.rs` · edit · type this

Replaces the lines from `use wasm_bindgen::prelude::*;` to `use wgpu::util::DeviceExt;` in `lessons/01/src/lib.rs`

```rust
--8<-- "lessons/02/src/lib.rs:step-2a"
```

Replaces the lines from `pipeline: wgpu::RenderPipeline,` to `scale: f64,` in `lessons/01/src/lib.rs`

```rust
--8<-- "lessons/02/src/lib.rs:step-2b"
```

Replaces the lines from `let _ = (dx, dy, pan);` to `let _ = (delta, x, y);` in `lessons/01/src/lib.rs`

```rust
--8<-- "lessons/02/src/lib.rs:step-2c"
```

Added below

```rust
            cache: None,
        });
```

```rust
--8<-- "lessons/02/src/lib.rs:step-2d"
```

Replaces the lines from `group,` to `scale: 1.0,` in `lessons/01/src/lib.rs`

```rust
--8<-- "lessons/02/src/lib.rs:step-2e"
```

Added above

```rust
        let output = match self.surface.get_current_texture() {
```

```rust
--8<-- "lessons/02/src/lib.rs:step-2f"
```

Replaces the lines from `Ok(serde_json::json!({` to `.to_string())` in `lessons/01/src/lib.rs`

```rust
--8<-- "lessons/02/src/lib.rs:step-2g"
```

## Step 3 · index.html

Copy the page: the title and status say checkpoint 02.
Copy this file from the lesson folder to the path shown.

`lessons/02/index.html` · edit · copy the file

Replaces the line `<title>01 - First WebGPU frame</title>` in `lessons/01/index.html`

```html
--8<-- "lessons/02/index.html:step-3a"
```

Replaces the line `<output id="status">Starting checkpoint 01</output>` in `lessons/01/index.html`

```html
--8<-- "lessons/02/index.html:step-3b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/01/index.html`

```html
--8<-- "lessons/02/index.html:step-3c"
```

## Check

Run `trunk serve` in `lessons/02/` and open <http://127.0.0.1:8770/>.

Expected: The triangle orbits, pans and zooms toward the cursor; status: **Checkpoint 02 · 1 objects**.

![Checkpoint 02: the same triangle seen from the production camera; drag to orbit, Shift-drag to pan, wheel to zoom at the cursor.](screenshots/02.png)

If it fails:

- Dragging moves twice as far on a high-DPI screen: the cursor is scaled twice.
- A distant model jitters: coordinates become f32 before the anchor is subtracted.
- The triangle disappears: reversed depth needs a zero clear and a Greater comparison.

## What changed

```text
lessons/02/src/
├── shaders/
│   └── first.wgsl
├── camera.rs  +
└── lib.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: gesture → camera → anchored matrix → uniform → vertex. Every file at this point: `lessons/02/`.

## Next

[03 · Object rows and identity](03-identity.md)

## Expected viewer result

Checkpoint 02: the same triangle seen from the production camera; drag to orbit, Shift-drag to pan, wheel to zoom at the cursor.

[![Full viewer result for 02 camera](screenshots/02.png)](screenshots/02.png)
