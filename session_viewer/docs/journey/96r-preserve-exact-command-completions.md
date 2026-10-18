# 19.2.10 · Prove exact commands across the expanded vocabulary

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 2–3 minutes.** [Estimate](typing-load.md).

The first vocabulary lesson already preserves exact words.

## Type

Continue from [Prove imported feature ownership and round trips](96q-prove-imported-feature-ownership-and-roundtrips.md). [Save or recover your work](recovery.md).

### 1. `src/command_vocabulary.rs`

Move the already-typed private Commands wrapper from panel.rs into this shared module; retain its existing methods.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
.sort_by_key(|name| {
        if name.eq_ignore_ascii_case(line) { 0 }
        else if name.get(line.len()..).is_some_and(|rest| rest.starts_with(' ')) { 1 }
        else { 2 }
    });
    choices
}
```

</details>

Move the existing code after that block:

```text
--8<-- "journey/code/96r-preserve-exact-command-completions-move-existing-wrapper.rs" skip=200
```

</details>

### 2. `src/command_vocabulary.rs`

Import the existing dock trait and expose the shared wrapper and its input list to the panel.

<details>
<summary>Locate the existing block</summary>

```rust
struct Commands(Vec<&'static str>);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::command_dock;

pub(crate) struct Commands(pub(crate) Vec<&'static str>);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96r-preserve-exact-command-completions-wire-moved-wrapper.rs"
```

</details>

### 3. `src/panel.rs`

Use the exact-first vocabulary in the actual browser panel.

<details>
<summary>Locate the existing block</summary>

```rust
struct Commands(Vec<&'static str>);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::commands::canonical(line)
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96r-preserve-exact-command-completions-02.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/command_vocabulary.rs`

Feed character-by-character input and Enter through the actual egui command field.

<details>
<summary>Locate the existing block</summary>

```text
        self.completions(line)
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96r-preserve-exact-command-completions-04.rs" skip=40
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

Type View Isometric and press Enter. The history must contain that exact command. Then type View Isometric Back and verify that the longer command remains available.

### Recorded visual result

**Previous native render — current verification pending.**

The original comparison frame is unchanged; actual command-field behavior is checked separately.

![Native renderer output for prove exact commands across the expanded vocabulary.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Move the current Commands wrapper into the shared module unchanged, so the native widget fixture and the browser use the same dictionary. Inspect the supplied character-by-character and Enter regression for the expanded CAD command list. No second completion algorithm is introduced.

typed characters → command vocabulary → exact match first → egui submission.



Does a larger command registry justify changing a fully typed command?

No. Reuse the exact-word ranking written at03cd, and verify that both inline typing and Enter preserve the expanded command vocabulary.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try Line beside its longer command names, and View Isometric beside View Isometric Back. Check both the fully typed field and the submitted history entry. Check Select Object beside Selection: a space after Select must continue the intended command.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.10
npm --prefix ../session_tests run course -- save 19.2.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The shared wrapper exercises the same existing ranking and canonicalization in native widget validation and the browser panel.

Run the cumulative native tests and the exact-completion browser exercise. The browser also rechecks imported feature ownership and archives.

Imported Element features after exact View Isometric command entry. Chrome checks original ownership, visibility, child Delete/Undo and repeated archive replacement before testing command completion.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 19.2.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
