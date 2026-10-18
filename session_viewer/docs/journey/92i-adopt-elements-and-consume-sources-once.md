# 18.4.11 · Adopt Elements and consume sources once

**Chapter 18 · Lighting, render quality and comparison · Section 18.4 · Wood Elements and archives** · [Contents](contents.md)

**Typing: 24–48 minutes.** [Estimate](typing-load.md).

Adopt Elements and consume sources once.

## Type

Continue from [Attach readonly Element attributes](92h-attach-readonly-element-attributes.md). [Save or recover your work](recovery.md).

### 1. `src/element_adopt.rs`

Adopt original Element payloads and readonly attributes as one edit.

Create the file and type:

```rust
--8<-- "journey/code/92i-adopt-elements-and-consume-sources-once-01.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_draft;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/92i-adopt-elements-and-consume-sources-once-02.rs" skip=24
```

### 3. `src/source_edit.rs`

Validate captured original Element identity by shared source ownership.

<details>
<summary>Locate the existing block</summary>

```rust
    match (a, b) {
        (Geometry::PointCloud(a), Geometry::PointCloud(b)) => Rc::ptr_eq(a,b),
        (Geometry::Plane(a), Geometry::Plane(b)) => Rc::ptr_eq(a,b),
        (Geometry::OBB(a), Geometry::OBB(b)) => Rc::ptr_eq(a, b),
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92i-adopt-elements-and-consume-sources-once-03.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.4.11
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

Actual original curved and flat surfaces use the same black outline width.

![Native renderer output for adopt elements and consume sources once.](../screenshots/journey/91i-prove-actual-outlines-and-original-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.3.10 · Prove actual outlines and original ownership](91i-prove-actual-outlines-and-original-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original ordered picks → current Wood builders → typed Element and attributes → one history edit.

![Adopt Elements and consume sources once.](../illustrations/92i-adopt-elements-and-consume-sources-once.svg)

Does Enter create a partial selection on failure?

No. Stage all Elements, base planes and serialized feature outlines, then replace the consumed sources in one history transaction.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect original feature payloads, readonly generated children, default hidden attributes and restored axes after Undo.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.4.11
npm --prefix ../session_tests run course -- save 18.4.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser contract uses the command dock to create a beam from original lines, adjust named values, show features, measure bounds and Undo/Redo. It is syntaxchecked and requires an actual enabled browser. Native fixtures do not constitute browser acceptance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.4.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
