# 22 · The egui layer

egui draws the viewer's panels: each frame the code describes every panel again, and egui turns that description into triangles drawn over the scene. This lesson builds the layer, the `Panel` trait with its `PANELS` list, the routing of pointer events between panels and scene, and the first panel, the gumball number box.

![One egui frame: winit events feed the egui context, its shapes are tessellated and uploaded with the font texture, and a last render pass draws them after the scene and the gumball.](illustrations/extend-ui.svg)

## Step 1 · src/engine/gpu/ui.rs

New file: the GPU painter, which owns egui's own wgpu renderer built for the canvas format.

`lessons/22/src/engine/gpu/ui.rs` · type this, new file

```rust
--8<-- "lessons/22/src/engine/gpu/ui.rs:painter"
```

## Step 2 · src/engine/gpu/ui.rs

Upload one frame of egui output: free old textures, upload changed ones, tessellate the shapes and fill the vertex buffers.

`lessons/22/src/engine/gpu/ui.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/engine/gpu/ui.rs:painter-prepare"
```

## Step 3 · src/engine/gpu/ui.rs

Draw the triangles in a last pass that keeps the finished scene, close the impl, and free every texture on drop.

`lessons/22/src/engine/gpu/ui.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/engine/gpu/ui.rs:painter-draw"
```

## Step 4 · src/engine/gpu/render.rs

Another `impl Gpu` block: `draw_panels` runs the egui pass when a painter exists, once the scene and gumball are drawn.

`lessons/22/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/engine/gpu/render.rs:panels"
```

## Step 5 · src/app/ui/theme.rs

New file: the panel fonts, the small ones inside the binary first, each font falling back to the next for missing glyphs.

`lessons/22/src/app/ui/theme.rs` · type this, new file

```rust
--8<-- "lessons/22/src/app/ui/theme.rs:theme-fonts"
```

## Step 6 · src/app/ui/theme.rs

The white theme: black text, white fields, light grey panels, no borders and a cursor that does not blink.

`lessons/22/src/app/ui/theme.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/theme.rs:theme-visuals"
```

## Step 7 · src/app/ui/mod.rs

New file: the module lines and the `panels!` macro, whose list starts with one entry, the number box.

`lessons/22/src/app/ui/mod.rs` · type this, new file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:panels-registry"
```

## Step 8 · src/app/ui/mod.rs

The `Panel` trait: ten hooks the frame calls on every panel, all but `show` with a default body.

`lessons/22/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:panel-trait"
```

## Step 9 · src/app/ui/mod.rs

Questions asked of all panels at once: does one take the keys, own Escape, or have a field or popup under the pointer.

`lessons/22/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:panel-queries"
```

## Step 10 · src/app/ui/mod.rs

The recorded controls for browser tests, what a frame hands back, and the `Ui` struct that holds egui between frames.

`lessons/22/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:ui-struct"
```

## Step 11 · src/app/ui/mod.rs

Open `impl Ui`: swap in the whole fonts later, and set egui up with one layout pass and the white theme.

`lessons/22/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:ui-new"
```

## Step 12 · src/app/ui/mod.rs

One frame: fill every panel, run egui over the events in arrival order, apply what the panels took, then upload.

`lessons/22/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:ui-frame"
```

## Step 13 · src/app/ui/mod.rs

Write the panel state onto the canvas for browser tests, close the impl, and two helpers that record a control's rectangle.

`lessons/22/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/mod.rs:ui-publish"
```

## Step 14 · src/app/ui/overlay.rs

New file: screen points joined, a square at the last and a label; for now a snapped drag's point and snap name.

`lessons/22/src/app/ui/overlay.rs` · type this, new file

```rust
--8<-- "lessons/22/src/app/ui/overlay.rs:overlay-drawing"
```

## Step 15 · src/app/ui/pointer.rs

New file: route each window event, so a press on a panel stays in egui and a press on the scene reaches the viewer.

`lessons/22/src/app/ui/pointer.rs` · type this, new file

```rust
--8<-- "lessons/22/src/app/ui/pointer.rs:pointer-event"
```

## Step 16 · src/app/ui/pointer.rs

A press goes to every panel, so the one whose field it lands on can focus it; the brace closes the impl.

`lessons/22/src/app/ui/pointer.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/pointer.rs:pointer-press"
```

## Step 17 · src/app/ui/number_box.rs

New file: what the number box remembers between frames, kept in a thread-local outside egui.

`lessons/22/src/app/ui/number_box.rs` · type this, new file

```rust
--8<-- "lessons/22/src/app/ui/number_box.rs:number-state"
```

## Step 18 · src/app/ui/number_box.rs

The number box's hooks: open from the lesson 21 state, apply a typed value as one undo step, and report its field.

`lessons/22/src/app/ui/number_box.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/number_box.rs:number-hooks"
```

## Step 19 · src/app/ui/number_box.rs

Draw the box beside its handle, keep the keys while it is open, and close it on Escape.

`lessons/22/src/app/ui/number_box.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/app/ui/number_box.rs:number-draw"
```

## Step 20 · src/lib.rs

Another `impl App` block: create both halves of the layer, share the fonts, and let the panels see each event first.

`lessons/22/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/22/src/lib.rs:panels"
```

## Step 21 · registration lines

Copy the lines tagged `register:ui` and `register:egui` from these files of `lessons/22/`:

- `src/app/mod.rs`: the `ui` module, browser and tests only.
- `src/engine/gpu/mod.rs`: the `ui` module, the painter field and its start value `None`.
- `src/engine/gpu/render.rs`: the `draw_panels` call after the gumball in `encode`.
- `src/lib.rs`: the `ui` field and its start value, `adopt_panels` when the window opens, `panel_fonts` when the whole fonts arrive, the event test with `panels_take`, and the frame's `ui.frame` with `repaint_if`.

Copy the empty `egui_tests` module at the end of `src/state/edit.rs`, and the browser test `tests/docked-workspace.cjs`; it drives the command line and the layers panel, so it runs from lesson 30 on.

Run `cargo check` in `lessons/22/`.

## Check

`cargo check` compiles. In `trunk serve`, select an object and click a gumball arrow without dragging: a white box opens beside it; type 200, press Enter, and the object moves 200 mm as one undo step. Escape closes the box. Its `field` hook stays unused until lesson 23 feeds it from the hidden input of [lesson 00](00-environment.md), step 6, so a phone keyboard can type into it.
