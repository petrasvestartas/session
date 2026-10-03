# 32ga · Prepare original kernel data without rebuilding its display

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–31 minutes.** 36 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Validate an immutable source version and restore its Session into a private reload candidate.

**Follow:** Original bytes → bounded decode/version check → candidate Session → original source-GUID lookup.

Introduce `document::decode` as the common bounded protobuf/schema boundary. Ordinary `load_at` still creates an Origin and prepares display meshes. `document::restore` instead validates the expected immutable byte version, constructs the kernel `Session` and returns its Rc directly.

`PreparedReload` owns one key and one candidate `Session`. It checks each matching row’s original source GUID, name and visibility/locking against the decoded kernel mesh. Missing source identity or metadata drift refuses adoption; retained displays are never treated as editable geometry.

A candidate does not change a row. This separation makes it possible to validate every candidate and history root before mutation. SHA-256 here identifies the immutable selected File; mutable published HTTP source policies remain later work.

The ampersand borrows bytes and the Origin for this call. `Result<Rc<Session>, &'static str>` returns either an owned `Session` handle or an error message; the messages are string literals that exist for the whole program. map wraps only a successful `Session` in Rc. A failed candidate cannot change the `Scene` because preparation never receives a mutable `Scene`.

![Validate an immutable source version and restore its `Session` into a private reload candidate.](../illustrations/journey-32ga.svg)

## Type the change

Continue from [Identify the source release a reload belongs to](32g-keys.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32ga-prepare` (from `session_viewer`).

### 1. `src/document.rs`

Share bounded decoding without sharing display preparation.

Find this exact block:

```rust
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    let message = proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
```

Replace that block with:

```rust
--8<-- "journey/code/32ga-prepare-01.rs"
```

### 2. `src/document.rs`

Restore only original kernel data after validating the immutable source version.

Find this exact block:

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
```

Replace that block with:

```rust
--8<-- "journey/code/32ga-prepare-02.rs"
```

### 3. `src/rehydrate.rs`

Keep candidate kernel ownership private until every matched row has a valid original source.

Find this exact block:

```rust
#[cfg(test)]
mod tests {
```

Replace that block with:

```rust
--8<-- "journey/code/32ga-prepare-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the preparation checks below. Matching immutable bytes produce private source candidates; a wrong version is rejected without restoring any live row.

**Verified checkpoint in Chrome.**

![Actual browser result: Prepare original kernel data without rebuilding its display.](../screenshots/journey/32ga-prepare-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Follow the difference between load_at and restore. Identify where display conversion occurs, and why the restored candidate should not invoke it.

</details>

## Explain the change

Why should reloading editable sources avoid preparing another display mesh?

<details>
<summary>Compare your explanation</summary>

The displayed rows and GPU buffers are already retained. Reload needs original kernel ownership for editing, not a second float approximation or a new upload. Decode and validate the exact source version, then keep a candidate Session until every affected row is checked.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32ga-prepare
npm --prefix ../session_tests run course -- save 32ga-prepare
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production hydration brings back kernel Sessions before editing. This preparation stage restores original source ownership while preserving the current display and placement policy.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The browser checks the existing command-only unload behavior and retained drawing. Source hydration at this endpoint is verified by the native state/GPU checks; browser fetch and automatic command replay are still pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32ga-prepare
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
