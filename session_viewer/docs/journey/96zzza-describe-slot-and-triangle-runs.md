# 20.7.7 · Describe slot and triangle runs

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

A definition needs slots for its placed members and IDs for their triangles. Describe both ranges in one run. Keep used length separate from capacity, and represent free room without a definition key. Reuse the existing sentinel and slot type.

## Type

Continue from [Prepare the integer primitive-slot texture](96zzz-prepare-the-integer-primitive-slot-texture.md). [Save or recover your work](recovery.md).

### 1. `src/slot_space.rs`

Describe slot and ID capacity, definitions and change reports.

Create the file and type:

```rust
--8<-- "journey/code/96zzza-describe-slot-and-triangle-runs-1.rs"
```

### 2. `src/lib.rs`

Register the preparatory allocator types.

<details>
<summary>Locate the existing block</summary>

```rust
mod primitive_slot_table;
```

</details>

After that block, type:

```rust
mod slot_space;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzza-describe-slot-and-triangle-runs-2.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.7
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

![Native renderer output for describe slot and triangle runs.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



definition → reserved slots and IDs → used members.



Why separate used length from capacity?

A small definition can gain members without moving its existing slots when reserved capacity remains.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Sketch a run with capacity four, two occupied members and three triangles per member. Mark the occupied slots separately from their reserved triangle-ID interval. The browser draw path is unchanged.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.7
npm --prefix ../session_tests run course -- save 20.7.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This types the maintained CPU slot-allocation rules. GPU record ownership, slot and lookup uploads, and fragment consumers remain to be connected.



[Full validation scope](release.md).

</details>
