# 7.1 · Active tools own their command input

**Steps:** 5 · **Source:** [app/command/mod.rs](../../../src/app/command/mod.rs) · [app/command/tool.rs](../../../src/app/command/tool.rs) · [app/cplane.rs](../../../src/app/cplane.rs) · [app/snap.rs](../../../src/app/snap.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Ray ∩ C-Plane: t = -dot(ro - O, n) / dot(rd, n)</text>
  <polygon points="48,144 128,84 316,84 236,144" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="88" y1="114" x2="276" y2="114" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="182" y1="84" x2="102" y2="144" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="74" y1="48" x2="182" y2="114" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#arr)"/>
  <circle cx="182" cy="114" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="200" y="110" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">P_snap(u,v)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Gumball Axes &amp; Arcs (Move / Rotate / Scale)</text>
  <circle cx="560" cy="116" r="46" fill="none" stroke="#2563eb" stroke-width="2" stroke-dasharray="5 3"/>
  <line x1="560" y1="116" x2="646" y2="116" stroke="#dc2626" stroke-width="3" marker-end="url(#arrR)"/>
  <line x1="560" y1="116" x2="560" y2="44" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <line x1="560" y1="116" x2="504" y2="154" stroke="#2563eb" stroke-width="3" marker-end="url(#arrB)"/>
  <rect x="618" y="84" width="10" height="10" fill="#dc2626" stroke="#111111" stroke-width="1.2"/>
  <circle cx="560" cy="116" r="5" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Transformed Geometry: M_delta * P_local</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="842,136 882,88 932,136" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
  <polygon points="918,126 958,78 1008,126" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <line x1="882" y1="112" x2="952" y2="102" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
</svg>
</div>

## <a id="checkpoint-46-tool-panel"></a>1. Give active tools ownership of the command panel

### 1. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let Some(line) = line else {
            return Ok(None);
        };
        if line == "Escape" {
```

Replace it with:

```rust
        let Some(line) = line else {
            return Ok(None);
        };
        if !self.model.options.is_empty() {
            return Ok(Some(line));
        }
        if line == "Escape" {
```

**Locate this block:**

```rust
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
```

Replace it with:

```rust
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
            "options": self.model.options, "options_rect": self.model.options_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
```

**Locate this block:**

```rust
            modifiers: Default::default(),
        });
        let over = pos.y >= self.top
            || self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos));
        if over {
            event.prevent_default();
        }
```

Replace it with:

```rust
            modifiers: Default::default(),
        });
        let over = pos.y >= self.top
            || (self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos))
                || self
                    .model
                    .options_rect
                    .is_some_and(|r| r.contains(pos)));
        if over {
            event.prevent_default();
        }
```

**Locate this block:**

```rust
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = pos.y >= self.top
            || self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
```

Replace it with:

```rust
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = pos.y >= self.top
            || (self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos))
                || self
                    .model
                    .options_rect
                    .is_some_and(|r| r.contains(pos)));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
```

**Locate this block:**

```rust
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }
```

Replace it with:

```rust
    }

    pub fn tool(&mut self, prompt: String, options: Vec<(String, String)>) {
        self.model.drawing_prompt = prompt;
        self.model.options = options;
        if !self.model.options.is_empty() {
            self.model.completion_visible = false;
            self.model.inline_suffix = false;
        }
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }
```

## <a id="checkpoint-47-registry"></a>2. Separate browser requests from editor actions

### 1. `src/commands.rs`

Create the file and type:

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host {
    Help,
    Open,
    Replace,
    CancelOpen,
    Reload,
    CancelReload,
    Save,
    Report,
    PreviousReport,
    Selection,
}

pub enum Command {
    Editor(Action),
    Host(Host),
}

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[];

pub const HOSTS: &[(&str, Host)] = &[
    ("Help", Host::Help),
    ("Open", Host::Open),
    ("Open Replace", Host::Replace),
    ("Cancel Open", Host::CancelOpen),
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
];

pub fn names() -> Vec<&'static str> {
    HOSTS
        .iter()
        .map(|(name, _)| *name)
        .chain(ACTIONS.iter().map(|(name, _)| *name))
        .chain(["Move"])
        .collect()
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod multi_selection_tests;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod multi_selection_tests;

pub mod commands;

#[cfg(test)]
```

## <a id="checkpoint-47a-actions"></a>3. Register the existing editor command factories

### 1. `src/commands.rs`

**Locate this existing block:**

```rust
pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[];

pub const HOSTS: &[(&str, Host)] = &[
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Example Box", || Action::AddBox),
    ("Example Line", || Action::AddLine),
    ("Example Point", || Action::AddPoint),
    ("Example Curve", || Action::AddCurve),
    ("Example Object Color", || {
        Action::AddColours(crate::colour::Mode::Object)
    }),
    ("Example Point Colors", || {
        Action::AddColours(crate::colour::Mode::Points)
    }),
    ("Example Face Colors", || {
        Action::AddColours(crate::colour::Mode::Faces)
    }),
    ("View Normals On", || {
        Action::NormalView(true)
    }),
    ("View Normals Off", || {
        Action::NormalView(false)
    }),
    ("View Grid On", || {
        Action::Grid(Some(crate::grid::Settings::default()))
    }),
    ("View Grid Off", || Action::Grid(None)),
    ("Example Sharp", || {
        Action::AddNormals(crate::normal_example::Mode::Sharp)
    }),
    ("Example Smooth", || {
        Action::AddNormals(crate::normal_example::Mode::Smooth)
    }),
    ("Example Winding", || {
        Action::AddNormals(crate::normal_example::Mode::Winding)
    }),
    ("Controls On", || Action::Controls(true)),
    ("Controls Off", || Action::Controls(false)),
    ("Example Polyline", || Action::AddPolyline),
    ("Example Arrow", || Action::AddArrow),
    ("Example Triangle", || Action::ToggleExtra),
    ("Select Next", || Action::SelectNext),
    ("Select All", || Action::SelectAll),
    ("Select None", || Action::Picked(None)),
    ("Select Object", || {
        Action::PickMode(crate::pick_sources::Mode::Object)
    }),
    ("Select Face", || {
        Action::PickMode(crate::pick_sources::Mode::Face)
    }),
    ("Select Edge", || {
        Action::PickMode(crate::pick_sources::Mode::Edge)
    }),
    ("Select Control", || {
        Action::PickMode(crate::pick_sources::Mode::Control)
    }),
    ("Delete", || Action::Delete),
    ("Undo", || Action::Undo),
    ("Redo", || Action::Redo),
    ("Background", || Action::Background),
    ("Zoom In", || Action::Zoom(2.0)),
    ("Zoom Out", || Action::Zoom(0.5)),
    ("Pan Left", || Action::Pan(-0.25, 0.0)),
    ("Pan Right", || Action::Pan(0.25, 0.0)),
    ("Orbit Right", || {
        Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)
    }),
    ("Orbit Up", || {
        Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)
    }),
    ("View Isometric", || Action::Isometric),
    ("Fit", || Action::Fit),
    ("Fit Selected", || Action::FitSelected),
    ("View Reset", || Action::ResetView),
    ("View Perspective", || {
        Action::Projection(crate::camera::Projection::Perspective)
    }),
    ("View Orthographic", || {
        Action::Projection(crate::camera::Projection::Orthographic)
    }),
    ("Close", || Action::Close),
    ("Unload Sources", || Action::UnloadSources),
];

pub const HOSTS: &[(&str, Host)] = &[
```

## <a id="checkpoint-47b-parse"></a>4. Parse the longest command and validate its arguments

### 1. `src/commands.rs`

**Locate this existing block:**

```rust
        .collect()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "Move" {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
            .map(|offset| Command::Editor(Action::Translate(offset)))
            .map_err(str::to_owned);
    }
    if words.len() != count {
        return Err(format!("{name} takes no arguments"));
    }
    if let Some((_, make)) = ACTIONS.iter().find(|(key, _)| *key == name) {
        return Ok(Command::Editor(make()));
    }
    let (_, host) = HOSTS
        .iter()
        .find(|(key, _)| *key == name)
        .ok_or("Command registry is incomplete")?;
    Ok(Command::Host(*host))
}
```

### 2. `src/offset.rs`

**Locate this existing block:**

```rust
    let rest = words.collect::<Vec<_>>().join(" ");
    let parts: Vec<_> = rest.split(',').collect();
    if parts.len() != 3 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let rest = words.collect::<Vec<_>>().join(" ");
    let parts: Vec<_> = if rest.contains(',') {
        rest.split(',').collect()
    } else {
        rest.split_whitespace().collect()
    };
    if parts.len() != 3 {
```

### 3. `src/commands_tests.rs`

Create the file and type:

```rust
use crate::{
    commands::{self, Command, Host},
    editor::{Action, Editor},
};

#[test]
fn longest_names_and_compact_spelling_keep_host_intent() {
    for text in [
        "Open Replace",
        "openreplace",
        " OPEN  replace ",
    ] {
        assert!(matches!(
            commands::parse(text).unwrap(),
            Command::Host(Host::Replace)
        ));
        assert_eq!(commands::canonical(text), "Open Replace");
    }
    assert!(matches!(
        commands::parse("Report Previous").unwrap(),
        Command::Host(Host::PreviousReport)
    ));
    assert!(matches!(
        commands::parse("SelectAll").unwrap(),
        Command::Editor(Action::SelectAll)
    ));
    assert!(matches!(
        commands::parse("View Normals Off").unwrap(),
        Command::Editor(Action::NormalView(false))
    ));
    assert_eq!(
        commands::canonical("mOvE 1E-12, 2, 3"),
        "Move 1E-12, 2, 3"
    );
}

#[test]
fn invalid_commands_leave_original_rows_and_redo_untouched() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    editor
        .apply(Action::Translate([1., 0., 0.]))
        .unwrap();
    editor.apply(Action::Undo).unwrap();
    let source = editor.scene.objects()[0].mesh.clone();
    let placement = editor.scene.objects()[0].model.m;
    for text in [
        "",
        "Unknown",
        "Delete all",
        "Open Replace extra",
        "Fit Selected extra",
        "Move",
        "Move 1 2",
        "Move 1,,2",
        "Move NaN,0,0",
        "Move 1,2,inf",
        "Move 1,2,3,4",
        "Move 1 2,3,4",
    ] {
        assert!(commands::parse(text).is_err(), "{text}");
        assert_eq!(editor.scene.objects()[0].model.m, placement);
        assert!(std::rc::Rc::ptr_eq(
            &source,
            &editor.scene.objects()[0].mesh
        ));
    }
    editor.apply(Action::Redo).unwrap();
    assert_eq!(
        editor.scene.objects()[0].model.m[12],
        placement[12] + 1.
    );
}

#[test]
fn typed_offsets_execute_once_and_one_undo_restores_them() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    let before = editor.scene.objects()[0].model.m;
    for text in [
        "Move 1 -2 3e-2",
        "move 1, -2, 0.03",
        "MOVE 1 -2 .03",
    ] {
        let Command::Editor(action) = commands::parse(text).unwrap() else {
            panic!("Move must reach the editor");
        };
        editor.apply(action).unwrap();
        assert_eq!(
            editor.scene.objects()[0].model.m[12],
            before[12] + 1.
        );
        assert_eq!(
            editor.scene.objects()[0].model.m[13],
            before[13] - 2.
        );
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects()[0].model.m, before);
    }
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod selection_cycle_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod commands_tests;

#[cfg(test)]
mod selection_cycle_tests;
```

## <a id="checkpoint-47c-route"></a>5. Route the command field through the typed registry

### 1. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        if !self.commands.0.iter().any(|name| {
            name.eq_ignore_ascii_case(&line)
                || (*name == "Move"
                    && line
                        .to_ascii_lowercase()
                        .starts_with("move "))
        }) {
            self.answer(&line, "Unknown command. Type Help.");
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line.to_ascii_lowercase()))
    }
```

Replace it with:

```rust
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line))
    }
```

**Locate this block:**

```rust
    pub fn new(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        commands: &'static [&'static str],
    ) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
```

Replace it with:

```rust
    pub fn new(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        commands: Vec<&'static str>,
    ) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
```

**Locate this block:**

```rust
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
```

Replace it with:

```rust
use crate::renderer::Renderer;

struct Commands(Vec<&'static str>);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::commands::canonical(line)
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Report",
            "Report Previous",
            "Example Box",
            "Example Line",
            "Example Point",
            "Example Curve",
            "Example Object Color",
            "Example Point Colors",
            "Example Face Colors",
            "View Normals On",
            "View Normals Off",
            "View Grid On",
            "View Grid Off",
            "Example Sharp",
            "Example Smooth",
            "Example Winding",
            "Controls On",
            "Controls Off",
            "Example Polyline",
            "Example Arrow",
            "Example Triangle",
            "Select Next",
            "Select All",
            "Select None",
            "Select Object",
            "Select Face",
            "Select Edge",
            "Select Control",
            "Selection",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    if let Some(message) = crate::browser_report::previous_notice() {
        panel.result(message);
    }
    if let Some(message) = &recovery {
        panel.result(message);
    }
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer
            .queue
            .on_submitted_work_done(move || {
                if crate::browser_runtime::owns(&completing)
                    && completing.message().is_none()
                {
                    crate::browser_phase::finish(
                        "first frame complete",
                        frame_started,
                        0,
                        "GPU",
                    );
                    let _ = crate::browser_report::observe(
                        "milestone",
                        "geometry on screen",
                    );
                }
            });
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared =
        std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if active_fault.message().is_some() {
                return;
            }
            let mut editor = shared.borrow_mut();
            if matches!(
                event.type_().as_str(),
                "blur" | "pagehide" | "viewer-reload"
            ) {
                renderer.cancel_pick();
            }
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
                    .dyn_ref::<web_sys::PageTransitionEvent>()
                    .is_some_and(|event| event.persisted())
                {
                    return;
                }
                wasm_bindgen_futures::spawn_local(async {
                    crate::browser_runtime::stop();
                });
                return;
            }
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
            let line = if let Some(shortcut) = shortcut {
                event.prevent_default();
                panel.answer(shortcut.command(), "");
                Some(shortcut.command().to_ascii_lowercase())
            } else {
                match panel.update(Some(&event), &input_canvas) {
                    Ok(line) => line,
                    Err(error) => {
                        report(&format!("Cannot read command: {error:?}"));
                        return;
                    }
                }
            };

            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
                    .target()
                    .and_then(|target| {
                        target
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .ok()
                    })
                    .is_some_and(|input| input.id() == "open")
            {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.answer("Open", "Open cancelled.");
                None
            } else if event.type_() == "change" {
                crate::file_input::choose(
                    &event,
                    std::rc::Rc::clone(&request),
                    read_mode.get(),
                    std::rc::Rc::clone(&delivery),
                );
                None
            } else if event.type_() == "viewer-file-error" {
                if let Some(message) = event
                    .dyn_ref::<web_sys::CustomEvent>()
                    .and_then(|event| event.detail().as_string())
                {
                    report(&message);
                    panel.answer("Open", &message);
                }
                None
            } else if event.type_() == "viewer-reload" {
                if let Some(reply) = reload_delivery.borrow_mut().take() {
                    let command = reply
                        .intent
                        .map_or("Reload Sources", |intent| intent.command());
                    match reply.complete(&mut editor) {
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            highlight_changed = true;
                            crate::browser_upload::prepare(
                                &mut renderer,
                                &editor.scene,
                                editor.whole_selection(),
                                active_fault.clone(),
                            );
                            crate::browser_phase::revision(&editor.scene);
                            let message = if command == "Reload Sources" {
                                "Editable sources restored; display retained."
                                    .to_owned()
                            } else {
                                format!("{command} completed after source reload.")
                            };
                            report(&message);
                            panel.answer(command, &message);
                        }
                        Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                            let message =
                                save_result(&bytes).unwrap_or_else(|error| error);
                            report(&message);
                            panel.answer(command, &message);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            report(&error);
                            panel.answer(command, &error);
                        }
                    }
                }
                None
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery) {
                    Ok(action) => action,
                    Err(error) => {
                        let message = format!("Cannot retain reload file: {error:?}");
                        report(&message);
                        panel.answer("Open", &message);
                        None
                    }
                }
            } else if let Some(line) = line {
                if line == "selection" {
                    let rows = editor.selected_objects();
                    panel.result(&if rows.len() > 1 {
                        format!(
                            "Objects {:?}",
                            rows.iter()
                                .map(|id| id.value())
                                .collect::<Vec<_>>()
                        )
                    } else {
                        editor.selection().map_or_else(
                            || "Nothing selected".to_owned(),
                            |target| target.describe(),
                        )
                    });
                    None
                } else if line == "reload sources" {
                    let result = crate::browser_reload::start(
                        std::rc::Rc::clone(&reload),
                        editor.reload_keys(),
                        std::rc::Rc::clone(&reload_delivery),
                    );
                    match result {
                        Ok(()) => panel.result("Reloading editable sources…"),
                        Err(error) => {
                            report(&error);
                            panel.result(&error);
                        }
                    }
                    None
                } else if line == "cancel reload" {
                    reload.borrow_mut().cancel();
                    report("Reload cancelled.");
                    panel.result("Reload cancelled.");
                    None
                } else if line == "unload sources" {
                    Some(Action::UnloadSources)
                } else if line == "close" {
                    request.borrow_mut().cancel();
                    Some(Action::Close)
                } else if line == "cancel open" {
                    request.borrow_mut().cancel();
                    report("Open cancelled.");
                    panel.result("Open cancelled.");
                    None
                } else if line == "open" || line == "open replace" {
                    let ticket = request.borrow_mut().begin();
                    if let Err(error) = ticket {
                        report(error);
                        panel.result(error);
                        return;
                    }
                    if line == "open replace" {
                        reload.borrow_mut().cancel();
                    }
                    read_mode.set(if line == "open replace" {
                        crate::file_input::Mode::Replace
                    } else {
                        crate::file_input::Mode::Append
                    });
                    if let Some(input) = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| document.get_element_by_id("open"))
                        .and_then(|element| {
                            element
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .ok()
                        })
                    {
                        input.click();
                    }
                    None
                } else if line == "report" || line == "report previous" {
                    let output = if line.ends_with(" previous") {
                        crate::browser_report::download_previous()
                    } else {
                        crate::browser_report::download()
                    };
                    let result = output
                        .map(|()| "Diagnostic report downloaded.".to_owned())
                        .unwrap_or_else(|error| {
                            format!("Diagnostic report failed: {error:?}")
                        });
                    panel.result(&result);
                    None
                } else if line == "save" {
                    let result = crate::browser_reload::restore_for(
                        crate::edit_intent::Intent::Save,
                        &editor,
                        std::rc::Rc::clone(&reload),
                        std::rc::Rc::clone(&reload_delivery),
                    )
                    .and_then(|waiting| {
                        if waiting {
                            Ok("Reloading editable sources…".to_owned())
                        } else {
                            crate::document::snapshot(&editor.scene)
                                .map_err(str::to_owned)
                                .and_then(|bytes| save_result(&bytes))
                        }
                    });
                    panel.result(&result.unwrap_or_else(|error| error));
                    None
                } else if line == "move" || line.starts_with("move ") {
                    match crate::offset::parse(&line) {
                        Ok(offset) => Some(Action::Translate(offset)),
                        Err(error) => {
                            panel.result(error);
                            None
                        }
                    }
                } else {
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example point" => Action::AddPoint,
                        "example curve" => Action::AddCurve,
                        "example object color" => {
                            Action::AddColours(crate::colour::Mode::Object)
                        }
                        "example point colors" => {
                            Action::AddColours(crate::colour::Mode::Points)
                        }
                        "example face colors" => {
                            Action::AddColours(crate::colour::Mode::Faces)
                        }
                        "view normals on" => Action::NormalView(true),
                        "view normals off" => Action::NormalView(false),
                        "view grid on" => {
                            Action::Grid(Some(crate::grid::Settings::default()))
                        }
                        "view grid off" => Action::Grid(None),
                        "example sharp" => {
                            Action::AddNormals(crate::normal_example::Mode::Sharp)
                        }
                        "example smooth" => {
                            Action::AddNormals(crate::normal_example::Mode::Smooth)
                        }
                        "example winding" => {
                            Action::AddNormals(crate::normal_example::Mode::Winding)
                        }

                        "controls on" => Action::Controls(true),
                        "controls off" => Action::Controls(false),
                        "example polyline" => Action::AddPolyline,
                        "example arrow" => Action::AddArrow,
                        "example triangle" => Action::ToggleExtra,
                        "select next" => Action::SelectNext,
                        "select all" => Action::SelectAll,
                        "select none" => Action::Picked(None),
                        "select object" => {
                            Action::PickMode(crate::pick_sources::Mode::Object)
                        }
                        "select face" => {
                            Action::PickMode(crate::pick_sources::Mode::Face)
                        }
                        "select edge" => {
                            Action::PickMode(crate::pick_sources::Mode::Edge)
                        }
                        "select control" => {
                            Action::PickMode(crate::pick_sources::Mode::Control)
                        }
                        "delete" => Action::Delete,
                        "undo" => Action::Undo,
                        "redo" => Action::Redo,
                        "background" => Action::Background,
                        "zoom in" => Action::Zoom(2.0),
                        "zoom out" => Action::Zoom(0.5),
                        "pan left" => Action::Pan(-0.25, 0.0),
                        "pan right" => Action::Pan(0.25, 0.0),
                        "orbit right" => {
                            Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)
                        }
                        "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                        "view isometric" => Action::Isometric,
                        "view perspective" => {
                            Action::Projection(crate::camera::Projection::Perspective)
                        }
                        "view orthographic" => {
                            Action::Projection(crate::camera::Projection::Orthographic)
                        }
                        "fit" if shortcut.is_some() && editor.selected.is_some() => {
                            Action::FitSelected
                        }
                        "fit" => Action::Fit,

                        "fit selected" => Action::FitSelected,
                        "view reset" => Action::ResetView,
                        _ => return,
                    };
                    Some(action)
                }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        config.format.add_srgb_suffix(),
        crate::commands::names(),
    );
    if let Some(message) = crate::browser_report::previous_notice() {
        panel.result(message);
    }
    if let Some(message) = &recovery {
        panel.result(message);
    }
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer
            .queue
            .on_submitted_work_done(move || {
                if crate::browser_runtime::owns(&completing)
                    && completing.message().is_none()
                {
                    crate::browser_phase::finish(
                        "first frame complete",
                        frame_started,
                        0,
                        "GPU",
                    );
                    let _ = crate::browser_report::observe(
                        "milestone",
                        "geometry on screen",
                    );
                }
            });
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared =
        std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if active_fault.message().is_some() {
                return;
            }
            let mut editor = shared.borrow_mut();
            if matches!(
                event.type_().as_str(),
                "blur" | "pagehide" | "viewer-reload"
            ) {
                renderer.cancel_pick();
            }
            if event.type_() == "pagehide" {
                gesture.cancel();
                if event
                    .dyn_ref::<web_sys::PageTransitionEvent>()
                    .is_some_and(|event| event.persisted())
                {
                    return;
                }
                wasm_bindgen_futures::spawn_local(async {
                    crate::browser_runtime::stop();
                });
                return;
            }
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
            let line = if let Some(shortcut) = shortcut {
                event.prevent_default();
                panel.answer(shortcut.command(), "");
                Some(shortcut.command().to_ascii_lowercase())
            } else {
                match panel.update(Some(&event), &input_canvas) {
                    Ok(line) => line,
                    Err(error) => {
                        report(&format!("Cannot read command: {error:?}"));
                        return;
                    }
                }
            };

            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
                    .target()
                    .and_then(|target| {
                        target
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .ok()
                    })
                    .is_some_and(|input| input.id() == "open")
            {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.answer("Open", "Open cancelled.");
                None
            } else if event.type_() == "change" {
                crate::file_input::choose(
                    &event,
                    std::rc::Rc::clone(&request),
                    read_mode.get(),
                    std::rc::Rc::clone(&delivery),
                );
                None
            } else if event.type_() == "viewer-file-error" {
                if let Some(message) = event
                    .dyn_ref::<web_sys::CustomEvent>()
                    .and_then(|event| event.detail().as_string())
                {
                    report(&message);
                    panel.answer("Open", &message);
                }
                None
            } else if event.type_() == "viewer-reload" {
                if let Some(reply) = reload_delivery.borrow_mut().take() {
                    let command = reply
                        .intent
                        .map_or("Reload Sources", |intent| intent.command());
                    match reply.complete(&mut editor) {
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            highlight_changed = true;
                            crate::browser_upload::prepare(
                                &mut renderer,
                                &editor.scene,
                                editor.whole_selection(),
                                active_fault.clone(),
                            );
                            crate::browser_phase::revision(&editor.scene);
                            let message = if command == "Reload Sources" {
                                "Editable sources restored; display retained."
                                    .to_owned()
                            } else {
                                format!("{command} completed after source reload.")
                            };
                            report(&message);
                            panel.answer(command, &message);
                        }
                        Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                            let message =
                                save_result(&bytes).unwrap_or_else(|error| error);
                            report(&message);
                            panel.answer(command, &message);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            report(&error);
                            panel.answer(command, &error);
                        }
                    }
                }
                None
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery) {
                    Ok(action) => action,
                    Err(error) => {
                        let message = format!("Cannot retain reload file: {error:?}");
                        report(&message);
                        panel.answer("Open", &message);
                        None
                    }
                }
            } else if let Some(line) = line {
                match crate::commands::parse(&line) {
                    Err(error) => {
                        panel.result(&error);
                        None
                    }
                    Ok(crate::commands::Command::Editor(mut action)) => {
                        if matches!(action, Action::Close) {
                            request.borrow_mut().cancel();
                        }
                        if shortcut.is_some()
                            && editor.selected.is_some()
                            && matches!(action, Action::Fit)
                        {
                            action = Action::FitSelected;
                        }
                        Some(action)
                    }
                    Ok(crate::commands::Command::Host(host)) => {
                        use crate::commands::Host;
                        match host {
                            Host::Help => {
                                panel.result(&crate::commands::names().join(" · "));
                                None
                            }
                            Host::Selection => {
                                let rows = editor.selected_objects();
                                panel.result(&if rows.len() > 1 {
                                    format!(
                                        "Objects {:?}",
                                        rows.iter()
                                            .map(|id| id.value())
                                            .collect::<Vec<_>>()
                                    )
                                } else {
                                    editor.selection().map_or_else(
                                        || "Nothing selected".to_owned(),
                                        |target| target.describe(),
                                    )
                                });
                                None
                            }
                            Host::Reload => {
                                let result = crate::browser_reload::start(
                                    std::rc::Rc::clone(&reload),
                                    editor.reload_keys(),
                                    std::rc::Rc::clone(&reload_delivery),
                                );
                                match result {
                                    Ok(()) => {
                                        panel.result("Reloading editable sources…")
                                    }
                                    Err(error) => {
                                        report(&error);
                                        panel.result(&error);
                                    }
                                }
                                None
                            }
                            Host::CancelReload => {
                                reload.borrow_mut().cancel();
                                report("Reload cancelled.");
                                panel.result("Reload cancelled.");
                                None
                            }
                            Host::CancelOpen => {
                                request.borrow_mut().cancel();
                                report("Open cancelled.");
                                panel.result("Open cancelled.");
                                None
                            }
                            Host::Open | Host::Replace => {
                                let ticket = request.borrow_mut().begin();
                                if let Err(error) = ticket {
                                    report(error);
                                    panel.result(error);
                                    return;
                                }
                                if host == Host::Replace {
                                    reload.borrow_mut().cancel();
                                }
                                read_mode.set(if host == Host::Replace {
                                    crate::file_input::Mode::Replace
                                } else {
                                    crate::file_input::Mode::Append
                                });
                                if let Some(input) = web_sys::window()
                                    .and_then(|window| window.document())
                                    .and_then(|document| {
                                        document.get_element_by_id("open")
                                    })
                                    .and_then(|element| {
                                        element
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .ok()
                                    })
                                {
                                    input.click();
                                }
                                None
                            }
                            Host::Report | Host::PreviousReport => {
                                let output = if host == Host::PreviousReport {
                                    crate::browser_report::download_previous()
                                } else {
                                    crate::browser_report::download()
                                };
                                let result = output
                                    .map(|()| {
                                        "Diagnostic report downloaded.".to_owned()
                                    })
                                    .unwrap_or_else(|error| {
                                        format!("Diagnostic report failed: {error:?}")
                                    });
                                panel.result(&result);
                                None
                            }
                            Host::Save => {
                                let result = crate::browser_reload::restore_for(
                                    crate::edit_intent::Intent::Save,
                                    &editor,
                                    std::rc::Rc::clone(&reload),
                                    std::rc::Rc::clone(&reload_delivery),
                                )
                                .and_then(|waiting| {
                                    if waiting {
                                        Ok("Reloading editable sources…".to_owned())
                                    } else {
                                        crate::document::snapshot(&editor.scene)
                                            .map_err(str::to_owned)
                                            .and_then(|bytes| save_result(&bytes))
                                    }
                                });
                                panel.result(&result.unwrap_or_else(|error| error));
                                None
                            }
                        }
                    }
                }
```

## Build & Run

![Recorded visual output for Lesson 7.1 (46-tool-panel)](../../screenshots/journey/46-tool-panel.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 7.1
npm --prefix ../session_tests run course -- save 7.1
```

Continue with [7.2 · Typed commands and host requests](47-registry.md).
