// --8<-- [start:step-1]
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

/// Everything needed to draw one frame.
#[wasm_bindgen]
pub struct Tutorial {
    canvas: web_sys::HtmlCanvasElement, // the page canvas
    surface: wgpu::Surface<'static>,    // where frames go
    device: wgpu::Device,               // makes GPU objects
    queue: wgpu::Queue,                 // sends commands
    config: wgpu::SurfaceConfiguration, // canvas size and format
    pipeline: wgpu::RenderPipeline,     // the drawing rules
    group: wgpu::BindGroup,             // the camera buffer, bound
    scale: f64,                         // device pixels per CSS pixel
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

    /// Orbit or pan; no camera yet, so nothing happens.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool) {
        let _ = (dx, dy, pan);
    }

    /// Zoom at the cursor; no camera yet.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64) {
        let _ = (delta, x, y);
    }
}
// --8<-- [end:step-1]
// --8<-- [start:step-2]

impl Tutorial {
    /// Open the GPU and build the triangle pipeline.
    async fn open(canvas: web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {

        // Instance, surface, adapter, device

        // the WebGPU API itself
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // the canvas, as wgpu sees it
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;

        // one GPU that can draw on this surface
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await?;

        // log which GPU was chosen
        log::info!("tutorial adapter: {:?}", adapter.get_info());

        // Device makes GPU objects, queue sends commands
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;

        // GPU errors panic, see gpu_error
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));
        // --8<-- [end:step-2]
        // --8<-- [start:step-3]

        // Surface configuration and the camera uniform

        // What formats this GPU can show on the canvas
        let caps = surface.get_capabilities(&adapter);

        // how the canvas texture is set up
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // You will render into it
            format: caps.formats[0], // pixel format, e.g. Bgra8UnormSrgb
            width: 1, // canvas size in pixels
            height: 1,
            present_mode: caps.present_modes[0], // how frames reach the screen
            alpha_mode: caps.alpha_modes[0], // how transparency is handled
            view_formats: vec![], // optional extra texture formats
            desired_maximum_frame_latency: 2, // frames queued ahead
        };

        // 4x4 identity matrix
        let identity = [
            1.0f32, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        // Create a GPU buffer containing that matrix
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera matrix"),
            contents: bytemuck::cast_slice(&identity), // the matrix as bytes
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // shader reads it, CPU can update it
        });

        // what the shader expects at group 0
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("camera"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0, // @group(0) @binding(0) in the shader
                visibility: wgpu::ShaderStages::VERTEX, // only the vertex shader can use it
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,  // the resource must be a uniform buffer
                    has_dynamic_offset: false, // always reads from one place
                    min_binding_size: None  // no minimum size
                },
                count: None // one buffer, not an array
            }],
        });

        // the buffer that fills that layout
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, // must match the binding:0 from BindGroupLayout
                resource: uniform.as_entire_binding(), // the whole uniform buffer
            }],
        });

        // --8<-- [end:step-3]
        // --8<-- [start:step-4]

        // Shader module and pipeline

        // Compile the shader file
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{ // the compiled shader
            label: Some("first triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/first.wgsl").into()),
        });

        // Which bind groups the pipeline uses
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("first triangle"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0, // not using any immediate data here
        });

        // The pipeline: every rule the GPU draws with
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("first triangle"),
            layout: Some(&pipeline_layout), // the camera bind group
            vertex: wgpu::VertexState { // the vertex shader
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // no vertex buffer, the shader makes the points
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"), // fs_main as the fragment shader
                targets: &[Some(wgpu::ColorTargetState { // color output, same format as the canvas
                    format: config.format,
                    blend: None, // no transparency blending
                    write_mask: wgpu::ColorWrites::ALL, // allow writing R, G, B, A
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None, // no depth buffer
            multisample: Default::default(), // no special MSAA settings
            multiview_mask: None,
            cache: None,
        });

        Ok(Self {
            canvas,
            surface,
            device,
            queue,
            config,
            pipeline,
            group,
            scale: 1.0,
        })
    }

    // --8<-- [end:step-4]
    // --8<-- [start:step-5]

    /// Draw one frame and present it.
    fn render_frame(&mut self, width: u32, height: u32, scale: f64) -> anyhow::Result<String>{

        anyhow::ensure!(scale.is_finite() && scale > 0.0, "invalid device scale");

        self.scale = scale;

        // CSS size times device scale = real pixels
        let width = (f64::from(width.max(1)) * scale).round() as u32;
        let height = (f64::from(height.max(1))* scale).round() as u32;

        // resize only when the size changed
        if self.canvas.width() != width || self.canvas.height() != height || self.config.width == 1 {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }

        // this frame's canvas texture
        let output = match self.surface.get_current_texture(){
            wgpu::CurrentSurfaceTexture::Success(output) | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            error => anyhow::bail!("surface unavailable: {error:?}"),
        };

        // a view of that texture
        let view = output.texture.create_view(&Default::default());

        // Commands are recorded first, then sent together
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("first frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{ // draw into the canvas texture
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { // clear to this color first
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
            pass.set_pipeline(&self.pipeline); // the drawing rules
            pass.set_bind_group(0, &self.group, &[]); // attach the uniform buffer
            pass.draw(0..3, 0..1); // run the vertex shader for one triangle
        }

        // send the recorded commands
        self.queue.submit([encoder.finish()]);

        // the canvas shows the finished frame
        output.present();

        // a status report for JavaScript
        Ok(serde_json::json!({
            "stage" : 1,
            "objects" : 1,
            "width" : width,
            "height" : height,
            "scale" : scale,
            "drawn" : true,
        }).to_string())

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
// --8<-- [end:step-5]
