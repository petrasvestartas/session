# 34f · Read previous reports from real browser storage

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 21–41 minutes.** 52 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Read admitted saved reports using stable tab identity, the real browser timestamp parser and bounded scanning.

**Follow:** sessionStorage tab → localStorage candidates → byte/schema/shape admission → Date.parse recency → latest eligible evidence.

Read previous reports from browser storage. `Store` keeps a storage handle, this tab’s stable ID and the new run key. Session storage preserves the tab ID across reloads; unavailable storage uses a fresh ID without stopping startup.

Scan at most 256 storage keys and 32 keys in the course namespace. Pass each candidate through bounded decoding and the recency policy. Browser `Date.parse` supplies actual timestamp interpretation.

Rank failures by first fatal time and unfinished runs by heartbeat. Reading removes nothing. This lesson introduces the reader; bounded writes and the previous-report command follow.

![Read validated storage evidence](../illustrations/journey-34f.svg)

## Type the change

Continue from [Prove saved-run exclusions before adopting storage](34ec-proof.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34f-storage` (from `session_viewer`).

### 1. `src/report_storage.rs`

Read bounded validated candidates from actual browser storage and rank with the real Date parser; retain only metadata and storage handles.

Create the file and type:

```rust
--8<-- "journey/code/34f-storage-01.rs"
```

### 2. `src/lib.rs`

Build the storage owner only for the browser target and expose its debug acceptance probe.

Find this exact block:

```rust
mod browser_report;
```

Replace that block with:

```rust
--8<-- "journey/code/34f-storage-02.rs"
```

### 3. `Cargo.toml`

Enable the pinned browser Storage bindings without changing package versions.

Find this exact block:

```toml
"Location", "Navigator",
```

Replace that block with:

```toml
--8<-- "journey/code/34f-storage-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the storage browser acceptance in the expandable notes below. The reader must choose eligible evidence from real browser storage and retain the tab ID across reloads. Live persistence follows next.

**Verified checkpoint in Chrome.**

![Actual browser result: Read previous reports from real browser storage.](../screenshots/journey/34f-storage-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Change selection to rank lastSeen, then compare the two saved failures whose heartbeat order disagrees with failure order. Explain which evidence should win and why. Deny browser storage and explain why current diagnostics must remain usable.

</details>

## Explain the change

Why keep tab identity separate from the new run’s storage key?

<details>
<summary>Compare your explanation</summary>

The tab identity survives a reload so interruption policy can recognize its old run. A fresh storage key identifies each run independently; reusing the tab as the run key would overwrite the previous evidence.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34f-storage
npm --prefix ../session_tests run course -- save 34f-storage
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The production reader uses browser storage and stable tab identity. This endpoint connects the native admission/recency policy to actual storage reads. Bounded writes, live report adoption and the previous-report command follow.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Headed Chrome uses real Web Storage and UTC timestamp parsing to test candidate selection, stable tab identity, independent run keys and denied storage. Current viewer reports are still not persisted at this checkpoint.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34f-storage
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
