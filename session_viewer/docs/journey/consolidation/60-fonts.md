# 10.1 · Explicit fonts and source label placement

Provide reproducible fonts and preserve original words, metrics and placement before shaping.

## Ownership and data flow

Pinned text dependencies and bundled font assets remove host-font ambiguity. TextLabel owns UTF-8 words, CSS metrics, colour and one explicit placement kind; its object identity remains separate from the glyphs that will display it. Load a known font database and inspect cmap coverage before deciding whether fallback or whole-font delivery is needed. A missing character is a coverage decision, not an excuse to replace original text.

## The maintained implementation

engine/text.rs owns TextLabel, TextPlacement and the Noto Sans subset byte statics plus FULL_FONTS. app/fonts.rs performs maintained font delivery. This endpoint prepares fonts and source labels; a new visible text specimen is connected after shaping and atlas ownership.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/text.rs](../../../src/engine/text.rs), [engine/gpu/text.rs](../../../src/engine/gpu/text.rs), [engine/gpu/text_plane.rs](../../../src/engine/gpu/text_plane.rs), [engine/gpu/text_plate.rs](../../../src/engine/gpu/text_plate.rs), [app/scene_text.rs](../../../src/app/scene_text.rs), [app/fonts.rs](../../../src/app/fonts.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-60-fonts"></a>

<details>
<summary>Implementation 60-fonts: Pin scene-text shaping and supply the viewer fonts</summary>

Pin scene-text shaping and supply the viewer fonts.

Data flow: pinned shaping renderer → fixed font assets → subset coverage → reproducible glyph layout.

### 1. `Cargo.toml`

Pin the production text renderer and explicit glyph coverage parser.

<details>
<summary>Locate the existing block</summary>

```toml
serde_json = "1.0"

[dev-dependencies]
pollster = "=0.4.0"
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/60-fonts-01.rs"
```

### 2. `index.html`

Make the supplied whole fonts available at the current text asset URLs.

<details>
<summary>Locate the existing block</summary>

```html
  <title>Session Viewer</title>
  <link data-trunk rel="rust">
  <style>
    html, body { width: 100%; height: 100%; margin: 0; overflow: hidden; }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```html
--8<-- "journey/code/60-fonts-02.rs"
```

</details>

<a id="checkpoint-60a-labels"></a>

<details>
<summary>Implementation 60a-labels: Keep source words metrics and placement together</summary>

Keep source words metrics and placement together.

Data flow: original words → label metrics and identity → placement kind → shaped run.

### 1. `src/text.rs`

Define current label/source placement types and explicit bundled font constants.

Create the file and type:

```rust
--8<-- "journey/code/60a-labels-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cad_display_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60a-labels-02.rs" skip=36
```

</details>

<a id="checkpoint-60b-coverage"></a>

<details>
<summary>Implementation 60b-coverage: Load explicit fonts and inspect missing coverage</summary>

Load explicit fonts and inspect missing coverage.

Data flow: bundled font bytes → explicit font database → Unicode cmap → coverage decision.

### 1. `src/text.rs`

Load shared font bytes and inspect their actual Unicode coverage.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60b-coverage-01.rs" skip=9
```

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 60b-coverage
npm --prefix ../session_tests run course -- check 60b-coverage
npm --prefix ../session_tests run course -- save 60b-coverage
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

This endpoint prepares source labels and explicit bundled font coverage; it does not yet draw a new GPU text specimen. Inspect the Lithuanian, German and CAD-symbol coverage cases and compare them with `Fixed Ω cube`. Confirm which font supplies each glyph and keep the original Unicode words unchanged.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Validated shaping and revision-based reuse](60ba-validation.md).
