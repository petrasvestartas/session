# 18b · Clipping planes and section caps

A clipping plane is a plane object that hides everything on the side its arrow points to, and where it passes through a closed solid a hatched or grey cap fills the cut. The [section plane design](capstone.md) explains the choices; this lesson builds them.

![A vertex-stage rejection leaves a staircase, a fragment-stage discard cuts exactly on the plane, and a cap fills the opening.](illustrations/section-plane.svg)

## Step 1 · registration lines

One line in `PASSES` adds the clip pass, and one line in `BEFORE_PICKS` hands the planes to it every frame.

`lessons/18b/src/engine/gpu/pass.rs` · type the line tagged `register:clip`

```rust
--8<-- "lessons/18b/src/engine/gpu/pass.rs:passes"
```

`lessons/18b/src/state/features.rs` · type the line tagged `register:clipping`

```rust
--8<-- "lessons/18b/src/state/features.rs:features-hooks"
```

Copy the other lines tagged `register:clip` and `register:clipping` from these files of `lessons/18b/`:

- `src/engine/gpu/mod.rs`: the `clip` module.
- `src/app/mod.rs` and `src/state.rs`: the two `clipping` modules.
- `src/state/features.rs`: the `clip_hidden` count in `Features`.
- `src/app/inspection.rs`: the planes in the inspection snapshot.

## Step 2 · src/app/clipping.rs

The five ways to place a plane, the word typed for each, and the prompt for every point.

`lessons/18b/src/app/clipping.rs` · type this, new file

```rust
--8<-- "lessons/18b/src/app/clipping.rs:clip-mode"
```

## Step 3 · src/app/clipping.rs

Build the plane object from the picked points, its rectangle `half` wide each way, and flip it.

`lessons/18b/src/app/clipping.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/app/clipping.rs:plane-from"
```

## Step 4 · src/app/clipping.rs

Turn a placed plane object into the world plane the GPU cuts with, hatch direction included.

`lessons/18b/src/app/clipping.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/app/clipping.rs:clip-plane"
```

## Step 5 · src/app/clipping.rs

Tests: each mode cuts the side it names, bad picks are refused, and the cut follows its placement.

`lessons/18b/src/app/clipping.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/18b/src/app/clipping.rs:clipping-tests"
```

## Step 6 · src/engine/gpu/clip.rs

A world clipping plane, its signed distance, and whether it passes through a box.

`lessons/18b/src/engine/gpu/clip.rs` · type this, new file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-plane"
```

## Step 7 · src/engine/gpu/clip.rs

What the uniform is built from, the crossing count texture, and the pick records under the caps.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-resources"
```

## Step 8 · src/engine/gpu/clip.rs

Instanced solids a plane crosses, one draw per definition, with a row and plane record per copy.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-placed"
```

## Step 9 · src/engine/gpu/clip.rs

The cap and pick pipelines, and Clip, which owns the planes and everything made for them.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-struct"
```

## Step 10 · src/engine/gpu/clip.rs

Open `impl Clip`: find, per plane, the closed solids it crosses, since only those can show a cap.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-find"
```

## Step 11 · src/engine/gpu/clip.rs

Take new planes, list them, and build this frame's uniform.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-set"
```

## Step 12 · src/engine/gpu/clip.rs

Count the crossings behind a plane into the count texture, then draw its caps into the face pass.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-counts"
```

## Step 13 · src/engine/gpu/clip.rs

Draw the caps into the outline masks, so a cut solid keeps its silhouette.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-masks"
```

## Step 14 · src/engine/gpu/clip.rs

Name the solid under each cap pixel for the pick, and count the bytes; the brace closes the impl.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-pick"
```

## Step 15 · src/engine/gpu/clip.rs

Which planes cap this frame, the `Gpu` calls that set planes and find solids, and the pass constructor.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-gpu"
```

## Step 16 · src/engine/gpu/clip.rs

The Clip pass: uniform, counts and caps before the faces, the last caps inside the face pass, then masks and ids.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-pass"
```

## Step 17 · src/engine/gpu/clip.rs

The uniform: planes relative to the anchor, their clip-space form, and hatch coordinates across the screen.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-uniform"
```

## Step 18 · src/engine/gpu/clip.rs

The cap shader text for one or four samples, and the count, cap, mask and pick pipelines, made on first use.

`lessons/18b/src/engine/gpu/clip.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-pipelines"
```

## Step 19 · src/engine/gpu/clip.rs

Tests: the uniform matches the shader, planes survive far origins and millimetre cameras, and the hatch follows the plane.

`lessons/18b/src/engine/gpu/clip.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/18b/src/engine/gpu/clip.rs:clip-tests"
```

## Step 20 · src/shaders/cap.wgsl

One fullscreen triangle per plane, the instance index naming the plane.

`lessons/18b/src/shaders/cap.wgsl` · type this, new file

```wgsl
--8<-- "lessons/18b/src/shaders/cap.wgsl:cap-vertex"
```

## Step 21 · src/shaders/cap.wgsl

The cap's outputs and its hatch: diagonal lines whose spacing stays readable at any zoom.

`lessons/18b/src/shaders/cap.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18b/src/shaders/cap.wgsl:cap-hatch"
```

## Step 22 · src/shaders/cap.wgsl

A black border wherever the section's coverage ends.

`lessons/18b/src/shaders/cap.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18b/src/shaders/cap.wgsl:cap-outline"
```

## Step 23 · src/shaders/cap.wgsl

A cap pixel lies inside a solid and is kept by the other planes; it draws hatch or grey at the plane's depth.

`lessons/18b/src/shaders/cap.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18b/src/shaders/cap.wgsl:cap-fragment"
```

## Step 24 · src/shaders/cap.wgsl

The caps into both outline masks, or selected caps into the selection mask alone.

`lessons/18b/src/shaders/cap.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18b/src/shaders/cap.wgsl:cap-masks"
```

## Step 25 · src/shaders/cap.wgsl

In the pick, a cap pixel names the solid it lies inside, else the nearest one the ray leaves.

`lessons/18b/src/shaders/cap.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18b/src/shaders/cap.wgsl:cap-pick"
```

## Step 26 · src/state/clipping.rs

Every frame, collect the clipping plane rows where they are drawn and hand their planes to the GPU.

`lessons/18b/src/state/clipping.rs` · type this, new file

```rust
--8<-- "lessons/18b/src/state/clipping.rs:update-clipping"
```

## Step 27 · src/state/clipping.rs

On the first cut, flag each mesh closed and outward or inward, walking each shared geometry once.

`lessons/18b/src/state/clipping.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/state/clipping.rs:verify-solids"
```

## Step 28 · src/state/clipping.rs

The planes as JSON for the inspection snapshot; the brace closes the impl.

`lessons/18b/src/state/clipping.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18b/src/state/clipping.rs:clipping-status"
```

## Step 29 · tests

Copy `tests/clipping-mixed.cjs` from `lessons/18b/`: a browser check on the mixed scene; it types commands, so it runs from lesson 33 on.

Run `cargo check` in `lessons/18b/`.

## Check

`cargo check` compiles, and `cargo xtest --lib clip` passes the plane, uniform and hatch tests. A scene that saved a clipping plane is cut when it loads; lesson 23 adds the Clipping Plane command that places, flips and switches planes.
