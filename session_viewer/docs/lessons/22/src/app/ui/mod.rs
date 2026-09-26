use crate::State;
use std::cell::Cell;
use theme::{BUNDLED, fonts, visuals};
use winit::window::Window;
mod overlay; // register:overlay
mod pointer; // register:pointer
mod theme; // register:theme

/// Declare each panel's module and list it in PANELS, so a panel is one file plus one line.
macro_rules! panels {
    ($($name:ident),* $(,)?) => {
        $(pub(crate) mod $name;)*

        /// The panels in drawing order; the number box first, so its Escape never reaches the command line.
        const PANELS: &[&dyn Panel] = &[$(&$name::Hooks),*];
    };
}

panels! {
    number_box,   // register:number_box
}

/// One panel. Its state lives in its own file; these hooks are all the frame needs from it.
trait Panel {
    /// Copy what it shows from the viewer, before the frame.
    fn fill(&self, _state: &mut State) {}

    /// Draw into the root area; a click or a line to run goes to `out`.
    fn show(&self, root: &mut egui::Ui, controls: &mut Option<Vec<Control>>, out: &mut Output);

    /// Apply what it took this frame, once egui is done; true when something changed.
    fn apply(&self, _state: &mut State) -> bool {
        false
    }

    /// True while it takes the keys.
    fn keys_taken(&self) -> bool {
        false
    }

    /// True while Escape is its own, so the command line leaves it alone.
    fn holds_escape(&self) -> bool {
        false
    }

    /// Its open text field for the phone keyboard: the egui id, after `edit` ran on the text.
    fn field(&self, _edit: &mut dyn FnMut(&mut String)) -> Option<&'static str> {
        None
    }

    /// Whether `point` is on its text field, which raises the keyboard, and on a floating area that keeps the pointer.
    fn hit(&self, _point: egui::Pos2) -> (bool, bool) {
        (false, false)
    }

    /// Its part of the state browser tests read.
    fn snapshot(&self, _json: &mut serde_json::Map<String, serde_json::Value>) {}

    /// True while it is open and Escape closes it.
    fn closes_on_escape(&self) -> bool {
        false
    }

    /// A press at `pointer` over the canvas: focus its field when the press is on it.
    fn press(&self, _context: &egui::Context, _pointer: egui::Pos2) {}
}

thread_local! { static MENU_OPEN: Cell<bool> = const { Cell::new(false) }; } // an egui popup, e.g. a layer or colour menu, is open

/// True while a popup such as a layer or colour menu is open.
fn menu_open() -> bool {
    MENU_OPEN.get()
}

/// True while a panel takes the keys: the command line, a layer rename, the number box or an open menu.
pub fn keys_taken() -> bool {
    menu_open() || PANELS.iter().any(|panel| panel.keys_taken())
}

/// True while another panel owns Escape, e.g. a layer rename.
fn escape_held() -> bool {
    PANELS.iter().any(|panel| panel.holds_escape())
}

/// Whether `point` is on a text field, which raises the keyboard, and on a floating area that keeps the pointer.
pub(crate) fn hit(point: egui::Pos2) -> (bool, bool) {
    PANELS.iter().fold((false, false), |(field, popup), panel| {
        let (on_field, on_popup) = panel.hit(point);
        (field || on_field, popup || on_popup)
    })
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    key: String,    // what it does
    label: String,  // text shown
    rect: [f32; 4], // left, top, right, bottom
}

/// What the panels hand back from one frame, applied once egui is done.
#[derive(Default)]
struct Output {
    action: Option<String>,  // a panel click, for `State::panel_action`
    command: Option<String>, // a line to run
}

/// The egui interface over the canvas.
pub struct Ui {
    context: egui::Context,                  // egui state
    input: egui_winit::State,                // winit events into egui
    controls: Option<Vec<Control>>,          // controls drawn this frame, when inspecting
    scene_rect: egui::Rect,                  // canvas area not covered by panels
    pointer: egui::Pos2,                     // last pointer position
    ui_drag: bool,                           // a drag started on a panel
    over_panel: bool,                        // the pointer was last over a panel or popup
    touches: std::collections::HashSet<u64>, // fingers on panels
}

impl Ui {
    /// Draw the panels with the whole fonts, main font first.
    pub fn use_fonts(&mut self, faces: [&'static [u8]; 3]) {
        self.context.set_fonts(fonts(faces));
    }

    /// Set up egui with the light theme.
    pub fn new(window: &Window, _logical_width: f64) -> Self {
        let context = egui::Context::default();
        context.set_fonts(fonts(BUNDLED));
        // one layout pass, so text events are never replayed
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        context.set_theme(egui::Theme::Light);
        context.set_visuals(visuals());
        let input = egui_winit::State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            window.theme(),
            Some(4096),
        );
        Self {
            context,
            input,
            #[cfg(not(target_arch = "wasm32"))]
            controls: Some(Vec::new()),
            #[cfg(target_arch = "wasm32")]
            controls: (crate::app::route::query("inspect").as_deref() == Some("1")).then(Vec::new),
            scene_rect: egui::Rect::EVERYTHING,
            pointer: egui::Pos2::ZERO,
            ui_drag: false,
            over_panel: false,
            touches: std::collections::HashSet::new(),
        }
    }

    /// Lay out and draw the panels; true when the frame must be redrawn.
    pub fn frame(&mut self, state: &mut State) -> bool {
        let mut input = self.input.take_egui_input(&state.window);
        // layout in CSS pixels
        let logical = state.logical_size();
        input.screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(logical[0] as f32, logical[1] as f32),
        ));

        if let Some(controls) = self.controls.as_mut() {
            controls.clear();
        }

        let mut out = Output::default();

        for panel in PANELS {
            panel.fill(state);
        }

        // a dragged object's snap, else the shape being drawn
        let mut drawing = state.drag_overlay();
        // keep clicks and keys in arrival order
        let mut batches = Vec::new();
        let mut events = Vec::new();
        let mut keyboard = None;
        for event in std::mem::take(&mut input.events) {
            let kind = match &event {
                egui::Event::PointerButton { .. } => Some(false),
                egui::Event::Key { .. } | egui::Event::Text(_) | egui::Event::Paste(_) => {
                    Some(true)
                }
                _ => None,
            };
            if kind.is_some() && keyboard.is_some() && kind != keyboard {
                let mut batch = input.clone();
                batch.events = std::mem::take(&mut events);
                batches.push(batch);
            }
            if kind.is_some() {
                keyboard = kind;
            }
            events.push(event);
        }
        input.events = events;
        batches.push(input);
        let mut draw = |root: &mut egui::Ui| {
            if let Some(controls) = self.controls.as_mut() {
                controls.clear();
            }
            for panel in PANELS {
                panel.show(root, &mut self.controls, &mut out);
            }
            self.scene_rect = root.available_rect_before_wrap();
            let painter = root.painter().with_clip_rect(self.scene_rect);
            let scale = state.pixel_scale() as f32;

            if let Some(drawing) = &drawing {
                overlay::drawing(&painter, drawing, scale);
            }

        };
        let mut batches = batches.into_iter();
        let mut output = self.context.run_ui(batches.next().unwrap(), &mut draw);
        for batch in batches {
            output.append(self.context.run_ui(batch, &mut draw));
        }
        MENU_OPEN.set(egui::Popup::is_any_open(&self.context));
        self.input
            .handle_platform_output(&state.window, std::mem::take(&mut output.platform_output));
        let mut changed = false;

        for panel in PANELS {
            changed |= panel.apply(state);
        }

        let Output { action, command } = out;
        changed |= action.is_some() || command.is_some();

        if let Some(key) = action {
        }

        if let Some(text) = command {
        }

        self.publish();
        let repaint = changed || self.context.has_requested_repaint();
        output.pixels_per_point = state.gpu.config.width as f32 / logical[0].max(1.0) as f32;

        if let Some(ui) = state.gpu.ui.as_mut() {
            ui.prepare(
                &state.gpu.ctx,
                &self.context,
                output,
                [state.gpu.config.width, state.gpu.config.height],
            );
        }

        repaint
    }

    /// Write the panel state onto the canvas for browser tests.
    fn publish(&self) {
        if self.controls.is_some()
            && let Some(canvas) = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id("canvas"))
        {
            let mut snapshot = serde_json::Map::new();
            snapshot.insert("framework".into(), "egui 0.34.3".into());
            snapshot.insert("scene_rect".into(), corners(Some(self.scene_rect)));
            snapshot.insert("controls".into(), serde_json::json!(self.controls));

            for panel in PANELS {
                panel.snapshot(&mut snapshot);
            }

            let snapshot = serde_json::Value::Object(snapshot).to_string();
            let _ = canvas.set_attribute("data-viewer-ui", &snapshot);
        }

        if let Some(status) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("viewer-status"))
        {
            let _ = status.set_attribute("hidden", "");
        }
    }
}

/// Remember one control's rectangle, when inspecting.
fn record(controls: &mut Option<Vec<Control>>, key: &str, label: &str, response: &egui::Response) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// A rectangle as `[left, top, right, bottom]` for browser tests, null when there is none.
fn corners(rect: Option<egui::Rect>) -> serde_json::Value {
    serde_json::json!(rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]))
}
