//! `Gpu` — our handle to the graphics card and the lowest layer of the viewer (ARCHITECTURE.md §1).
//!
//! It owns the three things wgpu needs to draw:
//!   • `device` — makes GPU resources (textures, buffers, pipelines)
//!   • `queue`  — sends work to the GPU
//!   • `surface`— the canvas pixels we present each frame
//! plus the `config` describing the surface size/format. It knows nothing app-specific — its whole
//! job is "hand me a cleared frame". Higher layers sit on top and only talk to this.

use std::any;

use crate::engine::pipelines::Pipelines;
use crate::engine::pipelines::build::MSAA_SAMPLES;
use crate::engine::performance::Performance;
mod adapters;
use adapters::{line_to_segment, point_to_glyph, polyline_to_segments};
use bytemuck::Zeroable;
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};

/// Re-anchor distance: the instance table is rebased about a snapped anchor.
/// The camera can drift this far (mm) before a full rebuild.
/// Within it, pan/zoon only changes the view matrix.
/// f32 error at 1e5 mm from the achor = 6e-3 mm - far below a pixel.
const REANCHOR_DIST: f64 = 1.0e5;

/// const for the unit_cylinder method
const CYL_SIDES: u32 = 12;

/// const for the unit_sphere method
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

pub struct Gpu {
    pub surface: wgpu::Surface<'static>,     // Screen to draw pixels on.
    pub device: wgpu::Device,                // Handle to the GPU, used to create resources (textures, buffers, pipelines).
    pub queue: wgpu::Queue,                  // Used to submit work to the GPU (draw calls, resource updates).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
    pub pipelines: Pipelines,
    pub mvp_buffer: wgpu::Buffer,            // Camera matrix
    pub mvp_bind_group: wgpu::BindGroup,     // Camera matrix
    pub line_buffer: wgpu::Buffer, // shared: px-sizing for cylinders + spheres
    pub line_bind_group: wgpu::BindGroup,
    pub time: f32,  // shared: animation
    pub time_buffer: wgpu::Buffer,
    pub time_bind_group: wgpu::BindGroup,
    pub arena_vbo: wgpu::Buffer,
    pub arena_vids: wgpu::Buffer,
    pub arena_ibo: wgpu::Buffer,
    pub arena_index_count: u32,
    instances: Vec<Instance>,
    last_origin: Option<Point>, // rebuild_instances skips when the camera target did not move
    objects_base: Vec<(Xform, [f32; 4])>, // TRUE world model+color; isntance[] is rebased from this
    instance_buffer: wgpu::Buffer, // new() builds this storage buffer as a local and drops it, only the bidn group survives; rebuild_instances() reuploads into it every frame, so the buffer handle itself must live on GPU, not vanish atht eh of new()
    pub instance_bind_group: wgpu::BindGroup,
    pub cyl_template_vbo: wgpu::Buffer,
    pub cyl_template_ibo: wgpu::Buffer,
    pub cyl_index_count: u32,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub point_buffer: wgpu::Buffer,
    pub point_bind_group: wgpu::BindGroup,
    pub point_count: u32,
    pub cloud_buffer: wgpu::Buffer,
    pub cloud_bind_group: wgpu::BindGroup,
    pub depth_view: wgpu::TextureView,
    pub msaa_view: wgpu::TextureView,
    pub performance: Performance,
    pub scene_min: [f32; 3],
    pub scene_max: [f32; 3],
}

impl Gpu {
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        session: &session_rust::Session) -> anyhow::Result<Self> {
        

        // 1. Instance — the driver entry point. WebGPU only in the browser, never WebGL.
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

        // 4. Device (creates resources) + Queue (submits work).
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




        let mut verts: Vec<RenderVertex> = Vec::new(); // slot 0 - every mesh's vertices, concatenated
        let mut vids: Vec<u32> = Vec::new(); // slot 1 - one row id per vertex (@location 3)
        let mut idx: Vec<u32> = Vec::new(); // the shared index buffer
        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut glyphs: Vec<GlyphPoint> = Vec::new();
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::with_capacity(session.lookup.len());

        // Each object's placement lives in its xform (kernel convention) - `to_render()`/
        // `start()`/`get_points()` read stored coordinates and ignore it, so the xform IS the
        // instance model. `ri` is the row in objects_base, not the lookup index - skipped
        // variants (Plane/OBB/...) leave no hole.
        for geom in session.lookup.values() {
            let ri = objects_base.len() as u32;
            match geom{
                Geometry::Mesh(m) => {
                    objects_base.push((m.xform.clone(), m.objectcolor().to_f32()));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    objects_base.push((b.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((l.xform.clone(), l.linecolor.to_f32()));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((pl.xform.clone(), pl.linecolor.to_f32()));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((p.xform.clone(), p.pointcolor.to_f32()));
                    glyphs.push(point_to_glyph(p, ri));
                }
                // Later lessons - the match must stay exhaustive over all 11 variants
                Geometry::Plane(_) |
                Geometry::OBB(_) |
                Geometry::PointCloud(_) |
                Geometry::Element(_) |
                Geometry::NurbsCurve(_) |
                Geometry::NurbsSurface(_) => {}
            }
        }

        let mut instances: Vec<Instance> = objects_base.iter()
        .map(|(m, c)| Instance {
            model: m.to_f32(),
            color: *c,
            flags: 0,
            _pad: [0; 3]
        }).collect();

        let segment_count = segments.len() as u32; // Before padding - the real draw-cell count
        let glyph_count = glyphs.len() as u32;
        let points: Vec<CloudPoint> = Vec::new();

        // A real file is not the five-mesh demo:
        // a pure line drawing has zero mesh vertices,
        // a pure mesh file zero segments.
        // WGPU buffers cannot be zero-size, so pad the CPU side with one placeholder -*_count
        // Above already capture the true numnber
        // So an empty catergory still draws nothing, it just does not cras the buffer upload
        if instances.is_empty(){
            instances.push(
                Instance {
                    model: Xform::identity().to_f32(),
                    color: [0.5, 0.5, 0.5, 1.0],
                    flags: 0,
                    _pad: [0; 3]
                }
            );
        }

        if verts.is_empty(){
            verts.push(RenderVertex::zeroed());
            vids.push(0);
            idx.extend_from_slice(&[0,0,0]);
        }

        if segments.is_empty(){
            segments.push(CylinderSegment::zeroed());
        }
        
        if glyphs.is_empty(){
            glyphs.push(GlyphPoint::zeroed());
        }

        let arena_index_count = idx.len() as u32;

        // Bounding Box
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
        for v in &verts{
            for k in 0..3{
                scene_min[k] = scene_min[k].min(v.position[k]);
                scene_max[k] = scene_max[k].max(v.position[k]);
            }
        }
        for s in &segments{
            for p in [s.p0, s.p1]{
                for k in 0..3 {
                    scene_min[k] = scene_min[k].min(p[k]);
                    scene_max[k] = scene_max[k].max(p[k]);
                }
            }
        }
        for g in &glyphs{
            for k in 0..3{
                scene_min[k] = scene_min[k].min(g.center[k]);
                scene_max[k] = scene_max[k].max(g.center[k]);
            }
        }

        log::info!("session '{}': {} objects, {} arena verts, {} segments, {} glyphs",
            session.name, instances.len(), verts.len(), segments.len(), glyphs.len());





        let instance_buffer =  storage_buffer(&device, "instance.buffer", &instances);

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
        let segment_buffer =  storage_buffer(&device, "segments.buffer", &segments);
        
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

        // Unit-sphere template (positions-only) - one mesh, instance per glyph
        let (sph_v, sph_i) = unit_sphere();
        let sph_index_count = sph_i.len() as u32;
        let sph_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("sph.template.vbo"), 
            contents: bytemuck::cast_slice(&sph_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sph_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("sph.template.ibo"),
            contents: bytemuck::cast_slice(&sph_i),
            usage: wgpu::BufferUsages::INDEX,
        });
        let glyph_buffer =  storage_buffer(&device, "glyphs.buffer", &glyphs);
        let glyph_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("glyphs.layout"),
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
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("glyphs.bind_group"),
            layout: &glyph_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0, 
                resource: glyph_buffer.as_entire_binding()
            }],
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

        // Point buffer + the cloud uniform
        let point_count = points.len() as u32;

        // point storage buffer
        let point_buffer = storage_buffer(&device, "points.buffer", &points);
        let point_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points.bind_group"),
            layout: &glyph_layout,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: point_buffer.as_entire_binding()}],
        });

        // point cloud unioform - the cloud's OWN global size + viewport (reuses line_layout)
        let cloud_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
                size: 4.0,
                vp_w: config.width as f32,
                vp_h: config.height as f32,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cloud_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("cloud.bind_group"),
            layout: &line_layout,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: cloud_buffer.as_entire_binding()}],
        });

        // Pipelines
        let pipelines = Pipelines::new(
            &device, 
            config.format,
            &mvp_layout, 
            &time_layout, 
            &instance_layout,
            &line_layout,
            &segment_layout,
            &glyph_layout,
        );


        // Output
        log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self { 
            surface, 
            device, 
            queue, 
            config, 
            pipelines, 
            mvp_buffer, // shared: camera
            mvp_bind_group, 
            line_buffer,  // shared: px-sizing for cylinders + spheres
            line_bind_group,
            time_buffer,    // shared: animation
            time_bind_group,
            time: 0.0, 
            arena_vbo,
            arena_vids,
            arena_ibo,
            arena_index_count,
            instances,
            last_origin: None,
            objects_base,
            instance_buffer, // was a dropped local in new(), now moved onto GPU so rebuild_instances() can write into every frame
            instance_bind_group,
            cyl_template_vbo,
            cyl_template_ibo,
            cyl_index_count,
            segment_buffer,
            segment_bind_group,
            segment_count,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            point_buffer,
            point_bind_group,
            point_count,
            cloud_buffer,
            cloud_bind_group,
            depth_view,
            msaa_view,
            performance: Performance::new(),
            scene_min,
            scene_max,
         })

    }

    /// The anchor the instance table is rebased about.
    /// A full rebuild (42 000 x at stress scale) runs
    /// only when the camera target strays REANCHOR_DIST from the current anchor - orbit newer moves the target.
    /// And pan/zoom within the budget just changes the view matrix
    pub fn rebase_anchor(&mut self, origin: &Point) -> Point{
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let (dx, dy, dz) = (a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]);
                (dx * dx + dy * dy + dz * dz).sqrt() > REANCHOR_DIST
            }
        };
        if need {
            self.rebuild_instances(origin);
        }
        self.last_origin.clone().unwrap()
    }

    /// Rebase every instance's translation around 'origin' - an f64 subtract agains the TRUE world transfrom in 'objects_base'
    /// Then cast to f32.
    /// 'instances', what GPU actually sees, never holds a coordinate bigger than the camera's distnace from 'origin',
    /// no matter how fas the scene fists from world (0,0,0).
    fn rebuild_instances(&mut self, origin: &Point){
        // let shift = Xform::translation(-origin[0], -origin[1], -origin[2]);
        // for (i, (model, color)) in self.objects_base.iter().enumerate() {
        //     self.instances[i].model = (&shift * model).to_f32(); // f64 multiply, f32 cast last
        //     self.instances[i].color = *color;
        // }
        // self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
        self.last_origin = Some(origin.clone());
        for (i, (model, color)) in self.objects_base.iter().enumerate() {
            let mut m = model.to_f32();
            m[12] = (model.m[12] - origin[0]) as f32;
            m[13] = (model.m[13] - origin[1]) as f32;
            m[14] = (model.m[14] - origin[2]) as f32;
            self.instances[i].model = m;
            self.instances[i].color = *color;
        }
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
    }

    /// Reconfigure the surface and recreate the depth + MSAA targets for a new canvas size.
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
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform, origin: &Point) -> anyhow::Result<()> {

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
            if self.segment_count > 0 {
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

            // Spheres
            if self.glyph_count > 0 {
                pass.set_pipeline(&self.pipelines.sphere);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.glyph_count); // one template, N glyphs
                draws += 1;
            }

            // Points
            if self.point_count > 0 {
                pass.set_pipeline(&self.pipelines.point);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.cloud_bind_group, &[]); // cloud size + viewport
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.point_bind_group, &[]);
                pass.draw(0..3 * self.point_count, 0..1); // 3 vertices per point, no template
                draws += 1;
            }






        }


        let objects = self.instances.len() as u32;
        self.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
        Ok(())
    }


    /// Create the reverse-Z depth texture view, sized to the surface at the MSAA sample count.
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

    /// Create the multisampled color target the frame renders into (resolved to the surface each frame).
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
/// Individual type memory layouts
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


// One instance of the unit-sphere template.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GlyphPoint{
    center: [f32; 3], // 12 B - mesh-local
    radius: f32, // 4 B - 0.0 - screen-constant px; 0 - world mm
    color:  [f32; 4],
    instance_id: u32, // 4 B - row insntaces
    _pad: [u32; 3], // 12 B - single trailing scalar is why we need pad
} // 48 B total, three 16-byte rows

// Points inscribed in circles used for pointclouds
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudPoint{
    position: [f32; 3], // 12 B - mesh local
    instance_id: u32, // 4 B - fills position's tail
    color: [f32; 4], // 16 B
} // 32 B total, two 16-byte rows, zero padding

// Points global attributes
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudUniform{
    size: f32, // global point-cloud dot size, px
    vp_w: f32, // framebuffer width, px
    vp_h: f32, // framebuffer height, px
    _pad: f32,
} // 16 B - one vec4; its own buffer + bind group

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Primitives
//////////////////////////////////////////////////////////////////////////////////////////////////



/// Unit-cylinder template mesh (positions + indices) along +Z, radius 1, z in [0,1], with cap fans.
/// The shader rescales xy by the screen-constant radius and maps z along (p1-p0), so it's registered ONCE.
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


// Unit sphere on the origin, radius 1. The shader offsets each template vertex by the
/// Unit-sphere template mesh (positions + indices) for the instanced sphere glyphs.
// screen-constant radius around the glyph's world centre — no frame needed (a sphere is
// symmetric), unlike 31's tube.
fn unit_sphere() -> (Vec<[f32; 3]>, Vec<u32>) {
    let pi = std::f32::consts::PI;
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    v.push([0.0, 0.0, 1.0]);                                   // north pole
    for k in 1..=SPH_LATS {
        let phi = k as f32 * pi / (SPH_LATS + 1) as f32;
        let (z, r) = (phi.cos(), phi.sin());
        for i in 0..SPH_LONS {
            let t = i as f32 * 2.0 * pi / SPH_LONS as f32;
            v.push([r * t.cos(), r * t.sin(), z]);
        }
    }
    let south = v.len() as u32; v.push([0.0, 0.0, -1.0]);      // south pole
    for i in 0..SPH_LONS {                                     // top cap fan
        idx.extend_from_slice(&[0, 1 + i as u32, 1 + ((i + 1) % SPH_LONS) as u32]);
    }
    for k in 0..(SPH_LATS - 1) {                               // middle bands
        let (ra, rb) = ((1 + k * SPH_LONS) as u32, (1 + (k + 1) * SPH_LONS) as u32);
        for i in 0..SPH_LONS {
            let (a0, a1) = (ra + i as u32, ra + ((i + 1) % SPH_LONS) as u32);
            let (b0, b1) = (rb + i as u32, rb + ((i + 1) % SPH_LONS) as u32);
            idx.extend_from_slice(&[a0, a1, b0, a1, b1, b0]);
        }
    }
    let lr = (1 + (SPH_LATS - 1) * SPH_LONS) as u32;           // bottom cap fan (reversed)
    for i in 0..SPH_LONS {
        idx.extend_from_slice(&[south, lr + ((i + 1) % SPH_LONS) as u32, lr + i as u32]);
    }
    (v, idx)
}

fn push_mesh(
    m: &Mesh,
    ri: u32,
    verts: &mut Vec<RenderVertex>,
    vids: &mut Vec<u32>,
    idx: &mut Vec<u32>,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>
){
    let base = verts.len() as u32;
    let rm = m.to_render();
    for v in &rm.vertices{
        verts.push(*v);
        vids.push(ri);
    }
    for &i in &rm.indices{
        idx.push(base+i);
    }

    for (a, b, col) in m.edges_with_colors(){
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: 0.0,
                p1: pb.to_f32(),
                instance_id: ri,
                color: col.to_f32()
            }
        )
    }

    for vk in m.vertices(){
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(
            GlyphPoint { 
                center: p.to_f32(), 
                radius: 0.0, 
                color: [0.1, 0.1, 0.1, 1.0], 
                instance_id: ri, 
                _pad: [0;3] }
        );
    }
}

/// A read-only storage buffer that is never zero-sized (wgpu can't bind a 0-byte buffer).
/// When `data` is empty we still allocate one zeroed element; the real element count is
/// tracked separately, so the draw call issues 0 instances and nothing renders.
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, label: &str, data: &[T]) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    let one = [T::zeroed()];
    let contents: &[u8] = if data.is_empty() { bytemuck::cast_slice(&one) } else { bytemuck::cast_slice(data) };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}

/// One loaded file, walked into GPU-ready tables.
/// Built by [`Gpu::walk_session`]
/// BEFORE `Gpu::new`
/// so the parsed `Session`
/// often 10x larger than these tables
/// can be dropped before the next file is fetched
/// peak memory holds one session at a time, not all of them
pub struct SceneTables {
    verts: Vec<RenderVertex>,
    vids: Vec<u32>,
    ids: Vec<u32>,
    segments: Vec<CylinderSegment>,
    glyphs: Vec<GlyphPoint>,
    objects: Vec<(Xform, [f32; 4])>,
    min: [f32; 3],
    max: [f32; 3],
}

impl SceneTables {
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[SceneTables]
    ) -> anyhow::Result<Self>{

        let mut verts: Vec<RenderVertex> = Vec::new(); // slot 0 - every mesh's vertices, concatenated
        let mut vids: Vec<u32> = Vec::new(); // slot 1 - one row id per vertex (@location 3)
        let mut idx: Vec<u32> = Vec::new(); // the shared index buffer
        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut glyphs: Vec<GlyphPoint> = Vec::new();
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::with_capacity(session.lookup.len());

        // Each object's placement lives in its xform (kernel convention) - `to_render()`/
        // `start()`/`get_points()` read stored coordinates and ignore it, so the xform IS the
        // instance model. `ri` is the row in objects_base, not the lookup index - skipped
        // variants (Plane/OBB/...) leave no hole.
        for geom in session.lookup.values() {
            let ri = objects_base.len() as u32;
            match geom{
                Geometry::Mesh(m) => {
                    objects_base.push((m.xform.clone(), m.objectcolor().to_f32()));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    objects_base.push((b.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((l.xform.clone(), l.linecolor.to_f32()));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((pl.xform.clone(), pl.linecolor.to_f32()));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((p.xform.clone(), p.pointcolor.to_f32()));
                    glyphs.push(point_to_glyph(p, ri));
                }
                // Later lessons - the match must stay exhaustive over all 11 variants
                Geometry::Plane(_) |
                Geometry::OBB(_) |
                Geometry::PointCloud(_) |
                Geometry::Element(_) |
                Geometry::NurbsCurve(_) |
                Geometry::NurbsSurface(_) => {}
            }
        }

    }
}