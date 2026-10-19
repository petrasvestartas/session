# 18.4.2 · Write original beam column and plate builders

**Chapter 18 · Lighting, render quality and comparison · Section 18.4 · Wood Elements and archives** · [Contents](contents.md)

**Typing: 27–54 minutes.** [Estimate](typing-load.md).

Parse positive finite lengths while retaining omitted defaults. Build original beams from selected axes, columns from points and plates from profiles using the existing Wood kernel.

## Type

Continue from [Link current Wood element builders](92-link-current-wood-element-builders.md). [Save or recover your work](recovery.md).

### 1. `src/element_values.rs`

Adapt the maintained positive-length recipe to the earlier number parser.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-1.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod global_outline_command_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-2.rs" skip=47
```

### 3. `src/element_beam.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-3.rs"
```

### 4. `src/lib.rs`

Register this original element builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_values;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-4.rs" skip=25
```

### 5. `src/element_column.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-5.rs"
```

### 6. `src/lib.rs`

Register this original element builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_beam;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-6.rs" skip=23
```

### 7. `src/element_plate.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-7.rs"
```

### 8. `src/lib.rs`

Register this original element builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_column;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/91zza-write-original-beam-column-and-plate-builders-8.rs" skip=25
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.4.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Beam, column and plate source builders are prepared before their values and command routing are connected.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for write original beam column and plate builders.](../screenshots/journey/91zza-write-original-beam-column-and-plate-builders.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original picks → positive dimensions → existing Wood kernel → typed element.



Why keep the untyped defaults?

A short dimension list overrides only its supplied values. Excess, nonfinite and nonpositive values are refused.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a partial dimension override and compare empty or invalid axis, point and profile selections.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.4.2
npm --prefix ../session_tests run course -- save 18.4.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original command helpers prepare the later editor bridge. The existing drawing stays unchanged.



[Full validation scope](release.md).

</details>
