// --8<-- [start:step-2a]
use session_rust::AABB;
use session_rust::Point;
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt; // a trait that adds create_buffer_init to Device; its methods work only once imported
pub mod camera;
// --8<-- [end:step-2a]

/// Everything needed to draw one frame: the browser owns the canvas, this struct owns the GPU.
#[wasm_bindgen] // JavaScript sees this struct as a class
pub struct Tutorial {
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>, // 'static: borrows nothing, so it may live as long as the struct
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    // --8<-- [start:step-2b]
    uniform: wgpu::Buffer, // kept, so every frame can rewrite the camera matrix
    group: wgpu::BindGroup,
    scale: f64, // device pixels per CSS pixel, 2.0 on most phones
    camera: camera::Camera,
    // --8<-- [end:step-2b]
}

#[wasm_bindgen] // every pub fn in this block becomes a JavaScript method
impl Tutorial {
    /// async: JavaScript receives a Promise, because asking the browser for a GPU takes a moment.
    #[cfg(target_arch = "wasm32")] // build this only for the browser: a PC has no canvas, and `cargo xtest` builds for the PC
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<Tutorial, JsValue> {
        console_error_panic_hook::set_once();
        Self::open(canvas).await.map_err(js_error) // JavaScript cannot read a Rust error, so it becomes text
    }

    /// The page calls this after loading, on resize and on every drag; it returns a JSON status line.
    pub fn render(&mut self, width: u32, height: u32, scale: f64) -> Result<String, JsValue> {
        self.render_frame(width, height, scale).map_err(js_error)
    }

    /// dx, dy: CSS pixels since the last pointer event.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool) {
        // --8<-- [start:step-2c]
        if pan {
            self.camera.pan(dx, dy);
        } else {
            self.camera.orbit(dx, dy);
        }
    }

    /// delta in wheel steps; x, y: the cursor in CSS pixels.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64) {
        self.camera.zoom_at(
            delta,
            (x * self.scale, y * self.scale), // CSS to device pixels, the unit of config.width
            (self.config.width as f64, self.config.height as f64),
        );
        // --8<-- [end:step-2c]
    }
}

// A second impl block without #[wasm_bindgen]: these helpers return Rust errors JavaScript cannot see.
impl Tutorial {
    /// anyhow::Result = Ok(value) or Err(any error with a message).
    #[cfg(target_arch = "wasm32")]
    async fn open(canvas: web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {
        // Instance = the WebGPU API itself
        // Surface  = the canvas, as something the GPU can draw into
        // Adapter  = one physical GPU that can draw to that surface
        // Device   = your connection to that GPU; it creates every GPU object
        // Queue    = where finished command lists are sent

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU, // WebGPU only, no WebGL fallback
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        // `?` returns the error to the caller at once, so below this line the surface exists.
        // clone() makes a second handle to the same canvas; the struct keeps the first.
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default() // every field not written above keeps its default
            })
            .await?; // await: wait for the browser's answer without freezing the page
        log::info!("tutorial adapter: {:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        // A validation error would otherwise only log a warning and leave the canvas black.
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));
        // What this canvas supports on this GPU; index 0 of each list is the preferred choice.
        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // a render pass may draw into it
            format: caps.formats[0], // e.g. Bgra8UnormSrgb: 4 bytes per pixel, blue first
            width: 1,                // 1 x 1 for now; the first frame sets the real size
            height: 1,
            present_mode: caps.present_modes[0], // on the web always Fifo: one frame per display refresh
            alpha_mode: caps.alpha_modes[0],     // whether the canvas blends with the page behind it
            view_formats: vec![],
            desired_maximum_frame_latency: 2,    // at most 2 frames queued ahead of the screen
        };
        // The identity matrix: points pass through unchanged until the first frame writes the camera.
        let identity = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        // A uniform buffer is one small block of GPU memory that every vertex reads, the same for all of them.
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera matrix"),
            contents: bytemuck::cast_slice(&identity), // the same 64 bytes seen as &[u8]; nothing is copied
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // shaders read it; write_buffer may overwrite it
        });
        // A bind group layout is the shape of the shader's inputs; the bind group below fills it with real buffers.
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0, // @binding(0) in first.wgsl
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false, // true would let one buffer hold several cameras, chosen per draw
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,                            // fills the layout entry with the same number
                resource: uniform.as_entire_binding(),
            }],
        });
        // include_str! pastes the shader text into the binary at compile time, so a missing file stops the build.
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("first triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/first.wgsl").into()),
        });
        // One bind group layout per @group number; this pipeline has only group 0.
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("first triangle"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        // Create render pipeline: the full set of rules the GPU uses to draw, built and checked once.
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("first triangle"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // the function name in first.wgsl
                buffers: &[], // no vertex buffer: the shader makes its three corners itself
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // must equal the canvas format, or the pass is rejected
                    format: config.format,
                    blend: None,                        // overwrite the pixel, no transparency
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(), // a list of triangles, none culled
            depth_stencil: None, // no depth test yet: a later draw simply covers an earlier one
            multisample: Default::default(), // 1 sample per pixel, no anti-aliasing
            multiview_mask: None,
            cache: None,
        });
        // --8<-- [start:step-2d]
        // Back off until a 3 x 2 m box around the triangle fills the view.
        let mut camera = camera::Camera::new();
        camera.unit = camera::Unit::Meters;
        camera.set_view(camera::View::Top);
        camera.fit(
            &AABB::from_points(
                &[Point::new(-1.5, -1.0, -0.1), Point::new(1.5, 1.0, 0.1)],
                0.0,
            ),
            1.5, // width / height, a guess until the first frame
        );
        // --8<-- [end:step-2d]

        // Create an instance of the struct; Ok is needed to return also the error message Err(...).
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

    /// One frame: resize if needed, record the commands, submit them, show the result.
    fn render_frame(&mut self, width: u32, height: u32, scale: f64) -> anyhow::Result<String> {
        anyhow::ensure!(scale.is_finite() && scale > 0.0, "invalid device scale"); // returns Err when false
        self.scale = scale;
        // Convert the logical size to real pixel size, e.g. width = 800, scale = 2.0 gives 1600.
        let width = (f64::from(width.max(1)) * scale).round() as u32;
        let height = (f64::from(height.max(1)) * scale).round() as u32;

        // Reconfigure only when the size changed: it reallocates the canvas textures.
        if self.canvas.width() != width || self.canvas.height() != height || self.config.width == 1
        {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }

        // --8<-- [start:step-2f]
        // Overwrite the 64-byte uniform; the queue copies it before this frame's draw runs.
        let mvp = self.camera.view_proj_anchored(
            width as f64 / height as f64,
            &session_rust::Point::new(0.0, 0.0, 0.0),
        );
        self.queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&mvp.to_f32()));
            // --8<-- [end:step-2f]
        // The texture the canvas shows next; any result other than these two ends the frame with an error.
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
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // fill with dark blue before drawing; channels run 0 to 1, not 0 to 255
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.025,
                            g: 0.035,
                            b: 0.055,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store, // keep the pixels, so they can be shown
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.group, &[]); // 0 = @group(0) in the shader; &[] = no dynamic offsets
            pass.draw(0..3, 0..1); // vertices 0..3, instances 0..1: the vertex shader runs three times
        }
        // The GPU starts working only now.
        self.queue.submit([encoder.finish()]);
        // The canvas shows the new frame at the next display refresh.
        output.present();
        // --8<-- [start:step-2g]
        Ok(serde_json::json!({"stage": 2, "objects": 1, "width":width,"height":height,"scale":scale,"drawn":true}).to_string())
        // --8<-- [end:step-2g]
    }
}

/// `impl Display` accepts any error type that can print itself.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

/// Panic, so a validation error appears in the console instead of a silently black canvas.
fn gpu_error(error: wgpu::Error) {
    panic!("tutorial WebGPU error: {error}");
}
