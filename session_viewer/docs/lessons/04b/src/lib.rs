pub mod app;
pub mod camera;
pub mod engine;
pub mod fixture;
use engine::gpu::{FrameInput, Gpu};
use wasm_bindgen::prelude::*;

/// The canvas, the camera and the GPU.
#[wasm_bindgen]
pub struct Tutorial {
    canvas: web_sys::HtmlCanvasElement,
    gpu: Gpu,
    camera: camera::Camera,
    scale: f64,
}

#[wasm_bindgen]
impl Tutorial {
    /// Build every lane and upload the objects once.
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<Tutorial, JsValue> {
        // panics print to the console
        console_error_panic_hook::set_once();
        let mut gpu = Gpu::new(canvas.clone()).await.map_err(js_error)?;
        let mut upload = fixture::scene();
        gpu.set_scene(&upload);
        upload.drop_uploaded();
        let mut camera = camera::Camera::new();
        camera.unit = camera::Unit::Meters;
        camera.set_view(camera::View::Top);
        camera.fit(&gpu.bounds, 1.5);
        Ok(Self {
            canvas,
            gpu,
            camera,
            scale: 1.0,
        })
    }

    /// Orbit, or pan when `pan` is set.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool) {
        if pan {
            self.camera.pan(dx, dy)
        } else {
            self.camera.orbit(dx, dy)
        }
    }

    /// Zoom at the cursor, in device pixels.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64) {
        self.camera.zoom_at(
            delta,
            (x * self.scale, y * self.scale),
            (self.gpu.config.width as f64, self.gpu.config.height as f64),
        );
    }

    /// Resize if needed, then draw one frame.
    pub fn render(&mut self, width: u32, height: u32, scale: f64) -> Result<String, JsValue> {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(JsValue::from_str("invalid scale"));
        }

        self.scale = scale;
        self.gpu.logical_size = [width.max(1) as f64, height.max(1) as f64];
        let w = (width.max(1) as f64 * scale).round() as u32;
        let h = (height.max(1) as f64 * scale).round() as u32;

        if self.canvas.width() != w || self.canvas.height() != h || self.gpu.config.width == 1 {
            self.canvas.set_width(w);
            self.canvas.set_height(h);
            self.gpu.resize(w, h);
        }

        let now = web_sys::window().and_then(window_time).unwrap_or(0.0);
        let rebase =
            self.gpu
                .rebase_anchor(&self.camera.origin(), self.camera.distance_world(), now);
        let input = FrameInput {
            view_proj: self
                .camera
                .view_proj_anchored(w as f64 / h as f64, &rebase.anchor),
            clear: wgpu::Color {
                r: 0.025,
                g: 0.035,
                b: 0.055,
                a: 1.0,
            },
            now_ms: now,
        };
        self.gpu.render(&input).map_err(js_error)?;
        Ok(serde_json::json!({"stage":4,"objects":self.gpu.objects.len(),"width":w,"height":h,"scale":scale,"drawn":true,
            // --8<-- [start:step-9]
            "meshVertices":self.gpu.arena.vert_count(),"segments":self.gpu.segments.ribbon_count()}).to_string())
            // --8<-- [end:step-9]
    }
}

/// Frame time from the browser clock.
fn window_time(window: web_sys::Window) -> Option<f64> {
    Some(window.performance()?.now())
}

/// Preserve a useful error string at the browser boundary.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}
