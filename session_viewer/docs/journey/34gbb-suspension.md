# 4.2.19 · Keep each heartbeat pause reason separate

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 10–19 minutes.** [Estimate](typing-load.md).

Give heartbeat scheduling three independent reasons to pause: hidden, frozen and cached. Clearing one reason must leave the others intact. Final closure permanently prevents resumption.

## Type

Continue from [Own diagnostic page transitions](34gba-lifecycle.md). [Save or recover your work](recovery.md).

### 1. `src/suspension.rs`

Keep hidden, frozen and cached reasons independent; final closure permanently refuses scheduling.

Create the file and type:

```rust
--8<-- "journey/code/34gbb-suspension-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/suspension_tests.rs`

Optional authoring fixture. Check overlapping reasons, initial hidden state, duplicate delivery and final closure that cannot resume.

Copy this supplied file:

```text
--8<-- "journey/code/34gbb-suspension-02.rs"
```

</details>

### 3. `src/lib.rs`

Register the pure policy and its native checks.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod diagnostic;
```

</details>

After that block, type:

```rust
pub mod suspension;
#[cfg(test)]
mod suspension_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gbb-suspension-03.rs"
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

Inspect the supplied state fixtures. Every remaining pause reason prevents resumption, and clearing all reasons permits it. A closed policy stays closed.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for keep each heartbeat pause reason separate.](../screenshots/journey/34gba-lifecycle.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [4.2.18 · Own diagnostic page transitions](34gba-lifecycle.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The policy is ordinary Rust, without browser or GPU ownership. Native checks exercise overlapping reasons and different release orders. Browser event delivery is connected in the next step.

hidden + frozen + cached + final closure → one scheduling decision.

![Each pause reason must clear before heartbeat scheduling resumes.](../illustrations/journey-34gbb.svg)

Why does becoming visible not always allow the heartbeat to resume?

Visibility clears only the hidden reason. A freeze or cached-page reason can still prohibit scheduling; final closure never resumes.

Study estimate, including typing and experiments: 0.75–1.25 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change the first test to clear Frozen last. Predict which earlier clear remains paused, then run the checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.19
npm --prefix ../session_tests run course -- save 4.2.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This prepares independent browser suspension reasons. The following step connects them to the real diagnostic timer; drawing and document ownership remain separate.

Native policy checks cover overlapping reasons in three release orders, initially hidden startup, duplicate events and final closure. Chrome independently retains the preceding page-transition route until this policy is connected.

This preparation step retains the preceding browser drawing and lifecycle behavior. Its new scheduling policy is tested natively.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.19
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
