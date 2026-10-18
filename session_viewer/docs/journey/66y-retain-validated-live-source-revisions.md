# 12.1.37 · Retain validated live source revisions

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 30–59 minutes.** [Estimate](typing-load.md).

Cache complete original documents. Invalid or incomplete updates keep the last displayed scene and remain retryable.

## Type

Continue from [Resolve scene and live URLs](66x-resolve-scene-live-urls.md). [Save or recover your work](recovery.md).

### 1. `src/publication_index.rs`

Keep original source revisions and build only complete changed scenes.

Create the file and type:

```rust
--8<-- "journey/code/66y-retain-validated-live-source-revisions-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod publication_route;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66y-retain-validated-live-source-revisions-02.rs" skip=28
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/publication_index.rs`

Copy source and integration regressions.

<details>
<summary>Locate the existing block</summary>

```text
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/66y-retain-validated-live-source-revisions-03.rs" skip=8
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.37
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

![Native renderer output for retain validated live source revisions.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



validated manifest → original source cache → complete replacement.

![Retain validated live source revisions.](../illustrations/66y-retain-validated-live-source-revisions.svg)

Should identical bytes allocate new row IDs?

No. Keep the current scene and shared original sources until a valid byte revision changes.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run changed/missing/weak validators, invalid updates, missing files and released obsolete cache owners.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.37
npm --prefix ../session_tests run course -- save 12.1.37
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.37
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
