// --8<-- [start:course-mathematics]
use super::{
    canvas::{self, Drawing},
    painting::Painter,
};
use crate::{
    app::stage::{deck, font::Fonts, presentation::Presentation},
    engine::gpu::device::DeviceSetup,
};
use winit::event::{ElementState, WindowEvent};

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|_, gpu| {
        let fonts = Fonts::new(vec![
            include_bytes!("../../assets/stage/NewCMMath-Book.otf").to_vec(),
            include_bytes!("../../assets/stage/NewCM10-Book.otf").to_vec(),
            include_bytes!("../../assets/stage/NewCM10-Bold.otf").to_vec(),
            include_bytes!("../../assets/stage/NewCMMono10-Book.otf").to_vec(),
        ])?;
        let index = deck::index(include_str!("index.yaml"))?;
        let chapter = deck::chapter(
            include_str!("../../docs/lessons/03a-math/example.yaml"),
            "example.yaml",
        )?;
        let mut presentation =
            Presentation::new(fonts, index, vec![chapter], vec![], canvas::now());
        presentation.stage.chapter_go(0, canvas::now());
        Ok(Box::new(Mathematics {
            painter: Painter::new(gpu),
            presentation,
        }))
    })
}

struct Mathematics {
    painter: Painter,
    presentation: Presentation,
}

impl Drawing for Mathematics {
    fn event(&mut self, event: &WindowEvent, now: f64) {
        if let WindowEvent::KeyboardInput { event, .. } = event
            && event.state == ElementState::Pressed
        {
            self.presentation.key(event.logical_key.as_ref(), now);
        }
    }

    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool {
        self.presentation.stage.tick(now);
        self.painter
            .draw(gpu, view, scale, now, Default::default(), |root, screen| {
                let [r, g, b, a] = self.presentation.stage.palette("bg");
                let background = egui::Color32::from_rgba_unmultiplied(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    (a * 255.0) as u8,
                );
                root.painter().rect_filled(screen, 0.0, background);
                self.presentation.paint(root.painter(), screen, scale)
            })
            .0
    }
}
// --8<-- [end:course-mathematics]
