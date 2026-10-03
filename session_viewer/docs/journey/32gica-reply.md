# 32gica · Pair captured intent with complete source bodies

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 30–59 minutes.** 59 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Build a completion value that retains captured intent and rejects incomplete source-body pairings.

**Follow:** current ticket → keys + Intent → equal body count → complete pairs or error.

A completion must keep its captured operation beside every returned source body, or beside its failure. Reply stores an optional Intent and one Result. None describes explicit Reload Sources; Some describes captured Move, Delete or Save.

Before pairing bodies and keys, the constructor checks equal lengths. zip stops when either iterator ends; without this check a missing or extra body could silently become a partial restoration. A failure retains the operation and creates no source-body pairs.

This checkpoint does not change the browser flight or connect automatic commands. The next lesson gives the browser this completion value.

![Pair intent with complete bodies](../illustrations/journey-32gica.svg)

## Type the change

Continue from [Own the intent with its pending source ticket](32gic-owner.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gica-reply` (from `session_viewer`).

### 1. `src/reload_reply.rs`

Keep the captured operation beside either its complete source bodies or its failure.

Create the file and type:

```rust
--8<-- "journey/code/32gica-reply-01.rs"
```

### 2. `src/lib.rs`

Register the completion value for native checks and browser delivery.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod edit_replay;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gica-reply-02.rs"
```

### 3. `src/reload_reply_tests.rs`

Check all captured operations, exact bodies, failures, partial counts and the explicit no-operation path.

Create the file and type:

```rust
--8<-- "journey/code/32gica-reply-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the completed-body checks below. The reply must contain every requested body paired with its key and retain the captured intent until adoption.

**Verified checkpoint in Chrome.**

![Actual browser result: Pair captured intent with complete source bodies.](../screenshots/journey/32gica-reply-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Remove the body-count check and run the missing/extra body tests. Explain why zip alone cannot establish a complete response.

</details>

## Explain the change

Why must body-count validation happen before pairing bodies with source keys?

<details>
<summary>Compare your explanation</summary>

zip stops at the shorter vector. Without checking equal lengths first, a partial response could look like a complete restoration request. Failure keeps the operation but exposes no partial body pairs.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gica-reply
npm --prefix ../session_tests run course -- save 32gica-reply
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Request ownership, complete source bodies and captured intent reach one completion boundary. Document-context validation still precedes replay; automatic commands follow next.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Native tests carry all three operations through a current ticket, preserve exact file bytes and hydrate without replaying the operation. They also check network errors, missing bodies, extra bodies and the explicit None path. Hydration must preserve selection, camera and placement.

Native checks exercise captured Move/Delete/Save, exact source bodies and failure counts. Chrome retains the existing explicit reload/editor; browser delivery integration follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gica-reply
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
