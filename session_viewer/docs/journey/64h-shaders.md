# 11.2.9 · Declare point-cloud records and decode source normals

**Chapter 11 · Outlines and point clouds · Section 11.2 · Point clouds and detail levels** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Match the existing forty-word row layout and decode packed normals with a guarded inverse-transpose transform.

## Type

Continue from [Walk only detail whose screen spacing needs refinement](64g-detail.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_points.wgsl`

Create the file and type:

```wgsl
--8<-- "journey/code/64h-shaders-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.2.9
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

This cloud section is written; its runtime passes are connected by the following buffer and pipeline lessons.

### Recorded visual result

**Previous native render — current verification pending.**

The selected CAD bore and black original and silhouette contours remain visible.

![Native renderer output for declare point-cloud records and decode source normals.](../screenshots/journey/63h-outline.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.1.10 · Check real contours widths reuse picks and cleanup](63h-outline.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use the original point columns and the existing forty-word placement table. The shader decodes the stored normal formats instead of expanding every source normal into another GPU column. Its guarded cofactor transform handles singular and reflected placements. The new cloud shader is developed in this same browser application.

declare point-cloud records and decode source normals.



Why does a reflected model need the determinant sign when transforming a normal?

The inverse transpose changes orientation under reflection. Keeping the determinant sign preserves the source normal’s facing after the cofactor transform.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one original point through this section. Why does a reflected model need the determinant sign when transforming a normal? Predict its values before the later real cloud pass connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.2.9
npm --prefix ../session_tests run course -- save 11.2.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited silhouette browser contract remains authored and unrun. Cloud acceptance connects at the final endpoint; native software rendering is not Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.2.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
