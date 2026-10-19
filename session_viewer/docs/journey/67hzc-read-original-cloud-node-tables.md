# 12.2.22 · Read original cloud node tables

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Read original node-table fields through the cached metadata window. Reject duplicate fields and oversized tables while retaining point-ID and normal ranges.

## Type

Continue from [Probe original source array layouts](67hzb-probe-original-source-array-layouts.md). [Save or recover your work](recovery.md).

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
--8<-- "journey/code/67hzc-read-original-cloud-node-tables-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.22
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original cloud node tables can be decoded; streamed drawing connects later.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read original cloud node tables.](../screenshots/journey/67hza-queue-original-source-range-reads.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.2.20 · Queue original source range reads](67hza-queue-original-source-range-reads.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



bounded HTTP read → revision and layout → source-array slice → decoded points.



Why bound the accumulated node-table bytes?

Many individually valid fields could otherwise require an excessive combined metadata allocation.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a duplicate field, absent required field and table-size limit.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.22
npm --prefix ../session_tests run course -- save 12.2.22
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These browser-only source readers connect the existing range gate and decoding helpers.



[Full validation scope](release.md).

</details>
