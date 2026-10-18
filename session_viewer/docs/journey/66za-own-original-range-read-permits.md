# 12.2.11 · Own original range-read permits

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 27–54 minutes.** [Estimate](typing-load.md).

Write the per-scene gate, queued turn, waiting-read and permit types. Bound simultaneous reads, wake queued turns and hand a released permit to the oldest waiting read.

## Type

Continue from [Validate complete octrees before range streaming](67si-validate-complete-octrees-before-range-streaming.md). [Save or recover your work](recovery.md).

### 1. `src/range_gate.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/66za-own-original-range-read-permits-1.rs"
```

### 2. `src/lib.rs`

Register the original browser range-read gate.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod stream;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66za-own-original-range-read-permits-2.rs" skip=17
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.11
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

A retained permit describes one original range read; the existing browser scene is unchanged.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for own original range-read permits.](../screenshots/journey/66za-own-original-range-read-permits.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



queued range read → permit → browser request → release.



Why does a permit own the gate?

The permit releases its slot when dropped. Its gate must remain alive until that release, even if the caller has already discarded its scene reference.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a permit release with and without an older queued turn. The waiting-read Future is completed in the next lesson before this gate is connected.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.11
npm --prefix ../session_tests run course -- save 12.2.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Prepare the per-scene range gate; following lessons connect it to browser range reads.



[Full validation scope](release.md).

</details>
