//! egui is immediate mode: the interface is rebuilt from this state every frame, so no widget object is kept in sync.

use crate::State;
use crate::app::feedback::LayerRow;
use std::cell::RefCell;
use std::collections::VecDeque;
use winit::window::Window;

// --8<-- [start:step-10a]
// Every panel reads and writes this one struct, because an immediate-mode frame keeps no widget state of its own.
#[derive(Default)]
pub struct Model {
    pub layers_open: bool,                          // layers panel shown
    pub rows: Vec<LayerRow>,                        // its rows
    pub command_open: bool,                         // command line shown
    pub command: String,                            // text in the command field
    pub drawing_prompt: String,
    // --8<-- [start:step-3a]
    drawing_command: String,                        // the drawing verb, e.g. polyline
    // --8<-- [end:step-3a]
    pub focus_command: bool,                        // give the field focus next frame
    pub status: String,                             // status line text
    history: VecDeque<String>,                      // past commands and answers
    command_collapsed: bool,
    layers_collapsed: bool,
    completion: usize,                              // highlighted completion index
    completion_prefix: String,
    inline_suffix: bool,                            // completion suffix shown in the field
    completion_visible: bool,
    // --8<-- [start:step-13a]
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    // --8<-- [end:step-10a]
    // --8<-- [end:step-13a]
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
    // --8<-- [start:step-13b]
    #[cfg(target_arch = "wasm32")]
    agent_value: String,
    // --8<-- [end:step-13b]
}

impl Ui {
    // --8<-- [start:step-10b]
    /// Set up egui with the light theme.
    pub fn new(window: &Window, _logical_width: f64) -> Self {
        let context = egui::Context::default();
        // one layout pass, so text events are never replayed
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        context.set_theme(egui::Theme::Light);
        context.set_visuals(visuals());
        MODEL.with_borrow_mut(|model| model.focus_command = true);
        // --8<-- [end:step-10b]
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
            // --8<-- [start:step-13c]
            #[cfg(target_arch = "wasm32")]
            agent_value: String::new(),
            // --8<-- [end:step-13c]
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
        // --8<-- [start:step-3b]
        let mut scene_rect = self.scene_rect;
        scene_rect.max.y -= 5.0;
        // --8<-- [start:step-10c]
        // --8<-- [end:step-3b]
        let in_popup =
            |point| MODEL.with_borrow(|m| m.completion_rect.is_some_and(|r| r.contains(point)));

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = egui::pos2(position.x as f32 / ratio, position.y as f32 / ratio);
                // --8<-- [start:step-3c]
                consumed =
                    self.ui_drag || in_popup(self.pointer) || !scene_rect.contains(self.pointer);
            }
            WindowEvent::MouseInput { state, .. } => {
                if *state == ElementState::Pressed {
                    self.ui_drag = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                    // focus now so the first key is not lost
                // --8<-- [end:step-3c]
                    let input = MODEL
                        .with_borrow(|m| m.command_rect.is_some_and(|r| r.contains(self.pointer)));
                    if input {
                        self.context.memory_mut(|memory| {
                            memory.request_focus(egui::Id::new("command-input"))
                        });
                        MODEL.with_borrow_mut(|model| model.command_open = true);
                    } else if !in_popup(self.pointer) {
                        self.context.memory_mut(|memory| {
                            memory.surrender_focus(egui::Id::new("command-input"))
                        });
                        MODEL.with_borrow_mut(|model| model.command_open = false);
                    }
                    // --8<-- [end:step-10c]
                }

                consumed = self.ui_drag;

                if *state == ElementState::Released {
                    self.ui_drag = false;
                }
            }
            // --8<-- [start:step-10d]
            WindowEvent::MouseWheel { .. } => {
                // --8<-- [start:step-3d]
                consumed = in_popup(self.pointer) || !scene_rect.contains(self.pointer)
                // --8<-- [end:step-3d]
            }
            // --8<-- [end:step-10d]
            WindowEvent::Touch(touch) => {
                self.pointer = egui::pos2(
                    touch.location.x as f32 / ratio,
                    touch.location.y as f32 / ratio,
                );

                if touch.phase == TouchPhase::Started {
                        // --8<-- [start:step-10e]
                    if self.touches.is_empty() {
                        // --8<-- [start:step-3e]
                        self.ui_drag = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                            // --8<-- [end:step-10e]
                        // --8<-- [end:step-3e]
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

        // --8<-- [start:step-10f]
        if matches!(event, WindowEvent::KeyboardInput { .. })
            && MODEL.with_borrow(|model| model.command_open)
        {
            consumed = true;
        }
        // --8<-- [end:step-10f]
        (consumed || escape, response.repaint || escape)
    // --8<-- [start:step-13d]
    }

    /// Feed the hidden input's typing into the field; returns keys for the viewport.
    #[cfg(target_arch = "wasm32")]
    pub fn agent(&mut self, event: super::agent::AgentEvent) {
        use super::agent::AgentEvent;
        let id = egui::Id::new("command-input");
        let key = |key, pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let events = &mut self.input.egui_input_mut().events;

        match event {
            AgentEvent::Text(value) => {
                let shared = self
                    .agent_value
                    .chars()
                    .zip(value.chars())
                    .take_while(|(a, b)| a == b)
                    .count();

                for _ in shared..self.agent_value.chars().count() {
                    events.push(key(egui::Key::Backspace, true));
                    events.push(key(egui::Key::Backspace, false));
                }

                let added: String = value.chars().skip(shared).collect();

                if !added.is_empty() {
                    events.push(egui::Event::Text(added));
                }

                self.agent_value = value;
                self.context.memory_mut(|memory| memory.request_focus(id));
                MODEL.with_borrow_mut(|model| model.command_open = true);
            }
            AgentEvent::Key(k) => {
                events.push(key(k, true));
                events.push(key(k, false));
            }
        }
    // --8<-- [end:step-13d]
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
        // --8<-- [start:step-3f]
        MODEL.with_borrow_mut(|model| {
            model.drawing_prompt = state.drawing_prompt();
            model.drawing_command = state.drawing_verb().to_owned();
        // --8<-- [start:step-10g]
        });
        let drawing = state.drawing_overlay();
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
        // --8<-- [end:step-3f]
        let mut draw = |root: &mut egui::Ui| {
            if let Some(controls) = self.controls.as_mut() {
                controls.clear();
            }
            MODEL.with_borrow_mut(|model| {
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
        // --8<-- [start:step-3g]
        let mut batches = batches.into_iter();
        let mut output = self.context.run_ui(batches.next().unwrap(), &mut draw);
        for batch in batches {
            output.append(self.context.run_ui(batch, &mut draw));
        }
        // --8<-- [end:step-3g]
        self.input
            .handle_platform_output(&state.window, std::mem::take(&mut output.platform_output));
        let changed = action.is_some() || command.is_some();
        // --8<-- [end:step-10g]

        if let Some(key) = action {
            state.panel_action(&key);
        }

        if let Some(text) = command {
            let message = state.run_command(&text).unwrap_or_else(|error| error);
            crate::app::feedback::status(&message);
            MODEL.with_borrow_mut(|model| {
                // --8<-- [start:step-10h]
                if model.history.len() == 200 {
                    model.history.pop_front();
                }

                model.history.push_back(format!("> {text}\n{message}"));
                if !model.command_open && !model.focus_command {
                    self.context.memory_mut(|memory| {
                        memory.surrender_focus(egui::Id::new("command-input"))
                    });
                }
                // --8<-- [end:step-10h]
            });
            state.touch();
        }

        self.publish();
        // --8<-- [start:step-13e]
        // the field changed on its own: the hidden input follows
        #[cfg(target_arch = "wasm32")]
        MODEL.with_borrow(|model| {
            if self.agent_value != model.command {
                self.agent_value.clone_from(&model.command);
                super::agent::sync(&model.command);
            }
        });
        // --8<-- [end:step-13e]
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
            // --8<-- [start:step-10i]
            let snapshot = MODEL.with_borrow(|model| serde_json::json!({"framework": "egui 0.34.3", "scene_rect": [self.scene_rect.min.x,self.scene_rect.min.y,self.scene_rect.max.x,self.scene_rect.max.y], "completion_rect": model.completion_rect.map(|r| [r.min.x,r.min.y,r.max.x,r.max.y]), "rows": model.rows, "controls": self.controls, "command_open": model.command_open, "layers_open": model.layers_open, "command": model.command, "history": model.history, "hint": crate::app::command::hint(&model.command)}));
            // --8<-- [end:step-10i]
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
    // --8<-- [start:step-10j]
    visuals.panel_fill = egui::Color32::from_gray(245);
    visuals.window_stroke = egui::Stroke::NONE;
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::from_rgb(200, 222, 245);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::BLACK);
    visuals.text_cursor.stroke = egui::Stroke::new(1.5_f32, egui::Color32::BLACK);
    visuals.text_cursor.blink = false; // frames are drawn on demand
    // --8<-- [end:step-10j]
    visuals.indent_has_left_vline = false;

    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        // --8<-- [start:step-10k]
        widget.bg_stroke = egui::Stroke::NONE;
        widget.bg_fill = egui::Color32::from_gray(245);
        // --8<-- [end:step-10k]
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

    // --8<-- [start:step-10l]
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
        // one line per row
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
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
                                    egui::Button::new(&row.label)
                                        .selected(row.selected)
                                        .frame(row.selected)
                                        .truncate(),
                                )
                                .on_hover_text(format!("{} · {} objects", row.label, row.count));
                            record(
                                controls,
                                &row.key,
                                &format!("Select {}", row.label),
                                &response,
                            );

                            if response.clicked() {
                                *action = Some(if ui.input(|i| i.modifiers.shift) {
                                    row.key.replacen("select/", "add/", 1)
                                } else {
                                    row.key.clone()
                                });
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
                            record(controls, &row.key, &row.label, &response);

                            if response.clicked() {
                                *action = Some(row.key.clone());
                            }
                        }
                    });
                }
            });
    });
    // --8<-- [end:step-10l]
}

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
    // --8<-- [start:step-10m]
    let response = egui::containers::menu::MenuButton::new(
        egui::RichText::new("■").color(egui::Color32::from_rgb(color[0], color[1], color[2])),
    )
    .config(
        egui::containers::menu::MenuConfig::default()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
    )
    .ui(ui, |ui| {
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
    })
    .0
    .on_hover_text("Change object and child colors");
    // --8<-- [end:step-10m]
    record(
        controls,
        &format!("color/{index}"),
        &format!("Color {}", row.label),
        &response,
    );
}

/// The command dock; an executed line goes to `command`.
fn commands(
    root: &mut egui::Ui,
    model: &mut Model,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
) {
    // --8<-- [start:step-10n]
    let previous_popup = model.completion_rect.take();
    // --8<-- [start:step-3h]
    let polyline_options = model.drawing_command == "polyline";
    let panel = if model.command_collapsed {
        egui::Panel::bottom("command-line-collapsed").exact_size(if polyline_options {
            62.0
        } else {
            34.0
        })
    } else {
        egui::Panel::bottom("command-line")
            .default_size(104.0)
            .resizable(true)
            .size_range(
                (if polyline_options { 92.0 } else { 64.0 })
                    ..=(root.available_height() * 0.75).max(104.0),
            )
    };
    let panel_response = panel
    // --8<-- [end:step-3h]
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .inner_margin(6),
        )
        .show_inside(root, |ui| {
            // --8<-- [start:step-3i]
            ui.set_min_height(ui.max_rect().height());
            // --8<-- [end:step-3i]
            ui.painter().hline(
                ui.max_rect().x_range().expand(6.0),
                ui.max_rect().top() - 6.0,
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(110)),
            );
            ui.set_clip_rect(ui.max_rect().expand(6.0));
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            if !model.command_collapsed {
                egui::ScrollArea::vertical()
                    .id_salt("command-history")
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    // --8<-- [start:step-3j]
                    .min_scrolled_height(0.0)
                    .max_height(
                        (ui.available_height() - if polyline_options { 66.0 } else { 38.0 })
                            .max(0.0),
                    )
                    // --8<-- [end:step-3j]
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
            // --8<-- [start:step-3k]
            }
            // Points / Rectangle / Polygon buttons while drawing a polyline
            if polyline_options {
                ui.horizontal_wrapped(|ui| {
                    for (label, text) in [
                        ("Points", "Polyline Points"),
                        ("Rectangle", "Polyline Rectangle"),
                        ("Polygon", "Polyline Polygon"),
                        ("Finish", ""),
                    ] {
                        let option = ui.button(label);
                        record(controls, &format!("command/option/{label}"), label, &option);
                        if option.clicked() {
                            *command = Some(text.into());
                            model.command.clear();
                            model.completion_visible = false;
                            model.focus_command = true;
                        }
                    }
                });
            // --8<-- [end:step-3k]
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
                // --8<-- [start:step-3l]
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
                // --8<-- [end:step-3l]
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
                let response = ui.add_sized(
                    [(ui.available_width() - option_width - 28.0).max(40.0), 22.0],
                    egui::TextEdit::singleline(&mut model.command)
                        .id(id)
                        .font(egui::FontId::proportional(14.0))
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
                if response.changed() {
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
                    // --8<-- [start:step-3m]
                    let (text, run) = crate::app::command::accept(name);
                    if run && !tab {
                        *command = Some(text);
                        model.command.clear();
                    } else {
                        model.command = if run { format!("{text} ") } else { text };
                    }
                    model.completion_visible = false;
                    // --8<-- [end:step-3m]
                    model.inline_suffix = false;
                    model.focus_command = true;
                }
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
                // Escape clears the field
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
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
                    .button(if model.command_collapsed { "+" } else { "−" })
                    .on_hover_text("Collapse or expand history");
                record(
                    controls,
                    "command/collapse",
                    "Collapse or expand history",
                    &collapse,
                );
                if collapse.clicked() {
                    model.command_collapsed = !model.command_collapsed;
                    ui.ctx().request_repaint();
                }
            });
        });
    // --8<-- [start:step-3n]
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
    // --8<-- [end:step-3n]
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
    // --8<-- [end:step-10n]
}
