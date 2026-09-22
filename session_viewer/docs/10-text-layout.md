# 10 · Text shaping

The text specimen page compares five shaped text sizes against browser text.

![Shape once, place per frame, raster per device scale, then a plate pass and a glyph pass.](illustrations/text-pipeline.svg)

Copy each file from the lesson folder to the path shown.

Copy from `lessons/10/` (tooling this checkpoint needs but the course does not teach):

- `lessons/10/assets/text/NotoSans-Regular.ttf` (binary)
- `lessons/10/assets/text/NotoSansSymbols-Regular.ttf` (binary)
- `lessons/10/assets/text/NotoSansSymbols2-Regular.ttf` (binary)

## Step 1 · assets/text/OFL.txt

Copy this file from the lesson folder to the path shown.

`lessons/10/assets/text/OFL.txt` · 94 lines · copy the file, new file

```text
--8<-- "lessons/10/assets/text/OFL.txt"
```

## Step 2 · assets/text/README.md

Copy this file from the lesson folder to the path shown.

`lessons/10/assets/text/README.md` · 39 lines · copy the file, new file

```markdown
--8<-- "lessons/10/assets/text/README.md"
```

## Step 3 · src/engine/performance.rs

Performance counters separate frame timing from resource capacity.

`lessons/10/src/engine/performance.rs` · 197 lines · type this, new file

```rust
--8<-- "lessons/10/src/engine/performance.rs"
```

## Step 4 · src/engine/text.rs

Text layout retains shaped glyph positions for rendering.

`lessons/10/src/engine/text.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4a"
```

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4b"
```

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4c"
```

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4d"
```

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4e"
```

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4f"
```

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4g"
```

Copy this part from the lesson folder to the path shown.

`lessons/10/src/engine/text.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:step-4h"
```

## Step 5 · src/engine/mod.rs

The engine module exposes the rendering implementation.

`lessons/10/src/engine/mod.rs` · edit · type this

Replaces `mod pipelines` in `lessons/09/src/engine/mod.rs`

```rust
--8<-- "lessons/10/src/engine/mod.rs:step-5"
```

Run `cargo check` in `lessons/10/`.

## Step 6 · src/text_layout.rs

Copy this file from the lesson folder to the path shown.

`lessons/10/src/text_layout.rs` · 64 lines · copy the file, new file

```rust
--8<-- "lessons/10/src/text_layout.rs"
```

## Step 7 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners.

`lessons/10/src/lib.rs` · edit · type this

Added after the `pub mod fixture;` line of `lessons/09/src/lib.rs`

```rust
--8<-- "lessons/10/src/lib.rs:step-7a"
```

Replaces the `Ok(serde_json::json!({"stage":9,"objects":sel…` line in `fn render` of `lessons/09/src/lib.rs`

```rust
--8<-- "lessons/10/src/lib.rs:step-7b"
```

## Step 8 · assets/text-layout.html

Copy this file from the lesson folder to the path shown.

`lessons/10/assets/text-layout.html` · 45 lines · copy the file, new file

```html
--8<-- "lessons/10/assets/text-layout.html"
```

## Step 9 · index.html

Copy this file from the lesson folder to the path shown.

`lessons/10/index.html` · edit · copy the file

Replaces the `<title>Session checkpoint 09</title>` line of `lessons/09/index.html`

```html
--8<-- "lessons/10/index.html:step-9a"
```

Replaces the 4 lines from `</head>` of `lessons/09/index.html`

```html
--8<-- "lessons/10/index.html:step-9b"
```

Replaces the `document.getElementById('status').textContent…` line of `lessons/09/index.html`

```html
--8<-- "lessons/10/index.html:step-9c"
```

## Check

Run `trunk serve` in `lessons/10/` and open <http://127.0.0.1:8770/>.

Expected: The text specimen page compares five shaped text sizes against browser text; status: **PASS**.

![Checkpoint 10: the reference page shapes one string at five sizes; the browser row behind each specimen has the same width, and the report lists every glyph with its cluster, advance and baseline.](screenshots/10-text-layout.png)

If it fails:

- The specimen widths differ: font bytes, size or kerning settings differ.
- Glyph order is wrong: character order replaces the shaped glyph sequence.

## What changed

```text
lessons/10/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── backdrop.rs
│   ├── buffers.rs
│   ├── cloud.rs
│   ├── frame.rs
│   ├── glyphs.rs
│   ├── instance.rs
│   ├── lod.rs
│   ├── mod.rs
│   ├── objects.rs
│   ├── segments.rs
│   ├── splat.rs
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs
│   └── view.rs
├── pipelines/
│   ├── layouts.rs
│   └── mod.rs
├── mod.rs  ~
├── performance.rs  +
└── text.rs  +
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/10/`.

## Next

[11 · Text rendering](11-text-rendering.md): placement, raster scale, coverage atlas and the black plates.

## Expected viewer result

Checkpoint 10: the canvas itself is unchanged.

[![Full viewer result for 10 text layout](screenshots/10.png)](screenshots/10.png)
