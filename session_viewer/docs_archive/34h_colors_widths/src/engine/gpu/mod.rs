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
mod adapters;
use adapters::{line_to_segment, nurbscurve_to_segments, point_to_glyph, polyline_to_segments, encode_width};
use bytemuck::Zeroable;
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};
use session_rust::mesh::ColorMode;

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
/// Routing lives in `walk_session`, one draw per lane in `clear`.

/// const for the unit_sphere method
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

/// Grid floor for load testing: at least STREE_GRID2 cells, cycling the loaded files
const STRESS_GRID: u32 = 1;

/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
/// Off: on 2D sheets (600k segments, all ribbons) the second pass doubles the frame.
const INK_DEPTH_PREPASS: bool = false;

/// One loaded file, walked into GPU-ready tables. Built by [`Gpu::walk_session`] BEFORE
/// `Gpu::new`, so the parsed `Session` (often 10× larger than these tables) can be dropped
/// before the next file is fetched — peak memory holds ONE session at a time, not all of them.
pub struct SceneTables {
    verts: Vec<RenderVertex>,
    vids: Vec<u32>,
    idx: Vec<u32>,
    pipes: Vec<CylinderSegment>,   // SOLID lane: mesh/BRep edges, drawn as 3D cylinders
    spheres: Vec<GlyphPoint>,      // SOLID lane: mesh/BRep vertices, radius matched to the pipes
    segments: Vec<CylinderSegment>,// FLAT lane: line/polyline, drawn as camera-facing ribbons
    glyphs: Vec<GlyphPoint>,       // FLAT lane: points, drawn as SDF dots
    objects: Vec<(Xform, [f32; 4])>,
    min: [f32; 3],
    max: [f32; 3],
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
    pub pipe_count: u32,  // segments[0..pipe_count] are the SOLID lane, the rest are ribbons
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub sphere_count: u32, // glyphs[0..sphere_count] are the SOLID lane, the rest are flat dots
    pub point_buffer: wgpu::Buffer,
    pub point_bind_group: wgpu::BindGroup,
    pub point_count: u32,
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
    /// `files` carries each loaded file WITH the placement the scene manifest gave it - the
    /// viewer no longer decides where a sheet goes (see `app::scene`).
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[(SceneTables, Xform)]) -> anyhow::Result<Self> {
        

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

        // Depth and MSAA
        let samples = Self::msaa_for(files);
        log::info!("msaa: {}x", samples);
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




        // Merge the per-file tables into one arena: mesh indices shift by the vertex base,
        // row ids (vids / instance_id) by the objects base, so every file keeps distinct rows.
        let mut verts: Vec<RenderVertex> = Vec::new(); // slot 0 - every mesh's vertices, concatenated
        let mut vids: Vec<u32> = Vec::new(); // slot 1 - one row id per vertex (@location 3)
        let mut idx: Vec<u32> = Vec::new(); // the shared index buffer
        // Two lanes per table, kept apart while merging and concatenated solid-first at upload,
        // so each pipeline draws ONE contiguous range of the same buffer (no second binding).
        let mut pipes: Vec<CylinderSegment> = Vec::new();
        let mut spheres: Vec<GlyphPoint> = Vec::new();
        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut glyphs: Vec<GlyphPoint> = Vec::new();
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::new();
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];

        // for t in files {
        //     let vbase = verts.len() as u32;
        //     let obase = objects_base.len() as u32;
        //     verts.extend_from_slice(&t.verts);
        //     vids.extend(t.vids.iter().map(|r| r + obase));
        //     idx.extend(t.idx.iter().map(|i| i + vbase));
        //     segments.extend(t.segments.iter().map(|s| CylinderSegment { instance_id: s.instance_id + obase, ..*s }));
        //     glyphs.extend(t.glyphs.iter().map(|g| GlyphPoint { instance_id: g.instance_id + obase, ..*g }));
        //     objects_base.extend(t.objects.iter().cloned());
        //     for k in 0..3 {
        //         scene_min[k] = scene_min[k].min(t.min[k]);
        //         scene_max[k] = scene_max[k].max(t.max[k]);
        //     }
        // }
        
        // The merge loop: cells cycle through loaded files (a different per cell).
        // STRESS_GRID² floors the cell count, cell size = largest file extend +5% gutters.
        // Bounds accumulate per placed cell (offset included) - F first the whole wall.
        let cells = if files.is_empty() {
            0
        } else {
            ((STRESS_GRID * STRESS_GRID) as usize).max(files.len())
        };

        if cells > 0 {
            let cols = (cells as f64).sqrt().ceil() as usize;
            let (mut cell_w, mut cell_h) = (0.0_f64, 0.0_f64);
            for (t, _) in files{
                cell_w = cell_w.max((t.max[0] - t.min[0]) as f64);
                cell_h = cell_h.max((t.max[1] - t.min[1]) as f64);
            }
            let (dx, dy) = (cell_w * 1.05, cell_h * 1.05); // 5% gutters
            for cell in 0..cells{
                let (t, place) = &files[cell % files.len()];
                // The file's own placement, from the manifest. STRESS_GRID copies still spread
                // out on a grid - that is a load test, not a scene - but with one copy per file
                // the offset is zero and `place` alone decides where the sheet sits.
                let o = if STRESS_GRID > 1 {
                    [(cell % cols) as f64 * dx, (cell / cols) as f64 * dy, 0.0]
                } else {
                    [0.0, 0.0, 0.0]
                };
                let off = &Xform::translation(o[0], o[1], o[2]) * place;
                let ri0 = objects_base.len() as u32; // this cell's first arena vertex
                let vbase = verts.len() as u32;
                for (m,c) in &t.objects {
                    objects_base.push((&off * m, *c));
                }
                verts.extend_from_slice(&t.verts);
                for id in &t.vids {
                    vids.push(id + ri0);
                }
                for i in &t.idx {
                    idx.push(i+vbase);
                }
                for s in &t.pipes {
                    let mut s2 = *s;
                    s2.instance_id += ri0;
                    pipes.push(s2);
                }
                for g in &t.spheres {
                    let mut g2 = *g;
                    g2.instance_id += ri0;
                    spheres.push(g2);
                }
                for s in &t.segments {
                    let mut s2 = *s;
                    s2.instance_id += ri0;
                    segments.push(s2);
                }
                for g in &t.glyphs {
                    let mut g2 = *g;
                    g2.instance_id += ri0;
                    glyphs.push(g2);
                }
                // Scene bounds include the placement - "f" fits everything that is on screen.
                for k in 0..3 {
                    let shift = off.m[12 + k] as f32;   // the placement's translation column
                    scene_min[k] = scene_min[k].min(t.min[k] + shift);
                    scene_max[k] = scene_max[k].max(t.max[k] + shift);
                }
            }
        }

        if !scene_min[0].is_finite() { // no geometry at all - the box the old padded scan produced
            scene_min = [0.0; 3];
            scene_max = [0.0; 3];
        }

        let mut instances: Vec<Instance> = objects_base.iter()
        .map(|(m, c)| Instance {
            model: m.to_f32(),
            color: *c,
            flags: 0,
            _pad: [0; 3]
        }).collect();

        // One buffer per table, SOLID lane first: cylinders draw instances 0..pipe_count and
        // ribbons the rest, spheres 0..sphere_count and dots the rest.
        let pipe_count = pipes.len() as u32;
        let sphere_count = spheres.len() as u32;
        let mut segments = { pipes.extend(segments); pipes };
        let mut glyphs = { spheres.extend(glyphs); spheres };

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

        log::info!("grid: {} cells x {} files, {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres)",
            cells, files.len(), instances.len(), verts.len(), segments.len(), pipe_count, glyphs.len(), sphere_count);





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
            samples,
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
            pipe_count,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            sphere_count,
            point_buffer,
            point_bind_group,
            point_count,
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

    /// One file → compact tables. Called from state.rs BEFORE Gpu::new, so the parsed
    /// Session (and its bytes) can be dropped before the next file is fetched.
    pub fn walk_session(session: &session_rust::Session) -> SceneTables {
        let mut t = SceneTables {
            verts: Vec::new(),
            vids: Vec::new(),
            idx: Vec::new(),
            pipes: Vec::new(),
            spheres: Vec::new(),
            segments: Vec::new(),
            glyphs: Vec::new(),
            objects: Vec::with_capacity(session.lookup.len()),
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        };
        // Placement lives in the SESSION, not on the geometry - `to_render()`/`start()`/
        // `get_points()` read stored coordinates, so the world xform IS the instance model.
        // `world_xforms()` resolves the whole tree in one pass; asking per object would
        // rescan the tree for every row. `ri` is the row in objects, not the lookup index -
        // skipped variants (Plane/OBB/...) leave no hole.
        let world = session.world_xforms();
        let placement = |guid: &str| world.get(guid).cloned().unwrap_or_else(session_rust::Xform::identity);
        for geom in session.lookup.values() {
            let ri = t.objects.len() as u32;
            match geom {
                // 3D geometry takes the SOLID lane: edges are real cylinders and vertices real
                // spheres, so ink is lifted off the surface by its own radius instead of being
                // a flat quad at the surface's depth (which loses the depth test at silhouettes).
                Geometry::Mesh(m) => {
                    t.objects.push((placement(m.guid()), [1.0; 4] ));
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    t.objects.push((placement(b.guid()), [1.0; 4]));
                    push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
                Geometry::Line(l) => {
                    t.objects.push((placement(l.guid()), [1.0; 4]));
                    t.segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    t.objects.push((placement(pl.guid()), [1.0; 4]));
                    t.segments.extend(polyline_to_segments(pl, ri));
                }
                // Curves ride the FLAT lane too - sampled to segments, they ARE polylines by
                // the time the GPU sees them. A PDF sheet is mostly these (every bezier, and
                // every glyph outline once fonts are flattened to paths).
                Geometry::NurbsCurve(c) => {
                    t.objects.push((placement(c.guid()), [1.0; 4]));
                    t.segments.extend(nurbscurve_to_segments(c, ri));
                }
                Geometry::Point(p) => {
                    t.objects.push((placement(p.guid()), [1.0; 4]));
                    t.glyphs.push(point_to_glyph(p, ri));
                }
                // Later lessons - the match must stay exhaustive over all 11 variants
                Geometry::Plane(_) | Geometry::OBB(_) |
                Geometry::PointCloud(_) | Geometry::Element(_) |
                Geometry::NurbsSurface(_) => {}
            }
        }
        // This FILE's extents in WORLD placement (apply each object's xform), so the stress-grid
        // cell size matches what is actually drawn and files do not overlap each other.
        for (i, v) in t.verts.iter().enumerate() {
            if let Some(&ri) = t.vids.get(i) {
                if let Some((xf, _)) = t.objects.get(ri as usize) {
                    grow_bounds(&mut t.min, &mut t.max, xform_point(xf, v.position));
                }
            }
        }
        for s in t.pipes.iter().chain(t.segments.iter()) {
            if let Some((xf, _)) = t.objects.get(s.instance_id as usize) {
                grow_bounds(&mut t.min, &mut t.max, xform_point(xf, s.p0));
                grow_bounds(&mut t.min, &mut t.max, xform_point(xf, s.p1));
            }
        }
        for g in t.spheres.iter().chain(t.glyphs.iter()) {
            if let Some((xf, _)) = t.objects.get(g.instance_id as usize) {
                grow_bounds(&mut t.min, &mut t.max, xform_point(xf, g.center));
            }
        }
        // 2D Drawing sheets (exactly planar, z = 0 - every PDF conversion get paper space)
        // lineweights: kernel width (mm on the sheet) - the radius world lane, so zooming out
        // thins the ink like a real print. 3D model files keep screen-constant px linework
        let planar = t.min[2].is_finite() && (t.max[2] - t.min[2]).abs() < 1e-3;
        if planar {
            for s in t.pipes.iter_mut().chain(t.segments.iter_mut()) {
                s.radius = if s.radius < 0.0 { -s.radius * 0.5 } else { 0.5 }
            }
        }
        t
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
       

            // Pipelines - sequence of drawing is important

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
            pass.set_vertex_buffer(0, self.arena_vbo.slice(..)); // slot 0 - vertices
            pass.set_vertex_buffer(1, self.arena_vids.slice(..)); // slot 1 - per-vertex row ids
            pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.arena_index_count, 0, 0..1); // whole scene, one call
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


    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
    /// while ribbons and dots antialias themselves in the shader. A 2D sheet therefore pays
    /// nothing, and a model with meshes gets clean silhouettes.
    fn msaa_for(files: &[(SceneTables, Xform)]) -> u32 {
        let solid = files.iter().any(|(f, _)| !f.verts.is_empty() || !f.pipes.is_empty() || !f.spheres.is_empty());
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

    // Edge width 0 = HIDDEN wireframe. A mesh only has explicit widths if someone called
    // set_linecolors, so the 1.0 default below leaves every ordinary mesh untouched - but a
    // triangulated PDF fill (a letter, a poché region) asks for no wireframe at all, and without
    // this every glyph would render outlined in tubes and dotted at each vertex.
    // A single width BROADCASTS to every edge - one entry instead of one per edge, which for
    // thousands of small glyph meshes is the difference between a lean .pb and a fat one.
    let width_at = |i: usize| -> f64 {
        let w = m.widths();
        if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
    };
    let hidden = |i: usize| width_at(i) == 0.0;

    for (i, (a, b, col)) in m.edges_with_colors().into_iter().enumerate(){
        if hidden(i) { continue }
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: encode_width(width_at(i)),
                p1: pb.to_f32(),
                instance_id: ri,
                color: col.to_f32()
            }
        )
    }

    // Dots honor user-set pointcolors.
    // The auto-seeded white vec is filtered by the MODE gate.
    // m.vertices() is sorted - the same order to_render indexes pointcolors by.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    // A vertex sphere must be as fat as the pipes that meet there, or the joint shows a pinch
    // (thinner sphere) or a bead (fatter one). The kernel has no per-vertex width, so take the
    // widest incident edge - the same encoding the pipes above are pushed with.
    let mut vwidth: std::collections::HashMap<usize, f64> = std::collections::HashMap::new();
    for (i, (a, b, _)) in m.edges_with_colors().into_iter().enumerate(){
        if hidden(i) { continue }   // a vertex whose every edge is hidden gets no dot either
        let w = width_at(i);
        for vk in [a, b] {
            let e = vwidth.entry(vk).or_insert(w);
            if w > *e { *e = w; }
        }
    }
    for (i,vk) in m.vertices().into_iter().enumerate(){
        let Some(&vw) = vwidth.get(&vk) else { continue };
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(
            GlyphPoint {
                center: p.to_f32(),
                radius: encode_width(vw),
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: ri,
                _pad: [0;3] }
        );
    }
}

fn xform_point(xf: &Xform, p: [f32; 3]) -> [f32; 3] {
    let x = p[0] as f64;
    let y = p[1] as f64;
    let z = p[2] as f64;
    [
        (xf.m[0] * x + xf.m[4] * y + xf.m[8] * z + xf.m[12]) as f32,
        (xf.m[1] * x + xf.m[5] * y + xf.m[9] * z + xf.m[13]) as f32,
        (xf.m[2] * x + xf.m[6] * y + xf.m[10] * z + xf.m[14]) as f32,
    ]
}

fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {
    for k in 0..3 {
        min[k] = min[k].min(p[k]);
        max[k] = max[k].max(p[k]);
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
