# 32gf · Give each source request its own owner

**Typing: 15–29 minutes.** [Estimate](typing-load.md).

Give each source reload a ticket. Only the current ticket can complete its batch, and it can complete once.

## Type

Continue from [Ignore old reload results before decoding them](32ge-stale.md). [Save or recover your work](recovery.md).

### 1. `src/lib.rs`

Expose the native request owner without adding browser APIs to it.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod rehydrate;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gf-request-01.rs"
```

### 2. `src/reload_job.rs`

Issue checked tickets and retain source keys only in current pending work.

Create the file and type:

```rust
--8<-- "journey/code/32gf-request-02.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the request checks below. A reload ticket must identify one current batch and permit its completion once. Browser fetching follows next.

**Verified checkpoint in Chrome.**

![Actual browser result: Give each source request its own owner.](../screenshots/journey/32gf-request-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Add a browser-independent `ReloadJob`. It owns only one pending set of release keys, issues checked tickets and gives the caller a `Request` containing a ticket and URL strings. A new request cancels older keys before validating its own candidates. Empty requests, duplicate origins and missing locations return errors.

`finish` first checks the ticket, then takes the matching keys once. `Option<Vec<ReloadKey>>` distinguishes accepted keys from obsolete work. `take` replaces the pending option with `None`; the removed owners drop when the caller finishes with them. Cancellation takes the keys without handing them to abandoned work.

The request ticket identifies this operation; each key still identifies the imported source and its release epoch. Both checks are needed. This native owner is not yet connected to fetch or a new browser command.

Active release keys → ReloadJob → ticket and URL strings → one accepted completion.

![Give each source request its own owner](../illustrations/journey-32gf.svg)

Why does the asynchronous request receive URL strings instead of cloned Origins?

A cloned Origin would retain its URL owner after cancellation or Close. Keep the keys in the pending job, clear them on cancellation, and let asynchronous work carry plain URL strings and a ticket. A late result then has no source ownership to revive.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Keep a returned Request alive after cancelling the job. Explain why its strings cannot keep the Origin or ReloadUrl alive.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32gf-request
npm --prefix ../session_tests run course -- save 32gf-request
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production reloads carry release tokens across asynchronous work. This request owner gives the browser a distinct current-operation boundary and keeps ownership separate from immutable source identity.

The browser checks existing command-only unloading and retained drawing. The new infrastructure is compiled here; the Reload Sources command is connected in the following command lesson.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gf-request
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
