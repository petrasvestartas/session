//! `Gpu` — our handle to the graphics card and the lowest layer of the viewer (ARCHITECTURE.md §1).
//!
//! It owns the three things wgpu needs to draw:
//!   • `device` — makes GPU resources (textures, buffers, pipelines)
//!   • `queue`  — sends work to the GPU
//!   • `surface`— the canvas pixels we present each frame
//! plus the `config` describing the surface size/format. It knows nothing app-specific — its whole
//! job is "hand me a cleared frame". Higher layers sit on top and only talk to this.

use crate::engine::pipelines::Pipelines;
use crate::engine::performance::Performance;

use session_rust::{Xform, RenderVertex, Point};

/// Re-anchor distance: the instance table is rebased about a snapped anchor.
/// The camera can drift this far (mm) before a full rebuild.
/// Within it, pan/zoon only changes the view matrix.
/// f32 error at 1e5 mm from the achor = 6e-3 mm - far below a pixel.
/// Re-anchor threshold, WORLD units (mm): a quarter of the current view distance, so a zoomed-out
/// pan does not rebuild constantly while a zoomed-IN pan re-anchors early enough that world
/// coordinates never regain the magnitude that eats f32 precision. Clamped to a sane band.
const REANCHOR_MIN: f64 = 1.0e3;
const REANCHOR_MAX: f64 = 1.0e5;

/// const for the unit_cylinder method
const CYL_SIDES: u32 = 6;

/// Linework lane is per GEOMETRY TYPE, not global (both stay screen-constant px):
/// SOLID (cylinder + sphere) for mesh/BRep, whose ink lies ON a surface - the tube radius lifts
///   it off that surface, so a silhouette edge cannot lose the depth test to its own face.
/// FLAT (ribbon + glyph) for line/polyline/point, which float free and have nothing to fight.
/// Routing lives in `app::scene::Scene`, one draw per lane in `clear`.

/// const for the unit_sphere method
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
/// Off: on 2D sheets (600k segments, all ribbons) the second pass doubles the frame.
const INK_DEPTH_PREPASS: bool = false;

/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs) 
/// and are spliced solid-first at upload.
/// `objects` holds the TRUE per-object transfrom + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
pub struct ArenaUpload{
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub pipes: Vec<CylinderSegment>, // Solid lane: Mesh/Brep edges, drawn as 3D cylinders
    pub spheres: Vec<GlyphPoint>, // Solid lane: Mesh/Brep vertices, radius matched to the pipes
    pub segments: Vec<CylinderSegment>, // Flat lane: line/polyline, drawn as camera-facing ribbons
    pub glyphs: Vec<GlyphPoint>, // Flat lane: points, draw as SDF dots,
    // Raw lane, SPLIT: 3 floats + 1 packed RGBA8 per point = 16 B, against CloudPoint's 32.
    pub cloud_pos: Vec<f32>,      // 3 per point
    pub cloud_col: Vec<u32>,      // RGBA8, 1 per point
    pub clouds: Vec<CloudDraw>,   // one entry per cloud - the instance rides here, not per point
    pub objects: Vec<(Xform, [f32; 4], u32)>,
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl ArenaUpload {
    pub fn new() -> Self {
        Self {
            verts: Vec::new(),
            vids: Vec::new(),
            idx: Vec::new(),
            pipes: Vec::new(),
            spheres: Vec::new(),
            segments: Vec::new(),
            glyphs: Vec::new(),
            cloud_pos: Vec::new(),
            cloud_col: Vec::new(),
            clouds: Vec::new(),
            objects: Vec::new(),
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        }
    }
}

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
    pub arena_vert_count: u32,   // rows already on the GPU - the base for the next append
    pub arena_vert_cap: u64,
    pub arena_index_cap: u64,
    instances: Vec<Instance>,
    last_origin: Option<Point>, // rebuild_instances skips when the camera target did not move
    objects_base: Vec<(Xform, [f32; 4], u32)>, // TRUE world model+color; isntance[] is rebased from this
    // Layouts surfvive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    mvp_layout: wgpu::BindGroupLayout,
    time_layout: wgpu::BindGroupLayout,
    instance_layout: wgpu::BindGroupLayout,
    line_layout: wgpu::BindGroupLayout,
    segment_layout: wgpu::BindGroupLayout,
    glyph_layout: wgpu::BindGroupLayout,

    instance_buffer: wgpu::Buffer, // new() builds this storage buffer as a local and drops it, only the bidn group survives; rebuild_instances() reuploads into it every frame, so the buffer handle itself must live on GPU, not vanish atht eh of new()
    pub instance_bind_group: wgpu::BindGroup,
    pub cyl_template_vbo: wgpu::Buffer,
    pub cyl_template_ibo: wgpu::Buffer,
    pub cyl_index_count: u32,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub pipe_count: u32,  // segments[0..pipe_count] are the SOLID lane, the rest are ribbons
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub sphere_count: u32, // glyphs[0..sphere_count] are the SOLID lane, the rest are flat dots
    pub point_pos_buffer: wgpu::Buffer,
    pub point_col_buffer: wgpu::Buffer,
    pub point_bind_group: wgpu::BindGroup,
    pub cloud_layout: wgpu::BindGroupLayout,
    pub cloud_draws: Vec<CloudDraw>, // one draw per cloud
    pub cloud_pos_at: u32,           // streaming write cursors, in POINTS
    pub cloud_col_at: u32,
    pub point_count: u32,
    pub point_capacity: u64,
    pub cloud_buffer: wgpu::Buffer,
    pub cloud_bind_group: wgpu::BindGroup,
    pub depth_view: wgpu::TextureView,
    pub msaa_view: wgpu::TextureView,
    pub samples: u32, // MSAA sample count this scene chose (see `msaa_for`)
    pub performance: Performance,
    pub scene_min: [f32; 3],
    pub scene_max: [f32; 3],
}

impl Gpu {

    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    /// The scene starts empty - every upload, including the first file, goes through `set_scene`
    /// (progressive loading calls it once per appended file), One code path, not two.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        

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
        // LowPower = the iGPU the compositor runs on. On hybrid laptops the discrete GPU renders
        // fine but its frames can't be shared to the compositor - the canvas stays black.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;
        let info = adapter.get_info();
        log::info!("adapter: {} ({:?}, {:?})", info.name, info.device_type, info.backend);
        if info.device_type == wgpu::DeviceType::Cpu {
            log::warn!("software adapter - rendering on the CPU will be slow");
        }

        // Limit to 128 mb, then the flat merge becomes the grid
        let mut limits = wgpu::Limits::default();
        let hw = adapter.limits();
        limits.max_storage_buffer_binding_size = hw.max_storage_buffer_binding_size;
        limits.max_buffer_size = hw.max_buffer_size;

        // 4. Device (creates resources) + Queue (submits work).
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: limits,  // unlock the WEBGpu storage buffers
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

        // Depth and MSAA - the emoty scene starts flat (1x)
        // Set_scene flips to 4x when the first solid geometry arrives
        let samples = 1;
        let depth_view = Self::create_depth_view(&device, &config, samples);
        let msaa_view = Self::create_msaa_view(&device, &config, samples);

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








        // The scene-shaped fields start as empty placeholders
        // WebGPU zero-initializes buffers, and every *_count is 0, so the first frame draws nothing.
        // The loader calls set_scene the moment the first file's tables exist.
        let instances: Vec<Instance> = vec![Instance{
            model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, _pad: [0;3],
        }];

        let instance_buffer =  storage_buffer(&device, &queue, "instance.buffer", &instances);
        let objects_base: Vec<(Xform, [f32; 4], u32)> = Vec::new();
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let arena_index_count = 0u32;
        let arena_vert_count = 0u32;
        let (arena_vert_cap, arena_index_cap) = (1u64, 1u64);
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);

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

        // One zeroed row each - wgpu cannot bind a 0-byte buffer, and arena_index_count starts
        // at 0 so nothing is drawn from them until real geometry appends.
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_vbo = zeroed_buffer(&device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu);
        let arena_vids = zeroed_buffer(&device, "arena.vids", 4, vu);
        let arena_ibo = zeroed_buffer(&device, "arena.ibo", 4, iu);

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
        let segment_buffer =  zeroed_buffer(
            &device, "segments.buffer", 
            std::mem::size_of::<CylinderSegment>() as u64, 
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        
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
        let glyph_buffer =  zeroed_buffer(
            &device, 
            "glyphs.buffer", 
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
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
                vp_h: config.height as f32,
                vp_w: config.width as f32,
                _pad0: [0.0; 3],
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
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

        // Point buffers + the cloud uniform. TWO buffers now, and its own layout: the glyph
        // layout has one binding, the split cloud needs two.
        let point_count = 0u32;
        let point_capacity = 1u64;
        let cloud_draws: Vec<CloudDraw> = Vec::new();

        let storage_entry = |binding: u32| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let cloud_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cloud.layout"),
            entries: &[storage_entry(0), storage_entry(1)],
        });

        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let point_pos_buffer = zeroed_buffer(&device, "cloud.pos", point_capacity * 12, usage);
        let point_col_buffer = zeroed_buffer(&device, "cloud.col", point_capacity * 4, usage);
        let point_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points.bind_group"),
            layout: &cloud_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: point_pos_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: point_col_buffer.as_entire_binding() },
            ],
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
            samples,
            config.format,
            &mvp_layout, 
            &time_layout, 
            &instance_layout,
            &line_layout,
            &segment_layout,
            &glyph_layout,
            &cloud_layout,
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
            arena_vert_count,
            arena_vert_cap,
            arena_index_cap,
            instances,
            last_origin: None,
            objects_base,
            mvp_layout,
            time_layout,
            instance_layout,
            line_layout,
            segment_layout,
            glyph_layout,
            instance_buffer, // was a dropped local in new(), now moved onto GPU so rebuild_instances() can write into every frame
            instance_bind_group,
            cyl_template_vbo,
            cyl_template_ibo,
            cyl_index_count,
            segment_buffer,
            segment_bind_group,
            segment_count,
            pipe_count,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            sphere_count,
            point_bind_group,
            point_count,
            point_capacity,
            point_pos_buffer,
            point_col_buffer,
            cloud_layout,
            cloud_draws,
            cloud_pos_at: 0,
            cloud_col_at: 0,
            cloud_buffer,
            cloud_bind_group,
            depth_view,
            msaa_view,
            samples,
            performance: Performance::new(),
            scene_min,
            scene_max,
         })

    }

    /// Replace the whole scene scene from the app's tables - called once per file while progressive loading appends.
    /// ZERO-COPY: lanes are written straight from the Scene's Vecs into fresh buffers (two write_buffer calls splice SOLID-first),
    /// so nothing is cloned per append.
    /// WebGPU zero-initializes buffers, so an empty category is just a  1-row zeroed buffer.
    /// An MSAA flip (first solid file after flat-only ones) also rebuilds the depth/msaa targets and every pipeline, since sample count belongs to the render PASS.
    pub fn set_scene(&mut self, up: &ArenaUpload){
        // Instance rows: rebuilt from the true transforms (rebase stete, must live CPU-side).
        self.objects_base = up.objects.clone();
        self.instances.clear();
        self.instances.extend(up.objects.iter().map(|(m, c, f)| Instance {
            model: m.to_f32(),
            color: *c,
            flags: *f,
            _pad: [0; 3],
        }));

        if self.instances.is_empty(){
            self.instances.push(Instance {model: Xform::identity().to_f32(), color: [0.5,0.5,0.5,1.0], flags: 0, _pad: [0; 3] });
        }

        self.instance_buffer = storage_buffer(&self.device, &self.queue, "instance.buffer", &self.instances);
        self.instance_bind_group = self.device.create_bind_group( &wgpu::BindGroupDescriptor{
            label: Some("instances.bind_group"),
            layout: &self.instance_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.instance_buffer.as_entire_binding()
            }],
        });

        // Mesh arena. Like the cloud lane, `up.verts/vids/idx` are a DELTA - the caller clears
        // them after upload (Scene::upload_to), because nothing reads them back: picking goes
        // through the kernel Meshes in Doc.session, never through these flattened rows.
        //
        // Appending rather than rebuilding is worth two separate things. It stops re-sending the
        // whole arena on every file (six files meant the 64 MB vertex table travelled six times),
        // and it lets the CPU-side Vecs go, which is ~70 MB of wasm heap that was being held for
        // the sole purpose of feeding the next rebuild.
        if !up.verts.is_empty() {
            let vstride = std::mem::size_of::<RenderVertex>() as u64;
            let need_v = self.arena_vert_count as u64 + up.verts.len() as u64;
            let need_i = self.arena_index_count as u64 + up.idx.len() as u64;

            if need_v > self.arena_vert_cap || need_i > self.arena_index_cap {
                let cap_v = need_v.max(self.arena_vert_cap);
                let cap_i = need_i.max(self.arena_index_cap);
                let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
                let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
                let vbo = zeroed_buffer(&self.device, "arena.vbo", cap_v * vstride, vu);
                let vids = zeroed_buffer(&self.device, "arena.vids", cap_v * 4, vu);
                let ibo = zeroed_buffer(&self.device, "arena.ibo", cap_i * 4, iu);
                if self.arena_vert_count > 0 {
                    // the prefix moves GPU-side; it never travels back through wasm memory
                    let mut enc = self.device.create_command_encoder(&Default::default());
                    enc.copy_buffer_to_buffer(&self.arena_vbo, 0, &vbo, 0, self.arena_vert_count as u64 * vstride);
                    enc.copy_buffer_to_buffer(&self.arena_vids, 0, &vids, 0, self.arena_vert_count as u64 * 4);
                    enc.copy_buffer_to_buffer(&self.arena_ibo, 0, &ibo, 0, self.arena_index_count as u64 * 4);
                    self.queue.submit([enc.finish()]);
                }
                self.arena_vbo = vbo;
                self.arena_vids = vids;
                self.arena_ibo = ibo;
                self.arena_vert_cap = cap_v;
                self.arena_index_cap = cap_i;
            }

            self.queue.write_buffer(&self.arena_vbo, self.arena_vert_count as u64 * vstride, bytemuck::cast_slice(&up.verts));
            self.queue.write_buffer(&self.arena_vids, self.arena_vert_count as u64 * 4, bytemuck::cast_slice(&up.vids));
            self.queue.write_buffer(&self.arena_ibo, self.arena_index_count as u64 * 4, bytemuck::cast_slice(&up.idx));
            self.arena_vert_count += up.verts.len() as u32;
            self.arena_index_count += up.idx.len() as u32;
        }

        // The two lane tables: one buffer each, solid rows firstm spliced by two writes.
        self.pipe_count = up.pipes.len() as u32;
        self.segment_count = (up.pipes.len() + up.segments.len()) as u32;
        let rows = (self.segment_count as u64).max(1);
        self.segment_buffer = zeroed_buffer(
            &self.device, "segments.buffer", 
            rows * std::mem::size_of::<CylinderSegment>() as u64, 
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        self.queue.write_buffer(
            &self.segment_buffer, 
            0, 
            bytemuck::cast_slice(&up.pipes));
        self.queue.write_buffer(
            &self.segment_buffer, 
            up.pipes.len() as u64 * std::mem::size_of::<CylinderSegment>() as u64, 
            bytemuck::cast_slice(&up.segments));
        self.segment_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("segments.bind_group"),
                layout: &self.segment_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.segment_buffer.as_entire_binding()
                }],
        });

        self.sphere_count = up.spheres.len() as u32;
        self.glyph_count = (up.spheres.len() + up.glyphs.len()) as u32;
        let rows = (self.glyph_count as u64).max(1);
        self.glyph_buffer = zeroed_buffer(
            &self.device,
            "glyphs.buffer",
            rows * std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        );
        self.queue.write_buffer(
            &self.glyph_buffer,
            0,
            bytemuck::cast_slice(&up.spheres),
        );
        self.queue.write_buffer(
            &self.glyph_buffer,
            up.spheres.len() as u64 * std::mem::size_of::<GlyphPoint>() as u64,
            bytemuck::cast_slice(&up.glyphs),
        );
        self.glyph_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("glyphs.bind_group"),
                layout: &self.glyph_layout,
                entries: &[wgpu::BindGroupEntry{
                    binding: 0,
                    resource: self.glyph_buffer.as_entire_binding()
                }],
        });

        // Raw cloud lane. `up.cloud_pos`/`up.cloud_col`/`up.clouds` are a DELTA - only what the
        // newest file added - because the caller clears them after each upload (Scene::upload_to).
        // Every other table in this function is cumulative. Two buffers, 12 B + 4 B per point.
        if !up.clouds.is_empty() {
            let need = self.point_count as u64 + (up.cloud_pos.len() / 3) as u64;

            self.cloud_reserve(need);

            self.queue.write_buffer(&self.point_pos_buffer, self.point_count as u64 * 12, bytemuck::cast_slice(&up.cloud_pos));
            self.queue.write_buffer(&self.point_col_buffer, self.point_count as u64 * 4, bytemuck::cast_slice(&up.cloud_col));
            // The delta's bases are relative to the delta; shift them into the shared buffers.
            for c in &up.clouds {
                self.cloud_draws.push(CloudDraw { base: self.point_count + c.base, count: c.count, instance: c.instance });
            }
            self.point_count += (up.cloud_pos.len() / 3) as u32;
        }

        self.last_origin = None; // force the next frame to rebase agains the new table
        // ONLY the walk tables know a box here, and a STREAMED cloud has none: its points never
        // pass through `add_file`, so `up.min` stays infinite and the cloud reports its box later
        // through `grow_scene`. Overwriting unconditionally therefore wiped the box of every cloud
        // already loaded - which is why F framed the LAST scan instead of all three.
        if up.min[0].is_finite() {
            self.scene_min = up.min;
            self.scene_max = up.max;
        }

        log::info!(
            "scene: {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres) {} cloud points",
            self.instances.len(), self.arena_vert_count, self.segment_count, self.pipe_count, self.glyph_count, self.sphere_count, self.point_count
        );

        let samples = self.msaa_now();
        if samples != self.samples {
            self.samples = samples;
            self.depth_view = Self::create_depth_view(&self.device, &self.config, samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, samples);
            self.pipelines = Pipelines::new(
                &self.device,
                samples,
                self.config.format,
                &self.mvp_layout,
                &self.time_layout,
                &self.instance_layout,
                &self.line_layout,
                &self.segment_layout,
                &self.glyph_layout,
                &self.cloud_layout
            );
            log::info!("msaa: {}x", samples);
        }
    }

    /// The anchor the instance table is rebased about.
    /// A full rebuild (42 000 x at stress scale) runs
    /// only when the camera target strays REANCHOR_DIST from the current anchor - orbit newer moves the target.
    /// And pan/zoom within the budget just changes the view matrix
    /// `origin` and `view_dist` are both in WORLD units (mm) - the same units as the instance
    /// table's translations. Feeding metres here (the camera's internal unit) makes the subtract
    /// below a no-op at 1/1000 scale, which silently turns camera-relative rendering off: the
    /// symptom is geometry that jitters and then clips away entirely as you zoom in, because the
    /// f32 mvp is differencing two large world magnitudes.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64) -> Point{
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let (dx, dy, dz) = (a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]);
                (dx * dx + dy * dy + dz * dz).sqrt() > thresh
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
        for (i, (model, color, _)) in self.objects_base.iter().enumerate() {
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
            self.depth_view = Self::create_depth_view(&self.device, &self.config, self.samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, self.samples);
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
            vp_w: self.config.width as f32,
            _pad0: [0.0; 3],
            anchor: self.last_origin.as_ref().map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]),
            _pad1: 0.0,
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
                // MSAA off (samples == 1): draw straight to the swapchain view - a
                // 1-sample attachment must NOT carry a resolve target.
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: if self.samples > 1 { &self.msaa_view } else { &view },
                    resolve_target: if self.samples > 1 { Some(&view) } else { None },
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
       

            // Pipelines - sequence of drawing is important:
            // background -> grid -> triangles -> cylinders -> CLOUD -> ink prepass -> ribbon
            // -> sphere -> glyph. Everything that WRITES depth comes first (the cloud included,
            // since it went opaque); the flat ink lanes read that depth and never write it.

            // Background
            pass.set_pipeline(&self.pipelines.background);
            pass.draw(0..3, 0..1); 
            draws += 1;

            // Grid first as the depth writes are off, all objects paints over it
            pass.set_pipeline(&self.pipelines.grid);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);   // for the anchor
            pass.draw(0..50, 0..1);
            draws += 1;

            // Meshes - coordinates, colors and normals are inside the gb.vbo computed
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);

            // Arena draw
            if self.arena_index_count > 0 {
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..)); // slot 0 - vertices
                pass.set_vertex_buffer(1, self.arena_vids.slice(..)); // slot 1 - per-vertex row ids
                pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_index_count, 0, 0..1); // whole scene, one call
            }
            draws += 1;

            // Linework, ONE draw per lane over the SAME segment table.
            // segments[0..pipe_count] = mesh/BRep edges -> real cylinders: the tube radius lifts
            // the ink off the surface it sits on, so silhouette edges never lose the depth test.
            // segments[pipe_count..] = line/polyline -> flat ribbons: nothing to fight with, and
            // they stay screen-constant and cheap.
            if self.pipe_count > 0 {
                pass.set_pipeline(&self.pipelines.cylinder);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count); // one template, N edges
                draws += 1;
            }

            // Raw cloud lane, drawn WITH THE SOLIDS: it is opaque and writes depth, so it belongs
            // before the flat ink, not after it. Here, ink in front of the cloud composites over
            // it and ink behind is rejected by the ribbon/glyph depth test. Drawn last - where it
            // sat while it was a blended overlay - an opaque cloud would instead overpaint every
            // polyline in front of it, because flat ink writes no depth of its own.
            if !self.cloud_draws.is_empty() {
                pass.set_pipeline(&self.pipelines.point);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.cloud_bind_group, &[]); // unused by the shader, kept to match the pipeline layout
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.point_bind_group, &[]);
                // ONE draw per cloud, not one per point: the draw's first_vertex makes
                // vertex_index absolute into the shared buffers, and first_instance puts the
                // cloud's instance row on instance_index. That pair is what let the per-point
                // instance_id (4 B x 13.8M) leave the row.
                for c in &self.cloud_draws {
                    pass.draw(c.base..c.base + c.count, c.instance..c.instance + 1);
                    draws += 1;
                }
            }
            // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
            // write depth (its AA feather would leave halos), so without this nothing in the flat
            // lane occludes anything else in it and pure draw order wins - a point dot then sits
            // on top of a polyline it is behind, at every camera angle.
            // COST: it draws the whole flat lane a SECOND time. On 2D sheets (600k segments, all
            // ribbons) that doubles the frame - so it is off by default and only worth enabling
            // for 3D scenes where ink-vs-ink order is actually visible.
            if INK_DEPTH_PREPASS && self.segment_count > self.pipe_count {
                pass.set_pipeline(&self.pipelines.ribbon_depth);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                pass.draw(6 * self.pipe_count..6 * self.segment_count, 0..1);
                draws += 1;
            }
            if INK_DEPTH_PREPASS && self.glyph_count > self.sphere_count {
                pass.set_pipeline(&self.pipelines.glyph_depth);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(3 * self.sphere_count..3 * self.glyph_count, 0..1);
                draws += 1;
            }

            if self.segment_count > self.pipe_count {
                pass.set_pipeline(&self.pipelines.ribbon);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                // vertex_index carries the base, so vid/6 still lands on the right segment row
                pass.draw(6 * self.pipe_count..6 * self.segment_count, 0..1);
                draws += 1;
            }

            // Vertex ink, same split: glyphs[0..sphere_count] = mesh/BRep vertices -> spheres
            // (radius encoded to match the pipes meeting there), the rest -> flat SDF dots.
            if self.sphere_count > 0 {
                pass.set_pipeline(&self.pipelines.sphere);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count); // one template, N glyphs
                draws += 1;
            }
            if self.glyph_count > self.sphere_count {
                pass.set_pipeline(&self.pipelines.glyph);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(3 * self.sphere_count..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
                draws += 1;
            }







        }


        let objects = self.instances.len() as u32;
        self.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
        Ok(())
    }


    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
    /// Forget what the arena holds, so the next upload writes from row 0 again. The buffers
    /// and their capacity stay - only the counters move - so a rebuild costs no allocation.
    /// Cloud buffers are deliberately untouched: streamed points are not re-walked.
    pub fn reset_arena(&mut self) {
        self.arena_vert_count = 0;
        self.arena_index_count = 0;
    }

    /// Make room for `need` point rows total, copying the live prefix GPU-side.
    ///
    /// EXACT, not doubling: appends here are few and huge, so doubling would waste 122 MB of
    /// buffer on the three-scan scene AND take the worse worst-transient (668 MB of old+new
    /// live at once against 540 MB). What doubling avoids is a GPU-side copy - the one thing
    /// here that never touches wasm memory.
    fn cloud_reserve(&mut self, need: u64) {
        if need <= self.point_capacity { return }
        let cap = need;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&self.device, "cloud.pos", cap * 12, usage);
        let col = zeroed_buffer(&self.device, "cloud.col", cap * 4, usage);
        if self.point_count > 0 {
            let mut enc = self.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.point_pos_buffer, 0, &pos, 0, self.point_count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.point_col_buffer, 0, &col, 0, self.point_count as u64 * 4);
            self.queue.submit([enc.finish()]);
        }
        self.point_pos_buffer = pos;
        self.point_col_buffer = col;
        self.point_capacity = cap;
        self.point_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points.bind_group"),
            layout: &self.cloud_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: self.point_pos_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: self.point_col_buffer.as_entire_binding() },
            ],
        });
    }

    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so both buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        self.cloud_reserve(self.point_count as u64 + count as u64);
        self.cloud_draws.push(CloudDraw { base: self.point_count, count, instance });
        self.cloud_pos_at = self.point_count;
        self.cloud_col_at = self.point_count;
        self.point_count += count;
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes a
    /// subarray VIEW of wasm memory - the slice is the only copy that exists.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        self.queue.write_buffer(&self.point_pos_buffer, self.cloud_pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.cloud_pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.queue.submit([]);
    }

    /// The colour run, packed to RGBA8.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.queue.write_buffer(&self.point_col_buffer, self.cloud_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.cloud_col_at += col.len() as u32;
        self.queue.submit([]);
    }

    /// Grow the scene box by a streamed cloud's world-space AABB, so the camera can fit it.
    pub fn grow_scene(&mut self, min: [f32; 3], max: [f32; 3]) {
        if !min[0].is_finite() { return }
        // set_scene collapses an empty upload to a zero box; the first cloud replaces it.
        if self.scene_min[0] >= self.scene_max[0] {
            self.scene_min = min;
            self.scene_max = max;
            return;
        }
        for k in 0..3 {
            self.scene_min[k] = self.scene_min[k].min(min[k]);
            self.scene_max[k] = self.scene_max[k].max(max[k]);
        }
    }

    /// while ribbons and dots antialias themselves in the shader. A 2D sheet therefore pays
    /// nothing, and a model with meshes gets clean silhouettes.
    /// MSAA follows what is ON THE GPU, not what arrived in the latest upload.
    ///
    /// This used to read `up.verts`/`up.pipes`/`up.spheres`, which was correct while every lane
    /// was cumulative. Now that the arena arrives as a DELTA, an upload carrying only cloud rows
    /// has an empty `up.verts` - so it reported "no solids", flipped 4x back to 1x, and rebuilt
    /// every pipeline and both render targets. In the mixed scene that thrashed 4x -> 1x -> 4x
    /// on every single append.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena_vert_count > 0 || self.pipe_count > 0 || self.sphere_count > 0;
        if solid { 4 } else { 1 }
    }

    /// Create the reverse-Z depth texture view, sized to the surface at the MSAA sample count.
    fn create_depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView{
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Create the multisampled color target the frame renders into (resolved to the surface each frame).
    fn create_msaa_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("msaa_color"),
            size: wgpu::Extent3d{ width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: samples,
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
pub struct Instance {
    model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
    color: [f32; 4], // 16 B
    flags: u32, // 4 B - reserved (selection)
    _pad: [u32; 3], // 12 B - pad the row to 96 B (storage array stride)
}

impl Instance {
    pub const FLAG_HIDDEN: u32 = 1 << 1; // Row is skipped by the draw, bit 0 is reserved for FLAG_SELECTED
}


//////////////////////////////////////////////////////////////////////////////////////////////////
/// Individual type memory layouts
//////////////////////////////////////////////////////////////////////////////////////////////////

// Memory layout is 16 (12+4), 16 (12+4) and 16
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment{
    pub p0: [f32; 3],   // 12 B - start point 
    pub radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 0 -> wolrd mm override
    pub p1: [f32; 3],   // 12 B - end point (p0..instance_id = 32 B of geometry)
    pub instance_id: u32,  // 4 B - row in instances[]: object model + flags (hide/select later)
    pub color: [f32; 4],  // 16 B - per - edge (black crease, naked color, ...)
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    _pad0: [f32; 3], // 12 B - WGSL aligns vec3<f32> to 16, so `anchor` starts at offset 32
    // The camera-relative ANCHOR, world units. Instance rows are rebased about it, so anything
    // NOT an instance - the grid, the axes - has to subtract it too or it drifts away from the
    // scene every time re-anchoring fires.
    anchor: [f32; 3],
    _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
} // 48 B - three vec4s

// The shaders declare this same struct with `anchor: vec3<f32>`, which WGSL aligns to 16 - so the
// uniform is 48 B there, not the 32 B a naive Rust layout gives. A mismatch is not a compile error:
// it surfaces at run time as "buffer bound with size 32 ... requires at least 48 bytes", every
// frame, from every pipeline that binds group 1.
const _: () = assert!(std::mem::size_of::<LineUniform>() == 48);


// One instance of the unit-sphere template.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint{
    pub center: [f32; 3], // 12 B - mesh-local
    pub radius: f32, // 4 B - 0.0 - screen-constant px; 0 - world mm
    pub color:  [f32; 4],
    pub instance_id: u32, // 4 B - row insntaces
    pub _pad: [u32; 3], // 12 B - single trailing scalar is why we need pad
} // 48 B total, three 16-byte rows

// The raw cloud lane has no row STRUCT any more - it has two parallel arrays, 12 B of position
// and 4 B of packed RGBA8 per point. What is left per CLOUD is this, and only this.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub base: u32,     // first point row, absolute in the shared buffers
    pub count: u32,    // how many points
    pub instance: u32, // which instance row - once per cloud instead of once per point
}

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

/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the empty-category placeholders both rely on that guarantee.
fn zeroed_buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages
) -> wgpu::Buffer {
    device.create_buffer(
        &wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
}

/// A read-only storage buffer that is never zero-sized (wgpu can't bind a 0-byte buffer).
/// When `data` is empty we still allocate one zeroed element; the real element count is
/// tracked separately, so the draw call issues 0 instances and nothing renders.
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, data: &[T]) -> wgpu::Buffer {
    let one = [T::zeroed()];
    let contents: &[u8] = if data.is_empty() { bytemuck::cast_slice(&one) } else { bytemuck::cast_slice(data) };
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: contents.len() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    queue.write_buffer(&buffer, 0, contents);
    buffer
}
