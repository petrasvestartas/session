use wasm_bindgen::prelude::*; // Rust/WASM toolchain 
use wgpu::util::DeviceExt; // wgpu utility functions

#[wasm_bindgen]
pub struct Tutorial{
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    group: wgpu::BindGroup,
    scale: f64,
}

#[wasm_bindgen] // Everything in this block is exported to JavaScript
impl Tutorial {

    /// Ngotiate a presentation compatible browser adapter and build the first pipeline.
    pub async fn create(canvas: web_sys::HtmlCanvasElement) -> Result<Tutorial, JsValue> {
        console_error_panic_hook::set_once(); //  Better error messages in the console
        Self::open(canvas).await.map_err(js_error)
    }

    /// Clear and draw one invalid frame at full device-pixel resolution.
    pub fn render(&mut self, width: u32, height: u32, scale: f64) -> Result<String, JsValue> {
        self.render_frame(width, height, scale).map_err(js_error)
    }

    /// Apply one camera gesture. The fisrt GPU checkpoint intentionally has no camera yet.
    pub fn drag(&mut self, dx: f32, dy: f32, pan: bool){
        let _ = (dx, dy, pan);
    }

    /// Apply one cursor-centered camera zoom whe the camera checpoint is installed.
    pub fn zoom(&mut self, delta: f32, x: f64, y: f64){
        let _ = (delta, x, y);
    }
}

impl Tutorial {

    /// Create surface, device, bindings and a trinagle without any scene loader.
    async fn open(canvas: web_sys::HtmlCanvasElement) -> anyhow::Result<Self>{

        // ═══════════════════════════════════════════════════════════════════════════
        // Instance, surface, adapter, device
        // Instance = access to the graphics system
        // Surface  = where you want to display frames
        // Adapter  = suitable GPU/backend for that surface
        // Device   = logical GPU connection you actually use
        // Queue    = submits commands to that device
        // ═══════════════════════════════════════════════════════════════════════════

        // Starting point to talk to GPU through wgpu.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // The surface is the thing the wgpu will render into.
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;

        // Select gpu device.
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await?;

        // Print information about selected wgpu
        log::info!("tutorial adapter: {:?}", adapter.get_info());

        // Device - logical connection to wgpu, Queue - is used to send command to the gpu
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;

        // Registers wgpu erros
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));

        // ═══════════════════════════════════════════════════════════════════════════
        // Surface configuration and the camera unform
        // ═══════════════════════════════════════════════════════════════════════════

        // What can this surface support when used with this GPU adapter?
        let caps = surface.get_capabilities(&adapter);

        // How the surface should behave?
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // You will render into it
            format: caps.formats[0], // pixel color format [Bgra8UnormSrgb, Rgba8UnormSrgb, ...]
            width: 1, // canvas size in pixes
            height: 1,
            present_mode: caps.present_modes[0], // how rendered frames are presented to the screen
            alpha_mode: caps.alpha_modes[0], // how transparency is handled
            view_formats: vec![], // optional extra texture formats
            desired_maximum_frame_latency: 2, // roughly how many frames maybe be queued ahead
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
            contents: bytemuck::cast_slice(&identity), // converts raw bytes so the GPU can receive it
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // GPU shader reads it | CPU / copy operation can update it
        });

        // Layout desribing what resource the shader expects
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("camera"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0, // the shader will receive one uniform buffer, -> @group(0) @binding(0)
                visibility: wgpu::ShaderStages::VERTEX, // only the vertex shader can use it
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform,  // the resource must be a uniform buffer
                    has_dynamic_offset: false, // This binding always reads from the same fixed place, if true we could read multiple cameras
                    min_binding_size: None  // No minimum buffer size is enforced here
                },
                count: None // This is one buffer, not arrays of buffers
            }],
        });

        // What actual buffer you gave to it.
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, // must match the binding:0 from BindGroupLayout
                resource: uniform.as_entire_binding(), // use the whole uniform buffer as the resource for this binding 
            }],
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // Shader module and pipeline
        // ═══════════════════════════════════════════════════════════════════════════

        // Load shader file and create a GPU shader module from it
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{ // create a shader module that wgpu can use later in a rende pipeline
            label: Some("first triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/first.wgsl").into()),
        });

        // This defines what bind groups the render pipeline will use, there can be a camera layout, material layout, lights layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("first triangle"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0, // not using any immediate data here
        });

        // Create render pipeline: the full set of rules the GPU uses to draw
        // Which shaders to run, what inputs they get, how triangles are interpreted, how colors are written, depth/MSAA settings
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("first triangle"),
            layout: Some(&pipeline_layout), // use the resource layout - camera uniform
            vertex: wgpu::VertexState { // uses vs_main from WGSL file as the vertex shader
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // no vertex buffer is defined here, as triangle is generated in the shader
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"), // fs_main as the fragment shader
                targets: &[Some(wgpu::ColorTargetState { // the fragment shader outputs color same format as the surface
                    format: config.format,
                    blend: None, // no transparencey blending
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

        // Create an instance of the struct, ok is needed to return also the error message Err(...)
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

    /// Render once, upload the camera transform, encode the lane, submit and present.
    fn render_frame(&mut self, width: u32, height: u32, scale: f64) -> anyhow::Result<String>{
        

        anyhow::ensure!(scale.is_finite() && scale > 0.0, "invalid device scale");

        self.scale = scale;

        // Convert the logival size to real pixel size, e.g. width = 800, scale = 2.0
        let width = (f64::from(width.max(1)) * scale).round() as u32;
        let height = (f64::from(height.max(1))* scale).round() as u32;

        // Resize step
        if self.canvas.width() != width || self.canvas.height() != height || self.config.width == 1 {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }

        // Hand you the browser's canvas image itself.
        let output = match self.surface.get_current_texture(){
            wgpu::CurrentSurfaceTexture::Success(output) | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            error => anyhow::bail!("surface unavailable: {error:?}"),
        };

        // Create a view GPU memory.
        let view = output.texture.create_view(&Default::default());

        // The GPU does not execute call ony by one, you record a list of commands and handle this recorder/encoder.
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("first frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{ // draw inzo the canvas texture view from above
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { // before drawing fill with the color
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
            pass.draw(0..3, 0..1); // run the vertex shader for one triangle
        }

        // close the recording and sends to GPU
        self.queue.submit([encoder.finish()]);

        // tells the browser the frame texture is finishes, the canvas now shows it
        output.present();

        // Reutnr a status report for JavaScript
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

/// A shader validation failure must fail the browser checkpoint rather than look like success.
fn gpu_error(error: wgpu::Error) {
    panic!("tutorial WebGPU error: {error}");
}