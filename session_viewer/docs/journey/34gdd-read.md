# 4.2.29 · Measure the selected file read

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 4–7 minutes.** [Estimate](typing-load.md).

Start the browser clock immediately before `File.array_buffer()`. After the await, accept the existing ticket before measuring or delivering anything.

## Type

Continue from [Measure GPU startup and the first frame](34gdc-startup.md). [Save or recover your work](recovery.md).

### 1. `src/file_input.rs`

Measure an actual completed read only after its ticket is accepted.

<details>
<summary>Locate the existing block</summary>

```rust
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let started = crate::browser_phase::now();
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
        let bytes = result.as_ref().map_or(0, |buffer| js_sys::Uint8Array::new(buffer).length() as u64);
        crate::browser_phase::finish(if result.is_ok() { "file read" } else { "file read failed" },
            started, bytes, &file.name());
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gdd-read-01.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Open a file, then type `Report`. Its phases list includes `file read`, the read duration, actual bytes and filename.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for measure the selected file read.](../screenshots/journey/34gba-lifecycle.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [4.2.18 · Own diagnostic page transitions](34gba-lifecycle.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Read the returned buffer’s length on success; rejected reads record zero bytes. The phase keeps a bounded filename with query and fragment suffixes removed. Recording diagnostics leaves the existing import transaction unchanged.

selected file → actual read promise → current ticket → retained phase → diagnostic download.

![Only the current read ticket can retain a completed measurement.](../illustrations/journey-34gdd.svg)

Why accept the read ticket before recording its duration?

Cancelled and superseded reads may still complete. Only the current ticket can append a measurement or deliver a file to this run.

Study estimate, including typing and experiments: 0.25–0.5 hours.

</details>

<details>
<summary>Optional experiment</summary>

Cancel a pending Open. Its late read must add neither a file nor a measurement.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.29
npm --prefix ../session_tests run course -- save 4.2.29
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This measures actual selected-file reads, including current failures. Decode, display preparation, GPU upload, network resources and live replacement measurements follow.

Held actual File.arrayBuffer promises verify clock boundaries, actual byte counts, sanitized names, quiet late completions, rejected reads, downloaded JSON and unchanged Undo/Redo behavior.

Chrome holds actual file reads to verify measured success, failure, cancellation, replacement and final exit; typed reports expose the retained values.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.29
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
