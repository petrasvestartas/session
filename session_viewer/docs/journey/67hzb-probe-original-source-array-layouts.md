# 12.2.21 · Probe original source array layouts

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Start owned asynchronous range tasks and inspect the bounded file probe. Locate the coordinate and colour arrays while checking file bounds, point stride and response status.

## Type

Continue from [Queue original source range reads](67hza-queue-original-source-range-reads.md). [Save or recover your work](recovery.md).

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hzb-probe-original-source-array-layouts-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.21
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Source-array layout probes are prepared for the bounded reader.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for probe original source array layouts.](../screenshots/journey/67hza-queue-original-source-range-reads.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.2.20 · Queue original source range reads](67hza-queue-original-source-range-reads.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



bounded HTTP read → revision and layout → source-array slice → decoded points.



Why require a range response?

The layout parser relies on a bounded partial response rather than an unbounded whole-file transfer.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace an empty array, invalid double stride and truncated colour header.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.21
npm --prefix ../session_tests run course -- save 12.2.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These browser-only source readers connect the existing range gate and decoding helpers.



[Full validation scope](release.md).

</details>
