# 12.1.23 · Prepare original saved documents

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Preserve source headers and tree depth; derive local placements from the current world models.

## Type

Continue from [Store cloud size as presentation](66j-store-cloud-size-as-presentation.md). [Save or recover your work](recovery.md).

### 1. `src/archive_document.rs`

Prepare original document records and precise relative placements for saving.

Create the file and type:

```rust
--8<-- "journey/code/66k-prepare-saved-documents-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod archive_identity;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66k-prepare-saved-documents-02.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.23
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

The original cloud and local point marker remain unchanged.

![Native renderer output for prepare original saved documents.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



source namespace → original geometry → local placement → saved tree.

![Prepare original saved documents.](../illustrations/66k-prepare-saved-documents.svg)

Should every Save add another tree wrapper?

No. Preserve the existing document root so repeated roundtrips keep their original depth.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Save and reopen six times, retaining precise pose, original document GUID and current layer.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.23
npm --prefix ../session_tests run course -- save 12.1.23
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.23
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
