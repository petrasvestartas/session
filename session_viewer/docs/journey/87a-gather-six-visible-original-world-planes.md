# 17.2.5 · Gather six visible original world planes

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

Gather six visible original world planes.

## Type

Continue from [Derive placed world half-spaces from original planes](87sc-derive-placed-world-half-spaces-from-original-planes.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_rows.rs`

Retain the original source and explicit derived owner.

Create the file and type:

```rust
--8<-- "journey/code/87a-gather-six-visible-original-world-planes-01.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod clipping_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/87a-gather-six-visible-original-world-planes-02.rs" skip=28
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.5
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

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for gather six visible original world planes.](../screenshots/journey/87a-gather-six-visible-original-world-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original source → checked derived clipping → current view.

![Gather six visible original world planes.](../illustrations/87a-gather-six-visible-original-world-planes.svg)

Does locking a plane disable cutting?

No. Visibility controls cutting; locking controls picking. Gather its world frame without changing original coordinates.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare hidden, locked and placed planes, the six-plane ceiling and derived preview size.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.5
npm --prefix ../session_tests run course -- save 17.2.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The browser contract is supplied and requires an actual enabled browser. Native software-GPU pixels and picks do not measure Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.2.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
