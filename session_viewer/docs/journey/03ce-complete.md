# 1.3.12 · Complete a command name

**Chapter 1 · Browser, Rust and the GPU · Section 1.3 · Shaders and the command dock** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

After the field changes, find a matching command name. Replace the text and select only the suggested suffix, so the next letter replaces that suffix.

## Type

Continue from [Recognize Help](03cd-vocabulary.md). [Save or recover your work](recovery.md).

### 1. `src/browser.rs`

Report that inline completion is connected.

<details>
<summary>Locate the existing block</summary>

```rust
    redraw.forget();
    report("Help recognizes the command vocabulary.");
    Ok(())
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ce-complete-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Match names after changes and select only the suggested suffix.

<details>
<summary>Locate the existing block</summary>

```rust
        self.history.push_back(line);
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/03ce-complete-direct-02.rs" skip=47
```

### 3. `src/panel.rs`

Keep the field response and refresh completion after editing.

<details>
<summary>Locate the existing block</summary>

```rust
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ce-complete-direct-03.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type He. The field shows Help with the suggested ending selected. Backspace leaves He.

### Recorded visual result

**Previous native render — current verification pending.**

This checkpoint was rendered independently and has the same native comparison frame as [Recognize Help](03cd-vocabulary.md). Use its state checks and browser exercise to assess the changed behavior.

![Native renderer output for complete a command name.](../screenshots/journey/03-triangle.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [1.3.1 · Give the GPU three corners](03-triangle.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Select only the suggested suffix while retaining typed text.

![Complete a command name](../illustrations/journey-direct-03ce-complete.svg)

Why count characters instead of bytes?

egui caret positions use characters; UTF-8 characters can occupy several bytes.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Type He, then x. The selected lp should be replaced by x, leaving Hex with no matching completion.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 1.3.12
npm --prefix ../session_tests run course -- save 1.3.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This step connects one responsibility of the production command dock. Scene commands are added in the following lessons.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 1.3.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
