# 19 · Give every action the same route

**Typing: 60–120 minutes.** [Estimate](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

Move editing decisions from the browser callback into `Editor`. It owns scene, selection, camera, background and history, with no browser handles or GPU buffers.

An `Action` enum describes requests. `Delete` carries no value; `Pan(dx, dy)` carries two numbers. Typed commands and mouse navigation translate to actions; `Editor::apply` matches and executes them.

## Type

Continue from [Read the shape through light](18-light.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Give document actions one owner, shared by every future input method.

Create the file and type:

```rust
--8<-- "journey/code/19-actions-01.rs"
```

### 2. `src/lib.rs`

Make the editor available to both the browser and native checks.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod scene;
pub mod picking;
pub mod history;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-fullscreen-1.rs"
```

### 3. `src/browser.rs`

Import the editor action types.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::background::Background;
use crate::camera::Camera;
use crate::history::History;
use crate::renderer::Renderer;
use crate::scene::Scene;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-1.rs"
```

### 4. `src/browser.rs`

Create one editor and initialize drawing from its scene.

<details>
<summary>Locate the existing block</summary>

```rust
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut scene = Scene::demo();
    let mut renderer = Renderer::new(device, queue, config.format.add_srgb_suffix(), &scene);
    renderer.resize(width, height);
    let mut panel = crate::panel::Panel::new(
        &renderer,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-2.rs"
```

### 5. `src/browser.rs`

Present the editor background and camera.

<details>
<summary>Locate the existing block</summary>

```rust
        ],
    );
    panel.update(None, &canvas)?;
    let mut history = History::default();
    let mut selected = None;
    let mut background = Background::default();
    let mut camera = Camera { aspect: width as f64 / height as f64, ..Camera::default() };
    present(
        &surface,
        &renderer,
        &background,
        &camera.uniform(),
        &mut panel,
    )?;
    let input_canvas = canvas.clone();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-3.rs"
```

### 6. `src/browser.rs`

Start converting submitted lines into actions.

<details>
<summary>Locate the existing block</summary>

```rust
            .then(|| "canvas".into())
        });
        if let Some(line) = line {
            match line.as_str() {
                "canvas" => {
                    let Some(event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                        return;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-4.rs"
```

### 7. `src/browser.rs`

Map commands to Action, apply through Editor, and synchronize according to Change.

<details>
<summary>Locate the existing block</summary>

```rust
                    if rect.width() <= 0.0 || rect.height() <= 0.0 {
                        return;
                    }
                    let screen = [
                        (2.0 * (event.client_x() as f64 - rect.left()) / rect.width() - 1.0) as f32,
                        (1.0 - 2.0 * (event.client_y() as f64 - rect.top()) / rect.height()) as f32,
                    ];
                    selected = camera
                        .ray(screen)
                        .and_then(|ray| crate::picking::pick(&scene, &ray));
                    renderer.set_scene(&scene, selected);
                }
                "example box" => {
                    if let Err(error) =
                        history.try_edit(&mut scene, |scene| scene.add_box().map(|_| ()))
                    {
                        report(error);
                        return;
                    }
                    renderer.set_scene(&scene, selected);
                }
                "example triangle" => {
                    history.edit(&mut scene, Scene::toggle_extra);
                    selected = selected.filter(|id| scene.contains(*id));
                    renderer.set_scene(&scene, selected);
                }
                "select next" => {
                    selected = scene.next(selected);
                    renderer.set_scene(&scene, selected);
                }
                "delete" => {
                    if let Some(id) = selected.take() {
                        history.edit(&mut scene, |scene| {
                            scene.remove(id);
                        });
                        renderer.set_scene(&scene, selected);
                    }
                }
                "undo" | "redo" => {
                    if line == "undo" {
                        history.undo(&mut scene);
                    } else {
                        history.redo(&mut scene);
                    }
                    selected = selected.filter(|id| scene.contains(*id));
                    renderer.set_scene(&scene, selected);
                }
                "background" => background.toggle(),
                "zoom in" => camera.zoom(2.0),
                "zoom out" => camera.zoom(0.5),
                "pan left" => camera.pan(-0.25, 0.0),
                "pan right" => camera.pan(0.25, 0.0),
                "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                "orbit up" => camera.orbit(0.0, std::f64::consts::FRAC_PI_6),
                "view isometric" => camera.isometric(),
                "view reset" => camera = Camera { aspect: camera.aspect, ..Camera::default() },
                _ => return,
            }
        }
        if let Err(error) = panel.update(None, &input_canvas) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-5.rs"
```

### 8. `src/browser.rs`

Redraw from the editor state.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Err(error) = present(
            &surface,
            &renderer,
            &background,
            &camera.uniform(),
            &mut panel,
        ) {
            report(&format!("Cannot redraw: {error:?}"));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-6.rs"
```

### 9. `src/browser.rs`

Report the shared action route.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Surface direction changes brightness, not geometry.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/19-actions-window-7.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Select a face, type `Delete`, then `Undo`. The editor handles both actions and restores the object without resetting the camera.

**Verified checkpoint in Chrome.**

![Actual browser result: Give every action the same route.](../screenshots/journey/19-actions-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Return a `Change` enum: a view change needs a redraw; a scene change also needs an upload. After a scene action, clear selection if its ID no longer exists.

Keep this path: browser translates → editor decides → history remembers → renderer draws. Moving these rules keeps the behavior while making it testable without a browser.

HTML event → Action → Editor → Change → GPU upload when needed → draw.

![Typed commands and canvas picking become Action; Editor chooses a redraw or scene upload.](../illustrations/journey-19.svg)

Where should Delete be handled so commands share one document history?

The command handler produces Action::Delete and calls Editor::apply. Editor owns selection, the document and history, so deletion stays one transaction and repairs selection consistently.

Study estimate, including typing and experiments: 3–5 hours.

</details>

<details>
<summary>Optional experiment</summary>

Add a box, then change the background and zoom. Undo once. Predict which of those three changes disappears. Only the box addition is a document edit; the other two belong to the view. Find the early return in apply that keeps those actions out of history.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19-actions
npm --prefix ../session_tests run course -- save 19-actions
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The full command system will build on this boundary. commands, keyboard input and tools request document operations through the same owner, while rendering consumes the result. More commands should add behavior here or in focused command modules, not duplicate transactions in UI handlers.

The box is yellow after Select Next reaches it. commands feed the shared Editor action path.

[Full validation scope](release.md).

</details>
