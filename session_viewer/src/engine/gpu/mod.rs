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
    /// Mesh-local AABB per object row, aligned with `objects`. None for linework/points/clouds:
    /// only the solid lane's facing cull needs it (see `Instance::FLAG_INSIDE`).
    pub object_bounds: Vec<Option<([f32; 3], [f32; 3])>>,
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
            object_bounds: Vec::new(),
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        }
    }
}

/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory - which is the whole reason
/// the two lanes were built over one buffer. Easy3D ships exactly this pair
/// (`lines_cylinders_*` against `lines_plain_*_width_control`) and lets you pick.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface it
    /// decorates so silhouette edges never lose the depth test.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader. Cheaper, and it
    /// lies IN the surface rather than proud of it.
    Flat,
}

pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // Screen to draw pixels on; None when headless.
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
    /// Per-object WORLD AABB (ArenaUpload.object_bounds through the true transform), aligned with
    /// `instances`. Drives FLAG_INSIDE - see update_inside_flags.
    object_bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    inside: Vec<bool>, // current FLAG_INSIDE state per instance row, for change detection
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
    /// Solid-lane style; `VIEWER_LINE_STYLE=flat` picks Flat at startup.
    pub line_style: LineStyle,
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
        let size = window.inner_size();
        Self::build(Some(window), size.width.max(1), size.height.max(1)).await
    }

    /// Same stack with no window and no surface, rendering into an offscreen texture. Exists so
    /// a shader can be checked against a PNG on this machine instead of against the user's eyes.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, width.max(1), height.max(1)).await
    }

    async fn build(
        window: Option<std::sync::Arc<winit::window::Window>>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        

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
        let surface = match &window { Some(w) => Some(instance.create_surface(w.clone())?), None => None };
        // LowPower = the iGPU the compositor runs on. On hybrid laptops the discrete GPU renders
        // fine but its frames can't be shared to the compositor - the canvas stays black.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: surface.as_ref(),
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
        // Headless has no capabilities to ask, so pick the format the readback path wants.
        let (format, present_mode, alpha_mode) = match &surface {
            Some(s) => {
                let caps = s.get_capabilities(&adapter);
                let f = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
                (f, caps.present_modes[0], caps.alpha_modes[0])
            }
            None => (
                wgpu::TextureFormat::Rgba8UnormSrgb,
                wgpu::PresentMode::Fifo,
                wgpu::CompositeAlphaMode::Auto,
            ),
        };
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        if let Some(s) = &surface { s.configure(&device, &config); }

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

        // Camera-facing quad template (positions-only) - one mesh, instance per marker
        let (sph_v, sph_i) = unit_quad();
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
                eye: [0.0; 3],   // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("line.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                // FRAGMENT too: the flat lane's fragment stage reads the viewport size to
                // recover the fragment's ndc for the face-plane depth solve (ribbon.wgsl
                // `ink_depth`). Everything else still only touches it from the vertex stage.
                visibility: wgpu::ShaderStages::VERTEX.union(wgpu::ShaderStages::FRAGMENT),
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
            object_bounds_world: Vec::new(),
            inside: Vec::new(),
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
            line_style: if std::env::var("VIEWER_LINE_STYLE").map(|v| v.eq_ignore_ascii_case("flat")).unwrap_or(false) {
                LineStyle::Flat
            } else {
                LineStyle::Tubes
            },
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
        debug_assert_eq!(up.objects.len(), up.object_bounds.len());
        self.object_bounds_world = up.objects.iter().zip(&up.object_bounds).map(|((m, _, _), b)| {
            b.map(|(lo, hi)| {
                // World AABB of the local box: the 8 corners through the true transform.
                // Conservative for rotated placements - FLAG_INSIDE is a hint, not a cull.
                let xp = |x: f64, y: f64, z: f64| [
                    m.m[0] * x + m.m[4] * y + m.m[8] * z + m.m[12],
                    m.m[1] * x + m.m[5] * y + m.m[9] * z + m.m[13],
                    m.m[2] * x + m.m[6] * y + m.m[10] * z + m.m[14],
                ];
                let mut wlo = [f64::INFINITY; 3];
                let mut whi = [f64::NEG_INFINITY; 3];
                for c in 0..8 {
                    let p = xp(
                        (if c & 1 == 0 { lo[0] } else { hi[0] }) as f64,
                        (if c & 2 == 0 { lo[1] } else { hi[1] }) as f64,
                        (if c & 4 == 0 { lo[2] } else { hi[2] }) as f64,
                    );
                    for k in 0..3 { wlo[k] = wlo[k].min(p[k]); whi[k] = whi[k].max(p[k]); }
                }
                (wlo, whi)
            })
        }).collect();
        self.inside = vec![false; up.objects.len()];
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
            if let Some(s) = &self.surface { s.configure(&self.device, &self.config); }
            self.depth_view = Self::create_depth_view(&self.device, &self.config, self.samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, self.samples);
        }
    }

    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
    /// (mesh, line, grid, …) get added here in later chapters.
    /// Draw one frame to the swapchain. The frame ENCODING lives in `encode_frame` so a
    /// headless harness can aim the same code at an offscreen texture and read the pixels back -
    /// see `selftest.rs`. Shader work that is only ever checked in a browser is shader work
    /// checked by somebody else's eyes.
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {
        self.write_frame_uniforms(view_proj);

        // wgpu 29: get_current_texture() returns an enum, not a Result.
        let Some(surface) = &self.surface else { return Ok(()) }; // headless: nothing to present
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => { surface.configure(&self.device, &self.config); return Ok(()); }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("clear encoder"),
        });
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        self.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
        Ok(())
    }

    /// Render one frame into an offscreen texture and read the pixels back (RGBA8, tightly
    /// packed, top row first). Native only - this is the harness that lets a shader be looked at
    /// on this machine before it is shipped to a browser.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_offscreen(&mut self, color: wgpu::Color, view_proj: &Xform) -> Vec<u8> {
        let (w, h) = (self.config.width, self.config.height);
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("headless.color"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        // copy_texture_to_buffer needs each row padded to 256 B
        let unpadded = w * 4;
        let pad = (256 - unpadded % 256) % 256;
        let padded = unpadded + pad;
        let readback = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("headless.readback"),
            size: (padded * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        self.write_frame_uniforms(view_proj);
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(h) },
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.queue.submit([encoder.finish()]);
        log::info!("headless frame: {draws} draws, {objects} objects, {w}x{h}");

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((unpadded * h) as usize);
        for row in 0..h {
            let a = (row * padded) as usize;
            out.extend_from_slice(&data[a..a + unpadded as usize]);
        }
        drop(data);
        readback.unmap();
        out
    }

    /// The camera position, recovered from the combined view-projection alone.
    ///
    /// The eye is the one point that projects to nothing: it is where the clip x, y and w all
    /// vanish at once, because every view ray passes through it. Three rows of the matrix, three
    /// unknowns, one 3x3 solve - no camera struct needed, so this works for any caller that can
    /// produce a view-projection, including the headless harness.
    ///
    /// Orthographic has no eye: rows 0, 1 and 3 are linearly dependent there (w is constant 1),
    /// the determinant collapses, and the fallback is the view direction pushed a long way back -
    /// which is exactly what an orthographic "eye at infinity" means.
    pub fn eye_from_view_proj(vp: &Xform) -> [f32; 3] {
        let r = |i: usize| [vp[(i, 0)], vp[(i, 1)], vp[(i, 2)], vp[(i, 3)]];
        let (a, b, c) = (r(0), r(1), r(3));

        // Cramer on [a b c] . p = -[a3 b3 c3]
        let det3 = |m: [[f64; 3]; 3]| {
            m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
        };
        let rows = [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
        let rhs = [-a[3], -b[3], -c[3]];
        let d = det3(rows);

        // Scale-free singularity test: compare against the product of the row magnitudes, so it
        // fires on genuine dependence rather than on a scene whose units make everything small.
        let norm: f64 = rows.iter().map(|r| (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()).product();
        if d.abs() <= 1e-9 * norm.max(1e-30) {
            // Orthographic: row 3 carries no direction, so take the view axis from row 2 (depth)
            // and stand a long way back along it.
            let f = [vp[(2, 0)], vp[(2, 1)], vp[(2, 2)]];
            let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(1e-30);
            return [0, 1, 2].map(|k| (f[k] / len * 1.0e9) as f32);
        }

        [0, 1, 2].map(|k| {
            let mut m = rows;
            for row in 0..3 {
                m[row][k] = rhs[row];
            }
            (det3(m) / d) as f32
        })
    }

    /// Per-frame uniforms: time, camera, and the line/pen block.
    fn write_frame_uniforms(&mut self, view_proj: &Xform) {
        // Time for triangle wgsl buffer.
        self.time += 1.0 / 60.0;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&view_proj.to_f32()));

        let line = LineUniform{
            thickness: line_thickness_px(), // env VIEWER_THICKNESS; later an egui slider
            proj_y: 1.0 / (30.0_f32).to_radians().tan() * 0.001, // cot(fovy/2) mm-m unit scale
            ortho_h: 0.0, // perspective, set the ortho half-height when ortho
            vp_h: self.config.height as f32,
            vp_w: self.config.width as f32,
            eye: Self::eye_from_view_proj(view_proj),
            anchor: self.last_origin.as_ref().map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]),
            _pad1: 0.0,
        };
        self.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
        self.update_inside_flags(view_proj);
    }

    /// Per-frame refresh of Instance::FLAG_INSIDE. The facing cull in both edge lanes assumes the
    /// eye is OUTSIDE the solid (both adjacent faces turned away = hidden edge); from inside, every
    /// face points away and the whole object loses its wireframe the moment the camera crosses a
    /// face. Per-edge data cannot tell "far side of the solid" from "eye inside it" - that
    /// difference is global - so the CPU answers it per object from the world AABBs, and the answer
    /// rides the instance row. One containment test per object per frame; the instance buffer is
    /// rewritten only when some answer flips, which orbit/zoom almost never does.
    fn update_inside_flags(&mut self, view_proj: &Xform) {
        if self.object_bounds_world.iter().all(Option::is_none) {
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let eye = Self::eye_from_view_proj(view_proj); // anchored world units, like instances[]
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= self.scene_min[k] as f64 && ew[k] <= self.scene_max[k] as f64);
        let mut dirty = false;
        for (i, b) in self.object_bounds_world.iter().enumerate() {
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
            if self.inside.get(i).copied().unwrap_or(false) == inside {
                continue;
            }
            if let Some(row) = self.instances.get_mut(i) {
                row.flags = if inside { row.flags | Instance::FLAG_INSIDE } else { row.flags & !Instance::FLAG_INSIDE };
            }
            if i < self.inside.len() { self.inside[i] = inside; }
            dirty = true;
        }
        if dirty {
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
        }
    }

    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
    ) -> (u32, u32) {
        let mut draws = 0u32;

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                // MSAA off (samples == 1): draw straight to the swapchain view - a
                // 1-sample attachment must NOT carry a resolve target.
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: if self.samples > 1 { &self.msaa_view } else { view },
                    resolve_target: if self.samples > 1 { Some(view) } else { None },
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
            // background -> grid -> triangles -> sphere markers -> cylinders -> CLOUD -> ink
            // prepass -> ribbon -> glyph. Everything that WRITES depth comes first (the cloud
            // included, since it went opaque); the flat ink lanes read that depth and never
            // write it. The markers go with the solids so the line ink tests against them -
            // a vertex marker is the topmost ink at its own joint.

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

            // Vertex markers, right after the faces and BEFORE any linework: the marker depth-tests
            // against surfaces only, so a vertex hidden by the solid stays hidden, and the bands
            // and tubes drawn afterwards test against the marker's depth and lose the joint to it
            // (their hug is face+eps, the marker's is face+eps+SPHERE_TIE). The line ink can still
            // cover a marker where it is genuinely nearer (an edge approaching the eye past a far
            // corner) - that is occlusion, not a lost tie.
            //
            // Under VIEWER_NO_DEPTH (the acceptance harness) the marker instead draws LATE, at the
            // end of the pass: bands are Always then and would otherwise overwrite the markers in
            // the oracle run only, and the on/off comparison would drown in disc-sized clusters.
            let markers_late = std::env::var("VIEWER_NO_DEPTH").is_ok();
            if self.sphere_count > 0 && !markers_late {
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

            // Linework, ONE draw per lane over the SAME segment table.
            // segments[0..pipe_count] = mesh/BRep edges -> real cylinders: the tube radius lifts
            // the ink off the surface it sits on, so silhouette edges never lose the depth test.
            // segments[pipe_count..] = line/polyline -> flat ribbons: nothing to fight with, and
            // they stay screen-constant and cheap.
            if self.pipe_count > 0 {
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                match self.line_style {
                    LineStyle::Tubes => {
                        pass.set_pipeline(&self.pipelines.cylinder);
                        pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                        pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count); // one template, N edges
                    }
                    // The flat lane's own shader over the SOLID half of the same table. vid/6
                    // picks the row, so the range is simply the pipes prefix.
                    LineStyle::Flat => {
                        pass.set_pipeline(&self.pipelines.ribbon_solid);
                        pass.draw(0..6 * self.pipe_count, 0..1);
                    }
                }
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

            // Vertex ink, same split: glyphs[0..sphere_count] = mesh/BRep vertices -> markers
            // (DRAWN EARLIER - right after the faces; see there), the rest -> flat SDF dots.
            if self.glyph_count > self.sphere_count {
                pass.set_pipeline(&self.pipelines.glyph);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(3 * self.sphere_count..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
                draws += 1;
            }

            // Acceptance-harness-only marker position (see the early draw for why): with every ink
            // lane's depth test forced off, the marker - also Always - must come LAST so the oracle
            // frame still shows the markers on top, matching the normal run.
            if self.sphere_count > 0 && markers_late {
                pass.set_pipeline(&self.pipelines.sphere);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count);
                draws += 1;
            }







        }

        (draws, self.instances.len() as u32)
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
    /// The eye is inside this object's bounds (per-frame CPU test, see `update_inside_flags`).
    /// Both edge lanes then skip the facing cull - from inside a solid every face points away -
    /// and the flat lane hugs BOTH adjacent face planes, since the back-facing ones are the
    /// visible surface from in there. Bit 2, matching FLAG_INSIDE in ribbon.wgsl/cylinder.wgsl.
    pub const FLAG_INSIDE: u32 = 1 << 2;
}


//////////////////////////////////////////////////////////////////////////////////////////////////
/// Individual type memory layouts
//////////////////////////////////////////////////////////////////////////////////////////////////

// Memory layout is 16 (12+4), 16 (12+4) and 16
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment{
    // The two ends are FLAT f32s, not `[f32; 3]`, and that is deliberate. WGSL gives `vec3<f32>`
    // an alignment of 16, so any struct containing one is padded to a multiple of 16 - this table
    // was 48 B and could not have been 40 whatever else was packed. Scalars align to 4, so the
    // stride is the honest sum of the fields. Costs one `vec3<f32>(..)` per end in the shaders.
    pub p0: [f32; 3],   // 12 B - start point
    pub radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 0 -> wolrd mm override
    pub p1: [f32; 3],   // 12 B - end point (p0..instance_id = 32 B of geometry)
    pub instance_id: u32,  // 4 B - row in instances[]: object model + flags (hide/select later)
    // Was `[f32; 4]` - 16 B carrying what is really 8-bit RGBA. Packing it paid for `facing`
    // AND took 8 B off every segment: 48 -> 40, which is 20% of the biggest table in the viewer
    // (118 MB at mesh-stress scale).
    pub color: u32,     // 4 B - RGBA8, low byte red
    // The two faces this edge belongs to, as octahedral unit normals, 16 bits each - about 1.4
    // degrees, when all that is asked of them is the SIGN of a dot product (the facing cull) and
    // a plane to hug (the flat lane's depth solve). This is what lets the shader answer "is this
    // edge facing the camera" without the depth buffer: both faces facing away means the edge is
    // hidden and must not be drawn at all. FACING_UNKNOWN = unknown, always draw (polylines,
    // drawing linework, BRep edges with no adjacency); 0 is a real value - a +Z/+Z face pair.
    pub facing: u32,    // 4 B
}                       // 40 B

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    // Camera position, in the SAME anchored frame the instance rows use - so a shader can build
    // the view ray to a point as `eye - p`. That is what the per-edge facing test needs, and it
    // has to be the real eye rather than a constant forward direction: at this 60 degree FOV a
    // constant direction is off by up to 30 degrees at the frame corner, and it is precisely the
    // near-silhouette edges - the ones whose classification is in doubt - that would flip.
    eye: [f32; 3],   // 12 B - and it fills the pad WGSL leaves before `anchor`'s 16 B alignment
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
    // Up to SIX incident face normals (oct16 pairs), widest incident edge's two first - the same
    // adjacency CylinderSegment carries one word of. A marker that hugs only the widest edge's
    // two faces still loses a sector of its disc to the THIRD face's band at a trihedral corner
    // (measured on a box corner); all-ones (FACING_UNKNOWN) means "no adjacency / no more".
    pub facing: u32,
    pub facing_ext: [u32; 2],
} // 48 B total, three 16-byte rows

// The WGSL GlyphPoint (glyph.wgsl AND sphere.wgsl - same table) is exactly this layout; the
// array stride is the struct's, so a drift here misreads every row.
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

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


/// Camera-facing quad template (positions + indices) for the instanced vertex markers. The
/// shader expands it in SCREEN space and trims to a circle in the fragment with a 1px AA ramp,
/// so the silhouette is a perfect circle at any radius. This replaced a tessellated unit sphere:
/// 6x3 segments was a comment-era choice ("a few pixels across") that reads as a hexagon at the
/// sizes world-mm pens reach, and any fixed tessellation is still a polygon when you zoom in -
/// the SDF is exact and cheaper (2 triangles instead of 36+).
fn unit_quad() -> (Vec<[f32; 3]>, Vec<u32>) {
    let v = vec![
        [-1.0, -1.0, 0.0],
        [ 1.0, -1.0, 0.0],
        [ 1.0,  1.0, 0.0],
        [-1.0,  1.0, 0.0],
    ];
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}

/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the empty-category placeholders both rely on that guarantee.
/// On-screen pen weight in px. `VIEWER_THICKNESS` overrides it so the headless harness can
/// sweep line weight without a rebuild; unset (and always on wasm) it is the usual 2.0.
fn line_thickness_px() -> f32 {
    std::env::var("VIEWER_THICKNESS").ok().and_then(|v| v.parse().ok()).unwrap_or(2.0)
}

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
