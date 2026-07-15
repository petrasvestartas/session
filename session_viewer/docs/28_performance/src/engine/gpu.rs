//! `Gpu` — our handle to the graphics card and the lowest layer of the viewer (ARCHITECTURE.md §1).
//!
//! It owns the three things wgpu needs to draw:
//!   • `device` — makes GPU resources (textures, buffers, pipelines)
//!   • `queue`  — sends work to the GPU
//!   • `surface`— the canvas pixels we present each frame
//! plus the `config` describing the surface size/format. It knows nothing app-specific — its whole
//! job is "hand me a cleared frame". Higher layers sit on top and only talk to this.

use crate::engine::pipelines::Pipelines;
use crate::engine::pipelines::build::MSAA_SAMPLES;
use crate::engine::performance::Performance;
use session_rust::{Color, Mesh, Xform, RenderVertex};

pub struct Gpu {
    pub surface: wgpu::Surface<'static>,     // Screen to draw pixels on.
    pub device: wgpu::Device,                // Handle to the GPU, used to create resources (textures, buffers, pipelines).
    pub queue: wgpu::Queue,                  // Used to submit work to the GPU (draw calls, resource updates).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
    pub pipelines: Pipelines,
    pub mvp_buffer: wgpu::Buffer,            // Camera matrix
    pub mvp_bind_group: wgpu::BindGroup,     // Camera matrix
    pub meshes: Vec<Mesh>,
    pub time: f32,
    pub time_buffer: wgpu::Buffer,
    pub time_bind_group: wgpu::BindGroup,
    pub depth_view: wgpu::TextureView,
    pub edge_buffers: Vec<(wgpu::Buffer, u32)>,
    pub msaa_view: wgpu::TextureView,
    pub performance: Performance,
}

impl Gpu {
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        

        // 1. Instance — the driver entry point. WebGPU first, WebGL2 fallback in the browser.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: if cfg!(target_arch = "wasm32") {
                wgpu::Backends::BROWSER_WEBGPU
            } else {
                wgpu::Backends::PRIMARY //Vulkan / Metal / DX12 for native selftest
            },
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // 2. Surface — the drawable canvas. 3. Adapter — a physical GPU compatible with it.
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // 4. Device (creates resources) + Queue (submits work). WebGL2 limits for browser reach.
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),  // unlock the WEBGpu storage buffers
                memory_hints: Default::default(),
                ..Default::default()
            })
            .await?;
    
        device.on_uncaptured_error(std::sync::Arc::new(|e|{ log::error!("wgpu on_uncaptured_error: {e}") }));

        // 5. Configure the surface: pixel format (prefer sRGB), size, vsync.
        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Depth and MSAA
        let depth_view = Self::create_depth_view(&device, &config);
        let msaa_view = Self::create_msaa_view(&device, &config);

        // Camera MVP uniform - buffer + layout + bind group (group 0)
        use wgpu::util::DeviceExt;
        let mvp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mvp_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("mvp.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu:: BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],    
        });

        let mvp_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &mvp_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        // Time Uniform
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("time.buffer"),
            contents: bytemuck::bytes_of(&0.0f32),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("time.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None},
                count: None,
            }],
        });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("time.bind_group"),
            layout: &time_layout,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: time_buffer.as_entire_binding() }],
        });





        // Pipelines
        let pipelines = Pipelines::new(&device, config.format, &mvp_layout, &time_layout);

        // Create three corners to transfer vertex data to the shader
        const H: f64 = 1000.0; // half size cube
        let mut mesh = Mesh::create_box(H, H, H);
        mesh.set_objectcolor(Color::new(0.2, 0.5, 0.9, 1.0));

        let mut flat = Mesh::create_dodecahedron(500.0);
        flat.transform(&Xform::translation(-1600.0, 0.0, 0.0));
        flat.set_objectcolor(Color::new(0.9, 0.5, 0.2, 1.0));

        let mut smooth = Mesh::create_dodecahedron(500.0);
        smooth.transform(&Xform::translation(1600.0, 0.0, 0.0));
        smooth.set_objectcolor(Color::new(0.9, 0.5, 0.2, 1.0));
        smooth.compute_vertex_normals(); // area-weighted, stored per vertex

        let meshes = vec![mesh, flat, smooth];

        let mut edge_buffers: Vec<(wgpu::Buffer, u32)> = Vec::new();
        for mesh in &meshes {
            let ec = mesh.get_linecolors().first().cloned().unwrap_or(Color::black());
            let mut verts: Vec<RenderVertex> = Vec::new();
            for (a,b) in mesh.edges(){
                let pa = mesh.vertex_point(a).unwrap();
                let pb = mesh.vertex_point(b).unwrap();
                verts.push(RenderVertex::point(pa, &ec ));
                verts.push(RenderVertex::point(pb, &ec ));
            }
            let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("edges.vbo"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
            edge_buffers.push((vbo, verts.len() as u32));
        }



        // Output
        log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self { 
            surface, 
            device, 
            queue, 
            config, 
            pipelines, 
            mvp_buffer, 
            mvp_bind_group, 
            meshes,
            time: 0.0, 
            time_buffer, 
            time_bind_group,
            depth_view,
            edge_buffers,
            msaa_view,
            performance: Performance::new()
         })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.depth_view = Self::create_depth_view(&self.device, &self.config);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config);
        }
    }

    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
    /// (mesh, line, grid, …) get added here in later chapters.
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {

        // Time for triangle wgsl buffer.
        self.time += 1.0 / 60.0;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&view_proj.to_f32()));

        // wgpu 29: get_current_texture() returns an enum, not a Result.
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => { self.surface.configure(&self.device, &self.config); return Ok(()); }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("clear encoder"),
        });

        let mut draws = 0u32;

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_view,
                    resolve_target: Some(&view),
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: &self.depth_view, 
                    depth_ops: Some(
                        wgpu::Operations{load: wgpu::LoadOp::Clear(0.0),
                        store:wgpu::StoreOp::Store,
                    }), 
                    stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
       

            // Pipelines - sequence of drawing is important

            // Background
            pass.set_pipeline(&self.pipelines.background);
            pass.draw(0..3, 0..1); 
            draws += 1;

            // Grid first as the depth writes are off, all objects paints over it
            pass.set_pipeline(&self.pipelines.grid);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.draw(0..50, 0..1);
            draws += 1;

            // Meshes - coordinates, colors and normals are inside the gb.vbo computed
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            for mesh in &mut self.meshes{
                let gm = mesh.gpu_mesh(&self.device); // build and upload once
                pass.set_vertex_buffer(0, gm.vbo.slice(..));
                pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..gm.index_count, 0, 0..1);
                draws += 1;
            }

            // Mesh Edges
            pass.set_pipeline(&self.pipelines.edges);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            for (vbo, count) in &self.edge_buffers{
                pass.set_vertex_buffer(0, vbo.slice(..));
                pass.draw(0..*count, 0..1);
                draws += 1;
            }
        }


        let objects = self.meshes.len() as u32;
        self.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
        Ok(())
    }


    fn create_depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView{
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: MSAA_SAMPLES,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn create_msaa_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("msaa_color"),
            size: wgpu::Extent3d{ width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: MSAA_SAMPLES,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
