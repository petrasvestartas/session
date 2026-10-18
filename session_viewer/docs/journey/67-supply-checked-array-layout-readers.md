# 12.2.1 · Bound scalar headers and byte addresses

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 7–13 minutes.** [Estimate](typing-load.md).

Reuse the varint decoder from 66b. A protobuf scalar has a wire-dependent byte length; byte addresses stay u64 until a bounded slice requires usize. checked_add must precede the end comparison. This is the boundary between an external file layout and later GPU-ready arrays.

## Type

Continue from [Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). [Save or recover your work](recovery.md).

### 1. `src/stream.rs`

Bound scalar headers and byte addresses.

Create the file and type:

```rust
--8<-- "journey/code/67-supply-checked-array-layout-readers-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
pub mod browser_publication;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67-supply-checked-array-layout-readers-02.rs" skip=60
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The cumulative browser application compiles. This parser preparation preserves the current drawing; actual range requests and streamed pages connect in the following lessons.

### Recorded visual result

**Previous native render — current verification pending.**

The mixed source archive scene remains unchanged.

![Native renderer output for bound scalar headers and byte addresses.](../screenshots/journey/67-supply-checked-array-layout-readers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original file bytes → checked source layout → existing GPU array owners → later range streaming.



Why check addition before comparing the message end?

An overflowing byte address could wrap into a small, apparently valid offset.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a body at u64::MAX-2 with length 4; predict the rejection before evaluating the end bound.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.1
npm --prefix ../session_tests run course -- save 12.2.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Parser preparation preserves the inherited scene; actual browser streaming is exercised after the range queue is connected.

[Full validation scope](release.md).

</details>
