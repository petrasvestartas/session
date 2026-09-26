use crate::State;
use crate::app::feedback::{EdgeRow, LayerRow};
use crate::app::gizmo::Handle;
use crate::state::number_box::NumberPrompt;
use std::cell::RefCell;
use std::collections::VecDeque;
use theme::{BUNDLED, fonts, visuals};
use winit::window::Window;
mod command_line; // register:command_line
mod graph; // register:graph
mod layers; // register:layers
mod number_box; // register:number_box
mod overlay; // register:overlay
#[cfg(target_arch = "wasm32")]
mod phone; // register:phone
mod pointer; // register:pointer
mod theme; // register:theme

/// Everything the panels show.
#[derive(Default)]
pub struct Model {
    pub layers_open: bool,                                    // layers panel shown
    pub rows: Vec<LayerRow>,                                  // its rows
    pub edges: Vec<EdgeRow>,                                  // graph table rows
    pub edge_total: usize,                                    // graph edges, listed or not
    pub(crate) graph_open: bool,                              // graph table unfolded
    pub(crate) renaming: Option<Rename>,                      // a layer name edited in its row
    menu_open: bool,                                          // a layer or colour menu is shown
    pub command_open: bool,                                   // command line shown
    pub command: String,                                      // text in the command field
    pub drawing_prompt: String,                               // prompt while drawing
    drawing_options: &'static [(&'static str, &'static str)], // buttons while drawing: (label, line)
    drawing_chosen: Option<&'static str>,                     // the option button shown as chosen
    pub focus_command: bool,                                  // give the field focus next frame
    pub status: String,                                       // status line text
    history: VecDeque<String>,                                // past commands and answers
    command_expanded: bool,                                   // history shown above the field
    layers_collapsed: bool,                                   // layers panel folded to its title
    completion: usize,                                        // highlighted completion index
    completion_prefix: String,                                // text the completions match
    inline_suffix: bool,      // completion suffix shown in the field
    completion_visible: bool, // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    pub(crate) keyboard_rects: Vec<egui::Rect>, // a layer name field or an item opening one
    snap_bar: bool,           // snap toolbar under the field
    snap_modes: u8,           // snap kinds switched on
    agent_edit: Option<bool>, // phone keyboard set the text, true on delete
    number_prompt: Option<NumberPrompt>, // the gumball number box, when open
    number_handle: Option<Handle>, // the handle the box was opened for
    number: String,           // text typed into the box
    number_error: String,     // why the typed value was refused
    pub(crate) number_rect: Option<egui::Rect>, // where the box is, for taps
}

thread_local! { pub static MODEL: RefCell<Model> = RefCell::default(); } // the one model

/// A layer name being edited in its row.
pub(crate) struct Rename {
    pub node: String,  // the row's node index
    pub text: String,  // the name typed so far
    pub focused: bool, // the field has the keys
    pub done: bool,    // kept by Enter or a click elsewhere, applied after the frame
}

/// True while a panel takes the keys: the command line, a layer rename, the number box or an open menu.
pub fn keys_taken() -> bool {
    MODEL.with_borrow(|model| {
        model.command_open
            || model.menu_open
            || model.number_prompt.is_some()
            || model.renaming.as_ref().is_some_and(|rename| rename.focused)
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
    typed: Option<String>,   // a value Enter took from the number box
    closed: bool,            // the number box was closed without a value
}

/// One panel: draws itself into the root area and reports through `Output`.
type Panel = fn(&mut egui::Ui, &mut Model, &mut Option<Vec<Control>>, &mut Output);

/// The panels in drawing order; the number box first, so its Escape never reaches the command line.
const PANELS: &[Panel] = &[
    number_box::show,   // register:number_box
    command_line::show, // register:command_line
    layers::show,       // register:layers
];

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
    #[cfg(target_arch = "wasm32")]
    agent_value: String, // last text taken from the hidden input
    #[cfg(target_arch = "wasm32")]
    field: Option<&'static str>, // the field the hidden input fed last frame, None for the command line
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
        MODEL.with_borrow_mut(|model| model.focus_command = true);
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
            #[cfg(target_arch = "wasm32")]
            agent_value: String::new(),
            #[cfg(target_arch = "wasm32")]
            field: None,
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
        let number_prompt = state.number_prompt();

        // a box whose handle went behind the eye closes, so no unseen field keeps the keys
        if number_prompt.is_none() {
            state.close_number_box();
        }

        MODEL.with_borrow_mut(|model| {
            model.drawing_prompt = state.drawing_prompt();
            model.drawing_options = state.drawing_options();
            model.drawing_chosen = state.drawing_chosen();
            model.snap_bar = state.features.snap.bar;
            model.snap_modes = state.features.snap.modes;
            model.number_prompt = number_prompt;
        });
        // a dragged object's snap, else the shape being drawn
        let drawing = state
            .drag_overlay()
            .unwrap_or_else(|| state.drawing_overlay());
        let marks = state.tool_marks().or_else(|| state.mark_overlay()); // a tool's parts, else a measured answer
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
            MODEL.with_borrow_mut(|model| {
                for panel in PANELS {
                    panel(root, model, &mut self.controls, &mut out);
                }
            });
            self.scene_rect = root.available_rect_before_wrap();
            let painter = root.painter().with_clip_rect(self.scene_rect);
            let scale = state.pixel_scale() as f32;
            overlay::drawing(&painter, &drawing, scale);

            if let Some(marks) = &marks {
                overlay::tool_marks(&painter, marks, scale);
            }
        };
        let mut batches = batches.into_iter();
        let mut output = self.context.run_ui(batches.next().unwrap(), &mut draw);
        for batch in batches {
            output.append(self.context.run_ui(batch, &mut draw));
        }
        MODEL.with_borrow_mut(|model| model.menu_open = egui::Popup::is_any_open(&self.context));
        self.input
            .handle_platform_output(&state.window, std::mem::take(&mut output.platform_output));
        // a kept layer name goes in before the click that ended its edit
        let renamed = MODEL.with_borrow_mut(|model| model.renaming.take_if(|rename| rename.done));
        let Output {
            action,
            command,
            typed,
            closed,
        } = out;
        let changed =
            action.is_some() || command.is_some() || renamed.is_some() || typed.is_some() || closed;

        if let Some(rename) = renamed {
            state.panel_action(&format!("rename/{}/{}", rename.node, rename.text));
        }

        if closed {
            state.close_number_box();
        }

        // Enter in the number box: one undo step, or the reason under the field
        if let Some(text) = typed {
            match state.type_number(&text) {
                Ok(Some(done)) => {
                    crate::app::feedback::status(&done);
                    MODEL.with_borrow_mut(|model| {
                        if model.history.len() == 200 {
                            model.history.pop_front();
                        }

                        model.history.push_back(format!("> {done}"));
                    });
                }
                Ok(None) => {}
                Err(error) => MODEL.with_borrow_mut(|model| model.number_error = error),
            }
        }

        if let Some(key) = action {
            state.panel_action(&key);
        }

        if let Some(text) = command {
            let message = state.run_command(&text).unwrap_or_else(|error| error);
            crate::app::feedback::status(&message);
            MODEL.with_borrow_mut(|model| {
                if model.history.len() == 200 {
                    model.history.pop_front();
                }

                model.history.push_back(format!(
                    "> {}\n{message}",
                    crate::app::command::canonical(&text)
                ));
                if !model.command_open && !model.focus_command {
                    self.context.memory_mut(|memory| {
                        memory.surrender_focus(egui::Id::new("command-input"))
                    });
                }
            });
            state.touch();
        }

        self.publish();
        #[cfg(target_arch = "wasm32")]
        self.follow_field();
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
            let snapshot = MODEL.with_borrow(|model| serde_json::json!({"framework": "egui 0.34.3", "scene_rect": [self.scene_rect.min.x,self.scene_rect.min.y,self.scene_rect.max.x,self.scene_rect.max.y], "completion_rect": model.completion_rect.map(|r| [r.min.x,r.min.y,r.max.x,r.max.y]), "rows": model.rows, "edges": model.edges, "edge_total": model.edge_total, "controls": self.controls, "command_open": model.command_open, "layers_open": model.layers_open, "command": model.command, "history": model.history, "hint": crate::app::command::hint(&model.command), "placeholder": command_line::placeholder(&model.drawing_prompt, &model.status, model.command_expanded), "number": model.number, "number_error": model.number_error, "number_rect": model.number_rect.map(|r| [r.min.x,r.min.y,r.max.x,r.max.y])}));
            let _ = canvas.set_attribute("data-viewer-ui", &snapshot.to_string());
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
