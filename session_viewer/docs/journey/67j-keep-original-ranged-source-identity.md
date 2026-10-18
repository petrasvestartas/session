# 12.2.24 · Keep original ranged source identity

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 9–17 minutes.** [Estimate](typing-load.md).

Read the original cloud GUID and name before its coordinate body. Refuse missing or repeated identity fields.

## Type

Continue from [Fetch original coordinates normals and colours](67i-read-source-arrays-through-the-bounded-queue.md). [Save or recover your work](recovery.md).

### 1. `src/stream.rs`

Read original source identity independently of display pages.

<details>
<summary>Locate the existing block</summary>

```rust
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67j-keep-original-ranged-source-identity-01.rs" skip=73
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.24
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

The mixed source archive scene remains unchanged.

![Native renderer output for keep original ranged source identity.](../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



small source header → original GUID → stable remote source identity.

![Keep original ranged source identity.](../illustrations/67j-keep-original-ranged-source-identity.svg)

Is an HTTP request ticket an original cloud GUID?

No. Tickets identify transient work; the GUID identifies the original geometry.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the parsed identity with the actual kernel protobuf and reject truncated headers.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.24
npm --prefix ../session_tests run course -- save 12.2.24
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The mixed-source Save/Open browser contract remains authored and unrun. These preparatory range helpers connect to actual browser streaming and page drawing in68.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.2.24
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
