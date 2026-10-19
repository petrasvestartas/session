# 12.1.15 · Describe the saved scene metadata

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 25–50 minutes.** [Estimate](typing-load.md).

Separate original document payloads from names, placement and display settings.

## Type

Continue from [Connect retained-session validation and independent witnesses](66bk-connect-retained-session-validation-and-independent-witnesses.md). [Save or recover your work](recovery.md).

### 1. `src/archive.rs`

Define original document framing and validate small presentation metadata.

Create the file and type:

```rust
--8<-- "journey/code/66c-describe-the-saved-scene-metadata-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_validate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66c-describe-the-saved-scene-metadata-02.rs" skip=26
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.15
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

![Native renderer output for describe the saved scene metadata.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



document payloads → small metadata → checked owners.

![Describe the saved scene metadata.](../illustrations/66c-describe-the-saved-scene-metadata.svg)

Are hidden and locked states part of display triangles?

No. Metadata addresses an original object within one document namespace.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace two documents using one original GUID with independent hidden and locked settings.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.15
npm --prefix ../session_tests run course -- save 12.1.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.15
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
