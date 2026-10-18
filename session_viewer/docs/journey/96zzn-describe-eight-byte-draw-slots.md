# 20.5.8 · Describe eight-byte draw slots

**Chapter 20 · Shared GPU rendering · Section 20.5 · Shared mesh arenas** · [Contents](contents.md)

**Typing: 14–28 minutes.** [Estimate](typing-load.md).

Store an object row and primitive ID base in one instance-step vertex record.

## Type

Continue from [Connect the renderer to one mesh arena](96zzm-connect-the-renderer-to-one-mesh-arena.md). [Save or recover your work](recovery.md).

### 1. `src/draw_slots.rs`

Type the compact integer layout and reuse the existing growing buffer for slot ownership.

Create the file and type:

```rust
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-1.rs"
```

### 2. `src/lib.rs`

Register the shared slot owner before connecting the instance pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;
```

</details>

After that block, type:

```rust

mod draw_slots;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/draw_slots_tests.rs`

Supply a real GPU readback fixture for exact large integer IDs, growth, reuse, sentinel and release.

Copy this supplied file:

```text
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional GPU fixture.

<details>
<summary>Locate the existing block</summary>

```text
mod draw_slots;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzn-describe-eight-byte-draw-slots-4.rs" skip=15
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.5.8
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

![Native renderer output for describe eight-byte draw slots.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Separate instance selection from object properties. A draw slot contains two integers: the object row to read and the primitive ID base. Reserve slot zero for an ordinary draw that keeps its own object row. Reuse the growing buffer and preserve that sentinel through reset and release.

object row + primitive ID base → eight-byte slot → instance-step vertex input.



Why does a draw slot use an instance-step buffer?

Every vertex of one placed instance reads the same slot; the next instance advances to another object row and ID base.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace slots for two placed copies. Predict the uploaded bytes and explain why slot zero remains available after clearing the instances.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.5.8
npm --prefix ../session_tests run course -- save 20.5.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This prepares the slot buffer and vertex layout. The current instance renderer still reads full object records; the following connection must teach and verify shader row selection and primitive IDs.



[Full validation scope](release.md).

</details>
