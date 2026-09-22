# current-15 · Translucent faces and the Opacity command
Elements open at 0.7 opacity so the features inside them show, `Opacity 0..1` sets it, and hidden ink dims through the glass instead of vanishing.

## Step 1 · src/app/command.rs
`Opacity 0..1` is parsed, listed and offered with four presets.

`lessons/current-15/src/app/command.rs` · edit · type this

Added after the line `Layers(Option<bool>),` in `lessons/current-14/src/app/command.rs`

```rust
--8<-- "lessons/current-15/src/app/command.rs:current-15-step-1a"
```

Added after the line `"Attributes (On Off): draw or remove the element features…` in `lessons/current-14/src/app/command.rs`

```rust
--8<-- "lessons/current-15/src/app/command.rs:current-15-step-1b"
```

Added after the line `},` in `lessons/current-14/src/app/command.rs`

```rust
--8<-- "lessons/current-15/src/app/command.rs:current-15-step-1c"
```

Added after the line `"attributes" => &["Attributes On", "Attributes Off"],` in `lessons/current-14/src/app/command.rs`

```rust
--8<-- "lessons/current-15/src/app/command.rs:current-15-step-1d"
```

Added after the line `"Object",` in `lessons/current-14/src/app/command.rs`

```rust
--8<-- "lessons/current-15/src/app/command.rs:step-1e"
```

Added after the line `assert_eq!(parse("Attributes"), Ok(Command::Attributes(No…` in `lessons/current-14/src/app/command.rs`

```rust
--8<-- "lessons/current-15/src/app/command.rs:step-1f"
```

## Step 2 · src/state.rs
Elements arrive at 0.7 unless a knob or command already chose an opacity.

`lessons/current-15/src/state.rs` · edit · type this

Added after the line `const SPIN_STEP: f32 = 0.004;` in `lessons/current-14/src/state.rs`

```rust
--8<-- "lessons/current-15/src/state.rs:step-2a"
```

Added after the line `show_selected_names: bool,` in `lessons/current-14/src/state.rs`

```rust
--8<-- "lessons/current-15/src/state.rs:step-2b"
```

Added after the line `show_selected_names: true,` in `lessons/current-14/src/state.rs`

```rust
--8<-- "lessons/current-15/src/state.rs:step-2c"
```

Added after the line `self.annotate_document(first_row);` in `lessons/current-14/src/state.rs`

```rust
--8<-- "lessons/current-15/src/state.rs:step-2d"
```

Added after the line `show` in `lessons/current-14/src/state.rs`

```rust
--8<-- "lessons/current-15/src/state.rs:step-2e"
```

## Step 3 · src/state/edit.rs
The selection label follows a moved object, and `Opacity` is an edit command.

`lessons/current-15/src/state/edit.rs` · edit · type this

Added after the line `}` in `lessons/current-14/src/state/edit.rs`

```rust
--8<-- "lessons/current-15/src/state/edit.rs:step-3a"
```

Added after the line `self.place_gizmo(Some(active.row));` in `lessons/current-14/src/state/edit.rs`

```rust
--8<-- "lessons/current-15/src/state/edit.rs:step-3b"
```

Added after the line `self.place_gizmo(Some(active.row));` in `lessons/current-14/src/state/edit.rs`

```rust
--8<-- "lessons/current-15/src/state/edit.rs:step-3c"
```

Added after the line `}` in `lessons/current-14/src/state/edit.rs`

```rust
--8<-- "lessons/current-15/src/state/edit.rs:step-3d"
```

Added after the line `self.place_gizmo(Some(row));` in `lessons/current-14/src/state/edit.rs`

```rust
--8<-- "lessons/current-15/src/state/edit.rs:step-3e"
```

## Step 4 · src/shaders/triangle.wgsl
A translucent solid drops its back faces, so it reads as one sheet of glass.

`lessons/current-15/src/shaders/triangle.wgsl` · edit · type this

Added after the line `let front = raster_front != (in.mirrored != 0u);` in `lessons/current-14/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/current-15/src/shaders/triangle.wgsl:step-4"
```

## Step 5 · src/shaders/ink_visibility.wgsl
`through_glass` returns how much ink behind a translucent face still shows.

`lessons/current-15/src/shaders/ink_visibility.wgsl` · edit · type this

Added after the line `}` in `lessons/current-14/src/shaders/ink_visibility.wgsl`

```wgsl
--8<-- "lessons/current-15/src/shaders/ink_visibility.wgsl:step-5"
```

## Step 6 · src/shaders/glyph.wgsl
Glyph coverage is scaled by `through_glass`.

`lessons/current-15/src/shaders/glyph.wgsl` · edit · type this

Replaces the 2 lines from `let alpha = coverage(in);` in `lessons/current-14/src/shaders/glyph.wgsl`

```wgsl
--8<-- "lessons/current-15/src/shaders/glyph.wgsl:step-6"
```

## Step 7 · src/shaders/ribbon.wgsl
Ribbon coverage is scaled by `through_glass`.

`lessons/current-15/src/shaders/ribbon.wgsl` · edit · type this

Replaces the 2 lines from `let alpha = coverage(in);` in `lessons/current-14/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/current-15/src/shaders/ribbon.wgsl:step-7"
```

## Step 8 · src/shaders/sphere.wgsl
Sphere coverage is scaled by `through_glass`.

`lessons/current-15/src/shaders/sphere.wgsl` · edit · type this

Replaces the 2 lines from `let alpha = coverage(in);` in `lessons/current-14/src/shaders/sphere.wgsl`

```wgsl
--8<-- "lessons/current-15/src/shaders/sphere.wgsl:step-8"
```

## Step 9 · src/app/ui.rs
With the command box closed, phone keys are the viewport's own bindings.

`lessons/current-15/src/app/ui.rs` · edit · type this

Replaces the lines from `pub fn agent(&mut self, event: super::agent::AgentEvent) {` in `lessons/current-14/src/app/ui.rs`

```rust
--8<-- "lessons/current-15/src/app/ui.rs:step-9a"
```

Added after the line `}` in `lessons/current-14/src/app/ui.rs`

```rust
--8<-- "lessons/current-15/src/app/ui.rs:step-9b"
```

Added after the line `.font(egui::FontId::proportional(14.0))` in `lessons/current-14/src/app/ui.rs`

```rust
--8<-- "lessons/current-15/src/app/ui.rs:step-9c"
```

## Step 10 · src/lib.rs
Keys the agent hands back are fed to the input handler.

`lessons/current-15/src/lib.rs` · edit · type this

Replaces the line `ui.agent(event);` in `lessons/current-14/src/lib.rs`

```rust
--8<-- "lessons/current-15/src/lib.rs:step-10"
```

## Check

Run `trunk serve` in `lessons/current-15/` and open <http://127.0.0.1:8770/>.

Load elements: faces are see-through and the features inside show dimmed; type `Opacity 1` for solid, `Opacity 0` for x-ray.

## Next

[current-16 · Attribute features in red and a one-row command dock](current-16.md)
