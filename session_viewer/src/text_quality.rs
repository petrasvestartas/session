//! Maintained same-font browser comparison. This standalone fixture uses the production
//! text lane and explicitly releases its device resources when its JS owner calls free().

use crate::engine::gpu::{
    buffers::GpuCtx,
    targets::{TextureSpec, texture_view},
    text::{TextFrame, TextLane},
};
use crate::engine::pipelines::Target;
use crate::engine::text::{FONT_FAMILY, TextLabel, TextPlacement};
use wasm_bindgen::prelude::*;

pub const SAMPLES: &[&str] = &[
    "Hamburgefontsiv   Il1 O0   mmmm iii WWW   gjpqy",
    "AVATAR To WA Yo   repeated words, repeated words.",
    "office ffi fl   ÄÖÜ äöü ß   ĄČĘĖĮŠŲŪŽ",
    "é e\u{301} Å A\u{30a}   X = -12.50 mm   Ø 25 ± 0.1   90°   m²",
    "  leading spaces; punctuation: (a/b), [0.25]!  ",
    "Fallback symbols: ⚙ ⏳ ⌘   object_012 / Surface A",
];
pub const SIZES: &[f32] = &[12.0, 14.0, 16.0, 18.0, 24.0];

/// Separate the matched-font page from the annotation's black-background browser probe.
enum Specimens {
    Reference,
    Nameplate,
}

/// The fixture owns its surface; production TextLane reuses the application's device.
#[wasm_bindgen]
pub struct TextQuality {
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    ctx: GpuCtx,
    config: wgpu::SurfaceConfiguration,
    lane: TextLane,
    depth: wgpu::TextureView,
    adapter: String,
}

#[wasm_bindgen]
impl TextQuality {
    /// Open browser WebGPU with baseline features and the same text pipeline as the viewer.
    #[wasm_bindgen(js_name = create)]
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<TextQuality, JsValue> {
        console_log::init_with_level(log::Level::Warn).ok();
        Self::open(canvas).await.map_err(js_error)
    }

    /// Render at actual browser DPR, or a positive explicit scale for deterministic fixtures.
    /// The returned JSON contains metrics; unsupported glyphs remain .notdef and are counted.
    pub fn render(
        &mut self,
        requested_scale: f64,
        selected: bool,
        background: u32,
    ) -> Result<String, JsValue> {
        self.render_inner(requested_scale, selected, background, Specimens::Reference)
            .map_err(js_error)
    }

    /// Render a centered white-on-black source annotation over white to expose both edges.
    pub fn render_nameplate(&mut self, requested_scale: f64) -> Result<String, JsValue> {
        self.render_inner(requested_scale, false, 0xffffff, Specimens::Nameplate)
            .map_err(js_error)
    }

    /// The specimen rows and layout positions are shared with the DOM reference.
    pub fn specimens(&self) -> Result<String, JsValue> {
        let labels = fixture_labels(false);
        let mut rows = Vec::new();
        for label in labels {
            let TextPlacement::Screen { left, top } = label.placement else {
                continue;
            };
            rows.push(
                serde_json::json!({ "text": label.text, "size": label.font_size,
                "lineHeight": label.line_height, "left": left, "top": top, "id": label.id }),
            );
        }
        serde_json::to_string(&rows).map_err(js_error)
    }

    /// Logical glyph origins and raster bitmap bounds, captured before the GPU pass.
    pub fn diagnostics(&mut self) -> Result<String, JsValue> {
        let glyphs = self.lane.document.diagnostics();
        let mut bitmaps = Vec::new();
        let mut raster = glyphon::SwashCache::new();
        let scale = self.config.height as f32 / self.canvas.client_height().max(1) as f32;
        for run in &self.lane.document.runs {
            let TextPlacement::Screen { left, top } = run.label.placement else {
                continue;
            };
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    let physical = glyph.physical((left * scale, top * scale), scale);
                    if let Some(image) =
                        raster.get_image_uncached(&mut self.lane.document.fonts, physical.cache_key)
                    {
                        let rect = [
                            physical.x + image.placement.left,
                            physical.y + (line.line_y * scale).round() as i32 - image.placement.top,
                            image.placement.width as i32,
                            image.placement.height as i32,
                        ];
                        bitmaps.push(
                            serde_json::json!({ "label": run.label.id, "glyph": glyph.glyph_id,
                            "rect": rect, "baseline": (top + line.line_y) * scale,
                            "origin": [(left + glyph.x) * scale, (top + line.line_y) * scale] }),
                        );
                    }
                }
            }
        }
        serde_json::to_string(&serde_json::json!({ "glyphs": glyphs, "bitmaps": bitmaps,
            "scale": scale, "atlas": "Glyphon internal R8Unorm coverage atlas; UV allocations are private" })).map_err(js_error)
    }

    /// Exercise disposal and regeneration on the same device, keeping the browser reference.
    pub fn reset(&mut self) {
        self.lane.release(&self.ctx);
    }
}

impl TextQuality {
    /// Device setup for the isolated regression page, without native backend or optional flags.
    async fn open(canvas: web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;
        let adapter_name = format!("{:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        device.on_uncaptured_error(std::sync::Arc::new(report_fixture_gpu_error));
        let capabilities = surface.get_capabilities(&adapter);
        let mut format = capabilities.formats[0];
        for candidate in capabilities.formats {
            if !candidate.is_srgb() {
                format = candidate;
                break;
            }
        }
        let ctx = GpuCtx { device, queue };
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 1,
            height: 1,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };
        let lane = TextLane::new(&ctx, Target { format, samples: 1 });
        let depth = fixture_depth(&ctx, [1, 1]);
        Ok(Self {
            canvas,
            surface,
            ctx,
            config,
            lane,
            depth,
            adapter: adapter_name,
        })
    }

    /// One complete invalidated frame, rendered into a full-resolution coverage-text target.
    fn render_inner(
        &mut self,
        requested_scale: f64,
        selected: bool,
        background: u32,
        specimens: Specimens,
    ) -> anyhow::Result<String> {
        let scale = if requested_scale > 0.0 {
            requested_scale
        } else {
            web_sys::window().map_or(1.0, window_scale)
        };
        anyhow::ensure!(
            scale.is_finite() && scale > 0.0 && scale <= 4.0,
            "fixture scale must be 0 (auto) or 0..4"
        );
        let logical = [
            self.canvas.client_width().max(1) as f64,
            self.canvas.client_height().max(1) as f64,
        ];
        let size = [
            (logical[0] * scale).round() as u32,
            (logical[1] * scale).round() as u32,
        ];
        if self.config.width != size[0] || self.config.height != size[1] {
            self.canvas.set_width(size[0]);
            self.canvas.set_height(size[1]);
            self.config.width = size[0];
            self.config.height = size[1];
            self.surface.configure(&self.ctx.device, &self.config);
            self.depth = fixture_depth(&self.ctx, size);
        }
        let labels = match specimens {
            Specimens::Reference => fixture_labels(selected),
            Specimens::Nameplate => vec![TextLabel {
                object: None,
                id: 100,
                text: "Source sphere Ø25".into(),
                font_size: 13.5,
                line_height: 19.5,
                color: [255; 4],
                placement: TextPlacement::Nameplate {
                    world: [0.0, 0.0, 0.5],
                    padding: [12.75, 3.0],
                    rounded: true,
                },
                clip: None,
            }],
        };
        self.lane.set_labels(labels)?;
        let frame = TextFrame {
            mvp: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            origin: [0.0; 3],
            framebuffer: size,
            logical,
            ortho_half_height: 0.0,
        };
        self.lane.prepare(&self.ctx, &frame)?;
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,
            error => anyhow::bail!("text fixture surface unavailable: {error:?}"),
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let rgb = [
            (background >> 16) & 255,
            (background >> 8) & 255,
            background & 255,
        ];
        let clear = wgpu::Color {
            r: rgb[0] as f64 / 255.0,
            g: rgb[1] as f64 / 255.0,
            b: rgb[2] as f64 / 255.0,
            a: 1.0,
        };
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("text-quality"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.lane.draw(&mut pass);
        }
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        let result = serde_json::json!({ "font": FONT_FAMILY, "fontRevision": self.lane.document.font_revision,
            "method": "Glyphon 0.11 / cosmic-text 0.18.2 advanced shaping / Swash coverage",
            "adapter": self.adapter, "framebuffer": size, "logical": logical, "effectiveScale": frame.scale()?,
            "stats": self.lane.stats });
        Ok(serde_json::to_string(&result)?)
    }
}

/// A deterministic specimen per size; CSS DOM reference consumes these exact coordinates.
fn fixture_labels(selected: bool) -> Vec<TextLabel> {
    let mut labels = Vec::new();
    let mut top = 12.0;
    for size in SIZES {
        let line_height = size * 1.5;
        let text = SAMPLES.join("\n");
        labels.push(TextLabel {
            object: None,
            id: labels.len() as u32,
            text,
            font_size: *size,
            line_height,
            color: if selected {
                [255, 255, 0, 255]
            } else {
                [255; 4]
            },
            placement: TextPlacement::Screen { left: 12.25, top },
            clip: None,
        });
        top += line_height * SAMPLES.len() as f32 + 28.0;
    }
    labels
}

/// A real read/write depth attachment makes fixture pipeline validation match production.
fn fixture_depth(ctx: &GpuCtx, size: [u32; 2]) -> wgpu::TextureView {
    texture_view(
        ctx,
        "text-quality.depth",
        &TextureSpec {
            size: (size[0], size[1]),
            format: wgpu::TextureFormat::Depth32Float,
            samples: 1,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        },
    )
}

/// Named adapter for the browser DPR accessor.
fn window_scale(window: web_sys::Window) -> f64 {
    window.device_pixel_ratio()
}
/// Preserve recoverable errors at the JavaScript boundary.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// Surface asynchronous validation errors in both browser automation and the fixture UI.
fn report_fixture_gpu_error(error: wgpu::Error) {
    log::error!("text fixture GPU validation: {error}");
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(root) = document.document_element()
    {
        let _ = root.set_attribute("data-gpu-error", &error.to_string());
    }
}
