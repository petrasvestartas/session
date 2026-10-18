# 19.3.1 · Resolve feature rows to original Element owners

**Chapter 19 · Integration and original ownership · Section 19.3 · Selecting, copying and presenting originals** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

The import now has read-only display pieces, but the maintained panel can select their owner.

## Type

Continue from [Prove exact commands across the expanded vocabulary](96r-preserve-exact-command-completions.md). [Save or recover your work](recovery.md).

### 1. `src/feature_owner.rs`

Resolve ownership through bounded hierarchy parent links and collect visible display highlights.

Create the file and type:

```rust
--8<-- "journey/code/96s-resolve-feature-rows-to-original-element-owners-01.rs"
```

### 2. `src/lib.rs`

Compile the owner resolver alongside the existing scene and hierarchy.

<details>
<summary>Locate the existing block</summary>

```rust
mod command_vocabulary;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96s-resolve-feature-rows-to-original-element-owners-02.rs" skip=24
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Keep the imported specimen and select feature rows through their original Element owner.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for resolve feature rows to original element owners.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Walk parent links from the clicked row and stop at the first original Element. Check that owner through the ordinary editable-object predicate. A hidden or locked owner therefore cannot be reached through its feature rows.

Use the same walk to collect visible derived paths for highlighting. Stop at an independent child Element, so selecting the host does not highlight that child’s own feature. Selection still contains only editable originals; the wider list is consumed only by the renderer.

feature row → nearest original Element → editing selection and derived highlight.



Does clicking a projected feature make the display path editable?

No. Resolve the original Element that owns the feature. Selection, camera Fit and editing target that owner; generated paths only contribute visible highlight.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a host feature and an independent child through the ownership helper. Stop at the nearest original Element. This preparation retains the previous browser selection behavior until the [input connection](96t-select-and-highlight-original-feature-owners.md).

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.3.1
npm --prefix ../session_tests run course -- save 19.3.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The maintained state/panel.rs maps hierarchy Pick.row to the original Element, then fits it. Generated feature paths remain display data and never become saved originals.



[Full validation scope](release.md).

</details>
