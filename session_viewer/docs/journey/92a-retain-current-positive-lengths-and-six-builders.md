# 18.4.3 · Write original support variable beam and block builders

**Chapter 18 · Lighting, render quality and comparison · Section 18.4 · Wood Elements and archives** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Add the remaining original Support, Beam Variable and Block wrappers. Reuse the kernel constructors and the length parser already written.

## Type

Continue from [Write original beam column and plate builders](91zza-write-original-beam-column-and-plate-builders.md). [Save or recover your work](recovery.md).

### 1. `src/element_support.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-1.rs"
```

### 2. `src/lib.rs`

Register this original element builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_plate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-2.rs" skip=24
```

### 3. `src/element_beam_variable.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-3.rs"
```

### 4. `src/lib.rs`

Register this original element builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_support;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-4.rs" skip=26
```

### 5. `src/element_block.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-5.rs"
```

### 6. `src/lib.rs`

Register this original element builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_beam_variable;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/92a-retain-current-positive-lengths-and-six-builders-6.rs" skip=32
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.4.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Positive length validation and the six Element builders compile before dispatch and history editing are connected.

### Recorded visual result

**Previous native render — current verification pending.**

Actual original curved and flat surfaces use the same black outline width.

![Native renderer output for write original support variable beam and block builders.](../screenshots/journey/91zza-write-original-beam-column-and-plate-builders.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.4.2 · Write original beam column and plate builders](91zza-write-original-beam-column-and-plate-builders.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original picks → positive dimensions → existing Wood kernel → typed element.



Where does the element geometry come from?

The existing Wood kernel constructs it. These short wrappers preserve the supported original inputs and return the typed element or a clear refusal.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the selected original families and refused empty selections for the three wrappers.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.4.3
npm --prefix ../session_tests run course -- save 18.4.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser contract uses the command dock to create a beam from original lines, adjust named values, show features, measure bounds and Undo/Redo. It is syntaxchecked and requires an actual enabled browser. Native fixtures do not constitute browser acceptance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.4.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
