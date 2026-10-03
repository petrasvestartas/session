# 32gja · Prove failed restoration cannot partly commit

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 20–40 minutes.** 40 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Verify automatic command failures preserve cold drawing, placement and history, including all-or-nothing multiple-source Save.

**Follow:** Captured operation → complete body set → validate every source → all restored or none → no edit/download on failure.

Test restoration as one transaction across two imported origins. If the second body is invalid, neither source may be restored and the captured Save must not download anything.

Use missing bodies, changed versions and stale keys as rejected examples. A late reply after Close must find its source URL already released. These checks exercise the existing implementation; they add no new viewer command.

![All sources restore together](../illustrations/journey-32gja.svg)

## Type the change

Continue from [Prove restored Save keeps the source doubles](32gj-precision.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gja-failures` (from `session_viewer`).

### 1. `src/lib.rs`

Register multiple-source completion acceptance checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod reload_precision_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gja-failures-01.rs"
```

### 2. `src/reload_failure_tests.rs`

A failure, missing body, changed file, duplicate key or stale epoch must not partially restore a multi-source Save.

Create the file and type:

```rust
--8<-- "journey/code/32gja-failures-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the failed-restoration checks below. An invalid or incomplete reply must leave the scene, source residency and history unchanged, with no edit or Save download.

**Verified checkpoint in Chrome.**

![Actual browser result: Prove failed restoration cannot partly commit.](../screenshots/journey/32gja-failures-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Install each prepared source as soon as its fetch succeeds, then fail the second source. Explain which native and browser residency assertions identify the partial commit.

</details>

## Explain the change

Why must the first valid source remain cold when a second source fails?

<details>
<summary>Compare your explanation</summary>

Save represents one document operation. Installing the first source before checking the remaining responses would leave a partially restored editor after the operation failed. Validate the complete set before installation.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gja-failures
npm --prefix ../session_tests run course -- save 32gja-failures
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Document restoration validates every response before committing editable source owners; errors and abandoned work cannot partially mutate the current document.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome sends actual Move/Delete/Save commands through HTTP failure, invalid Content-Length, missing body, rejected fetch, oversized body, failed stream, invalid response type and changed source bytes. Each failure must leave placement, selection, camera, displayed pixels, residency and GPU counters unchanged; Save starts no download. It also imports a second source and fails its fetch after the first body has succeeded. No partial source restoration or document edit is accepted.

The final proof placement moves the post clear of the beam. Keeping two front faces in exactly the same plane can produce depth competition; this demonstration separates the solids instead of claiming the later rendering-quality lessons are already implemented.

The restoration implementation already separates preparation from installation. This acceptance checkpoint makes the boundary observable across two independently imported origins. Native checks combine a current Save with a network failure, missing body or changed second source and prove both sources remain cold. Duplicate or stale keys revoke the complete reply without replay.

The precision checkpoint remains active. Finally, a held real response is released after Close and its source URL must already be revoked. Controlled failures and waits test ownership and atomicity, not real device speed.

Visible Chrome checks automatic Move/Delete/Save failures, unchanged drawing/state/GPU counters, multiple-source Save atomicity, no failed download, and Close revoking a source URL before a held reply is released.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gja-failures
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
