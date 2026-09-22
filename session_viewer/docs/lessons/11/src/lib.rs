// --8<-- [start:step-8a]
use session_rust::AABB;
pub mod app;
pub mod camera;
pub mod engine;
pub mod fixture;
pub mod text_quality;
// --8<-- [end:step-8a]
use engine::gpu::{FrameInput, Gpu};
use wasm_bindgen::prelude::*;

/// Canvas and input ownership remain separate from GPU lane ownership.
#[wasm_bindgen]
/// The teaching viewer the page talks to.
pub struct Tutorial {
    canvas: web_sys::HtmlCanvasElement,
    gpu: Gpu, // every GPU object
    camera: camera::Camera, // orbit, pan, zoom
    scale: f64, // device pixel ratio
    fixture: fixture::CadFixture, // the built-in scene
}

#[wasm_bindgen]
impl Tutorial {
    /// Retain local CAD sources while handing only prepared lane tables to the GPU.
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<Tutorial, JsValue> {
        // panics print to the console
        console_error_panic_hook::set_once();
        let mut gpu = Gpu::new(canvas.clone()).await.map_err(js_error)?;
        let mut fixture = fixture::build();
        gpu.set_scene(&fixture.upload);
        fixture.upload.drop_uploaded();
        // --8<-- [start:step-8b]
        gpu.text
            .set_labels(text_labels(&gpu.bounds))
            .map_err(js_error)?;
            // --8<-- [end:step-8b]
        let mut camera = camera::Camera::new();
        camera.unit = camera::Unit::Millimeters;
        camera.set_view(camera::View::Iso);
        camera.fit(&gpu.bounds, 1.5);

        if app::route::query("top").is_some() {
            camera.set_view(camera::View::Top);
        }

        camera.perspective = app::route::query("perspective").is_some();

        if let Some(distance) = app::route::query("distance").and_then(parse_distance) {
            camera.distance *= distance;
            camera.update_position();
        }

        gpu.view.show_grid = false;
        gpu.view.show_mesh_edges = app::route::query("fill").is_none();
        gpu.view.markers = false;
        Ok(Self {
            canvas,
            gpu,
            camera,
            scale: 1.0, // device pixel ratio
            fixture,
        })
    }

    /// Orbit or pan with the production camera.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool) {
        if pan {
            self.camera.pan(dx, dy)
        } else {
            self.camera.orbit(dx, dy)
        }
    }

    /// Cursor and viewport in physical pixels.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64) {
        self.camera.zoom_at(
            delta,
            (x * self.scale, y * self.scale),
            (self.gpu.config.width as f64, self.gpu.config.height as f64),
        );
    }

    /// Resize targets, share the anchor, draw one frame.
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
                r: 0.025, // background red
                g: 0.035, // background green
                b: 0.055, // background blue
                a: 1.0,
            },
            now_ms: now,
        };
        self.gpu.render(&input).map_err(js_error)?;
        // --8<-- [start:step-8c]
        Ok(serde_json::json!({"stage":11,"objects":self.gpu.objects.len(),"width":w,"height":h,"scale":scale,"drawn":true,"text":self.gpu.text.stats,"sourceObjects":self.fixture.identities,"sourceEdgeIds":self.fixture.pipe_source_edges,"samples":self.gpu.targets.samples,
        // --8<-- [end:step-8c]
            "meshVertices":self.gpu.arena.vert_count(),"segments":self.gpu.segments.ribbon_count(),"dots":self.gpu.glyphs.dot_count(),"cloudPoints":self.gpu.cloud.point_count}).to_string())
    }
}

/// Read a monotonic frame time from the browser when available.
fn window_time(window: web_sys::Window) -> Option<f64> {
    Some(window.performance()?.now())
}

/// Preserve a useful error string at the browser boundary.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// Limit the reproducible diagnostic distance knob to the verified range.
fn parse_distance(value: String) -> Option<f64> {
    let value = value.parse::<f64>().ok()?;
    (value.is_finite() && (1.0..=16.0).contains(&value)).then_some(value)
}
// --8<-- [start:step-8d]

/// Overlay text (CSS size) or world label (scene depth).
fn text_labels(bounds: &AABB) -> Vec<engine::text::TextLabel> {
    use engine::text::{TextLabel, TextPlacement};
    vec![
        TextLabel {
            id: 1,
            text: "AVATAR To office ffi — Ø 25 ± 0.1 mm".into(),
            font_size: 16.0,
            line_height: 24.0,
            color: [255; 4],
            placement: TextPlacement::Nameplate {
                world: [0.0, 0.0, bounds.max_point()[2] + 30.0],
                padding: [6.0, 4.0],
                rounded: false,
            },
            clip: None,
        },
        TextLabel {
            id: 2,
            text: "Centered white nameplate".into(),
            font_size: 13.5,
            line_height: 19.5,
            color: [255; 4],
            placement: TextPlacement::Nameplate {
                world: [bounds.cx, bounds.cy, bounds.max_point()[2]],
                padding: [12.75, 3.0],
                rounded: true,
            },
            clip: None,
        },
        TextLabel {
            id: 3,
            text: "text_not_oriented_to_camera".into(),
            font_size: 18.0,
            line_height: 26.0,
            color: [255; 4],
            placement: TextPlacement::WorldPlane {
                world: [
                    bounds.min_point()[0] - 100.0,
                    bounds.min_point()[1] - 100.0,
                    bounds.max_point()[2],
                ],
                right: [1.0, 0.0, 0.0],
                up: [0.0, 0.0, 1.0],
                world_height: 16.0, // text height in world units
            },
            clip: None,
        },
    ]
}
// --8<-- [end:step-8d]
