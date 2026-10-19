# 4.2.21 · Give pending startup a revocable ticket

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 5–10 minutes.** [Estimate](typing-load.md).

Give pending startup a shared permission flag. The task and close callback hold cloned tickets; revoking either ticket refuses every later result.

## Type

Continue from [Connect visibility and freezing to heartbeat scheduling](34gbba-browser.md). [Save or recover your work](recovery.md).

### 1. `src/startup.rs`

Share startup permission between the waiting task and its close callback.

Create the file and type:

```rust
--8<-- "journey/code/34gbc-authority-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/startup_tests.rs`

Optional authoring fixture. Check delayed tickets, repeated closure and an independent replacement owner.

Copy this supplied file:

```text
--8<-- "journey/code/34gbc-authority-02.rs"
```

</details>

### 3. `src/lib.rs`

Register startup authority and its native checks.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod suspension;
```

</details>

After that block, type:

```rust
pub mod startup;
#[cfg(test)]
mod startup_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gbc-authority-03.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Inspect the supplied state fixtures. Revocation reaches every waiting ticket, repeated closure stays revoked, and an independent replacement remains allowed.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for give pending startup a revocable ticket.](../screenshots/journey/34gba-lifecycle.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [4.2.18 · Own diagnostic page transitions](34gba-lifecycle.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A replacement gets a separate allocation. This step checks the policy natively; the next step connects it to actual GPU requests.

task ticket + close ticket → shared flag → delayed result refused after closure.

![The task and close callback share one revocable permission flag.](../illustrations/journey-34gbc.svg)

Why clone the ticket instead of copying its boolean value?

A copied boolean would keep its old value. Rc shares the Cell, so the waiting task sees the close callback’s revocation.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Revoke the device ticket instead of the owner in the first test. Predict whether the adapter ticket still permits startup, then run the checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.21
npm --prefix ../session_tests run course -- save 4.2.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This prepares shared startup authority. The next checkpoint checks it after actual adapter and device awaits, independently of optional diagnostic scheduling.

Native tests cover shared revocation, repeated closure and independent allocations. Chrome retains the previous route until startup uses this authority.

The preceding browser behavior remains unchanged; the new shared startup authority is checked natively.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.21
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
