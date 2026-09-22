// --8<-- [start:step-2a]
use session_rust::AABB;
use session_rust::Point;
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
pub mod camera;
// --8<-- [end:step-2a]

/// Everything needed to draw one frame.
#[wasm_bindgen]
pub struct Tutorial {
    canvas: web_sys::HtmlCanvasElement, // the page canvas
    surface: wgpu::Surface<'static>, // where frames go
    device: wgpu::Device, // makes GPU objects
    queue: wgpu::Queue, // sends commands
    config: wgpu::SurfaceConfiguration, // canvas size and format
    pipeline: wgpu::RenderPipeline, // the drawing rules
    // --8<-- [start:step-2b]
    uniform: wgpu::Buffer, // the camera matrix on the GPU
    group: wgpu::BindGroup, // the camera buffer, bound
    scale: f64, // device pixels per CSS pixel
    camera: camera::Camera, // orbit, pan and zoom state
    // --8<-- [end:step-2b]
}

#[wasm_bindgen]
impl Tutorial {
    /// Open the GPU and build the pipeline.
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<Tutorial, JsValue> {
        // panics print to the console
        console_error_panic_hook::set_once();
        Self::open(canvas).await.map_err(js_error)
    }

    /// Draw one frame at device-pixel size.
    pub fn render(&mut self, width: u32, height: u32, scale: f64) -> Result<String, JsValue> {
        self.render_frame(width, height, scale).map_err(js_error)
    }

    /// Orbit, or pan when `pan` is set.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool) {
        // --8<-- [start:step-2c]
        if pan {
            self.camera.pan(dx, dy);
        } else {
            self.camera.orbit(dx, dy);
        }
    }

    /// Zoom at the cursor.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64) {
        self.camera.zoom_at(
            delta,
            (x * self.scale, y * self.scale),
            (self.config.width as f64, self.config.height as f64),
        );
        // --8<-- [end:step-2c]
    }
}

impl Tutorial {
    /// Open the GPU and build the triangle pipeline.
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
        log::info!("tutorial adapter: {:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));
        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: 1,
            height: 1,
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let identity = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera matrix"),
            contents: bytemuck::cast_slice(&identity),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("first triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/first.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("first triangle"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("first triangle"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        // --8<-- [start:step-2d]
        let mut camera = camera::Camera::new();
        camera.unit = camera::Unit::Meters;
        camera.set_view(camera::View::Top);
        camera.fit(
            &AABB::from_points(
                &[Point::new(-1.5, -1.0, -0.1), Point::new(1.5, 1.0, 0.1)],
                0.0,
            ),
            1.5,
        );
        // --8<-- [end:step-2d]

        Ok(Self {
            canvas,
            surface,
            device,
            queue,
            config,
            pipeline,
            // --8<-- [start:step-2e]
            uniform,
            group,
            scale: 1.0,
            camera,
            // --8<-- [end:step-2e]
        })
    }

    /// Draw one frame and present it.
    fn render_frame(&mut self, width: u32, height: u32, scale: f64) -> anyhow::Result<String> {
        anyhow::ensure!(scale.is_finite() && scale > 0.0, "invalid device scale");
        self.scale = scale;
        let width = (f64::from(width.max(1)) * scale).round() as u32;
        let height = (f64::from(height.max(1)) * scale).round() as u32;

        if self.canvas.width() != width || self.canvas.height() != height || self.config.width == 1
        {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }

        // --8<-- [start:step-2f]
        let mvp = self.camera.view_proj_anchored(
            width as f64 / height as f64,
            &session_rust::Point::new(0.0, 0.0, 0.0),
        );
        self.queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&mvp.to_f32()));
            // --8<-- [end:step-2f]
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            error => anyhow::bail!("surface unavailable: {error:?}"),
        };
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("first frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.025,
                            g: 0.035,
                            b: 0.055,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.group, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
        output.present();
        // --8<-- [start:step-2g]
        Ok(serde_json::json!({"stage": 2, "objects": 1, "width":width,"height":height,"scale":scale,"drawn":true}).to_string())
        // --8<-- [end:step-2g]
    }
}

/// Keep initialization/render errors visible at the browser boundary.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// A GPU error panics, never a black canvas.
fn gpu_error(error: wgpu::Error) {
    panic!("tutorial WebGPU error: {error}");
}
