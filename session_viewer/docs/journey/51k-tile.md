# 7.6.13 · Own color, depth and resolve textures together

**Chapter 7 · Drawing, snapping and transform tools · Section 7.6 · Move, rotate and scale handles** · [Contents](contents.md)

**Typing: 15–30 minutes.** [Estimate](typing-load.md).

A tile owns matching4x color/depth views and its single-sample resolve image. The bind group retains the sampled view and sampler; replacing the tile releases the old resources together.

## Type

Continue from [Separate handle depth from scene depth](51j-pipelines.md). [Save or recover your work](recovery.md).

### 1. `src/widget.rs`

Own the widget’s three matching texture attachments and sampled image group.

Create the file and type:

```rust
--8<-- "journey/code/51k-tile-01.rs"
```

### 2. `src/lib.rs`

Register the handle component and supplied checks.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod widget_layout;

pub mod widget_pipelines;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/51k-tile-02.rs" skip=50
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The earlier two gold boxes and temporary Move band remain.

![Native renderer output for own color, depth and resolve textures together.](../screenshots/journey/50d-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [7.5.5 · Cancel previews without consuming document history](50d-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



bounded extent →4x color and depth →1x resolve → sampled image owner.

![Own color, depth and resolve textures together](../illustrations/journey-51k.svg)

How many bytes per physical tile pixel does the ledger count?

36bytes:16for4xRGBA color,16for4x32-bit depth and4for the resolved RGBA image.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare64-pixel tile increments with the1024-pixel cap. Allocation stays bounded independently of the drawing’s object count.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 7.6.13
npm --prefix ../session_tests run course -- save 7.6.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This preparation retains the earlier Move input. Current handle math and drawing are introduced in dependency order; selection and pointer input are connected later in this subject.

The earlier temporary Move remains while handle infrastructure is introduced.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 7.6.13
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
