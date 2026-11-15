// --8<-- [start:course-rows]
use super::{
    canvas::{self, Drawing},
    painting::Painter,
};
use crate::engine::{
    gpu::{
        buffers::GpuCtx,
        device::DeviceSetup,
        instance::Instance,
        objects::{InstanceTable, ObjectRow, ObjectRows},
    },
    pipelines::Layouts,
};
use session_rust::{AABB, Xform};
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{Key, NamedKey},
};

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|_, gpu| {
        let ctx = GpuCtx::new(gpu.device.clone(), gpu.queue.clone());
        let layouts = Layouts::new(&gpu.device);
        let mut table = InstanceTable::new(&ctx, &layouts);
        let rows = ObjectRows {
            rows: (0..2)
                .map(|i| {
                    let mut row = ObjectRow::new(
                        Xform::translation(if i == 0 { -120.0 } else { 120.0 }, 0.0, 0.0),
                        if i == 1 {
                            Instance::FLAG_NON_PICKABLE
                        } else {
                            0
                        },
                    );
                    row.bounds = AABB::new(0.0, 0.0, 0.0, 45.0, 45.0, 0.0);
                    row.color = if i == 0 {
                        [0.9, 0.2, 0.4, 1.0]
                    } else {
                        [0.2, 0.5, 0.9, 1.0]
                    };
                    row
                })
                .collect(),
        };
        table.append(&ctx, &layouts, &rows);
        Ok(Box::new(Rows {
            painter: Painter::new(gpu),
            ctx,
            table,
            x: -120.0,
            hidden: false,
            short: false,
            blue: false,
        }))
    })
}

struct Rows {
    painter: Painter,
    ctx: GpuCtx,
    table: InstanceTable,
    x: f64,
    hidden: bool,
    short: bool,
    blue: bool,
}

impl Drawing for Rows {
    fn event(&mut self, event: &WindowEvent, _: f64) {
        let WindowEvent::KeyboardInput { event, .. } = event else {
            return;
        };
        if event.state != ElementState::Pressed {
            return;
        }
        match event.logical_key.as_ref() {
            Key::Named(NamedKey::ArrowLeft) => self.x -= 20.0,
            Key::Named(NamedKey::ArrowRight) => self.x += 20.0,
            Key::Character("h" | "H") => {
                self.hidden = !self.hidden;
                self.table
                    .set_flag(&self.ctx, 0, Instance::FLAG_HIDDEN, self.hidden);
            }
            Key::Character("r" | "R") => {
                self.short = !self.short;
                self.table
                    .set_reveal(&self.ctx, 0, if self.short { 0.5 } else { 1.0 }, 1.0);
            }
            Key::Character("c" | "C") => {
                self.blue = !self.blue;
                self.table.set_color(
                    &self.ctx,
                    0,
                    false,
                    if self.blue {
                        Some([51, 128, 230])
                    } else {
                        None
                    },
                );
            }
            _ => return,
        }
        self.table
            .set_placement(&self.ctx, 0, &Xform::translation(self.x, 0.0, 0.0));
    }

    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool {
        self.painter.draw(gpu, view, scale, now, Default::default(), |root, screen| {
            let fit = (screen.width() / 500.0).min(1.0);
            for i in 0..self.table.len() {
                let row = self.table.row(i).unwrap();
                if row.flags & Instance::FLAG_HIDDEN != 0 { continue }
                let place = self.table.placement(i).unwrap();
                let center = screen.center() + egui::vec2(place.m[12] as f32 * fit, 0.0);
                // This inspector displays the CPU mirror. Later lanes consume the same uploaded rows.
                let tint = if row.flags & Instance::FLAG_COLOR != 0 {
                    row.color
                } else if i == 0 { [0.9, 0.2, 0.4, 1.0] } else { [0.2, 0.5, 0.9, 1.0] };
                let color = egui::Color32::from_rgb((tint[0] * 255.0) as u8,
                    (tint[1] * 255.0) as u8, (tint[2] * 255.0) as u8);
                root.painter().rect_filled(egui::Rect::from_center_size(center,
                    egui::vec2(90.0 * row.effect[0] * fit, 90.0 * fit)), 0.0, color);
                root.painter().text(center + egui::vec2(0.0, 60.0), egui::Align2::CENTER_TOP,
                    if row.flags & Instance::FLAG_NON_PICKABLE != 0 { "row 1: non-pickable" } else { "row 0" },
                    egui::FontId::proportional(14.0), egui::Color32::BLACK);
            }
            root.painter().text(screen.min + egui::vec2(16.0, 16.0), egui::Align2::LEFT_TOP,
                "Object rows: CPU mirror + GPU uploads\nArrows: placement; C: colour\nR: reveal; H: visibility",
                egui::FontId::proportional(16.0), egui::Color32::BLACK);
            false
        }).0
    }
}
// --8<-- [end:course-rows]
