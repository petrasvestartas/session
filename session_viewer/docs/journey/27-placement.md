# 27 · Give each object a placement

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 21–41 minutes.** 53 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Keep local mesh coordinates and an independent object placement.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Local vertex → Object::world_point → placement matrix → world point.

**Before you finish, explain:** Which value changes when an object moves without changing shape?

A mesh describes a shape. A placement describes where one use of that shape belongs. Keeping both in Object lets history retain the same shared mesh while remembering a different placement.

Add an identity matrix to every newly inserted object. Identity leaves coordinates unchanged, so this checkpoint preserves the working picture. Object::world_point borrows a six-float vertex, uses its first three values as a position, and returns a double-precision world point. Its color is not a coordinate and never enters the transform.

![Local coordinates stay in the shared mesh; Object owns the placement.](../illustrations/journey-27.svg)

Scene::place finds the stable ID before replacing its matrix. It rejects a non-finite matrix and a projective last row. Object placements are affine: they can translate, rotate, scale or shear a shape, but perspective belongs to Camera. Returning Result makes a missing object or invalid placement visible to the caller.

The two checks deliberately use the same Rc<Mesh> before and after a placement. Clone on Rc creates another owner of the allocation. It does not duplicate or rewrite the vertex array. Bounds, picking and GPU presentation will adopt this placement in the next two small lessons; no Move command is exposed yet.

## Type the change

Continue [Frame one object without changing its size](26-selected.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-27-placement`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/scene.rs`

Store one placement beside the shared local mesh.

Find this exact block:

```rust
    pub mesh: Rc<Mesh>,
```

Replace that block with:

```rust
--8<-- "journey/code/27-placement-01.rs"
```

### 2. `src/scene.rs`

Transform position only; return world coordinates without changing the mesh.

Find this exact block:

```rust
#[derive(Clone)]
pub struct Scene {
```

Replace that block with:

```rust
--8<-- "journey/code/27-placement-02.rs"
```

### 3. `src/scene.rs`

Identity preserves the behavior of all existing objects.

Find this exact block:

```rust
        self.objects.push(Object { id, mesh: Rc::new(mesh), source: None });
```

Replace that block with:

```rust
--8<-- "journey/code/27-placement-03.rs"
```

### 4. `src/scene.rs`

Check the placement before mutating the object found by its stable ID.

Find this exact block:

```rust
    pub fn remove(&mut self, id: ObjectId) -> bool {
```

Replace that block with:

```rust
--8<-- "journey/code/27-placement-04.rs"
```

### 5. `src/lib.rs`

Register the placement checks.

Find this exact block:

```rust
#[cfg(test)]
mod selected_fit_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/27-placement-05.rs"
```

### 6. `src/placement_tests.rs`

Prove mesh identity and local coordinates survive placement, and failed updates leave it unchanged.

Create the file and type:

```rust
--8<-- "journey/code/27-placement-06.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run View Isometric, Select Next twice, and Fit Selected. The existing turquoise triangle is framed. Run the Rust checks to compare one local vertex with its placed world point and to verify that a missing object or invalid matrix leaves the object alone.

**Actual Chrome screenshot.**

Give each object a placement. The actual command dock drives this checkpoint; the selected object is highlighted.

![Actual browser result: Give each object a placement.](../screenshots/journey/27-placement-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

In the state check, change translation from (2, −1, 0.5) to (−3, 4, 0). Predict the resulting world point before running the check, then restore it. Explain why the local vertex and Rc allocation stay unchanged.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Its placement matrix changes. The shared mesh keeps local coordinates. Multiplying a local point by the placement produces its world position; multiplying that world point by the camera later produces its screen position.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 27-placement
npm --prefix ../session_tests run course -- save 27-placement
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer stores placement separately from source geometry and composes placements for instances. This checkpoint establishes the document value. World-space queries and GPU model matrices come next.

[Validation status and course release](release.md).
