# 14.3.9 · Match solid command options

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 12–23 minutes.** [Estimate](typing-load.md).

Compare normalized option words while keeping Enter and Escape outside shape choices.

## Type

Continue from [Exercise oriented original planar constructions](73g-exercise-oriented-original-planar-constructions.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_options.rs`

Normalize option labels once and consume matching input words.

Create the file and type:

```rust
--8<-- "journey/code/73h-match-solid-command-options-1.rs"
```

### 2. `src/lib.rs`

Register the shared option matcher.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod planar_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73h-match-solid-command-options-2.rs" skip=31
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.9
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

![Native renderer output for match solid command options.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



Match solid command options → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why exclude Escape from the option iterator?

Escape is an action that cancels an owner, rather than a solid representation.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try matching mixed case and a multiword label on paper; count how many input words the match consumes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.9
npm --prefix ../session_tests run course -- save 14.3.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.



[Full validation scope](release.md).

</details>
