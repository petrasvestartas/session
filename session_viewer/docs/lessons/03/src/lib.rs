use session_rust::AABB;
use session_rust::Point;
use wasm_bindgen::prelude::*; // Rust/WASM toolchain
use wgpu::util::DeviceExt; // wgpu utility functions
pub mod camera;
// --8<-- [start:step-5a]
pub mod engine;
pub mod scene;
// --8<-- [end:step-5a]

/// Everything needed to draw one frame: the browser owns the canvas, this struct owns the GPU.
#[wasm_bindgen]
pub struct Tutorial {
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    uniform: wgpu::Buffer, // the camera matrix on the GPU
    group: wgpu::BindGroup,
    scale: f64,
    camera: camera::Camera, // orbit, pan and zoom state
    // --8<-- [start:step-5b]
    objects: [scene::SourceObject; 2],
    // --8<-- [end:step-5b]
}

#[wasm_bindgen] // Everything in this block is exported to JavaScript
impl Tutorial {
    /// Negotiate a presentation compatible browser adapter and build the first pipeline.
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<Tutorial, JsValue> {
        console_error_panic_hook::set_once(); // Better error messages in the console
        Self::open(canvas).await.map_err(js_error)
    }

    /// Clear and draw one frame at full device-pixel resolution.
    pub fn render(&mut self, width: u32, height: u32, scale: f64) -> Result<String, JsValue> {
        self.render_frame(width, height, scale).map_err(js_error)
    }

    /// Apply one camera gesture. The first GPU checkpoint intentionally has no camera yet.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool) {
        if pan {
            self.camera.pan(dx, dy);
        } else {
            self.camera.orbit(dx, dy);
        }
    }

    /// Apply one cursor-centered camera zoom when the camera checkpoint is installed.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64) {
        self.camera.zoom_at(
            delta,
            (x * self.scale, y * self.scale),
            (self.config.width as f64, self.config.height as f64),
        );
    }
}

// A second impl block without #[wasm_bindgen]: these helpers return Rust errors JavaScript cannot see.
impl Tutorial {
    /// Create surface, device, bindings and a triangle without any scene loader.
    async fn open(canvas: web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {
        // Starting point to talk to GPU through wgpu.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        // The surface is the thing the wgpu will render into.
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;
        // Select gpu device. The ? returns early with the error, so the next line always has an adapter.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;
        // Print information about selected wgpu
        log::info!("tutorial adapter: {:?}", adapter.get_info());
        // Device - logical connection to wgpu, Queue - is used to send command to the gpu
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        // Registers wgpu errors
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));
        // What can this surface support when used with this GPU adapter?
        let caps = surface.get_capabilities(&adapter);
        // How the surface should behave?
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // You will render into it
            format: caps.formats[0], // pixel color format [Bgra8UnormSrgb, Rgba8UnormSrgb, ...]
            width: 1,                // canvas size in pixels, the first frame replaces this
            height: 1,
            present_mode: caps.present_modes[0], // how rendered frames are presented to the screen
            alpha_mode: caps.alpha_modes[0],     // how transparency is handled
            view_formats: vec![],                // optional extra texture formats
            desired_maximum_frame_latency: 2,    // roughly how many frames may be queued ahead
        };
        // Starting contents of the camera buffer: a matrix that changes nothing.
        let identity = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        // A uniform buffer is one small block of GPU memory that every vertex reads, the same for all of them.
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera matrix"),
            contents: bytemuck::cast_slice(&identity), // converts raw bytes so the GPU can receive it
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // GPU shader reads it | CPU / copy operation can update it
        });
        // Layout describing what resource the shader expects
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera"),
            // --8<-- [start:step-5c]
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, // the shader will receive one uniform buffer, -> @group(0) @binding(0)
                    visibility: wgpu::ShaderStages::VERTEX, // only the vertex shader can use it
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform, // the resource must be a uniform buffer
                        has_dynamic_offset: false, // This binding always reads from the same fixed place, if true we could read multiple cameras
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let records = [scene::objects()[0].row, scene::objects()[1].row];
        let rows = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("source instances"),
            contents: bytemuck::cast_slice(&records),
            usage: wgpu::BufferUsages::STORAGE,
        });
        // What actual buffer you gave to it.
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0, // must match the binding:0 from BindGroupLayout
                    resource: uniform.as_entire_binding(), // use the whole uniform buffer as the resource for this binding
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: rows.as_entire_binding(),
                },
            ],
            // --8<-- [end:step-5c]
        });
        // include_str! pastes the shader text into the binary at compile time, so a missing file stops the build.
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("first triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/first.wgsl").into()),
        });
        // This defines what bind groups the render pipeline will use, there can be a camera layout, material layout, lights layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("first triangle"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0, // not using any immediate data here
        });
        // Create render pipeline: the full set of rules the GPU uses to draw, built and checked once.
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("first triangle"),
            layout: Some(&pipeline_layout), // use the resource layout - camera uniform
            vertex: wgpu::VertexState {
                // uses vs_main from WGSL file as the vertex shader
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // no vertex buffer is defined here, as triangle is generated in the shader
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"), // fs_main as the fragment shader
                targets: &[Some(wgpu::ColorTargetState {
                    // the fragment shader outputs color same format as the surface
                    format: config.format,
                    blend: None,                        // no transparency blending
                    write_mask: wgpu::ColorWrites::ALL, // allow writing R, G, B, A
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None, // no depth buffer - front / behind testing
            multisample: Default::default(), // no special MSAA settings
            multiview_mask: None,
            cache: None,
        });
        // fit() picks the distance where this box fills the view, so the triangle is framed at startup.
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
        // --8<-- [start:step-5d]
        let objects = scene::objects();
        // --8<-- [end:step-5d]
        // Create an instance of the struct, Ok is needed to return also the error message Err(...)
        Ok(Self {
            canvas,
            surface,
            device,
            queue,
            config,
            pipeline,
            uniform,
            group,
            scale: 1.0,
            camera,
            // --8<-- [start:step-5e]
            objects,
            // --8<-- [end:step-5e]
        })
    }

    /// Render once, upload the camera transform, encode the lane, submit and present.
    fn render_frame(&mut self, width: u32, height: u32, scale: f64) -> anyhow::Result<String> {
        anyhow::ensure!(scale.is_finite() && scale > 0.0, "invalid device scale");
        self.scale = scale;
        // Convert the logical size to real pixel size, e.g. width = 800, scale = 2.0
        let width = (f64::from(width.max(1)) * scale).round() as u32;
        let height = (f64::from(height.max(1)) * scale).round() as u32;

        // Resize step
        if self.canvas.width() != width || self.canvas.height() != height || self.config.width == 1
        {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }

        // The camera changed since the last frame, so its matrix is written into the uniform buffer again.
        let mvp = self.camera.view_proj_anchored(
            width as f64 / height as f64,
            &session_rust::Point::new(0.0, 0.0, 0.0),
        );
        self.queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&mvp.to_f32()));
        // Hand you the browser's canvas image itself.
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            error => anyhow::bail!("surface unavailable: {error:?}"),
        };
        // A view is a handle onto that texture saying how to read or write it; the pass draws through it.
        let view = output.texture.create_view(&Default::default());
        // The GPU does not execute calls one by one, you record a list of commands and hand over this encoder.
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // The inner braces end the pass early, because it borrows the encoder and finish() needs it back.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("first frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    // draw into the canvas texture view from above
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // before drawing fill with the color
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.025,
                            g: 0.035,
                            b: 0.055,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None, // no depth buffer
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline); // which shaders + vertex layout + blend state to use
            pass.set_bind_group(0, &self.group, &[]); // attach the uniform buffer
// --8<-- [start:step-5f]

            for row in 0..self.objects.len() as u32 {
                pass.draw(0..3, row..row + 1);
            }
        }
        // close the recording and send to GPU
        self.queue.submit([encoder.finish()]);
        // tells the browser the frame texture is finished, the canvas now shows it
        output.present();
        Ok(serde_json::json!({"stage": 3, "objects": self.objects.len(), "width":width,"height":height,"scale":scale,"drawn":true}).to_string())
        // --8<-- [end:step-5f]
    }
}

/// Keep initialization/render errors visible at the browser boundary.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// A shader validation failure must fail the browser checkpoint rather than look like success.
fn gpu_error(error: wgpu::Error) {
    panic!("tutorial WebGPU error: {error}");
}
