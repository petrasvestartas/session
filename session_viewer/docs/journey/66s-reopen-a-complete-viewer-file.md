# 12.1.31 · Reopen a complete viewer file

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 7–14 minutes.** [Estimate](typing-load.md).

Prepare all original documents, labels and settings before replacing the active scene.

## Type

Continue from [Restore scoped presentation settings](66r-restore-scoped-presentation-settings.md). [Save or recover your work](recovery.md).

### 1. `src/scene_document.rs`

Reopen every original document and restore all supported scene presentation.

<details>
<summary>Locate the existing block</summary>

```rust
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66s-reopen-a-complete-viewer-file-01.rs" skip=3
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/scene_document.rs`

Copy mixed-source, precision, tree-depth, namespace, release and atomic refusal tests.

<details>
<summary>Locate the existing block</summary>

```text
    scene.restore(candidate);Ok(())
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/66s-reopen-a-complete-viewer-file-02.rs" skip=38
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.31
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

![Native renderer output for reopen a complete viewer file.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



checked archive → temporary scene → complete metadata → one adoption.

![Reopen a complete viewer file.](../illustrations/66s-reopen-a-complete-viewer-file.svg)

May invalid archive metadata clear the displayed scene?

No. Keep the active scene until the complete candidate passes.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run mixed source roundtrips, repeated IDs, six stable tree roundtrips and rejected-file checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.31
npm --prefix ../session_tests run course -- save 12.1.31
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.31
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
