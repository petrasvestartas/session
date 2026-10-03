# 32fc · Record a reload version without retaining geometry

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 21–41 minutes.** 52 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Own a small document header, a distinct import ID and an exact file fingerprint.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Own a small document header, a distinct import ID and an exact file fingerprint..

**Before you finish, explain:** Why can two imports share a file fingerprint but still need different origin IDs?

An unloaded row will need enough information to find its source again. Define Origin as owned document metadata plus an import ID and FileVersion. Its header retains the source name, GUID, tree, graph, bounds and original placement entries. It deliberately has no objects or definitions.

FileVersion hashes the original bytes with SHA-256. It is a byte-version identifier, not an authentication scheme. An immutable selected File must return that same version when fetched through its later Blob URL. A server that reserializes equivalent protobuf maps may produce different bytes; mutable-server policies belong to the later publication/streaming lessons.

UUID separates two imports of the same file. None of these values owns a kernel Mesh or Session. The native tests include the standard SHA-256 abc vector, changed bytes, independent origin identities and a geometry-free header. Add the pinned dependency and use the supplied lock, while typing this module yourself.

![Own a small document header, a distinct import ID and an exact file fingerprint.](../illustrations/journey-32fc.svg)

## Type the change

Continue [Make editable ownership a private row boundary](32fb-boundary.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32fc-version`. A save keeps your own work; it does not fill in the next lesson.

### 1. `Cargo.toml`

Pin SHA-256 for immutable file-version checks.

Find this exact block:

```toml
prost = "=0.14.4"
```

Replace that block with:

```toml
--8<-- "journey/code/32fc-version-01.toml"
```

### 2. `src/lib.rs`

Introduce geometry-free reload metadata.

Find this exact block:

```rust
pub mod row_metadata;
```

Replace that block with:

```rust
--8<-- "journey/code/32fc-version-02.rs"
```

### 3. `src/origin.rs`

Keep descriptive protobuf data and a byte version without retaining kernel objects.

Create the file and type:

```rust
--8<-- "journey/code/32fc-version-03.rs"
```

## Run and look

After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates Cargo.lock, preserves the previous lock, and installs any supplied binary font assets. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 32fc-version
```

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the state tests. The viewer picture is unchanged: origins are not attached to rows until the next endpoint.

**Actual Chrome screenshot.**

![Actual browser result: Record a reload version without retaining geometry.](../screenshots/journey/32fc-version-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change one source byte and predict the version comparison. Then import identical bytes twice and explain why their origin IDs should differ.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A fingerprint identifies the exact bytes. An origin ID identifies one import into this viewer. Repeated imports can share bytes and source GUIDs while having independent rows, history and reload lifetimes.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fc-version
npm --prefix ../session_tests run course -- save 32fc-version
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production released sources retain document/tree metadata. This module prepares that ownership boundary; selected immutable files have an exact byte version, while the later HTTP publication flow needs its own version policy.

[Validation status and course release](release.md).
