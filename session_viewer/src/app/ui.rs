use crate::State;
use crate::app::feedback::{EdgeRow, LayerRow};
use crate::app::gizmo::Handle;
use crate::state::number_box::NumberPrompt;
use std::cell::RefCell;
use std::collections::VecDeque;
use winit::window::Window;

/// Everything the panels show.
#[derive(Default)]
pub struct Model {
    pub layers_open: bool,                          // layers panel shown
    pub rows: Vec<LayerRow>,                        // its rows
    pub edges: Vec<EdgeRow>,                        // graph table rows
    pub edge_total: usize,                          // graph edges, listed or not
    pub(crate) graph_open: bool,                    // graph table unfolded
    pub(crate) renaming: Option<Rename>,            // a layer name edited in its row
    menu_open: bool,                                // a layer or colour menu is shown
    pub command_open: bool,                         // command line shown
    pub command: String,                            // text in the command field
    pub drawing_prompt: String,                     // prompt while drawing
    drawing_command: String,                        // the drawing verb, e.g. polyline
    pub focus_command: bool,                        // give the field focus next frame
    pub status: String,                             // status line text
    history: VecDeque<String>,                      // past commands and answers
    command_expanded: bool,                         // history shown above the field
    layers_collapsed: bool,                         // layers panel folded to its title
    completion: usize,                              // highlighted completion index
    completion_prefix: String,                      // text the completions match
    inline_suffix: bool,                            // completion suffix shown in the field
    completion_visible: bool,                       // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    pub(crate) keyboard_rects: Vec<egui::Rect>,     // a layer name field or an item opening one
    snap_bar: bool,                                 // snap toolbar under the field
    snap_modes: u8,                                 // snap kinds switched on
    agent_edit: Option<bool>,                       // phone keyboard set the text, true on delete
    number_prompt: Option<NumberPrompt>,            // the gumball number box, when open
    number_handle: Option<Handle>,                  // the handle the box was opened for
    number: String,                                 // text typed into the box
    number_error: String,                           // why the typed value was refused
    pub(crate) number_rect: Option<egui::Rect>,     // where the box is, for taps
}

thread_local! { pub static MODEL: RefCell<Model> = RefCell::default(); } // the one model

/// A text field the phone keyboard types into, besides the command line.
struct TextField {
    id: &'static str,                            // the egui id of its text edit
    text: fn(&mut Model) -> Option<&mut String>, // its text, None while it is not shown
}

/// The fields that take the phone keyboard before the command line, the first open one.
const FIELDS: &[TextField] = &[
    // register:layer-rename
    TextField {
        id: "layer-rename",
        text: |model| model.renaming.as_mut().map(|rename| &mut rename.text),
    },
    // register:number-box
    TextField {
        id: "number-input",
        text: |model| model.number_prompt.is_some().then_some(&mut model.number),
    },
];

/// The open field the phone keyboard types into, None for the command line.
fn open_field(model: &mut Model) -> Option<&'static TextField> {
    FIELDS.iter().find(|field| (field.text)(model).is_some())
}

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

/// The panel fonts: egui's Ubuntu Light, then the label fonts for the symbols it lacks.
fn fonts() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::empty();
    let faces = [
        ("Ubuntu-Light", epaint_default_fonts::UBUNTU_LIGHT),
        ("Noto Sans", crate::engine::text::FONT_BYTES),
        ("Noto Sans Symbols", crate::engine::text::SYMBOL_BYTES),
        ("Noto Sans Symbols 2", crate::engine::text::FALLBACK_BYTES),
    ];

    for (name, bytes) in faces {
        let data = std::sync::Arc::new(egui::FontData::from_static(bytes));
        fonts.font_data.insert(name.to_owned(), data);

        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .push(name.to_owned());
        }
    }

    fonts
}

impl Ui {
    /// Set up egui with the light theme.
    pub fn new(window: &Window, _logical_width: f64) -> Self {
        let context = egui::Context::default();
        context.set_fonts(fonts());
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
            controls: (super::route::query("inspect").as_deref() == Some("1")).then(Vec::new),
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

    /// Offer one event to the panels; (consumed, needs repaint).
    pub fn event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> (bool, bool) {
        let response = self.input.on_window_event(window, event);
        let escape = matches!(event, winit::event::WindowEvent::KeyboardInput { event, .. }
            if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape))
            && MODEL.with_borrow(|model| model.command_open);
        // use the current pointer, not last frame's hover
        use winit::event::{ElementState, TouchPhase, WindowEvent};
        let ratio = window.scale_factor() as f32;
        let mut consumed = response.consumed;
        let mut scene_rect = self.scene_rect;
        scene_rect.max.y -= 5.0;
        let context = self.context.clone();
        // the completion list, the number box, a context menu or a colour menu
        let in_popup = |point| {
            MODEL.with_borrow(|m| {
                m.completion_rect.is_some_and(|r| r.contains(point))
                    || m.number_rect.is_some_and(|r| r.contains(point))
            }) || context
                .layer_id_at(point)
                .is_some_and(|layer| layer.order != egui::Order::Background)
        };

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = egui::pos2(position.x as f32 / ratio, position.y as f32 / ratio);
                self.over_panel = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                consumed = self.ui_drag || self.over_panel;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *state == ElementState::Pressed {
                    self.ui_drag = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                    self.over_panel = self.ui_drag;
                    self.press();
                }

                consumed = self.ui_drag;

                // a left drag from the scene let go over a panel is dropped, not applied
                if *state == ElementState::Released {
                    consumed |= *button == winit::event::MouseButton::Left && self.over_panel;
                    self.ui_drag = false;
                }
            }
            WindowEvent::MouseWheel { .. } => {
                consumed = in_popup(self.pointer) || !scene_rect.contains(self.pointer)
            }
            WindowEvent::Touch(touch) => {
                self.pointer = egui::pos2(
                    touch.location.x as f32 / ratio,
                    touch.location.y as f32 / ratio,
                );

                if touch.phase == TouchPhase::Started {
                    if self.touches.is_empty() {
                        self.ui_drag = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                        self.press();
                    }

                    self.touches.insert(touch.id);
                }

                consumed = self.ui_drag;

                if matches!(touch.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    self.touches.remove(&touch.id);

                    if self.touches.is_empty() {
                        self.ui_drag = false;
                    }
                }
            }
            WindowEvent::Focused(false) => {
                self.ui_drag = false;
                self.touches.clear();
            }
            _ => {}
        }

        // an open menu or number box keeps the keys too, so Escape only closes it
        if matches!(event, WindowEvent::KeyboardInput { .. })
            && MODEL.with_borrow(|model| {
                model.command_open || model.menu_open || model.number_prompt.is_some()
            })
        {
            consumed = true;
        }
        (consumed || escape, response.repaint || escape)
    }

    /// A press on the field opens it, anywhere else but the list closes it.
    fn press(&mut self) {
        let id = egui::Id::new("command-input");
        let (input, popup) = MODEL.with_borrow(|m| {
            (
                m.command_rect.is_some_and(|r| r.contains(self.pointer)),
                m.completion_rect.is_some_and(|r| r.contains(self.pointer)),
            )
        });

        // focus now so the first key is not lost
        if input {
            self.context.memory_mut(|memory| memory.request_focus(id));
            MODEL.with_borrow_mut(|model| model.command_open = true);
        } else if !popup {
            self.context.memory_mut(|memory| memory.surrender_focus(id));
            MODEL.with_borrow_mut(|model| model.command_open = false);
        }
    }

    /// Feed the hidden input's typing into the field; returns keys for the viewport.
    #[cfg(target_arch = "wasm32")]
    pub fn agent(&mut self, event: super::agent::AgentEvent) -> Vec<String> {
        use super::agent::AgentEvent;
        let id = egui::Id::new("command-input");

        // a layer name or the number box takes the typing before the command line
        if let Some(field) = MODEL.with_borrow_mut(open_field) {
            self.type_into(field, event);
            return Vec::new();
        }

        // an empty line while drawing still finishes the shape
        let (open, empty) = MODEL.with_borrow(|m| {
            (
                m.command_open,
                m.command.is_empty() && m.drawing_prompt.is_empty(),
            )
        });

        match &event {
            AgentEvent::Text(value) if !open => {
                let shared = self
                    .agent_value
                    .chars()
                    .zip(value.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                let keys = value.chars().skip(shared).map(String::from).collect();
                self.agent_value.clear();
                super::agent::sync("");
                return keys;
            }
            AgentEvent::Key(egui::Key::Enter) if open && empty => {
                super::feedback::command_line(false);
                self.context.memory_mut(|memory| memory.surrender_focus(id));
                super::feedback::status("Keys go to the viewer · type : for the command line");
                return Vec::new();
            }
            _ => {}
        }

        let key = |key, pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let events = &mut self.input.egui_input_mut().events;

        match event {
            // the input's whole text replaces the field, so keyboard composition cannot duplicate it
            AgentEvent::Text(value) => {
                if value == self.agent_value {
                    return Vec::new();
                }

                let deleted = value.chars().count() < self.agent_value.chars().count();
                self.agent_value.clone_from(&value);
                MODEL.with_borrow_mut(|model| {
                    model.command = value;
                    model.agent_edit = Some(deleted);
                    model.command_open = true;
                    model.focus_command = true;
                });
                self.context.memory_mut(|memory| memory.request_focus(id));
            }
            AgentEvent::Key(k) => {
                events.push(key(k, true));
                events.push(key(k, false));
            }
        }

        Vec::new()
    }

    /// Feed the hidden input's typing into a field other than the command line.
    #[cfg(target_arch = "wasm32")]
    fn type_into(&mut self, field: &TextField, event: super::agent::AgentEvent) {
        use super::agent::AgentEvent;
        let id = egui::Id::new(field.id);

        match event {
            // the input's whole text replaces the field's, as for the command line
            AgentEvent::Text(value) => {
                command_cursor_end(&self.context, id, &value);
                MODEL.with_borrow_mut(|model| {
                    if let Some(text) = (field.text)(model) {
                        text.clone_from(&value);
                    }
                });
                self.agent_value = value;
                self.context.memory_mut(|memory| memory.request_focus(id));
            }
            AgentEvent::Key(key) => {
                // Enter keeps the text, Escape drops it; both lower the keyboard
                if matches!(key, egui::Key::Enter | egui::Key::Escape) {
                    super::agent::blur();
                }

                for pressed in [true, false] {
                    self.input.egui_input_mut().events.push(egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    });
                }
            }
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

        let mut action = None;
        let mut command = None;
        let mut typed = None; // a value Enter took from the number box
        let mut closed = false; // the number box was closed without a value
        let number_prompt = state.number_prompt();

        // a box whose handle went behind the eye closes, so no unseen field keeps the keys
        if number_prompt.is_none() {
            state.close_number_box();
        }

        MODEL.with_borrow_mut(|model| {
            model.drawing_prompt = state.drawing_prompt();
            model.drawing_command = state.drawing_verb().to_owned();
            model.snap_bar = state.snap_bar;
            model.snap_modes = state.snap_modes;
            model.number_prompt = number_prompt;
        });
        // a dragged object's snap, else the shape being drawn
        let drawing = state
            .drag_overlay()
            .unwrap_or_else(|| state.drawing_overlay());
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
                // first, so its Escape never reaches the command line
                number_box(root, model, &mut self.controls, &mut typed, &mut closed);
                commands(root, model, &mut self.controls, &mut command);
                layers(root, model, &mut self.controls, &mut action);
            });
            self.scene_rect = root.available_rect_before_wrap();
            let painter = root.painter().with_clip_rect(self.scene_rect);
            let scale = state.pixel_scale() as f32;
            let points: Vec<egui::Pos2> = drawing
                .0
                .iter()
                .map(|p| egui::pos2(p.0 as f32 / scale, p.1 as f32 / scale))
                .collect();
            for pair in points.windows(2) {
                painter.line_segment(
                    [pair[0], pair[1]],
                    egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(30, 110, 170)),
                );
            }
            if let Some(p) = points.last() {
                painter.rect_stroke(
                    egui::Rect::from_center_size(*p, egui::vec2(8.0, 8.0)),
                    0.0,
                    egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(30, 110, 170)),
                    egui::StrokeKind::Middle,
                );
                painter.text(
                    *p + egui::vec2(10.0, -12.0),
                    egui::Align2::LEFT_BOTTOM,
                    &drawing.1,
                    egui::FontId::proportional(14.0),
                    egui::Color32::from_rgb(20, 80, 130),
                );
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

                model.history.push_back(format!("> {text}\n{message}"));
                if !model.command_open && !model.focus_command {
                    self.context.memory_mut(|memory| {
                        memory.surrender_focus(egui::Id::new("command-input"))
                    });
                }
            });
            state.touch();
        }

        self.publish();
        // the field changed on its own: the hidden input follows
        #[cfg(target_arch = "wasm32")]
        MODEL.with_borrow_mut(|model| {
            let field = open_field(model).map(|field| field.id);

            // a field that just opened takes the input, selected so typing replaces it
            if field != self.field {
                self.field = field;

                if let Some(text) = open_field(model).and_then(|field| (field.text)(model)) {
                    self.agent_value.clone_from(text);
                    super::agent::edit(text);
                }
            }

            // the input keeps what was typed: no completion suffix, no rewrite mid-word
            if self.field.is_none()
                && self.agent_value != model.command
                && !model.inline_suffix
                && !super::agent::composing()
            {
                self.agent_value.clone_from(&model.command);
                super::agent::sync(&model.command);
            }
        });
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
            let snapshot = MODEL.with_borrow(|model| serde_json::json!({"framework": "egui 0.34.3", "scene_rect": [self.scene_rect.min.x,self.scene_rect.min.y,self.scene_rect.max.x,self.scene_rect.max.y], "completion_rect": model.completion_rect.map(|r| [r.min.x,r.min.y,r.max.x,r.max.y]), "rows": model.rows, "edges": model.edges, "edge_total": model.edge_total, "controls": self.controls, "command_open": model.command_open, "layers_open": model.layers_open, "command": model.command, "history": model.history, "hint": crate::app::command::hint(&model.command), "number": model.number, "number_error": model.number_error, "number_rect": model.number_rect.map(|r| [r.min.x,r.min.y,r.max.x,r.max.y])}));
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

/// The white theme.
fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.window_fill = egui::Color32::WHITE;
    visuals.panel_fill = egui::Color32::from_gray(245);
    visuals.window_stroke = egui::Stroke::NONE;
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::from_rgb(200, 222, 245);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::BLACK);
    visuals.text_cursor.stroke = egui::Stroke::new(1.5_f32, egui::Color32::BLACK);
    visuals.text_cursor.blink = false; // frames are drawn on demand
    visuals.indent_has_left_vline = false;

    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        widget.bg_stroke = egui::Stroke::NONE;
        widget.bg_fill = egui::Color32::from_gray(245);
        widget.weak_bg_fill = egui::Color32::WHITE;
        widget.fg_stroke.color = egui::Color32::BLACK;
    }

    visuals
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

/// The layers panel; a click sets `action`.
fn layers(
    root: &mut egui::Ui,                 // the panel area
    model: &mut Model,                   // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) {
    model.keyboard_rects.clear();

    if !model.layers_open {
        return;
    }

    let collapsed = model.layers_collapsed;
    let width = if collapsed {
        32.0
    } else {
        (root.available_width() * 0.25).clamp(180.0, 310.0)
    };
    egui::Panel::right(if collapsed {
        "session-layers-collapsed"
    } else {
        "session-layers"
    })
    .default_size(width)
    .size_range(if collapsed {
        32.0..=32.0
    } else {
        180.0..=360.0
    })
    .show_separator_line(false)
    .frame(
        // the panel frame
        egui::Frame::new()
            .fill(egui::Color32::from_gray(245))
            .inner_margin(4),
    )
    .resizable(!collapsed)
    .show_inside(root, |ui| {
        ui.set_min_width(ui.available_width());
        let collapse = ui
            .button(if collapsed { "+" } else { "−" })
            .on_hover_text("Collapse or expand panel");
        record(
            controls,
            "layers/collapse",
            "Collapse or expand panel",
            &collapse,
        );
        if collapse.clicked() {
            model.layers_collapsed = !model.layers_collapsed;
            ui.ctx().request_repaint();
        }
        if collapsed {
            return;
        }
        let height = ui.text_style_height(&egui::TextStyle::Body); // one compact row
        // the graph section sits below the tree, a table when unfolded
        let graph = !model.rows.is_empty();
        let rest = (ui.available_height() - if graph { height + 4. } else { 0. }).max(0.);
        let tree = if graph && model.graph_open {
            rest * 0.6
        } else {
            rest
        };
        // one line per row
        egui::ScrollArea::vertical()
            .id_salt("layer-rows")
            .max_height(tree)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.;
                ui.spacing_mut().interact_size.y = height;
                ui.spacing_mut().button_padding.y = 0.;
                let Model {
                    rows,
                    renaming,
                    keyboard_rects,
                    ..
                } = &mut *model;

                for row in rows.iter() {
                    // painted under the row once its height is known
                    let strip = ui.painter().add(egui::Shape::Noop);
                    let line = ui.horizontal(|ui| {
                        if let Some(index) = row.key.strip_prefix("select/") {
                            layer_row(ui, row, index, height, controls, action, renaming)
                        } else {
                            let response = ui.button(&row.label);
                            record(controls, &row.key, &row.label, &response);

                            if response.clicked() {
                                *action = Some(row.key.clone());
                            }

                            Vec::new()
                        }
                    });
                    keyboard_rects.extend(line.inner);

                    // the whole strip, buttons included
                    if row.selected {
                        let rect = egui::Rect::from_x_y_ranges(
                            ui.max_rect().x_range(),
                            line.response.rect.y_range(),
                        );
                        ui.painter()
                            .set(strip, egui::Shape::rect_filled(rect, 0., SELECTED));
                    }
                }
            });

        if graph {
            edges(ui, model, height, controls, action);
        }
    });
}

/// The selection yellow, as in the scene.
const SELECTED: egui::Color32 = egui::Color32::from_rgb(255, 255, 0);

/// One tree row: arrow, name, bulb or check, lock, swatch; returns where a tap raises the keyboard.
fn layer_row(
    ui: &mut egui::Ui,
    row: &LayerRow,
    index: &str,
    height: f32,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
    renaming: &mut Option<Rename>,
) -> Vec<egui::Rect> {
    let mut keyboard = Vec::new();
    ui.spacing_mut().item_spacing.x = 2.;
    ui.add_space(row.depth.min(8) as f32 * 10.);
    let response = layer_icon(ui, "open", row, height);
    record(controls, &format!("open/{index}"), &row.label, &response);

    if response.clicked() && row.expanded.is_some() {
        *action = Some(format!("open/{index}"));
    }

    // the name takes what the three icons leave
    let width = (ui.available_width() - 3. * (height + 6.)).max(24.);

    if let Some(rename) = renaming.as_mut()
        && rename.node == index
        && !rename.done
    {
        let edit = ui.add_sized(
            [width, height],
            egui::TextEdit::singleline(&mut rename.text)
                .id(egui::Id::new("layer-rename"))
                .margin(egui::vec2(2., 0.)),
        );
        record(controls, &format!("rename/{index}"), &row.label, &edit);
        keyboard.push(edit.rect);

        // the whole name selected, so typing replaces it
        if !rename.focused {
            edit.request_focus();
            command_cursor_select(ui.ctx(), edit.id, 0, rename.text.chars().count());
            rename.focused = true;
        }

        // Escape cancels; Enter or a click elsewhere keeps a changed name
        if edit.lost_focus() {
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) || rename.text == row.label {
                *renaming = None;
            } else {
                rename.done = true;
            }
        }
    } else {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
        let galley = egui::WidgetText::from(row.label.as_str()).into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            width - 2.,
            egui::TextStyle::Body,
        );
        let top = rect.center().y - galley.size().y / 2.;
        ui.painter().galley(
            egui::pos2(rect.left() + 2., top),
            galley,
            ui.visuals().text_color(),
        );
        let response = response.on_hover_text(format!("{} · {} objects", row.label, row.count));
        record(
            controls,
            &row.key,
            &format!("Select {}", row.label),
            &response,
        );

        // egui counts a quick third click as a triple, not a double
        if row.layer && (response.double_clicked() || response.triple_clicked()) {
            *action = Some(format!("current/{index}"));
        } else if response.clicked() {
            *action = Some(if ui.input(|i| i.modifiers.shift) {
                row.key.replacen("select/", "add/", 1)
            } else {
                row.key.clone()
            });
        }

        if row.layer {
            response.context_menu(|ui| {
                layer_menu(ui, row, index, controls, action, renaming, &mut keyboard)
            });
        }
    }

    for kind in ["hide", "lock"] {
        let response = layer_icon(ui, kind, row, height);
        let verb = if kind == "hide" {
            if row.current {
                "Current"
            } else if row.hidden {
                "Show"
            } else {
                "Hide"
            }
        } else if row.locked {
            "Unlock"
        } else {
            "Lock"
        };
        record(
            controls,
            &format!("{kind}/{index}"),
            &format!("{verb} {}", row.label),
            &response,
        );

        // the current layer cannot be hidden
        if response.clicked() && !(kind == "hide" && row.current) {
            *action = Some(format!("{kind}/{index}"));
        }
    }

    layer_color(ui, row, index, height, controls, action);
    keyboard
}

/// The right-click menu of a layer row; items that open a name field go to `keyboard`.
fn layer_menu(
    ui: &mut egui::Ui,
    row: &LayerRow,
    index: &str,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
    renaming: &mut Option<Rename>,
    keyboard: &mut Vec<egui::Rect>, // a tap on these raises the phone keyboard
) {
    // the top layer of a document keeps its name and place
    let root = row.root.then_some("The top layer of a document stays");

    for (key, label) in [
        ("current", "Set Current"),
        ("new_layer", "New Layer"),
        ("new_sublayer", "New Sublayer"),
    ] {
        let item = menu_item(ui, &format!("{key}/{index}"), label, None, controls, action);

        // a new layer is named right away
        if key != "current" {
            keyboard.push(item.rect);
        }
    }

    let response = ui
        .add_enabled(root.is_none(), egui::Button::new("Rename Layer"))
        .on_disabled_hover_text(root.unwrap_or_default());
    record(
        controls,
        &format!("menu-rename/{index}"),
        "Rename Layer",
        &response,
    );

    if root.is_none() {
        keyboard.push(response.rect);
    }

    if response.clicked() {
        *renaming = Some(Rename {
            node: index.to_string(),
            text: row.label.clone(),
            focused: false,
            done: false,
        });
        ui.close();
    }

    let delete = format!("delete_layer/{index}");
    let refusal = if row.current {
        Some("The current layer cannot be deleted")
    } else {
        root
    };

    // a layer with objects asks first
    if refusal.is_some() || row.count == 0 {
        menu_item(ui, &delete, "Delete Layer", refusal, controls, action);
    } else {
        let response = ui
            .menu_button("Delete Layer", |ui| {
                let confirm = "Delete Layer and Objects";
                ui.label(format!("Also deletes its {} objects", row.count));
                menu_item(ui, &delete, confirm, None, controls, action);
            })
            .response;
        record(
            controls,
            &format!("menu-delete/{index}"),
            "Delete Layer",
            &response,
        );
    }

    let duplicate = format!("duplicate_layer/{index}");
    menu_item(ui, &duplicate, "Duplicate Layer", root, controls, action);

    for (key, label) in [
        ("change_layer", "Change Object Layer"),
        ("copy_layer", "Copy Object Layer"),
    ] {
        menu_item(ui, &format!("{key}/{index}"), label, None, controls, action);
    }
}

/// One menu button, greyed with its reason when refused; a click sets `action` and closes the menu.
fn menu_item(
    ui: &mut egui::Ui,
    key: &str,
    label: &str,
    refusal: Option<&str>,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) -> egui::Response {
    let response = ui
        .add_enabled(refusal.is_none(), egui::Button::new(label))
        .on_disabled_hover_text(refusal.unwrap_or_default());
    record(controls, key, label, &response);

    if response.clicked() {
        *action = Some(key.to_string());
        ui.close();
    }

    response
}

/// The graph section: a header folding a table of edges; a row click selects both objects.
fn edges(
    ui: &mut egui::Ui,
    model: &Model,
    height: f32,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) {
    let (rect, header) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height + 4.),
        egui::Sense::click(),
    );
    let ink = ui.visuals().text_color();
    let c = rect.left_center() + egui::vec2(8., 0.);
    // the same arrow as a tree row
    let points = if model.graph_open {
        vec![
            c + egui::vec2(-4., -2.),
            c + egui::vec2(4., -2.),
            c + egui::vec2(0., 3.),
        ]
    } else {
        vec![
            c + egui::vec2(-2., -4.),
            c + egui::vec2(-2., 4.),
            c + egui::vec2(3., 0.),
        ]
    };
    ui.painter()
        .add(egui::Shape::convex_polygon(points, ink, egui::Stroke::NONE));
    let plural = if model.edge_total == 1 { "" } else { "s" };
    let title = format!("Graph · {} edge{plural}", model.edge_total);
    ui.painter().text(
        rect.left_center() + egui::vec2(18., 0.),
        egui::Align2::LEFT_CENTER,
        &title,
        egui::TextStyle::Body.resolve(ui.style()),
        ink,
    );
    record(controls, "graph/toggle", &title, &header);

    if header.clicked() {
        *action = Some("graph/toggle".into());
    }

    if !model.graph_open {
        return;
    }

    let half = ui.available_width() / 2.;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );

    for (title, left) in [("From", rect.left()), ("To", rect.left() + half)] {
        ui.painter().text(
            egui::pos2(left + 2., rect.center().y),
            egui::Align2::LEFT_CENTER,
            title,
            egui::TextStyle::Body.resolve(ui.style()),
            egui::Color32::from_gray(110),
        );
    }

    egui::ScrollArea::vertical()
        .id_salt("graph-edges")
        .auto_shrink([false, false])
        .show_rows(ui, height, model.edges.len(), |ui, range| {
            ui.spacing_mut().item_spacing.y = 0.;

            for edge in &model.edges[range] {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), height),
                    egui::Sense::click(),
                );

                if edge.selected {
                    ui.painter().rect_filled(rect, 0., SELECTED);
                }

                // from and to, each truncated to its column
                for (text, left) in [(&edge.from, rect.left()), (&edge.to, rect.left() + half)] {
                    let galley = egui::WidgetText::from(text.as_str()).into_galley(
                        ui,
                        Some(egui::TextWrapMode::Truncate),
                        half - 6.,
                        egui::TextStyle::Body,
                    );
                    let top = rect.center().y - galley.size().y / 2.;
                    ui.painter().galley(
                        egui::pos2(left + 2., top),
                        galley,
                        ui.visuals().text_color(),
                    );
                }

                let response = response.on_hover_text(&edge.guids);
                record(
                    controls,
                    &edge.key,
                    &format!("{} → {}", edge.from, edge.to),
                    &response,
                );

                if response.clicked() {
                    *action = Some(edge.key.clone());
                }
            }
        });
}

/// One icon of a layer row: eye, lock or arrow.
fn layer_icon(ui: &mut egui::Ui, kind: &str, row: &LayerRow, height: f32) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(height + 4., height), egui::Sense::click());
    let scale = height / 18.; // drawn for an 18 pixel row
    let c = rect.center();
    let at = |x: f32, y: f32| c + egui::vec2(x, y) * scale;
    let ink = ui.visuals().text_color();
    let stroke = egui::Stroke::new(1.4_f32, ink);

    if response.hovered() {
        ui.painter()
            .rect_filled(rect.shrink(1.), 3., ui.visuals().widgets.hovered.bg_fill);
    }

    match kind {
        "open" => {
            if let Some(open) = row.expanded {
                let points = if open {
                    vec![at(-4., -2.), at(4., -2.), at(0., 3.)]
                } else {
                    vec![at(-2., -4.), at(-2., 4.), at(3., 0.)]
                };
                ui.painter()
                    .add(egui::Shape::convex_polygon(points, ink, egui::Stroke::NONE));
            }

            response.on_hover_text("Expand or collapse")
        }
        "hide" if row.current => {
            // a check mark instead of the bulb
            ui.painter().add(egui::Shape::line(
                vec![at(-5., 0.), at(-1.5, 4.), at(5., -5.)],
                egui::Stroke::new(2_f32, egui::Color32::BLACK),
            ));

            response.on_hover_text("Current layer: new objects go here")
        }
        "hide" => {
            let fill = if row.hidden {
                egui::Color32::TRANSPARENT
            } else {
                egui::Color32::from_rgb(255, 216, 80)
            };
            ui.painter().circle(at(0., -2.), 4. * scale, fill, stroke);

            for y in [3., 5.5] {
                ui.painter().line_segment([at(-2.5, y), at(2.5, y)], stroke);
            }

            if row.hidden {
                ui.painter()
                    .line_segment([at(-6., 7.), at(6., -8.)], stroke);
            }

            response.on_hover_text(if row.hidden {
                "Show object and children"
            } else {
                "Hide object and children"
            })
        }
        _ => {
            ui.painter().rect(
                egui::Rect::from_center_size(at(0., 2.5), egui::vec2(10., 8.) * scale),
                1.,
                if row.locked {
                    egui::Color32::from_rgb(225, 180, 90)
                } else {
                    egui::Color32::TRANSPARENT
                },
                stroke,
                egui::StrokeKind::Inside,
            );
            let x = if row.locked { 0. } else { 3. };
            ui.painter().add(egui::Shape::line(
                vec![
                    at(-2.5 + x, -1.5),
                    at(-2.5 + x, -5.5),
                    at(2.5 + x, -5.5),
                    at(2.5 + x, -1.5),
                ],
                stroke,
            ));
            response.on_hover_text(if row.locked {
                "Unlock object and children"
            } else {
                "Lock selection of object and children"
            })
        }
    }
}

/// The colour swatches of a layer row.
fn layer_color(
    ui: &mut egui::Ui,
    row: &LayerRow,
    index: &str,
    height: f32,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) {
    let mut color = row.color.unwrap_or([180, 180, 180]);
    // one icon wide and unframed, so the row keeps its width and a selected strip shows through
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(height + 4., height), egui::Sense::click());

    if response.hovered() {
        ui.painter()
            .rect_filled(rect.shrink(1.), 3., ui.visuals().widgets.hovered.bg_fill);
    }

    ui.painter().rect_filled(
        egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(height * 0.5)),
        1.,
        egui::Color32::from_rgb(color[0], color[1], color[2]),
    );
    let menu =
        egui::Popup::menu(&response).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
    menu.show(|ui| {
        let channel_id = egui::Id::new(("layer-color-channel", index));
        let mut edge = ui
            .ctx()
            .data_mut(|data| data.get_temp::<bool>(channel_id).unwrap_or(false))
            && row.has_faces;

        if row.has_faces {
            ui.horizontal(|ui| {
                for (label, value) in [("Faces", false), ("Edges", true)] {
                    let response = ui.selectable_label(edge == value, label);
                    record(
                        controls,
                        &format!("color-channel/{index}/{label}"),
                        label,
                        &response,
                    );

                    if response.clicked() {
                        edge = value;
                    }
                }
            });
        } else {
            ui.label("Object and child colors");
        }

        ui.ctx().data_mut(|data| data.insert_temp(channel_id, edge));
        let channel = if edge { "edge" } else { "face" };
        color = if edge { row.edge_color } else { row.color }.unwrap_or([180; 3]);
        let response = ui
            .button("Original")
            .on_hover_text("Restore the source colors for this channel and its children");
        let key = format!("color/{index}/{channel}/original");
        record(controls, &key, "Original", &response);

        if response.clicked() {
            *action = Some(key);
            ui.close();
        }

        ui.separator();
        for colors in [
            [
                ("Red", [230, 65, 55]),
                ("Orange", [240, 145, 45]),
                ("Yellow", [240, 210, 60]),
            ],
            [
                ("Green", [60, 170, 100]),
                ("Blue", [65, 130, 225]),
                ("Violet", [160, 85, 210]),
            ],
            [
                ("White", [245, 245, 245]),
                ("Gray", [150, 150, 150]),
                ("Black", [35, 35, 35]),
            ],
        ] {
            ui.horizontal(|ui| {
                for (name, rgb) in colors {
                    let response = ui
                        .add_sized(
                            [48., 28.],
                            egui::Button::new(
                                egui::RichText::new("■")
                                    .color(egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2])),
                            ),
                        )
                        .on_hover_text(name);
                    let key = format!(
                        "color/{index}/{channel}/{:02x}{:02x}{:02x}",
                        rgb[0], rgb[1], rgb[2]
                    );
                    record(controls, &key, name, &response);

                    if response.clicked() {
                        *action = Some(key);
                        ui.close();
                    }
                }
            });
        }
        ui.separator();
        let mut changed = false;

        for (channel, value) in ["R", "G", "B"].into_iter().zip(color.iter_mut()) {
            changed |= ui
                .add(egui::Slider::new(value, 0..=255).text(channel))
                .changed();
        }

        if changed {
            *action = Some(format!(
                "color/{index}/{channel}/{:02x}{:02x}{:02x}",
                color[0], color[1], color[2]
            ));
        }
    });
    let response = response.on_hover_text("Change object and child colors");
    record(
        controls,
        &format!("color/{index}"),
        &format!("Color {}", row.label),
        &response,
    );
}

/// The gumball number box beside its handle; Enter hands the text to `typed`, Escape sets `closed`.
fn number_box(
    root: &mut egui::Ui,                 // the panel area
    model: &mut Model,                   // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    typed: &mut Option<String>,
    closed: &mut bool,
) {
    let Some(prompt) = model.number_prompt.as_ref() else {
        model.number_handle = None;
        model.number_rect = None;
        return;
    };
    let id = egui::Id::new("number-input");
    let opened = model.number_handle != Some(prompt.handle);

    // a new box starts empty, with the keys
    if opened {
        model.number_handle = Some(prompt.handle);
        model.number.clear();
        model.number_error.clear();
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let focused = root.memory(|memory| memory.focused());

    // Escape, or another field taking the keys, closes it
    if root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        || focused.is_some_and(|other| other != id)
    {
        *closed = true;
        return;
    }

    // a click in the scene dropped the focus: the box keeps the keys while it is open
    if focused.is_none() {
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let enter = root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let area = egui::Area::new(egui::Id::new("number-box"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(prompt.at[0] + 12.0, prompt.at[1] - 12.0))
        .show(root.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_gray(215)))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
                    ui.horizontal(|ui| {
                        ui.label(&prompt.title);
                        let edit = ui.add_sized(
                            [72.0, 22.0],
                            egui::TextEdit::singleline(&mut model.number)
                                .id(id)
                                .hint_text(prompt.hint)
                                .char_limit(64),
                        );
                        record(controls, "number/input", &prompt.title, &edit);
                        ui.label(prompt.unit);
                    });

                    if !model.number_error.is_empty() {
                        ui.colored_label(egui::Color32::from_rgb(170, 30, 30), &model.number_error);
                    }
                });
        });
    model.number_rect = Some(area.response.rect);

    if enter {
        *typed = Some(model.number.clone());
    }
}

/// The command dock; an executed line goes to `command`.
fn commands(
    root: &mut egui::Ui,                 // the panel area
    model: &mut Model,                   // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    command: &mut Option<String>,
) {
    let previous_popup = model.completion_rect.take();
    let drawing_options = matches!(model.drawing_command.as_str(), "polyline" | "curve");
    // each button row adds this much
    let extra = 28.0 * (usize::from(drawing_options) + usize::from(model.snap_bar)) as f32;
    let panel = if !model.command_expanded {
        egui::Panel::bottom("command-line-collapsed").exact_size(30.0 + extra)
    } else {
        egui::Panel::bottom("command-line")
            .default_size(104.0)
            .resizable(true)
            .size_range((64.0 + extra)..=(root.available_height() * 0.75).max(104.0 + extra))
    };
    let panel_response = panel
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .inner_margin(egui::Margin::symmetric(6, 4)),
        )
        .show_inside(root, |ui| {
            ui.set_min_height(ui.max_rect().height());
            ui.painter().hline(
                ui.max_rect().x_range().expand(6.0),
                ui.max_rect().top() - 4.0,
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(110)),
            );
            ui.set_clip_rect(ui.max_rect().expand(6.0));
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            // the history above the field
            if model.command_expanded {
                egui::ScrollArea::vertical()
                    .id_salt("command-history")
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .min_scrolled_height(0.0)
                    .max_height((ui.available_height() - 38.0 - extra).max(0.0))
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width());
                        for text in &model.history {
                            ui.add(egui::Label::new(text).wrap());
                        }
                        if !model.status.is_empty()
                            && !model
                                .history
                                .back()
                                .is_some_and(|text| text.ends_with(&model.status))
                        {
                            ui.label(&model.status);
                        }
                        if !model.drawing_prompt.is_empty() {
                            let response = ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                            record(controls, "command/hint", &model.drawing_prompt, &response);
                        }
                    });
                // divider between history and the field
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 1.0),
                    egui::Sense::hover(),
                );
                ui.painter().hline(
                    ui.max_rect().x_range().expand(6.0),
                    rect.center().y,
                    egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
                );
            }
            // construction buttons while drawing a polyline or curve
            if drawing_options {
                let polyline = model.drawing_command == "polyline";
                let choices: &[(&str, &str)] = if polyline {
                    &[
                        ("Points", "Polyline Points"),
                        ("Rectangle", "Polyline Rectangle"),
                        ("Polygon", "Polyline Polygon"),
                        ("Close", "Close"),
                        ("Finish", ""),
                    ]
                } else {
                    &[("Close", "Close"), ("Finish", "")]
                };
                ui.horizontal_wrapped(|ui| {
                    for (label, text) in choices {
                        let option = ui.button(*label);
                        record(controls, &format!("command/option/{label}"), label, &option);
                        if option.clicked() {
                            *command = Some((*text).into());
                            model.command.clear();
                            model.completion_visible = false;
                            model.focus_command = true;
                        }
                    }
                });
            }
            ui.horizontal(|ui| {
                // keep clear of the docs corner
                ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                ui.label("Command:");
                let id = egui::Id::new("command-input");
                if model.focus_command || model.command_open {
                    ui.memory_mut(|memory| memory.request_focus(id));
                    if model.focus_command {
                        command_cursor_end(ui.ctx(), id, &model.command);
                        model.focus_command = false;
                    }
                    model.command_open = true;
                }
                // mouse wheel over the field browses the completions
                let wheel = ui.input_mut(|i| {
                    let over = i.pointer.hover_pos().is_some_and(|p| {
                        model.command_rect.is_some_and(|r| r.contains(p))
                            || previous_popup.is_some_and(|r| r.contains(p))
                    });
                    if !over {
                        return 0;
                    }
                    let delta: f32 = i
                        .events
                        .iter()
                        .filter_map(|e| match e {
                            egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                            _ => None,
                        })
                        .sum();
                    i.smooth_scroll_delta = egui::Vec2::ZERO;
                    if delta > 0.0 {
                        -1
                    } else if delta < 0.0 {
                        1
                    } else {
                        0
                    }
                });
                if wheel != 0 {
                    ui.memory_mut(|memory| memory.request_focus(id));
                    model.command_open = true;
                }
                let has_focus = ui.memory(|memory| memory.has_focus(id));
                // wheel or arrow keys: -1 up, +1 down
                let browse = if wheel != 0 {
                    wheel
                } else if has_focus {
                    ui.input_mut(|i| {
                        if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                            1
                        } else if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                            -1
                        } else {
                            0
                        }
                    })
                } else {
                    0
                };
                let enter = has_focus
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)); // run the line
                let tab = has_focus
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)); // accept the completion
                let deletes = ui.input(|i| {
                    i.key_pressed(egui::Key::Backspace) || i.key_pressed(egui::Key::Delete)
                });
                // space accepts the completion
                if model.inline_suffix
                    && has_focus
                    && ui.input(|i| {
                        i.events
                            .iter()
                            .find_map(|event| match event {
                                egui::Event::Text(text) => Some(text),
                                _ => None,
                            })
                            .is_some_and(|text| text.starts_with(' '))
                    })
                {
                    command_cursor_end(ui.ctx(), id, &model.command);
                    model.inline_suffix = false;
                }
                let option_prefix = if model.inline_suffix {
                    &model.completion_prefix
                } else {
                    &model.command
                };
                let inline_options = if option_prefix.contains(' ')
                    && !option_prefix
                        .trim_start()
                        .to_ascii_lowercase()
                        .starts_with("polyline")
                {
                    crate::app::command::options(option_prefix)
                } else {
                    &[]
                };
                let option_width: f32 = inline_options
                    .iter()
                    .map(|name| {
                        let label = name.split_once(' ').map_or(*name, |(_, option)| option);
                        ui.painter()
                            .layout_no_wrap(
                                label.into(),
                                egui::FontId::proportional(14.0),
                                egui::Color32::BLACK,
                            )
                            .size()
                            .x
                            + 16.0
                    })
                    .sum();
                // options before the field
                for name in inline_options {
                    let label = name.split_once(' ').map_or(*name, |(_, option)| option);
                    let selected = model.command.trim().eq_ignore_ascii_case(name)
                        || (model.command.ends_with(' ')
                            && model.command.split_whitespace().count() == 1
                            && Some(name) == inline_options.first());
                    let option = ui.selectable_label(selected, label);
                    record(controls, &format!("command/option/{label}"), label, &option);
                    if option.clicked() {
                        let (text, run) = crate::app::command::accept(name);
                        if run {
                            *command = Some(text);
                            model.command.clear();
                        } else {
                            model.command = text;
                        }
                        model.completion_visible = false;
                        model.inline_suffix = false;
                        model.focus_command = true;
                    }
                }
                let response = ui.add_sized(
                    [(ui.available_width() - option_width - 28.0).max(40.0), 22.0],
                    egui::TextEdit::singleline(&mut model.command)
                        .id(id)
                        .font(egui::FontId::proportional(14.0))
                        .vertical_align(egui::Align::Center)
                        .frame(egui::Frame::NONE)
                        .clip_text(true)
                        .char_limit(2048)
                        .hint_text("Type a command"),
                );
                // caret visible on an empty field
                if model.command_open && model.command.is_empty() {
                    let y = response.rect.center().y;
                    ui.painter().vline(
                        response.rect.left(),
                        y - 7.0..=y + 7.0,
                        ui.visuals().text_cursor.stroke,
                    );
                }
                model.command_rect = Some(response.rect);
                record(controls, "command/input", "Command", &response);
                let focused = model.command_open || response.has_focus() || response.lost_focus();
                if response.gained_focus() {
                    model.command_open = true;
                }
                let agent_edit = model.agent_edit.take();
                // the : that opened the line is not part of the command
                if (response.changed() || agent_edit.is_some()) && model.command.starts_with(':') {
                    model.command.remove(0);
                }
                let deletes = deletes || agent_edit == Some(true);
                if response.changed() || agent_edit.is_some() {
                    model.completion = 0;
                    model.completion_visible = !model.command.is_empty();
                    model.completion_prefix.clone_from(&model.command);
                    model.inline_suffix = false;
                    // complete only when typing at the end
                    let at_end = egui::TextEdit::load_state(ui.ctx(), id)
                        .and_then(|state| state.cursor.char_range())
                        .is_some_and(|range| {
                            range.is_empty() && range.primary.index == model.command.chars().count()
                        });
                    if !deletes
                        && at_end
                        && !model.command.is_empty()
                        && !model.command.ends_with(' ')
                        && let Some(name) = crate::app::command::completions(&model.command).first()
                    {
                        let prefix = model.command.chars().count();
                        if name.chars().count() > prefix {
                            model.command = (*name).into();
                            command_cursor_select(
                                ui.ctx(),
                                id,
                                prefix,
                                model.command.chars().count(),
                            );
                            model.inline_suffix = true;
                            ui.ctx().request_repaint();
                        }
                    }
                } else if !model.inline_suffix {
                    model.completion_prefix.clone_from(&model.command);
                }
                // the completion list
                let choices = crate::app::command::browse(&model.completion_prefix);
                let mut complete = None; // completion chosen this frame
                let opening_list = !model.completion_visible;
                if browse != 0 || tab {
                    model.completion_visible = true;
                }
                if focused && model.completion_visible && !choices.is_empty() {
                    model.completion = model.completion.min(choices.len() - 1);
                    if browse != 0 {
                        model.completion = if opening_list && !model.completion_prefix.contains(' ')
                        {
                            if browse < 0 { choices.len() - 1 } else { 0 }
                        } else {
                            (model.completion as isize + browse).rem_euclid(choices.len() as isize)
                                as usize
                        };
                        model.command = choices[model.completion].into();
                        command_cursor_select(
                            ui.ctx(),
                            id,
                            if model
                                .command
                                .to_ascii_lowercase()
                                .starts_with(&model.completion_prefix.to_ascii_lowercase())
                            {
                                model.completion_prefix.chars().count()
                            } else {
                                0
                            },
                            model.command.chars().count(),
                        );
                        model.inline_suffix = true;
                        ui.ctx().request_repaint();
                    }
                    if tab {
                        complete = Some(choices[model.completion]);
                    }
                    if !model.completion_prefix.contains(' ') {
                        let popup_width =
                            (ui.ctx().content_rect().right() - response.rect.left() - 12.0)
                                .clamp(60.0, 220.0);
                        let popup_height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
                        let popup = egui::Area::new(egui::Id::new("command-completions"))
                            .pivot(egui::Align2::LEFT_BOTTOM)
                            .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
                            .order(egui::Order::Foreground)
                            .show(ui.ctx(), |ui| {
                                egui::Frame::new()
                                    .fill(egui::Color32::WHITE)
                                    .stroke(egui::Stroke::new(
                                        1.0_f32,
                                        egui::Color32::from_gray(215),
                                    ))
                                    .inner_margin(5)
                                    .show(ui, |ui| {
                                        ui.set_width(popup_width);
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                                        egui::ScrollArea::vertical()
                                            .id_salt("command-choices")
                                            .max_height(popup_height)
                                            .show(ui, |ui| {
                                                for (index, name) in choices.iter().enumerate() {
                                                    let item = ui.selectable_label(
                                                        index == model.completion,
                                                        *name,
                                                    );
                                                    record(
                                                        controls,
                                                        &format!("command/completion/{name}"),
                                                        name,
                                                        &item,
                                                    );
                                                    if index == model.completion && browse != 0 {
                                                        item.scroll_to_me(None);
                                                    }
                                                    if item.clicked() {
                                                        complete = Some(*name);
                                                    }
                                                }
                                            });
                                    });
                            });
                        model.completion_rect = Some(popup.response.rect);
                    }
                }
                // a chosen completion fills the field, maybe runs it
                if let Some(name) = complete {
                    let (text, run) = crate::app::command::accept(name);
                    if run && !tab {
                        *command = Some(text);
                        model.command.clear();
                    } else {
                        model.command = if run { format!("{text} ") } else { text };
                    }
                    model.completion_visible = false;
                    model.inline_suffix = false;
                    model.focus_command = true;
                }
                // Enter runs the line
                if enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
                    let (text, run) =
                        crate::app::command::accept(&std::mem::take(&mut model.command));
                    if run {
                        *command = Some(text);
                    } else {
                        model.command = text;
                    }
                    model.completion_visible = false;
                    model.inline_suffix = false;
                    model.focus_command = true;
                }
                // Escape clears the field, unless it closes a layer menu or cancels a rename
                if ui.input(|i| i.key_pressed(egui::Key::Escape))
                    && !model.menu_open
                    && model.renaming.is_none()
                {
                    model.command.clear();
                    model.completion_visible = false;
                    model.inline_suffix = false;
                    model.command_open = false;
                    model.focus_command = false;
                    response.surrender_focus();
                    *command = Some("Escape".into());
                    crate::app::feedback::focus_canvas();
                }
                // the +/− button folds the history
                let collapse = ui
                    .button(if model.command_expanded { "−" } else { "+" })
                    .on_hover_text("Collapse or expand history");
                record(
                    controls,
                    "command/collapse",
                    "Collapse or expand history",
                    &collapse,
                );
                if collapse.clicked() {
                    model.command_expanded = !model.command_expanded;
                    ui.ctx().request_repaint();
                }
            });
            // one toggle per snap kind, under the field
            if model.snap_bar {
                ui.horizontal_wrapped(|ui| {
                    for (label, bit) in crate::app::snap::MODES {
                        let toggle = ui.selectable_label(model.snap_modes & bit != 0, label);
                        record(controls, &format!("snap/{label}"), label, &toggle);
                        if toggle.clicked() {
                            *command = Some(format!("Snap {label}"));
                            model.focus_command = true;
                        }
                    }
                });
            }
        });
    if let Some(controls) = controls {
        let rect = panel_response.response.rect;
        controls.push(Control {
            key: "command/resize".into(),
            label: "Drag to resize command history".into(),
            rect: [
                rect.left(),
                rect.top() - 4.0,
                rect.right(),
                rect.top() + 4.0,
            ],
        });
    }
}

/// Put the caret at the end of the field.
fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// Select `start..end` in the field.
fn command_cursor_select(context: &egui::Context, id: egui::Id, start: usize, end: usize) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}
