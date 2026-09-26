# 10 · Text shaping

Shaping turns a string into glyphs placed on a line, with kerning, ligatures and fallback fonts. This lesson builds the CPU half of text: fonts, labels, and a document that reshapes only the labels whose text changed.

![Shape once, place per frame, raster per device scale, then a plate pass and a glyph pass.](illustrations/text-pipeline.svg)

## Step 1 · assets/text/

The three Noto fonts, their small subsets that go into the wasm, the font license and the notes on where they came from.

`lessons/10/assets/text/` · copy the files

- `NotoSans-Regular.ttf`, `NotoSans-Regular.subset.ttf`
- `NotoSansSymbols-Regular.ttf`, `NotoSansSymbols-Regular.subset.ttf`
- `NotoSansSymbols2-Regular.ttf`, `NotoSansSymbols2-Regular.subset.ttf`
- `OFL.txt`, `README.md`

## Step 2 · src/engine/text.rs

New file: the three font subsets compiled into the wasm, and the whole fonts that lesson 14 fetches only when a label needs them.

`lessons/10/src/engine/text.rs` · type this, new file

```rust
--8<-- "lessons/10/src/engine/text.rs:fonts"
```

## Step 3 · src/engine/text.rs

A label: its text, size, colour, one of five placements, and the object it belongs to.

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:labels"
```

## Step 4 · src/engine/text.rs

Open `impl TextDocument`: shape a new label set, reusing the glyphs of every label whose text and size did not change.

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:document"
```

## Step 5 · src/engine/text.rs

Swap the font set and reshape, list every glyph for the tests; the impl block closes, then `Default` and the glyph record follow.

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:font-swap"
```

## Step 6 · src/engine/text.rs

Ask whether the bundled subsets can draw a string, and load them as the font system.

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:coverage"
```

## Step 7 · src/engine/text.rs

Reject a label whose size, position or clip box is not finite or out of range.

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:validate"
```

## Step 8 · src/engine/text.rs

Shape one label with cosmic-text: no wrapping, advanced shaping, and the label id carried along as metadata.

`lessons/10/src/engine/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:shape"
```

## Step 9 · src/engine/text.rs

Tests: ligatures, accents and symbols shape to real glyphs, and moving or recolouring a label never reshapes it.

`lessons/10/src/engine/text.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/10/src/engine/text.rs:tests"
```

## Step 10 · registration lines

Copy the line tagged `register:text` from `lessons/10/src/engine/mod.rs`: it declares the new module.

Run `cargo check` in `lessons/10/`.

## Check

Run `cargo xtest --lib engine::text` in `lessons/10/`: five tests pass, and nothing on the canvas changes yet, because lesson 11 draws the labels.
