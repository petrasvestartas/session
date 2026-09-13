use crate::State;
use crate::app::feedback::LayerRow;
use std::cell::RefCell;
use std::collections::VecDeque;
use winit::window::Window;

#[derive(Default)]
pub struct Model {
    pub layers_open: bool,
    pub rows: Vec<LayerRow>,
    pub command_open: bool,
    pub command: String,
    pub focus_command: bool,
    pub status: String,
    history: VecDeque<String>,
}

thread_local! { pub static MODEL: RefCell<Model> = RefCell::default(); }

#[derive(serde::Serialize)]
pub struct Control {
    key: String,
    label: String,
    rect: [f32; 4],
}

pub struct Ui {
    context: egui::Context,
    input: egui_winit::State,
    controls: Option<Vec<Control>>,
}

impl Ui {
    pub fn new(window: &Window) -> Self {
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
        }
    }

    pub fn event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> (bool, bool) {
        let response = self.input.on_window_event(window, event);
        let escape = matches!(event, winit::event::WindowEvent::KeyboardInput { event, .. }
            if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape))
            && MODEL.with_borrow(|model| model.command_open);
        (response.consumed || escape, response.repaint || escape)
    }

    pub fn frame(&mut self, state: &mut State) -> bool {
        let input = self.input.take_egui_input(&state.window);
        if let Some(controls) = self.controls.as_mut() {
            controls.clear();
        }
        let mut action = None;
        let mut command = None;
        let mut output = self.context.run_ui(input, |root| {
            let context = root.ctx();
            MODEL.with_borrow_mut(|model| {
                layers(context, model, &mut self.controls, &mut action);
                commands(context, model, &mut self.controls, &mut command);
            });
        });
        self.input
            .handle_platform_output(&state.window, std::mem::take(&mut output.platform_output));
        let changed = action.is_some() || command.is_some();
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
        output.pixels_per_point *=
            state.gpu.config.width as f32 / state.window.inner_size().width.max(1) as f32;
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
            let hidden = MODEL.with_borrow(|model| model.command_open);
            if hidden {
                let _ = status.set_attribute("hidden", "");
            } else {
                let _ = status.remove_attribute("hidden");
            }
        }
    }
}

fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.window_fill = egui::Color32::WHITE;
    visuals.panel_fill = egui::Color32::WHITE;
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

fn layers(
    context: &egui::Context,
    model: &mut Model,
    controls: &mut Option<Vec<Control>>,
    action: &mut Option<String>,
) {
    if !model.layers_open {
        return;
    }
    egui::Window::new("Session layers")
        .default_pos([12.0, 12.0])
        .default_width(310.0)
        .resizable(false)
        .collapsible(false)
        .open(&mut model.layers_open)
        .show(context, |ui| {
            egui::ScrollArea::vertical()
                .max_height(context.content_rect().height() * 0.65)
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

fn layer_button(ui: &mut egui::Ui, row: &LayerRow) -> egui::Response {
    let label = row.label.trim_start();
    if row.key.starts_with("open/") {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(12.0, 18.0), egui::Sense::click());
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
    ui.button(text).on_hover_text(label)
}

fn commands(
    context: &egui::Context,
    model: &mut Model,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
) {
    if !model.command_open {
        return;
    }
    let mut open = model.command_open;
    egui::Window::new("Command line")
        .anchor(egui::Align2::LEFT_BOTTOM, [12.0, -12.0])
        .default_width(480.0)
        .resizable(false)
        .collapsible(false)
        .open(&mut open)
        .show(context, |ui| {
            for text in &model.history {
                ui.label(text);
            }
            ui.label("World coordinates: x,y,z. Select a curve before trim, extend or explode.");
            let response = ui.add(
                egui::TextEdit::singleline(&mut model.command)
                    .char_limit(2048)
                    .hint_text(
                        egui::RichText::new("line 0,0,0 100,0,0").color(egui::Color32::BLACK),
                    )
                    .desired_width(f32::INFINITY),
            );
            record(controls, "command/input", "Command", &response);
            if model.focus_command {
                response.request_focus();
                model.focus_command = false;
            }
            let enter =
                response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
            ui.horizontal(|ui| {
                let run = ui.button("Run");
                record(controls, "command/run", "Run", &run);
                if (enter || run.clicked()) && !model.command.trim().is_empty() {
                    *command = Some(std::mem::take(&mut model.command));
                }
                let close = ui.button("Close (Esc)");
                record(controls, "command/close", "Close", &close);
                if close.clicked() || ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                    model.command_open = false;
                }
                ui.label("point · line · polyline · trim · extend · explode · undo");
            });
            if !model.status.is_empty() {
                ui.label(&model.status);
            }
        });
    model.command_open &= open;
}
