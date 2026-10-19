# 12.1.21 · Map saved source identities

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Map row IDs back to original document GUIDs, retaining unique identities for copies moved between documents.

## Type

Continue from [Inventory saved document namespaces](66h-inventory-saved-document-namespaces.md). [Save or recover your work](recovery.md).

### 1. `src/archive_identity.rs`

Map original sources, tree nodes and copies inside their document namespace.

Create the file and type:

```rust
--8<-- "journey/code/66i-map-saved-source-identities-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod scene_archive;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66i-map-saved-source-identities-02.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.21
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

![Native renderer output for map saved source identities.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



row GUID → namespace alias → source tree identity.

![Map saved source identities.](../illustrations/66i-map-saved-source-identities.svg)

May a moved copy overwrite an original source record?

No. Reserve existing row identities before selecting source aliases.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Move one of two repeated imports into the first layer and verify every saved source remains distinct.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.21
npm --prefix ../session_tests run course -- save 12.1.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.21
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
