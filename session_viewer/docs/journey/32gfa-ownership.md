# 32gfa · Prove cancelled work releases its source owners

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 26–51 minutes.** 54 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Prove cancelled work releases its source owners.

**Follow:** Pending job holds keys → Close drops rows → cancel drops keys → abandoned request is inert.

Add three native checks. The first starts requests out of order and proves that an older ticket cannot consume current keys; a matching ticket consumes them once. Cancellation, empty requests and duplicate origins leave no pending work.

The ownership check closes the editor while a pending job still owns the source. Then it cancels the job and keeps a simulated abandoned asynchronous owner alive. `Weak::upgrade` must return `None` for the Origin and URL while the request strings remain readable. This directly tests the reason for separating URL strings from source owners.

Finally force ticket exhaustion. A checked increment must fail, never wrap, and leave older keys cancelled. These tests do not claim the browser has aborted a network read yet; that controller belongs to the later browser flight.

![Prove cancelled work releases its source owners](../illustrations/journey-32gfa.svg)

## Type the change

Continue from [Give each source request its own owner](32gf-request.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gfa-ownership` (from `session_viewer`).

### 1. `src/reload_job.rs`

Check ordering, exhaustion and actual owner expiration while abandoned work remains alive.

Find this exact block:

```rust
    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}
```

Replace that block with:

```rust
--8<-- "journey/code/32gfa-ownership-01.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the request-owner checks below. Cancelling a batch releases its request metadata; a late completion must not revive that owner.

**Verified checkpoint in Chrome.**

![Actual browser result: Prove cancelled work releases its source owners.](../screenshots/journey/32gfa-ownership-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Change Request to store an `Rc<Origin>` in a scratch copy and predict which Weak assertion would fail after cancellation.

</details>

## Explain the change

Can a held reference to the job keep the source alive after cancellation?

<details>
<summary>Compare your explanation</summary>

It can keep the empty job alive, but not its removed keys. Weak observers of the Origin and URL must expire even while an abandoned caller still holds the job and its request strings.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gfa-ownership
npm --prefix ../session_tests run course -- save 32gfa-ownership
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Ownership proofs matter even when the browser ignores an abort temporarily. Cancelled asynchronous work must have neither authority to commit nor a retained kernel/URL owner.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The browser checks existing command-only unloading and retained drawing. The new infrastructure is compiled here; the Reload Sources command is connected in the following command lesson.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gfa-ownership
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
