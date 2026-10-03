# 34b · Describe a viewer run without keeping its document

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–49 minutes.** 65 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Define owned diagnostic context and a serializable run outcome, without retaining scene or GPU owners.

**Follow:** browser metadata → owned Context → Report header → explicit JSON → independent snapshot.

Create a report that survives losing the GPU. `Context` stores page/browser details and dimensions. `Report` stores that context, run identity, timestamps and outcome; it owns no document or device.

Owned strings and copied dimensions make a clone an independent snapshot. `Outcome` distinguishes Running, Ready, Closed and Failed. A Running value alone is not proof of a crash.

Serde’s derives serialize and parse the value. `rename_all` writes camelCase JSON names; `flatten` puts Context fields in the report’s top level. `version` identifies the format. `started` stays fixed while `lastSeen` can advance.

This lesson builds the Rust value. Reading live browser metadata and downloading it are separate steps below.

![Independent diagnostic context](../illustrations/journey-34b.svg)

## Type the change

Continue from [Stop the viewer when its GPU device fails](34a-stop.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34b-report` (from `session_viewer`).

### 1. `src/diagnostic.rs`

Create a serializable report header with owned browser context and explicit run outcome.

Create the file and type:

```rust
--8<-- "journey/code/34b-report-01.rs"
```

### 2. `src/diagnostic_tests.rs`

Check the actual JSON field names, round trip and independence of a retained snapshot.

Create the file and type:

```rust
--8<-- "journey/code/34b-report-02.rs"
```

### 3. `src/lib.rs`

Expose the report model and include its native tests.

Find this exact block:

```rust
pub mod gpu_fault;
```

Replace that block with:

```rust
--8<-- "journey/code/34b-report-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the report checks below. Serialize a report and inspect its flat JSON header; changing a cloned snapshot must not change the live value. Browser metadata is connected later.

**Verified checkpoint in Chrome.**

![Actual browser result: Describe a viewer run without keeping its document.](../screenshots/journey/34b-report-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Remove serde(flatten), run the JSON assertion, and explain how the public format changed. Then compare the live and cloned report after changing its drawing-buffer size.

</details>

## Explain the change

Why does a diagnostic snapshot own small strings and dimensions instead of keeping the editor or renderer?

<details>
<summary>Compare your explanation</summary>

The report must survive the viewer’s disposal without extending scene or GPU lifetimes. Owned metadata records the relevant context independently; document values and rendering handles are absent.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34b-report
npm --prefix ../session_tests run course -- save 34b-report
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production diagnostics retain browser/page/viewport/drawing-buffer context outside the renderer and version their reports. This checkpoint establishes that value; bounded observations, downloads, storage policy and recovery follow.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Native tests check actual JSON names, header values, round-trip decoding and independent context after resize. Chrome verifies the already-wired loss/input/lifetime behavior and a new proof view. This header is not yet populated from the live browser, downloadable or stored. Events, first-failure retention and browser connection follow.

Chrome verifies the inherited real GPU-loss and restart acceptance, then captures Orbit Up. The report header’s schema and ownership are checked natively; live reporting is not wired yet.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34b-report
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
