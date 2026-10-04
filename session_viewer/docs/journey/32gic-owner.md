# 32gic · Own the intent with its pending source ticket

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Generalize ReloadJob with a payload type P. Pending stores that payload beside its ticket and keys; begin_with installs them together, finish_with returns them together once, and cancel drops them together. The request sent to a future still owns only URL strings and a ticket.

P is a Rust type parameter: ReloadJob<Intent> owns a captured modeling request, while ReloadJob<()> carries the unit value, which means no additional data. The default type keeps existing explicit Reload Sources callers unchanged. A manual Default implementation starts empty without requiring Intent to implement Default.

## Type

Continue from [Route captured Move, Delete and Save results](32gibb-reply.md). [Save or recover your work](recovery.md).

### 1. `src/reload_job.rs`

Keep the intent and source keys inside the same ticket-owned value.

<details>
<summary>Locate the existing block</summary>

```rust
struct Pending {
    ticket: u64,
    keys: Vec<ReloadKey>,
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-01.rs"
```

### 2. `src/reload_job.rs`

Allow any payload without requiring that payload to have a default value.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Default)]
pub struct ReloadJob {
    issued: u64,
    pending: Option<Pending>,
}

impl ReloadJob {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-02.rs"
```

### 3. `src/reload_job.rs`

Capture the requested operation when beginning its scoped source request.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-03.rs"
```

### 4. `src/reload_job.rs`

Install keys and payload together after request validation succeeds.

<details>
<summary>Locate the existing block</summary>

```rust
        self.pending = Some(Pending { ticket, keys });
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-04.rs"
```

### 5. `src/reload_job.rs`

Take the current payload once; stale replies leave newer work untouched.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        if self.pending() != Some(ticket) { return None; }
        self.pending.take().map(|pending| pending.keys)
    }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-05.rs"
```

### 6. `src/reload_job.rs`

Keep explicit Reload Sources using a unit payload and its existing interface.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-06.rs"
```

### 7. `src/lib.rs`

Register native ownership checks without exposing test code to the browser.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod edit_replay;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gic-owner-07.rs"
```

### 8. `src/edit_owner_tests.rs`

Prove stale/duplicate/cancelled replies cannot consume an intent or retain its source metadata.

Create the file and type:

```rust
--8<-- "journey/code/32gic-owner-08.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the owner checks below. A pending batch keeps its keys and captured intent together; cancellation and one-shot delivery must release that ownership.

**Verified checkpoint in Chrome.**

![Actual browser result: Own the intent with its pending source ticket.](../screenshots/journey/32gic-owner-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

The normal begin/finish methods are thin wrappers for the unit specialization. Existing cancellation, ticket exhaustion, duplicate-origin and browser fetch checks still run. New native checks prove an old reply cannot consume the newer Save, the current reply returns its exact intent once, and cancellation drops pending metadata without retaining a kernel document.

A matching ticket establishes current request ownership, not document validity. The next browser lesson must still hydrate with the captured keys and reject changed origin/epoch context before replay. Success and failure both consume only their own current ticket; a stale failure must never abort newer work.

ReloadJob<Intent> → Pending { ticket, keys, payload } → current finish_with takes both once.

![Own the intent and ticket together](../illustrations/journey-32gic.svg)

Why does a stale completion check the current ticket before taking the pending value?

Taking first would destroy a newer request. Checking first preserves newer keys and intent; only the matching current completion can consume the pair. Cancellation drops the whole pending owner.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Move pending.take before the ticket comparison in finish_with. Run the stale completion test and explain which newer request was lost.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32gic-owner
npm --prefix ../session_tests run course -- save 32gic-owner
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Request identity and its payload share one owner. Completion ownership is checked before document context; hydration and replay follow only after both checks.

Chrome checks the unchanged explicit reload/download/editor paths; this lesson does not yet connect an intent-bearing browser completion.

Chrome preserves the existing explicit reload, download and modeling paths. Native tests exercise intent-bearing ticket ownership, stale/duplicate replies and cancellation. Automatic browser replay follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gic-owner
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
