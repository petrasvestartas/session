// --8<-- [start:course-painting]
use super::{
    canvas::{self, Drawing},
    frame::clear,
};
use crate::engine::gpu::{device::DeviceSetup, ui::Ui};

pub struct Painter {
    context: egui::Context,
    ui: Ui,
}

impl Painter {
    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub fn new(gpu: &DeviceSetup) -> Self {
        let context = egui::Context::default();
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "text".into(),
            egui::FontData::from_static(include_bytes!(
                "../../assets/text/NotoSans-Regular.subset.ttf"
            ))
            .into(),
        );
        fonts
            .families
            .insert(egui::FontFamily::Proportional, vec!["text".into()]);
        context.set_fonts(fonts);
        Self {
            context,
            ui: Ui::new(&gpu.device, gpu.config.format),
        }
    }

    pub fn draw(
        &mut self,
        gpu: &DeviceSetup,
        view: &wgpu::TextureView,
        scale: f32,
        now: f64,
        mut input: egui::RawInput,
        mut paint: impl FnMut(&mut egui::Ui, egui::Rect) -> bool,
    ) -> (bool, egui::PlatformOutput) {
        self.context.set_pixels_per_point(scale);
        let screen = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(
                gpu.config.width as f32 / scale,
                gpu.config.height as f32 / scale,
            ),
        );
        input.screen_rect = Some(screen);
        input.time = Some(now / 1000.0);
        let mut more = false;
        let mut output = self.context.run_ui(input, |root| {
            more = paint(root, screen);
        });
        let platform = std::mem::take(&mut output.platform_output);
        self.ui.prepare(
            &gpu.device,
            &gpu.queue,
            &self.context,
            output,
            [gpu.config.width, gpu.config.height],
        );
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        clear(&mut encoder, view, wgpu::Color::WHITE);
        self.ui.draw(&mut encoder, view);
        gpu.queue.submit([encoder.finish()]);
        (more, platform)
    }
}

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|_, gpu| Ok(Box::new(Painting(Painter::new(gpu)))))
}

struct Painting(Painter);
impl Drawing for Painting {
    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool {
        self.0
            .draw(gpu, view, scale, now, Default::default(), |root, screen| {
                let painter = root.painter();
                let center = screen.center();
                painter.add(egui::Shape::convex_polygon(
                    vec![
                        center + egui::vec2(0.0, -90.0),
                        center + egui::vec2(-100.0, 80.0),
                        center + egui::vec2(100.0, 80.0),
                    ],
                    egui::Color32::from_rgb(230, 110, 160),
                    egui::Stroke::NONE,
                ));
                painter.text(
                    egui::pos2(18.0, screen.bottom() - 18.0),
                    egui::Align2::LEFT_BOTTOM,
                    "Command:",
                    egui::FontId::proportional(18.0),
                    egui::Color32::BLACK,
                );
                false
            })
            .0
    }
}
// --8<-- [end:course-painting]
