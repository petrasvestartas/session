use crate::State;
use crate::app::feedback::LayerRow;
use std::cell::RefCell;
use std::collections::VecDeque;
use winit::window::Window;

pub struct Model {
    pub layers_open: bool,                          // layers panel shown
    pub rows: Vec<LayerRow>,                        // its rows
    pub command_open: bool,                         // command line shown
    pub command: String,                            // text in the command field
    pub focus_command: bool,                        // give the field focus next frame
    pub status: String,                             // status line text
    history: VecDeque<String>,                      // past commands and answers
}

// --8<-- [start:step-11b]
impl Default for Model {
    /// The panel state at start.
    fn default() -> Self {
        Self {
            layers_open: true, // layers panel shown
            rows: Vec::new(), // no layer rows yet
            command_open: false, // command line hidden
            command: String::new(), // nothing typed yet
            focus_command: false, // no focus request
            status: String::new(), // no status text
            history: VecDeque::new(), // no past commands
        }
    }
}

// --8<-- [end:step-11b]
thread_local! { pub static MODEL: RefCell<Model> = RefCell::default(); } // the one model

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    key: String,    // what it does
    label: String,  // text shown
    rect: [f32; 4], // left, top, right, bottom
}

/// The egui interface over the canvas.
pub struct Ui {
    context: egui::Context,                    // egui state
    input: egui_winit::State,                  // winit events into egui
    controls: Option<Vec<Control>>,            // controls drawn this frame, when inspecting
    // --8<-- [start:step-11c]
    scene_rect: egui::Rect,                    // canvas area not covered by panels
    pointer: egui::Pos2,                       // last pointer position
    ui_drag: bool,                             // a drag started on a panel
    touches: std::collections::HashSet<u64>,   // fingers on panels
}

impl Ui {
    /// Create the egui state for a window.
    pub fn new(window: &Window, logical_width: f64) -> Self {
        MODEL.with_borrow_mut(|model| {
            model.layers_open = logical_width >= 700.0;
        });
        // --8<-- [end:step-11c]
        let context = egui::Context::default();
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
            controls: (super::route::query("inspect").as_deref() == Some("1")).then(Vec::new),
            // --8<-- [start:step-11d]
            scene_rect: egui::Rect::EVERYTHING,
            pointer: egui::Pos2::ZERO,
            ui_drag: false,
            touches: std::collections::HashSet::new(),
            // --8<-- [end:step-11d]
        }
    }

    /// Offer one event to the panels; (consumed, needs repaint).
    pub fn event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> (bool, bool) {
        let response = self.input.on_window_event(window, event);
        let escape = matches!(event, winit::event::WindowEvent::KeyboardInput { event, .. }
            if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape))
            && MODEL.with_borrow(|model| model.command_open);
        // --8<-- [start:step-11e]
        // use the current pointer, not last frame's hover
        use winit::event::{ElementState, TouchPhase, WindowEvent};
        let ratio = window.scale_factor() as f32;
        let mut consumed = response.consumed;

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = egui::pos2(position.x as f32 / ratio, position.y as f32 / ratio);
                consumed = self.ui_drag || !self.scene_rect.contains(self.pointer);
            }
            WindowEvent::MouseInput { state, .. } => {
                if *state == ElementState::Pressed {
                    self.ui_drag = !self.scene_rect.contains(self.pointer);
                }

                consumed = self.ui_drag;

                if *state == ElementState::Released {
                    self.ui_drag = false;
                }
            }
            WindowEvent::MouseWheel { .. } => consumed = !self.scene_rect.contains(self.pointer),
            WindowEvent::Touch(touch) => {
                self.pointer = egui::pos2(
                    touch.location.x as f32 / ratio,
                    touch.location.y as f32 / ratio,
                );

                if touch.phase == TouchPhase::Started {
                    if self.touches.is_empty() {
                        self.ui_drag = !self.scene_rect.contains(self.pointer);
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

        (consumed || escape, response.repaint || escape)
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
        // --8<-- [end:step-11e]

        if let Some(controls) = self.controls.as_mut() {
            controls.clear();
        }

        let mut action = None;
        let mut command = None;
        // --8<-- [start:step-11f]
        let mut tool = None;
        let mut output = self.context.run_ui(input, |root| {
            MODEL.with_borrow_mut(|model| {
                commands(root, model, &mut self.controls, &mut command);
                toolbar(root, model, &mut self.controls, &mut tool);
                layers(root, model, &mut self.controls, &mut action);
            });
            self.scene_rect = root.available_rect_before_wrap();
        });
        self.input
            .handle_platform_output(&state.window, std::mem::take(&mut output.platform_output));
        let changed = action.is_some() || command.is_some() || tool.is_some();

        if let Some(tool) = tool {
            match tool {
                "layers" => state.toggle_layers_panel(),
                "controls" => {
                    state.selection_tool = crate::app::selection::SelectionTool::Object;
                    state.enable_controls();
                }
                "object" | "edge" | "face" => {
                    state.escape_selection();
                    state.selection_tool = match tool {
                        "edge" => crate::app::selection::SelectionTool::Edge,
                        "face" => crate::app::selection::SelectionTool::Face,
                        _ => crate::app::selection::SelectionTool::Object,
                    };
                }
                _ => command = Some(tool.to_string()),
            }

            state.touch();
        }
        // --8<-- [end:step-11f]

        if let Some(key) = action {
            state.panel_action(&key);
        }

        if let Some(text) = command {
            let message = state.run_command(&text).unwrap_or_else(|error| error);
            crate::app::feedback::status(&message);
            MODEL.with_borrow_mut(|model| {
                if model.history.len() == 8 {
                    model.history.pop_front();
                }

                model.history.push_back(format!("> {text}\n{message}"));
            });
            state.touch();
        }

        self.publish();
        let repaint = changed || self.context.has_requested_repaint();
        // --8<-- [start:step-11g]
        output.pixels_per_point = state.gpu.config.width as f32 / logical[0].max(1.0) as f32;
        // --8<-- [end:step-11g]

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
            let snapshot = MODEL.with_borrow(|model| serde_json::json!({"framework": "egui 0.34.3", "controls": self.controls, "command_open": model.command_open, "layers_open": model.layers_open, "command": model.command, "history": model.history}));
            let _ = canvas.set_attribute("data-viewer-ui", &snapshot.to_string());
        }

        if let Some(status) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("viewer-status"))
        {
            // --8<-- [start:step-11h]
            let _ = status.set_attribute("hidden", "");
            // --8<-- [end:step-11h]
        }
    }
}

/// The white theme.
fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.window_fill = egui::Color32::WHITE;
    // --8<-- [start:step-11i]
    visuals.panel_fill = egui::Color32::from_gray(247);
    // --8<-- [end:step-11i]
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::BLACK;
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);
    visuals.indent_has_left_vline = false;

    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        widget.bg_fill = egui::Color32::WHITE;
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
    // --8<-- [start:step-11j]
    root: &mut egui::Ui, // the panel area
    // --8<-- [end:step-11j]
    model: &mut Model, // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) {
    if !model.layers_open {
        return;
    }

    // --8<-- [start:step-11k]
    let width = (root.available_width() * 0.25).clamp(180.0, 310.0);
    egui::Panel::right("session-layers")
        .default_size(width)
        .size_range(160.0..=360.0)
        .resizable(true)
        .show_inside(root, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Layers");
                let close = ui.button("Close");
                record(controls, "layers/close", "Close layers", &close);

                if close.clicked() {
                    model.layers_open = false;
                }
            });
            ui.separator();
            // one line per row
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                // --8<-- [end:step-11k]
                .show(ui, |ui| {
                    let mut at = 0;

                    while at < model.rows.len() {
                        let start = at;
                        let id = model.rows[at].key.split_once('/').map(|(_, id)| id);
                        at += 1;

                        if id.is_some() {
                            while at < model.rows.len()
                                && model.rows[at].key.split_once('/').map(|(_, id)| id) == id
                            {
                                at += 1;
                            }
                        }

                        ui.horizontal(|ui| {
                            let first = &model.rows[start];
                            let indent = first.label.len() - first.label.trim_start().len();
                            ui.add_space(indent as f32 * 4.0);

                            for row in &model.rows[start..at] {
                                let response = layer_button(ui, row);
                                record(controls, &row.key, &row.label, &response);

                                if response.clicked() {
                                    *action = Some(row.key.clone());
                                }
                            }
                        });
                    }
                });
        });
}

/// One layer row as a button.
fn layer_button(ui: &mut egui::Ui, row: &LayerRow) -> egui::Response {
    let label = row.label.trim_start();

    if row.key.starts_with("open/") {
        // --8<-- [start:step-11l]
        let (rect, response) = ui.allocate_exact_size(egui::vec2(20.0, 28.0), egui::Sense::click());
        // --8<-- [end:step-11l]
        let c = rect.center();
        let points = if label.starts_with('▾') {
            vec![
                c + egui::vec2(-4.0, -2.0),
                c + egui::vec2(4.0, -2.0),
                c + egui::vec2(0.0, 3.0),
            ]
        } else {
            vec![
                c + egui::vec2(-2.0, -4.0),
                c + egui::vec2(-2.0, 4.0),
                c + egui::vec2(3.0, 0.0),
            ]
        };
        ui.painter().add(egui::Shape::convex_polygon(
            points,
            egui::Color32::BLACK,
            egui::Stroke::NONE,
        ));
        return response;
    }

    let label: String = label.chars().take(160).collect();
    let text = if row.key.starts_with("hide/") {
        if row.hidden {
            "Show".to_string()
        } else {
            "Hide".to_string()
        }
    } else {
        format!("{} ({})", label.trim_start_matches("Select "), row.count)
    };
    // --8<-- [start:step-11m]
    let width = if row.key.starts_with("hide/") {
        44.0
    } else {
        (ui.available_width()
            - if row.key.starts_with("select/") {
                52.0
            } else {
                0.0
            })
        .max(40.0)
    };
    ui.add_sized([width, 28.0], egui::Button::new(text).truncate())
        .on_hover_text(label)
}

/// The command dock; an executed line goes to `command`.
fn commands(
    root: &mut egui::Ui, // the panel area
    model: &mut Model, // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    command: &mut Option<String>,
) {
    let height = if model.command_open { 160.0 } else { 76.0 };
    egui::Panel::bottom("command-line")
        .default_size(height)
        .size_range(76.0..=260.0)
        .resizable(true)
        .show_inside(root, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("command-history")
                .stick_to_bottom(true)
                .max_height((ui.available_height() - 40.0).max(20.0))
                .show(ui, |ui| {
                    for text in &model.history {
                        ui.label(text);
                    }

                    if !model.status.is_empty() {
                        ui.label(&model.status);
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.strong("Command:");
                let width = (ui.available_width() - 100.0).max(40.0);
                let response = ui.add_sized(
                    [width, 28.0],
                    egui::TextEdit::singleline(&mut model.command)
                        .char_limit(2048)
                        .hint_text("Type a command"),
                );
                record(controls, "command/input", "Command", &response);

                if model.focus_command {
                    response.request_focus();
                    model.focus_command = false;
                }

                if response.gained_focus() {
                    model.command_open = true;
                }

                let enter =
                    response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                let run = ui.add_sized([40.0, 28.0], egui::Button::new("Run"));
                record(controls, "command/run", "Run", &run);

                if (enter || run.clicked()) && !model.command.trim().is_empty() {
                    *command = Some(std::mem::take(&mut model.command));
                    model.focus_command = true;
                }

                let close = ui.add_sized([40.0, 28.0], egui::Button::new("Esc"));
                record(controls, "command/close", "Close", &close);

                if close.clicked() || ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                    model.command_open = false;
                    response.surrender_focus();
                    crate::app::feedback::focus_canvas();
                }
            });
        });
}

/// Add a button by adding its label, tooltip and command to this table.
const TOOLBAR: &[(&str, &str, &str)] = &[
    ("Obj", "Select objects", "object"),
    (
        "Vtx",
        "Show and edit source vertices / control points",
        "controls",
    ),
    ("Edge", "Select source edges", "edge"),
    ("Face", "Select source faces", "face"),
    ("Fit", "Fit selection or scene", "fit"),
    ("Layer", "Show or hide layers", "layers"),
    ("+Pt", "Create a point", "point 0,0,0"),
    ("+Ln", "Create a line", "line 0,0,0 100,0,0"),
    (
        "+Poly",
        "Create a polyline",
        "polyline 0,0,0 100,0,0 100,100,0",
    ),
    ("Undo", "Undo the last edit", "undo"),
    ("Redo", "Redo the last edit", "redo"),
    ("Save", "Save the whole session", "save"),
    ("Open", "Open a saved session", "open"),
];

/// The button row above the command field.
fn toolbar(
    root: &mut egui::Ui, // the panel area
    model: &mut Model, // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<&'static str>, // the button pressed, if any
) {
    egui::Panel::left("tools")
        .exact_size(58.0)
        .resizable(false)
        .show_inside(root, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for &(label, help, command) in TOOLBAR {
                    let response = ui
                        .add_sized([44.0, 44.0], egui::Button::new(label))
                        .on_hover_text(help);
                    record(controls, &format!("toolbar/{label}"), help, &response);

                    if response.clicked() {
                        if command.contains(' ') {
                            model.command = command.to_string();
                            model.command_open = true;
                            model.focus_command = true;
                        } else {
                            *action = Some(command);
                        }
                    }
                }
            });
            // --8<-- [end:step-11m]
        });
}
