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
use session_rust::{Mesh, BRep, Xform, RenderVertex};

pub struct Gpu {
    pub surface: wgpu::Surface<'static>,     // Screen to draw pixels on.
    pub device: wgpu::Device,                // Handle to the GPU, used to create resources (textures, buffers, pipelines).
    pub queue: wgpu::Queue,                  // Used to submit work to the GPU (draw calls, resource updates).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
    pub pipelines: Pipelines,
    pub mvp_buffer: wgpu::Buffer,            // Camera matrix
    pub mvp_bind_group: wgpu::BindGroup,     // Camera matrix
    pub arena_vbo: wgpu::Buffer,
    pub arena_vids: wgpu::Buffer,
    pub arena_ibo: wgpu::Buffer,
    pub arena_index_count: u32,
    instances: Vec<Instance>,
    pub instance_bind_group: wgpu::BindGroup,
    pub cyl_template_vbo: wgpu::Buffer,
    pub cyl_template_ibo: wgpu::Buffer,
    pub cyl_index_count: u32,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub line_buffer: wgpu::Buffer,
    pub line_bind_group: wgpu::BindGroup,
    pub time: f32,
    pub time_buffer: wgpu::Buffer,
    pub time_bind_group: wgpu::BindGroup,
    pub depth_view: wgpu::TextureView,
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




        // A scene of difference meshes 
        let objects: Vec<(Mesh, Xform, [f32; 4])> = vec![
            (Mesh::create_box(600.0, 600.0, 600.0),      Xform::translation(-2400.0, 0.0, 0.0), [0.90, 0.30, 0.30, 1.0]),
            (Mesh::create_dodecahedron(400.0),           Xform::translation(-1200.0, 0.0, 0.0), [0.90, 0.70, 0.20, 1.0]),
            (BRep::create_sphere(380.0).mesh(),          Xform::translation(    0.0, 0.0, 0.0), [0.30, 0.80, 0.40, 1.0]),
            (BRep::create_cylinder(320.0, 800.0).mesh(), Xform::translation( 1200.0, 0.0, 0.0), [0.30, 0.60, 0.90, 1.0]),
            (BRep::create_torus(360.0, 140.0).mesh(),    Xform::translation( 2400.0, 0.0, 0.0), [0.70, 0.40, 0.90, 1.0]),
        ];

        let mut verts: Vec<RenderVertex> = Vec::new(); // slot 0 - every mesh's vertices, concatenated
        let mut vids: Vec<u32> = Vec::new(); // slot 1 - one row id per vertex (@location 3)
        let mut idx: Vec<u32> = Vec::new(); // the shared index buffer
        let mut instances: Vec<Instance> = Vec::with_capacity(objects.len());
        let mut segments: Vec<CylinderSegment> = Vec::new();

        for (ri, (mesh, model, color)) in objects.into_iter().enumerate(){
            instances.push(Instance{model: model.to_f32(), color, flags: 0, _pad: [0; 3]});
            let base = verts.len() as u32; // where the mesh vertices begin in the arena
            let rm = mesh.to_render(); // f64 -> f32 flatten
            for v in &rm.vertices {
                verts.push(*v);
                vids.push(ri as u32);
            }
            for &i in &rm.indices{
                idx.push(base + i);
            }
            // Edges → one cylinder segment each; instance_id = this object's row (ri).
            // Point::to_f32() / Color::to_f32() are the kernel's GPU-edge casts
            for (a, b, col) in mesh.edges_with_colors(){
                let pa = mesh.vertex_point(a).unwrap();
                let pb = mesh.vertex_point(b).unwrap();
                segments.push(CylinderSegment {
                    p0: pa.to_f32(),
                    radius: 0.0,                // screen-constant px
                    p1: pb.to_f32(),
                    instance_id: ri as u32,
                    color: col.to_f32(),        // per-edge rgba (opaque by default)
                });
            }

        }

        let arena_index_count = idx.len() as u32;


        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("instance.buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let instance_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("instance.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                    has_dynamic_offset: false, 
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("instances.bind_group"),
            layout: &instance_layout,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: instance_buffer.as_entire_binding()}],
        });

        let arena_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("arena.vbo"),
            contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX,
        });

        let arena_vids = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("arena.vids"),
            contents: bytemuck::cast_slice(&vids), usage: wgpu::BufferUsages::VERTEX,
        });

        let arena_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("arena.ibo"),
            contents: bytemuck::cast_slice(&idx), usage: wgpu::BufferUsages::INDEX,
        });

        // Unit-cylinder tempalte (positions only) - one mesh, instance per edge.
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let cyl_index_count = cyl_i.len() as u32;

        let cyl_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("cyl.template.vbo"),
            contents: bytemuck::cast_slice(&cyl_v),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let cyl_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("cyl.template.ibo"),
            contents: bytemuck::cast_slice(&cyl_i),
            usage: wgpu::BufferUsages::INDEX,
        });

        // One storage row per edge (VERTEX-visible, read-only) - the segment table.
        let segment_count = segments.len() as u32;
        let segment_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("segments.buffer"),
            contents: bytemuck::cast_slice(&segments),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        
        let segment_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("segments.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false, min_binding_size: None,
                },
                count: None,
            }],
        });

        let segment_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("segments.bind_group"),
            layout: &segment_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: segment_buffer.as_entire_binding() 
            }]
        });

        // Line uniform - scree-constant thickness
        let line_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: config.height as f32
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("line.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count:None
            }],
        });

        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"), 
            layout: &line_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding()
            }],
        });


        // Pipelines
        let pipelines = Pipelines::new(
            &device, 
            config.format,
            &mvp_layout, 
            &time_layout, 
            &instance_layout,
            &line_layout,
            &segment_layout);


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
            arena_vbo,
            arena_vids,
            arena_ibo,
            arena_index_count,
            instances,
            instance_bind_group,
            cyl_template_vbo,
            cyl_template_ibo,
            cyl_index_count,
            segment_buffer,
            segment_bind_group,
            segment_count,
            line_buffer,
            line_bind_group,
            time: 0.0, 
            time_buffer, 
            time_bind_group,
            depth_view,
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

        let line = LineUniform{
            thickness: 2.0, // later driven by the egui slider
            proj_y: 1.0 / (30.0_f32).to_radians().tan() * 0.001, // cot(fovy/2) mm-m unit scale
            ortho_h: 0.0, // perspective, set the ortho half-height when ortho
            vp_h: self.config.height as f32,
        };
        self.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));

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
            pass.set_bind_group(2, &self.instance_bind_group, &[]);

            // Arena draw
            pass.set_vertex_buffer(0, self.arena_vbo.slice(..)); // slot 0 - vertices
            pass.set_vertex_buffer(1, self.arena_vids.slice(..)); // slot 1 - per-vertex row ids
            pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.arena_index_count, 0, 0..1); // whole scene, one call
            draws += 1;

            //Edges - ONE draw fro the WHOLE scene linework:
            // segment table + unit-cylinder templates
            pass.set_pipeline(&self.pipelines.cylinder);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
            pass.set_bind_group(3, &self.segment_bind_group, &[]);
            pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
            pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.segment_count); // one template, N edges
            draws += 1;





        }


        let objects = self.instances.len() as u32;
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

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
    color: [f32; 4], // 16 B
    flags: u32, // 4 B - reserved (selection)
    _pad: [u32; 3], // 12 B - pad the row to 96 B (storage array stride)
}


//////////////////////////////////////////////////////////////////////////////////////////////////
/// Individual type memoery layouts
//////////////////////////////////////////////////////////////////////////////////////////////////

// Memory layout is 16 (12+4), 16 (12+4) and 16
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CylinderSegment{
    p0: [f32; 3],   // 12 B - start point 
    radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 0 -> wolrd mm override
    p1: [f32; 3],   // 12 B - end point (p0..instance_id = 32 B of geometry)
    instance_id: u32,  // 4 B - row in instances[]: object model + flags (hide/select later)
    color: [f32; 4],  // 16 B - per - edge (black crease, naked color, ...)
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale 
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
} // 16 B - one vec4, no padding


//////////////////////////////////////////////////////////////////////////////////////////////////
/// Primitives
//////////////////////////////////////////////////////////////////////////////////////////////////

const CYL_SIDES: u32 = 12;

// Unit cylinder along +Z (radius 1. z in [0,1]) with cap fans.
// The shader rescale xy by the screen-constant radius and maps z along (p1-p0), so this template is registered ONCE.
fn unit_cylinder(sides: u32) -> (Vec<[f32; 3]>, Vec<u32>){
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    for s in 0..sides{
        let a = s as f32 / sides as f32 * std::f32::consts::TAU;
        v.push([a.cos(), a.sin(), 0.0]);
        v.push([a.cos(), a.sin(), 1.0]);
    }
    for s in 0..sides{
        let b0 = 2 * s;
        let b1 = 2 * ((s+1) % sides);
        idx.extend_from_slice(&[b0, b1, b1 + 1, b0, b1+1, b0+1]); // Two triangles per side face
    }
    let cb = v.len() as u32;
    v.push([0.0, 0.0, 0.0]);
    let ct = v.len() as u32;
    v.push([0.0, 0.0, 1.0]);
    for s in 0..sides{
        let b0 = 2 * s;
        let b1 = 2 * ((s+1)%sides);
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]); // bottom + top fan
    }
    (v, idx)
}

