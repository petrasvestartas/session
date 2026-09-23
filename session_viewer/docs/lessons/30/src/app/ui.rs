//! egui is immediate mode: the interface is rebuilt from this state every frame, so no widget object is kept in sync.

use crate::State;
use crate::app::feedback::LayerRow;
use std::cell::RefCell;
use std::collections::VecDeque;
use winit::window::Window;

// Every panel reads and writes this one struct, because an immediate-mode frame keeps no widget state of its own.
pub struct Model {
    pub layers_open: bool,                          // layers panel shown
    pub rows: Vec<LayerRow>,                        // its rows
    pub command_open: bool,                         // command line shown
    pub command: String,                            // text in the command field
    pub focus_command: bool,                        // give the field focus next frame
    pub status: String,                             // status line text
    history: VecDeque<String>,                      // past commands and answers
}

impl Default for Model {
    /// The panel state at start.
    fn default() -> Self {
        Self {
            layers_open: true,
            rows: Vec::new(),
            command_open: false,
            command: String::new(),
            focus_command: false,
            status: String::new(),
            history: VecDeque::new(),
        }
    }
}

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
            scene_rect: egui::Rect::EVERYTHING,
            pointer: egui::Pos2::ZERO,
            ui_drag: false,
            touches: std::collections::HashSet::new(),
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

        if let Some(controls) = self.controls.as_mut() {
            controls.clear();
        }

        let mut action = None;
        let mut command = None;
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
            // --8<-- [start:step-11a]
            let snapshot = MODEL.with_borrow(|model| serde_json::json!({"framework": "egui 0.34.3", "controls": self.controls, "command_open": model.command_open, "layers_open": model.layers_open, "command": model.command, "history": model.history, "hint": crate::app::command::hint(&model.command)}));
            // --8<-- [end:step-11a]
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
    visuals.panel_fill = egui::Color32::from_gray(247);
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
    root: &mut egui::Ui,
    model: &mut Model,
    controls: &mut Option<Vec<Control>>,
    action: &mut Option<String>,
) {
    if !model.layers_open {
        return;
    }

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
                .show(ui, |ui| {
                    // --8<-- [start:step-11b]
                    for row in &model.rows {
                        ui.horizontal(|ui| {
                            if let Some((_, index)) = row.key.split_once('/')
                                && row.key.starts_with("select/")
                            {
                                ui.spacing_mut().item_spacing.x = 2.;
                                ui.add_space(row.depth.min(8) as f32 * 10.);
                                let response = layer_icon(ui, "open", row);
                                record(controls, &format!("open/{index}"), &row.label, &response);

                                if response.clicked() && row.expanded.is_some() {
                                    *action = Some(format!("open/{index}"));
                                }

                                let width = (ui.available_width() - 86.).max(24.);
                                let response = ui
                                    .add_sized(
                                        [width, 28.],
                                        egui::Button::new(&row.label).frame(false).truncate(),
                                    )
                                    .on_hover_text(format!(
                                        "{} · {} objects",
                                        row.label, row.count
                                    ));
                                record(
                                    controls,
                                    &row.key,
                                    &format!("Select {}", row.label),
                                    &response,
                                );

                                if response.clicked() {
                                    *action = Some(row.key.clone());
                                }

                                for kind in ["hide", "lock"] {
                                    let response = layer_icon(ui, kind, row);
                                    record(
                                        controls,
                                        &format!("{kind}/{index}"),
                                        &format!(
                                            "{} {}",
                                            if kind == "hide" {
                                                if row.hidden { "Show" } else { "Hide" }
                                            } else if row.locked {
                                                "Unlock"
                                            } else {
                                                "Lock"
                                            },
                                            row.label
                                        ),
                                        &response,
                                    );

                                    if response.clicked() {
                                        *action = Some(format!("{kind}/{index}"));
                                    }
                                }

                                layer_color(ui, row, index, controls, action);
                            } else {
                                let response = ui.button(&row.label);
                                // --8<-- [end:step-11b]
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

// --8<-- [start:step-11c]
/// One icon of a layer row: eye, lock or arrow.
fn layer_icon(ui: &mut egui::Ui, kind: &str, row: &LayerRow) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(26., 28.), egui::Sense::click());
    let c = rect.center();
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
            }

            response.on_hover_text("Expand or collapse")
        }
        "hide" => {
            let fill = if row.hidden {
                egui::Color32::TRANSPARENT
            } else {
                egui::Color32::from_rgb(255, 216, 80)
            };
            ui.painter()
                .circle(c + egui::vec2(0., -3.), 5., fill, stroke);

            for y in [3., 6.] {
                ui.painter()
                    .line_segment([c + egui::vec2(-3., y), c + egui::vec2(3., y)], stroke);
            }

            if row.hidden {
                ui.painter()
                    .line_segment([c + egui::vec2(-7., 8.), c + egui::vec2(7., -9.)], stroke);
            }

            response.on_hover_text(if row.hidden {
                "Show object and children"
            } else {
                "Hide object and children"
            })
        }
        _ => {
            ui.painter().rect(
                egui::Rect::from_center_size(c + egui::vec2(0., 3.), egui::vec2(11., 9.)),
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
                    c + egui::vec2(-3. + x, -1.),
                    c + egui::vec2(-3. + x, -6.),
                    c + egui::vec2(3. + x, -6.),
                    c + egui::vec2(3. + x, -1.),
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
    controls: &mut Option<Vec<Control>>,
    action: &mut Option<String>,
) {
    let mut color = row.color.unwrap_or([180, 180, 180]);
    let response = ui
        .menu_button(
            egui::RichText::new("■").color(egui::Color32::from_rgb(color[0], color[1], color[2])),
            |ui| {
                ui.label("Object and child colors");
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
                            let key =
                                format!("color/{index}/{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
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
                        "color/{index}/{:02x}{:02x}{:02x}",
                        color[0], color[1], color[2]
                    ));
                }
            },
        )
        .response
        .on_hover_text("Change object and child colors");
    record(
        controls,
        &format!("color/{index}"),
        &format!("Color {}", row.label),
        &response,
    );
    // --8<-- [end:step-11c]
}

/// The command dock; an executed line goes to `command`.
fn commands(
    root: &mut egui::Ui,
    model: &mut Model,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
) {
    // --8<-- [start:step-11d]
    let height = if model.command_open { 160.0 } else { 104.0 };
    egui::Panel::bottom("command-line")
        .default_size(height)
        .size_range(104.0..=260.0)
        .resizable(true)
        .show_inside(root, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("command-history")
                .stick_to_bottom(true)
                .max_height((ui.available_height() - 68.0).max(20.0))
                // --8<-- [end:step-11d]
                .show(ui, |ui| {
                    for text in &model.history {
                        ui.label(text);
                    }

                    if !model.status.is_empty() {
                        ui.label(&model.status);
                    }
                });
            ui.separator();
            // --8<-- [start:step-11e]
            ui.small(crate::app::command::hint(&model.command));
            // --8<-- [end:step-11e]
            ui.horizontal(|ui| {
                ui.strong("Command:");
                let width = (ui.available_width() - 100.0).max(40.0);
                let response = ui.add_sized(
                    [width, 28.0],
                    egui::TextEdit::singleline(&mut model.command)
                        .char_limit(2048)
                        // --8<-- [start:step-11f]
                        .hint_text("Point 0,0,0"),
                        // --8<-- [end:step-11f]
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
    root: &mut egui::Ui,
    model: &mut Model,
    controls: &mut Option<Vec<Control>>,
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
        });
}
