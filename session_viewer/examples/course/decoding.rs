// --8<-- [start:course-decoding]
use super::{canvas::{self, Drawing}, painting::Painter};
use crate::{app::decode, engine::gpu::device::DeviceSetup};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use winit::{event::{ElementState, WindowEvent}, keyboard::Key, window::Window};

const FILE: &str = "course-comparison.pb";
const BYTES: &[u8] = include_bytes!("../../assets/pb/course-comparison.pb");

enum ResultState {
    Reading,
    Ready(String),
    Error(String),
}

struct Document {
    painter: Painter,
    window: Arc<Window>,
    result: Rc<RefCell<ResultState>>,
}

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|window, gpu| {
        let document = Document {
            painter: Painter::new(gpu),
            window: window.clone(),
            result: Rc::new(RefCell::new(ResultState::Ready(String::new()))),
        };
        document.read(false);
        Ok(Box::new(document))
    })
}

impl Document {
    fn read(&self, broken: bool) {
        // A second request cannot replace a result still being decoded.
        if matches!(*self.result.borrow(), ResultState::Reading) { return }
        *self.result.borrow_mut() = ResultState::Reading;
        let result = self.result.clone();
        let window = self.window.clone();
        let mut bytes = BYTES.to_vec();
        if broken { bytes.push(0x80); } // An unfinished varint field header.
        wasm_bindgen_futures::spawn_local(async move {
            let next = match decode::session_from_bytes(FILE, bytes).await {
                Ok(session) => ResultState::Ready(format!(
                    "Document: {}\nMeshes: {}\nBReps: {}\nCurves: {}\nComponents: {}\nStored placements: {}",
                    session.name, session.objects.meshes.len(), session.objects.breps.len(),
                    session.objects.nurbscurves.len(), session.objects.components.len(), session.xforms.len(),
                )),
                Err(error) => ResultState::Error(format!("{FILE}: {error}")),
            };
            *result.borrow_mut() = next;
            window.request_redraw();
        });
    }
}

impl Drawing for Document {
    fn event(&mut self, event: &WindowEvent, _now: f64) {
        let WindowEvent::KeyboardInput { event, .. } = event else { return };
        if event.state != ElementState::Pressed { return }
        match event.logical_key.as_ref() {
            Key::Character("r" | "R") => self.read(false),
            Key::Character("e" | "E") => self.read(true),
            _ => {}
        }
    }

    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool {
        let result = self.result.borrow();
        let (state, message, color) = match &*result {
            ResultState::Reading => ("reading", "Reading document...", egui::Color32::GRAY),
            ResultState::Ready(message) => ("ready", message.as_str(), egui::Color32::BLACK),
            ResultState::Error(message) => ("error", message.as_str(), egui::Color32::from_rgb(180, 25, 45)),
        };
        if let Some(canvas) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id("canvas")) {
            let value = serde_json::json!({"state": state, "message": message, "bytes": BYTES.len()});
            let _ = canvas.set_attribute("data-course-document", &value.to_string());
        }
        self.painter.draw(gpu, view, scale, now, Default::default(), |root, screen| {
            let painter = root.painter();
            let width = (screen.width() - 36.0).max(1.0);
            for (y, text, size, ink) in [
                (20.0, "Decode a document", 24.0, egui::Color32::BLACK),
                (64.0, "R: read valid bytes\nE: append a broken field header", 16.0, egui::Color32::BLACK),
                (130.0, message, 18.0, color),
            ] {
                let galley = painter.layout(text.to_string(), egui::FontId::proportional(size), ink, width);
                painter.galley(screen.min + egui::vec2(18.0, y), galley, ink);
            }
            false
        }).0
    }
}
// --8<-- [end:course-decoding]
