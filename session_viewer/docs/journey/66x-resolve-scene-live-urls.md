# 12.1.36 · Resolve scene and live URLs

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 26–52 minutes.** [Estimate](typing-load.md).

Name source routes separately from their bucket-relative geometry files. Bound live polling and accept an optional announcement stream.

## Type

Continue from [Assemble placed manifest entries](66w-assemble-placed-manifest-entries.md). [Save or recover your work](recovery.md).

### 1. `src/publication_route.rs`

Resolve current named-scene, bucket-root, local-live and notification routes.

Create the file and type:

```rust
--8<-- "journey/code/66x-resolve-scene-live-urls-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod scene_manifest;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66x-resolve-scene-live-urls-02.rs" skip=25
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/publication_route.rs`

Copy source and integration regressions.

<details>
<summary>Locate the existing block</summary>

```text
    Some(Route {manifest:join(&base,&name),base,poll_ms:None,notify:None})
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/66x-resolve-scene-live-urls-03.rs" skip=77
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.36
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

![Native renderer output for resolve scene and live urls.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



scene route → source base → resolved files → live schedule.

![Resolve scene and live URLs.](../illustrations/66x-resolve-scene-live-urls.svg)

Are bucket files relative to the manifest folder?

Published manifests name bucket-root files; a local live source uses its own directory.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare named scenes, data=off, local live paths, unsafe parent traversal and poll bounds.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.36
npm --prefix ../session_tests run course -- save 12.1.36
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.36
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
