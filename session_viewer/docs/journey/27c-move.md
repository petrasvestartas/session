# 27c · Move a placed object with a typed offset

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 21–42 minutes.** 44 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Translate selected geometry through the existing command and history route.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Move x,y,z → finite offset → Editor → history transaction → new placement → GPU upload.

**Before you finish, explain:** Why is a world translation multiplied on the left of the existing placement?

The complete path is ready for a real command: local vertices, world queries and a GPU model matrix. Add Move x,y,z without adding a second editing owner.

The offset parser accepts exactly three comma-separated finite numbers. Spaces around those numbers are harmless. A missing coordinate, extra coordinate, NaN or infinity is an error. Result separates a valid offset from a useful explanation.

![One typed Move becomes one document transaction and one new object placement.](../illustrations/journey-27c.svg)

Editor asks for the selected stable ID, composes a world translation with its existing model, and submits Scene::place through History::try_edit. The local mesh is still shared. A zero offset changes nothing and does not create an undo entry.

The dock recognizes Move with arguments. It records the submitted line once; result replaces that line’s preliminary answer when parsing or editing fails. The browser still redraws the dock on an error, so an explanation is visible immediately. Later tool lessons will add Move with picked base/target points and command repetition. Today Move needs its complete offset on the line.

## Type the change

Continue [Apply object placement on the GPU](27b-model.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-27c-move`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/lib.rs`

Register the small numeric parser.

Find this exact block:

```rust
pub mod editor;
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-01.rs"
```

### 2. `src/offset.rs`

Validate a complete numeric offset before it reaches the document.

Create the file and type:

```rust
--8<-- "journey/code/27c-move-02.rs"
```

### 3. `src/editor.rs`

Carry the offset as data in the existing Action enum.

Find this exact block:

```rust
    AddBox,
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-03.rs"
```

### 4. `src/editor.rs`

Move along world axes and commit one placement change through document history.

Find this exact block:

```rust
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-04.rs"
```

### 5. `src/browser.rs`

Offer the named command in the dock vocabulary.

Find this exact block:

```rust
            "Fit Selected",
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-05.rs"
```

### 6. `src/panel.rs`

Recognize Move followed by arguments; the parser validates those arguments.

Find this exact block:

```rust
            .any(|name| name.eq_ignore_ascii_case(&line))
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-06.rs"
```

### 7. `src/panel.rs`

Replace the latest command’s answer without recording the line twice.

Find this exact block:

```rust
    pub fn update(
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-07.rs"
```

### 8. `src/browser.rs`

A valid offset becomes an Action; a parse error becomes a dock answer and still redraws.

Find this exact block:

```rust
            } else {
                let action = match line.as_str() {
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-08.rs"
```

### 9. `src/browser.rs`

Display editing failures in the command dock and continue to its normal redraw.

Find this exact block:

```rust
                Err(error) => {
                    report(error);
                    return;
                }
```

Replace that block with:

```rust
--8<-- "journey/code/27c-move-09.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run Example Box, Select Next three times, View Isometric, and Move 0.6,0.2,0. Then run Fit. The box moves while the other objects stay put. Undo restores its previous placement; Redo restores the moved placement. Move with nothing selected must explain the problem rather than changing another object.

**Actual Chrome screenshot.**

Move a placed object with a typed offset. The actual command dock drives this checkpoint; the selected object is highlighted.

![Actual browser result: Move a placed object with a typed offset.](../screenshots/journey/27c-move-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Run Move 0,0,0 followed by Undo. Explain why the zero offset creates no history entry. Try Move 1,2 and Move NaN,0,0: both must report an error without moving geometry. Restore the checkpoint before saving.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The existing placement first takes a local point into world coordinates. The new translation then shifts that world point. Translation times existing placement moves along world axes; reversing their order can move along scaled or rotated local axes instead.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 27c-move
npm --prefix ../session_tests run course -- save 27c-move
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer’s Move accepts a typed offset or starts a tool that collects points. This lesson completes the direct typed-offset route using the shared placement/history boundary. Interactive Move and repeating tools will extend it in the tool chapters.

[Validation status and course release](release.md).
