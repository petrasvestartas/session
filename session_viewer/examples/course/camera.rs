// --8<-- [start:course-camera]
use super::{
    canvas::{self, Drawing},
    painting::Painter,
};
use crate::{
    camera::{Camera, View},
    engine::gpu::device::DeviceSetup,
};
use session_rust::{AABB, Point};
use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{Key, NamedKey},
};

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|_, gpu| {
        Ok(Box::new(Projection {
            painter: Painter::new(gpu),
            camera: Camera::new(),
            fit: true,
            dragging: false,
            cursor: None,
        }))
    })
}

struct Projection {
    painter: Painter,
    camera: Camera,
    fit: bool,
    dragging: bool,
    cursor: Option<(f64, f64)>,
}

impl Drawing for Projection {
    fn event(&mut self, event: &WindowEvent, _: f64) {
        match event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => self.dragging = *state == ElementState::Pressed,
            WindowEvent::CursorMoved { position, .. } => {
                if self.dragging
                    && let Some((x, y)) = self.cursor
                {
                    self.camera
                        .orbit((position.x - x) as f32, (position.y - y) as f32);
                }
                self.cursor = Some((position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y * 40.0,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                self.camera.zoom(amount);
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                match event.logical_key.as_ref() {
                    Key::Named(NamedKey::ArrowLeft) => self.camera.orbit(-40.0, 0.0),
                    Key::Named(NamedKey::ArrowRight) => self.camera.orbit(40.0, 0.0),
                    Key::Named(NamedKey::ArrowUp) => self.camera.orbit(0.0, -40.0),
                    Key::Named(NamedKey::ArrowDown) => self.camera.orbit(0.0, 40.0),
                    Key::Character("p" | "P") => self.camera.toggle_projection(),
                    Key::Character("f" | "F") => self.fit = true,
                    Key::Named(NamedKey::Home) => {
                        self.camera.set_view(View::Iso);
                        self.fit = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool {
        self.painter.draw(gpu, view, scale, now, Default::default(), |root, screen| {
            let aspect = screen.width() as f64 / screen.height() as f64;
            if self.fit {
                self.camera.fit(&AABB::new(0.0, 0.0, 0.0, 500.0, 500.0, 500.0), aspect);
                self.fit = false;
            }
            // The matrix receives scene coordinates relative to its anchor.
            let matrix = self.camera.view_proj_anchored(aspect, &Point::new(0.0, 0.0, 0.0));
            let points: Vec<_> = (0..8).map(|i| {
                let p = Point::new(
                    if i & 1 == 0 { -500.0 } else { 500.0 },
                    if i & 2 == 0 { -500.0 } else { 500.0 },
                    if i & 4 == 0 { -500.0 } else { 500.0 },
                );
                let p = matrix.transform_point(&p);
                egui::pos2(screen.center().x + p[0] as f32 * screen.width() * 0.5,
                    screen.center().y - p[1] as f32 * screen.height() * 0.5)
            }).collect();
            for i in 0..8 {
                for bit in [1, 2, 4] {
                    if i & bit == 0 {
                        root.painter().line_segment([points[i], points[i | bit]],
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(70, 130, 220)));
                    }
                }
            }
            let projection = if self.camera.perspective { "Perspective" } else { "Orthographic" };
            root.painter().text(screen.min + egui::vec2(16.0, 16.0), egui::Align2::LEFT_TOP,
                format!("{projection}\nDrag/arrows: orbit; wheel: zoom\nP: projection; F: fit; Home: isometric"),
                egui::FontId::proportional(16.0), egui::Color32::BLACK);
            false
        }).0
    }
}
// --8<-- [end:course-camera]
