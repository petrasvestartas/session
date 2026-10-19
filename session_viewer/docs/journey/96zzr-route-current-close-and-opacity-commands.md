# 20.6.3 · Route current Close and Opacity commands

**Chapter 20 · Shared GPU rendering · Section 20.6 · Current file and command behavior** · [Contents](contents.md)

**Typing: 5–10 minutes.** [Estimate](typing-load.md).

Route Close through the existing curve builder and one history transaction, protecting other tools and construction-plane picks. Read the first Opacity value as a finite double before narrowing for the shader. Later words follow the current parser convention.

## Type

Continue from [Open files through one measured Session reader](96zzq-open-files-through-one-measured-session-reader.md). [Save or recover your work](recovery.md).

### 1. `src/commands.rs`

Route the public command through the existing curve-closing action.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Close", || Action::Close),
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    ("Close", || Action::CloseTool),
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-1.rs"
```

</details>

### 2. `src/editor.rs`

Protect other tools and construction-plane picks before building a closed curve.

<details>
<summary>Locate the existing block</summary>

```rust
                if draft.gather.is_some()||draft.project.is_some()||draft.extend.is_some(){return Err("Close works on a polyline or curve")}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
                if self.plane_only || draft.plane_points.is_some() || !matches!(draft.tool.name(),"Polyline"|"Curve"){return Err("Close works on a polyline or curve")}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-2.rs"
```

</details>

### 3. `src/commands.rs`

Validate the first argument in the same precision as the current command parser.

<details>
<summary>Locate the existing block</summary>

```rust
    if name=="Opacity"{let [word]=&words[count..]else{return Err("Opacity 0..1 · Example: Opacity 0.5".into())};let value=word.parse::<f32>().ok().filter(|v|v.is_finite()&&(0.0..=1.0).contains(v)).ok_or("Opacity takes a value from 0 to 1")?;return Ok(Command::Editor(Action::Opacity(value)));}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    if name=="Opacity"{let word=words.get(count).ok_or("Opacity 0..1 · Example: Opacity 0.5")?;let value=word.parse::<f64>().ok().filter(|v|v.is_finite()&&(0.0..=1.0).contains(v)).ok_or("Opacity takes a value from 0 to 1")?;return Ok(Command::Editor(Action::Opacity(value as f32)));}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/opacity_command_tests.rs`

Update the original invalid-input fixture for the required current parser behavior.

<details>
<summary>Locate the existing block</summary>

```text
"Opacity NaN","Opacity 0.5 extra"
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-4.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/current_command_tests.rs`

Supply original-document, one-undo, retained-draft and precision-boundary fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-5.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/lib.rs`

Register the optional command fixtures.

<details>
<summary>Locate the existing block</summary>

```text
&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

mod draw_slots;
#[cfg(test)] mod draw_slots_tests;

#[cfg(test)] mod source_phase_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-6.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/inventory_tests.rs`

Update the comparison fixture to require the now-matched first-argument convention and its exact value.

<details>
<summary>Locate the existing block</summary>

```text
assert!(crate::commands::parse("Opacity 0.4 extra").is_err());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzr-route-current-close-and-opacity-commands-7.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.6.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for route current close and opacity commands.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



command words → existing editor action → draft or display state.



Why keep document release separate from Close?

Close finishes an unfinished curve. A document lifetime action owns release and history cleanup; a drawing command must not erase existing geometry.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

With existing geometry visible, run Close without a draft and confirm it remains. Begin Polyline, enter three points separately, run Close, and Undo once. Try Opacity 0.5 extra, then Opacity 1.000000001 extra; only the first is valid.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.6.3
npm --prefix ../session_tests run course -- save 20.6.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Document release remains the existing internal action and empty-source operation. Close now uses the drawing command convention; the rest of the command registry still needs comparison.

Actual Chrome command input at DPR 1/2: Close preserves the open document, closes one polyline with one Undo/Redo, and Opacity validates its first argument while retaining unfinished points.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.6.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
