# 32fc · Record a reload version without retaining geometry

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

An unloaded row will need enough information to find its source again. Define Origin as owned document metadata plus an import ID and FileVersion. Its header retains the source name, GUID, tree, graph, bounds and original placement entries. It deliberately has no objects or definitions.

## Type

Continue from [Make editable ownership a private row boundary](32fb-boundary.md). [Save or recover your work](recovery.md).

### 1. `Cargo.toml`

Pin SHA-256 for immutable file-version checks.

<details>
<summary>Locate the existing block</summary>

```toml
prost = "=0.14.4"
```

</details>

Replace that block with:

```toml
--8<-- "journey/code/32fc-version-01.toml"
```

### 2. `src/lib.rs`

Introduce geometry-free reload metadata.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod row_metadata;
```

</details>

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

## Run and check

After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates Cargo.lock, preserves the previous lock, and installs any supplied binary font assets. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 32fc-version
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the version checks below. Changing one file byte changes its fingerprint; importing the same bytes again gets a distinct import ID. The browser picture is unchanged.

**Verified checkpoint in Chrome.**

![Actual browser result: Record a reload version without retaining geometry.](../screenshots/journey/32fc-version-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

FileVersion hashes the original bytes with SHA-256. It is a byte-version identifier, not an authentication scheme. An immutable selected File must return that same version when fetched through its later Blob URL. A server that reserializes equivalent protobuf maps may produce different bytes; mutable-server policies belong to the later publication/streaming lessons.

UUID separates two imports of the same file. None of these values owns a kernel Mesh or Session. The native tests include the standard SHA-256 abc vector, changed bytes, independent origin identities and a geometry-free header. Add the pinned dependency and use the supplied lock, while typing this module yourself.

Own a small document header, a distinct import ID and an exact file fingerprint..

![Own a small document header, a distinct import ID and an exact file fingerprint.](../illustrations/journey-32fc.svg)

Why can two imports share a file fingerprint but still need different origin IDs?

A fingerprint identifies the exact bytes. An origin ID identifies one import into this viewer. Repeated imports can share bytes and source GUIDs while having independent rows, history and reload lifetimes.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change one source byte and predict the version comparison. Then import identical bytes twice and explain why their origin IDs should differ.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32fc-version
npm --prefix ../session_tests run course -- save 32fc-version
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production released sources retain document/tree metadata. This module prepares that ownership boundary; selected immutable files have an exact byte version, while the later HTTP publication flow needs its own version policy.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32fc-version
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
