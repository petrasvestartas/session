# 16.1.12 · Own the originals selected for projection

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 11–22 minutes.** [Estimate](typing-load.md).

Capture selected eligible originals and count unsupported lanes without copying derived proxies.

## Type

Continue from [Replace an original without replacing its row](81c-replace-an-original-without-replacing-its-row.md). [Save or recover your work](recovery.md).

### 1. `src/project_draft.rs`

Capture selected original projection owners.

Create the file and type:

```rust
--8<-- "journey/code/81d-own-the-originals-selected-for-projection-01.rs"
```

### 2. `src/lib.rs`

Register the editing component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_edit;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/81d-own-the-originals-selected-for-projection-02.rs" skip=22
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.12
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

The original network and four-corner patch remain visible.

![Native renderer output for own the originals selected for projection.](../screenshots/journey/80d-exercise-networks-corner-retries-and-exact-original-archives.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.5 · Exercise networks corner retries and exact original archives](80d-exercise-networks-corner-retries-and-exact-original-archives.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



selected rows → original owners → pending plane questions.

![Own the originals selected for projection.](../illustrations/81d-own-the-originals-selected-for-projection.svg)

Does a pending plane edit alter the originals?

No. It owns references and accepted questions. The transaction starts only after a valid plane and all replacement results exist.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the draft’s selected row identities and unsupported count. Confirm empty selection rejects the start.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.12
npm --prefix ../session_tests run course -- save 16.1.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding network/patch contract remains authored and unrun. The final editing contract adds projected originals, atomic Three Point retry, interval extension and active distance/cancellation. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.1.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
