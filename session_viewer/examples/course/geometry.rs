// --8<-- [start:course-geometry]
use super::canvas::{self, Drawing};
use crate::{
    app::walk::{self, Walk, WalkCx},
    camera::{Camera, View},
    engine::gpu::{FrameInput, Gpu, ObjectRow, Upload, device::DeviceSetup},
};
use session_rust::{Arrowhead, BRep, Geometry, Matrix, Mesh, NurbsCurve, Point, Xform};
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{Key, NamedKey},
};

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|_, setup| {
        let mut gpu = Gpu::from_setup(DeviceSetup {
            surface: None,
            device: setup.device.clone(),
            queue: setup.queue.clone(),
            config: setup.config.clone(),
            device_type: setup.device_type,
            failure: setup.failure.clone(),
        });
        let mut upload = Upload::default();
        let mut curve = NurbsCurve::create(
            false,
            3,
            &[
                Point::new(-300.0, 0.0, 0.0),
                Point::new(-100.0, 300.0, 200.0),
                Point::new(100.0, -200.0, 200.0),
                Point::new(300.0, 0.0, 0.0),
            ],
        );
        curve.width = 4.0;
        curve.arrowhead = Arrowhead::BOTH;
        for (i, geometry) in [
            Geometry::Mesh(Mesh::create_box(350.0, 350.0, 350.0).into()),
            Geometry::BRep(BRep::create_box(350.0, 350.0, 350.0).into()),
            Geometry::NurbsCurve(curve.into()),
        ]
        .iter()
        .enumerate()
        {
            let cx = WalkCx {
                quality: Default::default(),
                vert_base: 0,
                cloud_px: 0.0,
                row: i as u32,
                interactions: true,
                attributes: true,
            };
            let drawn = walk::walk_geometry(&mut Walk::of(&mut upload), &cx, geometry);
            let mut row = ObjectRow::new(
                Xform::translation((i as f64 - 1.0) * 600.0, 0.0, 0.0),
                drawn.flags,
            );
            row.bounds = drawn.bounds;
            row.spacing = drawn.spacing;
            row.faces = drawn.faces;
            upload
                .bounds
                .union_with(&row.bounds.transformed(&row.place));
            upload.obj.rows.push(row);
        }
        let mut matrix = Matrix::new(2, 3);
        matrix.name = "Kernel Matrix".into();
        matrix.cell = 150.0;
        matrix.data = vec![-2.0, 0.0, 2.0, 1.0, -1.0, 3.0];
        let drawn = walk::matrix::walk(upload.lanes.get_mut(), &matrix.to_component(), 3);
        let mut row = ObjectRow::new(Xform::translation(-225.0, 600.0, 0.0), drawn.flags);
        row.bounds = drawn.bounds;
        row.faces = drawn.faces;
        upload
            .bounds
            .union_with(&row.bounds.transformed(&row.place));
        upload.obj.rows.push(row);
        gpu.set_scene(&upload);
        gpu.view.show_grid = false;
        gpu.view.opacity = 1.0;
        let mut camera = Camera::new();
        camera.set_view(View::Iso);
        Ok(Box::new(Scene {
            gpu,
            camera,
            fit: true,
            short: false,
        }))
    })
}

struct Scene {
    gpu: Gpu,
    camera: Camera,
    fit: bool,
    short: bool,
}
impl Drawing for Scene {
    fn event(&mut self, event: &WindowEvent, _: f64) {
        let WindowEvent::KeyboardInput { event, .. } = event else {
            return;
        };
        if event.state != ElementState::Pressed {
            return;
        }
        match event.logical_key.as_ref() {
            Key::Named(NamedKey::ArrowLeft) => self.camera.orbit(-40.0, 0.0),
            Key::Named(NamedKey::ArrowRight) => self.camera.orbit(40.0, 0.0),
            Key::Named(NamedKey::ArrowUp) => self.camera.orbit(0.0, -40.0),
            Key::Named(NamedKey::ArrowDown) => self.camera.orbit(0.0, 40.0),
            Key::Character("f" | "F") => self.fit = true,
            Key::Character("p" | "P") => self.camera.toggle_projection(),
            Key::Character("r" | "R") => {
                self.short = !self.short;
                self.gpu.objects.set_reveal(
                    &self.gpu.ctx,
                    2,
                    if self.short { 0.5 } else { 1.0 },
                    1.0,
                );
            }
            Key::Character("w" | "W") => self.gpu.view.show_lines = !self.gpu.view.show_lines,
            Key::Character("o" | "O") => {
                self.gpu.view.opacity = if self.gpu.view.opacity == 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            _ => {}
        }
    }
    fn draw(
        &mut self,
        setup: &DeviceSetup,
        view: &wgpu::TextureView,
        scale: f32,
        now: f64,
    ) -> bool {
        let (width, height) = (setup.config.width, setup.config.height);
        self.gpu.logical_size = [width as f64 / scale as f64, height as f64 / scale as f64];
        if self.gpu.config.width != width || self.gpu.config.height != height {
            self.gpu.resize(width, height);
        }
        let aspect = width as f64 / height as f64;
        if self.fit {
            self.camera.fit(&self.gpu.bounds, aspect);
            self.fit = false;
        }
        let anchor = self
            .gpu
            .rebase_anchor(&self.camera.origin(), self.camera.distance_world(), now)
            .anchor;
        self.gpu.write_frame_uniforms(&FrameInput {
            view_proj: self.camera.view_proj_anchored(aspect, &anchor),
            clear: wgpu::Color::WHITE,
            now_ms: now,
        });
        let mut encoder = setup.device.create_command_encoder(&Default::default());
        self.gpu
            .encode_frame(&mut encoder, view, wgpu::Color::WHITE);
        setup.queue.submit([encoder.finish()]);
        false
    }
}
// --8<-- [end:course-geometry]
