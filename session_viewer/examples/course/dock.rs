// --8<-- [start:course-dock]
use super::{
    canvas::{self, Drawing},
    painting::Painter,
};
use crate::{
    command_dock::{self, CommandLine, Commands},
    engine::gpu::device::DeviceSetup,
};
use std::sync::Arc;
use winit::{event::WindowEvent, window::Window};

struct Vocabulary;
impl Commands for Vocabulary {
    fn canonical(&self, line: &str) -> String {
        ["Clear", "Color", "Color Blue", "Color Pink", "Hello"]
            .into_iter()
            .find(|name| name.eq_ignore_ascii_case(line.trim()))
            .unwrap_or(line.trim())
            .to_owned()
    }

    fn choosing_option(&self, line: &str) -> bool {
        line.trim().eq_ignore_ascii_case("Color")
    }

    fn accept(&self, line: &str) -> (String, bool) {
        let line = self.canonical(line);
        let complete = matches!(
            line.as_str(),
            "Clear" | "Color" | "Color Blue" | "Color Pink" | "Hello"
        );
        (line, complete)
    }

    fn completions(&self, line: &str) -> Vec<&'static str> {
        let choices: &[&str] = if line.trim().to_lowercase().starts_with("color ") {
            &["Color Blue", "Color Pink"]
        } else {
            &["Clear", "Color", "Hello"]
        };
        let prefix = line.trim().to_lowercase();
        choices
            .iter()
            .copied()
            .filter(|name| name.to_lowercase().starts_with(&prefix))
            .collect()
    }

    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|window, gpu| {
        let painter = Painter::new(gpu);
        painter
            .context()
            .set_visuals(command_dock::theme::visuals());
        painter.context().set_fonts(command_dock::theme::fonts([
            include_bytes!("../../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        let input = egui_winit::State::new(
            painter.context().clone(),
            egui::ViewportId::ROOT,
            window.as_ref(),
            Some(window.scale_factor() as f32),
            window.theme(),
            Some(4096),
        );
        Ok(Box::new(Dock {
            window: window.clone(),
            painter,
            input,
            model: CommandLine {
                focus_command: true,
                ..Default::default()
            },
            color: egui::Color32::from_rgb(230, 110, 160),
            message: "Type Hello, Color or Clear".into(),
            choosing_color: false,
        }))
    })
}

struct Dock {
    window: Arc<Window>,
    painter: Painter,
    input: egui_winit::State,
    model: CommandLine,
    color: egui::Color32,
    message: String,
    choosing_color: bool,
}

impl Drawing for Dock {
    fn event(&mut self, event: &WindowEvent, _: f64) {
        let _ = self.input.on_window_event(&self.window, event);
    }

    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool {
        let input = self.input.take_egui_input(&self.window);
        self.model.options = if self.choosing_color {
            vec![
                ("Blue".into(), "Color Blue".into()),
                ("Pink".into(), "Color Pink".into()),
            ]
        } else {
            vec![]
        };
        let mut controls = Some(Vec::new());
        let mut command = None;
        let mut focus_canvas = false;
        let (_, platform) = self
            .painter
            .draw(gpu, view, scale, now, input, |root, screen| {
                let center = screen.center();
                root.painter()
                    .circle_filled(center - egui::vec2(0.0, 45.0), 35.0, self.color);
                root.painter().text(
                    center + egui::vec2(0.0, 25.0),
                    egui::Align2::CENTER_CENTER,
                    &self.message,
                    egui::FontId::proportional(20.0),
                    egui::Color32::BLACK,
                );
                focus_canvas = command_dock::draw(
                    root,
                    &mut self.model,
                    &mut controls,
                    &mut command,
                    &Vocabulary,
                    true,
                );
                false
            });
        self.input.handle_platform_output(&self.window, platform);
        if focus_canvas {
            self.window.focus_window();
        }
        if let Some(canvas) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("canvas"))
        {
            let _ = canvas.set_attribute(
                "data-course-controls",
                &serde_json::to_string(&controls).unwrap(),
            );
        }
        if let Some(line) = command {
            match line.as_str() {
                "Clear" => {
                    self.model.history.clear();
                    self.message = "History cleared".into();
                    self.choosing_color = false;
                }
                "Color" => {
                    self.choosing_color = true;
                    self.message = "Choose Blue or Pink".into();
                }
                "Color Blue" => {
                    self.color = egui::Color32::from_rgb(70, 130, 220);
                    self.message = "Blue".into();
                    self.choosing_color = false;
                }
                "Color Pink" => {
                    self.color = egui::Color32::from_rgb(230, 110, 160);
                    self.message = "Pink".into();
                    self.choosing_color = false;
                }
                "Hello" => self.message = "Hello!".into(),
                "Escape" => {
                    self.choosing_color = false;
                    self.message = "Cancelled".into();
                }
                _ => return false,
            }
            if line != "Clear" && line != "Escape" {
                self.model.remember(line);
            }
            self.model.status.clone_from(&self.message);
            return true;
        }
        self.painter.context().has_requested_repaint()
    }
}
// --8<-- [end:course-dock]
