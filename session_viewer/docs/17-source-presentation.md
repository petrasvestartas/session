# 17 · Source faces, text objects and one silhouette

Ctrl + Shift + click selects one face of a solid, scene text becomes objects you can pick and undo, and O draws one black border around every visible solid. The border comes from a mask of what is visible, searched pixel by pixel for its edge.

## Step 1 · src/engine/gpu/faces.rs

A second `impl Faces` block: the face behind a pick id, a face's address, and the highlighted draw.

`lessons/17/src/engine/gpu/faces.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:face-source"
```

## Step 2 · src/state.rs

A face pick selects that face, moves the selection outline to its object and names it in the status line.

`lessons/17/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state.rs:pick-face"
```

## Step 3 · src/app/scene_text.rs

A text object is a label plus the scene row that makes it pickable, hideable and undoable.

`lessons/17/src/app/scene_text.rs` · type this, new file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:scene-text"
```

## Step 4 · src/app/scene_text.rs

Open `impl Scene`: the manifest's texts and a document title replace the old ones and keep their rows.

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:manifest-texts"
```

## Step 5 · src/app/scene_text.rs

A known key reuses its row, a new one pushes a TEXT row, so a reloaded manifest keeps its selection.

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:register-text"
```

## Step 6 · src/app/scene_text.rs

Add, delete and step texts by undo label, then flag each changed row hidden or shown on the GPU.

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:text-undo"
```

## Step 7 · src/app/scene_text.rs

Find a text by row and list the visible ones with their selection flag; the brace closes the impl.

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:text-lookup"
```

## Step 8 · src/state/text.rs

Open `impl State`: a loaded document's name floats above its solids, and a new text is one undo step.

`lessons/17/src/state/text.rs` · type this, new file

```rust
--8<-- "lessons/17/src/state/text.rs:annotate-document"
```

## Step 9 · src/state/text.rs

Load the full fonts, then send the scene's texts and the selected object's name to the text lane.

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:update-label"
```

## Step 10 · src/state/text.rs

Grow each text row's box around the shaped words, so fit, pick and select see the text; the brace closes the impl.

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:text-bounds"
```

## Step 11 · src/state/text.rs

Two helpers: the centre of a box and a rounded white-on-black nameplate.

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:nameplate"
```

## Step 12 · src/state/text.rs

A second `impl State` block with the one call the loader makes when texts arrive.

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:set-texts"
```

## Step 13 · src/shaders/surface_outline.wgsl

The two masks, their radius and coarse copies, and the table of neighbour offsets the search walks.

`lessons/17/src/shaders/surface_outline.wgsl` · type this, new file

```wgsl
--8<-- "lessons/17/src/shaders/surface_outline.wgsl:outline-inputs"
```

## Step 14 · src/shaders/surface_outline.wgsl

Search both masks within the radius, nearest offsets first, and store the outline strength as one byte per pixel.

`lessons/17/src/shaders/surface_outline.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/17/src/shaders/surface_outline.wgsl:outline-search"
```

## Step 15 · src/shaders/surface_outline.wgsl

Blend black over the scene with that strength.

`lessons/17/src/shaders/surface_outline.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/17/src/shaders/surface_outline.wgsl:outline-blend"
```

## Step 16 · src/shaders/surface_outline.wgsl

Shrink a mask to block maxima, then grow those by one block, so the search can skip empty regions.

`lessons/17/src/shaders/surface_outline.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/17/src/shaders/surface_outline.wgsl:outline-pool"
```

## Step 17 · src/shaders/face_coverage.wgsl

A mask pixel's coverage is the share of its samples where the face pass left a triangle.

`lessons/17/src/shaders/face_coverage.wgsl` · type this, new file

```wgsl
--8<-- "lessons/17/src/shaders/face_coverage.wgsl:face-fraction"
```

## Step 18 · src/shaders/face_coverage.wgsl

Inside an MSAA mask pass: full coverage on exactly the samples that show a face.

`lessons/17/src/shaders/face_coverage.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/17/src/shaders/face_coverage.wgsl:face-samples"
```

## Step 19 · src/engine/gpu/surface_outline.rs

One coverage mask with its MSAA target and its two coarse copies.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, new file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-mask"
```

## Step 20 · src/engine/gpu/surface_outline.rs

The tap table, the alpha texture and the face coverage pipelines.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-tables"
```

## Step 21 · src/engine/gpu/surface_outline.rs

The key a mask was drawn for, the two outline kinds, and the SurfaceOutline that owns everything above.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-struct"
```

## Step 22 · src/engine/gpu/surface_outline.rs

Open `impl SurfaceOutline`: layouts, the radius uniform and the pipelines; textures wait for the first frame.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-new"
```

## Step 23 · src/engine/gpu/surface_outline.rs

The cache check, one pass that writes both masks against the scene depth, and selection bookkeeping that frees an unused mask.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-masks"
```

## Step 24 · src/engine/gpu/surface_outline.rs

Make the mask textures for this canvas size, sample count and block size, and write the radius.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-prepare"
```

## Step 25 · src/engine/gpu/surface_outline.rs

Build the tap table and the one-byte alpha texture, or free both when no outline is due.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-alpha"
```

## Step 26 · src/engine/gpu/surface_outline.rs

Open a mask pass, fill it from the face pass's triangle ids, then pool and dilate it.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-passes"
```

## Step 27 · src/engine/gpu/surface_outline.rs

Search the masks into the alpha only when one changed, blend it, and count the bytes; the brace closes the impl.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-composite"
```

## Step 28 · src/engine/gpu/surface_outline.rs

The outline width in pixels, the block size and the sorted tap offsets.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-radius"
```

## Step 29 · src/engine/gpu/surface_outline.rs

The composite, alpha, pool and face coverage pipelines.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-pipelines"
```

## Step 30 · src/engine/gpu/surface_outline.rs

Tests: one border around touching solids, none for a hidden selection, freed masks, and no pipeline compiled twice.

`lessons/17/src/engine/gpu/surface_outline.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-tests"
```

## Step 31 · src/engine/gpu/surface_outline.rs

The Outline pass: both outlines as lanes, masks redrawn after the faces only when their key changed, and the blend over the ink.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:outline-pass"
```

## Step 32 · examples and tests

Copy these files from `lessons/17/`; they are checked, not explained.

- `examples/mk_selection_overlap.rs` and `tests/selection-overlap.py`: selected strokes over solids against identical controls.
- `examples/mk_stroke_joins.rs` and `tests/stroke-joins.py`: joined strokes with no dark dots at the joints.

## Step 33 · registration lines

One line in `PASSES` adds the outline to every frame, after the faces and over the ink.

`lessons/17/src/engine/gpu/pass.rs` · type the line tagged `register:outline`

```rust
--8<-- "lessons/17/src/engine/gpu/pass.rs:passes"
```

Copy the other lines tagged `register:surface_outline`, `register:scene_text` and `register:text` from these files of `lessons/17/`:

- `src/engine/gpu/mod.rs`: the `surface_outline` module.
- `src/app/scene.rs`: the `text` module read from `scene_text.rs`, the `texts` list, its reset and the text name lookup.
- `src/state.rs`: the `text` module, the document title after a load, the label refresh after each selection change, and the face pick.
- `src/engine/gpu/render.rs`: the selected face highlight.
- `src/app/loader.rs` and `src/lib.rs`: the `Texts` message, its handler, the full fonts and the texts restored after a load.

Run `cargo check` in `lessons/17/`.

## Check

`cargo check` compiles, and `cargo xtest --lib surface_outline` passes the tap table and shader tests. With `trunk serve` in `lessons/17/`, O puts one black border around the visible solids, and Ctrl + Shift + click on a solid selects one face with the status **Face N selected**.

![Checkpoint 17. Left: Ctrl + Shift + click inside the mesh selects one source face, the rest of the object stays grey. Middle: the selected BRep with its silhouette after pressing O, one black border of uniform width around the yellow fill. Right: without the silhouette only the yellow strokes remain.](screenshots/17-face-silhouette.png)
