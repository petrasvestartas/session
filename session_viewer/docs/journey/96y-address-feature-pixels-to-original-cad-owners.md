# 19.3.7 · Address feature pixels to original CAD owners

**Chapter 19 · Integration and original ownership · Section 19.3 · Selecting, copying and presenting originals** · [Contents](contents.md)

**Typing: 6–12 minutes.** [Estimate](typing-load.md).

The integer picking pass must write original editing identities.

## Type

Continue from [Regenerate display for copied feature owners](96x-regenerate-display-for-copied-feature-owners.md). [Save or recover your work](recovery.md).

### 1. `src/feature_owner.rs`

Reuse the existing original-owner traversal for a displayed GPU primitive.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) { return scene.pick_id(id.value()); }
    for doc in &scene.forest.documents {
        if let Some(index) = doc.nodes.iter().position(|node| node.object == Some(id)) {
            return ancestor(scene, doc, index);
        }
    }
    None
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-01.rs"
```

</details>

### 2. `src/pick_sources.rs`

Write an original editing ID and suppress invented subpart tags for derived ink.

<details>
<summary>Locate the existing block</summary>

```rust
        let id = |owner: ObjectId, tag| [scene.pick_id(owner.value()).map_or(0, ObjectId::value), tag];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/feature_pick_tests.rs`

Supply independent original-owner address and guard witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-03.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied original-owner address fixture.

<details>
<summary>Locate the existing block</summary>

```rust
mod copy_feature_tests;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod feature_pick_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pick_gpu_tests.rs`

Supply a real integer GPU feature-pixel readback witness.

<details>
<summary>Locate the existing block</summary>

```text
#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-05.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 19.3.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for address feature pixels to original cad owners.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A derived feature path is visible geometry, but it is not an editable Session object. Resolve its nearest original Element or InstanceRef with the ownership traversal already taught for tree rows. Original paths keep their existing IDs.

Use this resolver when building the GPU ink-ID arrays. A generated feature gets its owner ID and a zero subpart tag: its tessellated segments are display data, not original PathEdge indices. Object mode therefore selects the original CAD owner; edge and control modes cannot invent a source subpart. Hidden and locked original owners still fail the existing eligibility guard.

This change reuses the integer target, readback and stale-result handling already developed. It adds no second picking pass. The supplied GPU witness clicks a feature line away from its Element mesh and verifies the returned owner ID, then locks the original and repeats the click.

The dependency command also supplies assets/pb/course-feature-pick.pb: two original Elements with feature lines away from their meshes, with the second Element locked. Open it in the browser and use object selection to check the original-owner address directly.

original source → fresh copy identity → original placement → regenerated display.



Why is the subpart tag zero for a generated feature outline?

A tessellated display segment has no editable original PathEdge index. Zero identifies only the original object; it prevents the edge-mode resolver from treating generated geometry as a kernel source part.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Click a visible interaction outline in object mode and Delete, then Undo. The original Element and its features disappear and return together. Try edge mode: it must not expose a generated outline segment as an original editable edge. Lock the owner and repeat.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.3.7
npm --prefix ../session_tests run course -- save 19.3.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Current app/walk/mod.rs draws features with cx.row, the original Element or instance row. This stage gives the cumulative integer pick pass the same original ownership. Current packed draw formats remain a separate rendering bridge.

An original interaction fixture checks six generated path addresses, nearest child ownership, zero subparts and locked/hidden guards. A real integer GPU readback clicks an isolated feature line and verifies original-owner selection and locked rejection.

Actual browser feature-pixel ownership and original archive regeneration acceptance pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 19.3.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
