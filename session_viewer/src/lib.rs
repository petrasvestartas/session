// ============================================================
// IMPORTS
// ============================================================
use std::collections::{HashMap, HashSet};
use std::iter;
use std::sync::Arc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use winit::{
    application::ApplicationHandler,
    event::{MouseScrollDelta, *},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    window::Window,
};

mod camera;
mod pipelines;
mod gpu_arena;
mod gpu_adapters;
mod gpu_session;
mod gpu_instance_groups;
mod gumball;
mod pick;
use camera::{Camera, CameraController, ProjMode};
use pipelines::{
    build_bind_group, create_camera_buffer, create_glyph_bind_group, CameraUniform, Pipelines,
};

use wgpu::util::DeviceExt;
use gpu_session::{GpuSession, InstanceData};
use gumball::{Gumball, HandleKind};
use pick::screen_to_world_ray;
use session_rust::session::Geometry;
use session_rust::{BRep, Color, Line, Point, Polyline, Primitives, Session, TreeNode, Xform};

mod demo;
mod text;
mod tree_ui;
use tree_ui::{
    auto_lock_leaf_groups, collect_group_leaf_guids, collect_group_leaves,
    locked_group_for_guid, populate_leaf_cache, render_tree_node,
};

fn labels_from_session(session: &Session) -> Vec<text::TextLabel> {
    session.lookup.iter()
        .filter_map(|(guid, geom)| {
            if let Geometry::Point(p) = geom {
                if !p.name.is_empty() && p.name != "my_point" {
                    return Some(text::TextLabel {
                        guid: guid.clone(),
                        position: [p[0], p[1], p[2]],
                        text: p.name.clone(),
                        color: [
                            (p.pointcolor.r * 255.0) as u8,
                            (p.pointcolor.g * 255.0) as u8,
                            (p.pointcolor.b * 255.0) as u8,
                            255,
                        ],
                    });
                }
            }
            None
        })
        .collect()
}


/// Column-major 4×4 matrix multiply: out = a * b.
fn mat4_mul_cm(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0f32; 4]; 4];
    for c in 0..4 {
        for row in 0..4 {
            let mut s = 0.0f32;
            for k in 0..4 { s += a[k][row] * b[c][k]; }
            out[c][row] = s;
        }
    }
    out
}


fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32, sample_count: u32) -> (wgpu::Texture, wgpu::TextureView) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, view)
}

fn create_msaa_texture(device: &wgpu::Device, width: u32, height: u32, format: wgpu::TextureFormat, sample_count: u32) -> wgpu::TextureView {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("msaa_texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    tex.create_view(&wgpu::TextureViewDescriptor::default())
}


// ============================================================
// STATE
// ============================================================
pub struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    mouse_position: (f64, f64),
    clear_color: wgpu::Color,
    pipelines: Pipelines,
    gpu_session: GpuSession,
    session: Session,
    selected_guids: HashSet<String>,
    pending_pick: Option<(f64, f64)>,
    camera: Camera,
    controller: CameraController,
    camera_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    depth_tex_raw: wgpu::Texture,
    depth_view: wgpu::TextureView,
    msaa_view: wgpu::TextureView,
    gumball: Option<Gumball>,
    gumball_scale: f32,
    #[allow(dead_code)]
    gumball_instance_buf: wgpu::Buffer,
    gumball_bind_group: wgpu::BindGroup,
    gumball_seg_buf: wgpu::Buffer,
    gumball_seg_bg: wgpu::BindGroup,
    gumball_cone_buf: wgpu::Buffer,
    gumball_cone_bg: wgpu::BindGroup,
    gumball_glyph_buf: wgpu::Buffer,
    gumball_glyph_bg: wgpu::BindGroup,
    drag_origins: HashMap<String, [[f32; 4]; 4]>,
    hidden_guids: HashSet<String>,
    group_locked: HashSet<String>,
    geom_guid_set: HashSet<String>,
    leaf_guid_cache: HashMap<String, Vec<String>>,
    leaf_cache_dirty: bool,
    glyphs_hidden_guids: HashSet<String>,
    line_thickness: f32,
    shading_enabled: bool,
    backface_highlight: bool,
    egui_ctx: egui::Context,
    egui_renderer: egui_wgpu::Renderer,
    egui_state: egui_winit::State,
    text_labels: Vec<text::TextLabel>,
    #[allow(dead_code)]
    font_atlas_view: wgpu::TextureView,
    #[allow(dead_code)]
    font_sampler: wgpu::Sampler,
    glyph_bind_group: wgpu::BindGroup,
    cmd_input: String,
    cmd_log: Vec<String>,
    cmd_counter: u32,
    cmd_history: Vec<String>,
    cmd_history_idx: Option<usize>,
    cmd_history_saved: String,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
                ..Default::default()
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let depth_format = wgpu::TextureFormat::Depth32Float;
        const MSAA_SAMPLES: u32 = 4;
        let pipelines = Pipelines::new(&device, config.format, Some(depth_format), MSAA_SAMPLES);

        let aspect = size.width as f32 / size.height.max(1) as f32;
        let camera = Camera::new(aspect);
        let controller = CameraController::new();
        let w = config.width.max(1);
        let h = config.height.max(1);
        let (depth_tex_raw, depth_view) = create_depth_texture(&device, w, h, MSAA_SAMPLES);
        let msaa_view = create_msaa_texture(&device, w, h, config.format, MSAA_SAMPLES);

        let (session, demo_cones) = demo::active_scene();
        let mut gpu_session = GpuSession::new(&device, &pipelines.geom_bgl);
        gpu_session.rebuild_from(&session, &device, &queue);
        // demo::instancing_demo(&mut gpu_session, &device, &queue);   // uncomment to test instancing
        if !demo_cones.is_empty() {
            gpu_session.cones_cpu.extend(demo_cones);
            gpu_session.cones_dirty = true;
        }
        let mut session = session;
        session.invalidate_bvh_cache();
        let camera_buf = create_camera_buffer(&device);
        let bind_group = build_bind_group(&device, &pipelines.bind_group_layout, &camera_buf, &gpu_session.instance_buffer);

        let gumball_instance = InstanceData::new(0);
        let gumball_instance_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gumball.instance"),
            contents: bytemuck::bytes_of(&gumball_instance),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let gumball_bind_group = build_bind_group(&device, &pipelines.bind_group_layout, &camera_buf, &gumball_instance_buf);
        let gumball_seg_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gumball.segments"),
            size: 512 * std::mem::size_of::<gpu_session::CylinderSegment>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let gumball_seg_bg = gpu_session::make_geom_bind_group(&device, &pipelines.geom_bgl, &gumball_seg_buf);
        let gumball_cone_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gumball.cones"),
            size: 16 * std::mem::size_of::<gpu_session::CylinderSegment>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let gumball_cone_bg = gpu_session::make_geom_bind_group(&device, &pipelines.geom_bgl, &gumball_cone_buf);
        let gumball_glyph_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gumball.glyphs"),
            size: 16 * std::mem::size_of::<gpu_session::GlyphPoint>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let gumball_glyph_bg = gpu_session::make_geom_bind_group(&device, &pipelines.geom_bgl, &gumball_glyph_buf);

        let egui_ctx = egui::Context::default();
        egui_ctx.set_visuals(egui::Visuals::light());
        let egui_renderer = egui_wgpu::Renderer::new(&device, config.format, egui_wgpu::RendererOptions::default());
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &*window,
            None,
            None,
            None,
        );

        let mut geom_guid_set: HashSet<String> = session.lookup.keys().cloned().collect();
        for ns in &session.objects.nurbssurfaces { geom_guid_set.insert(ns.guid().to_string()); }
        for nc in &session.objects.nurbscurves  { geom_guid_set.insert(nc.guid().to_string()); }

        let text_labels = labels_from_session(&session);
        let (font_atlas_view, font_sampler) = text::create_font_atlas(&device, &queue);
        let glyph_bind_group = create_glyph_bind_group(
            &device,
            &pipelines.glyph_bgl,
            &font_atlas_view,
            &font_sampler,
        );

        let mut state = Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            clear_color: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 },
            window,
            mouse_position: (0.0, 0.0),
            pipelines,
            gpu_session,
            session,
            selected_guids: HashSet::new(),
            pending_pick: None,
            camera,
            controller,
            camera_buf,
            bind_group,
            depth_tex_raw,
            depth_view,
            msaa_view,
            gumball: None,
            gumball_scale: 1.0,
            gumball_instance_buf,
            gumball_bind_group,
            gumball_seg_buf,
            gumball_seg_bg,
            gumball_cone_buf,
            gumball_cone_bg,
            gumball_glyph_buf,
            gumball_glyph_bg,
            drag_origins: HashMap::new(),
            hidden_guids: HashSet::new(),
            group_locked: HashSet::new(),
            geom_guid_set,
            leaf_guid_cache: HashMap::new(),
            leaf_cache_dirty: false,
            glyphs_hidden_guids: HashSet::new(),
            line_thickness: 2.0,
            shading_enabled: true,
            backface_highlight: true,
            egui_ctx,
            egui_renderer,
            egui_state,
            text_labels,
            font_atlas_view,
            font_sampler,
            glyph_bind_group,
            cmd_input: String::new(),
            cmd_log: Vec::new(),
            cmd_counter: 0,
            cmd_history: Vec::new(),
            cmd_history_idx: None,
            cmd_history_saved: String::new(),
        };
        state.apply_thickness();
        // Auto-lock atomic element groups (mesh + polylines) for joint movement
        if let Some(root) = state.session.tree.root() {
            auto_lock_leaf_groups(&root, &state.geom_guid_set, &mut state.group_locked);
        }
        // Auto-hide edges and vertex glyphs for FloorModel meshes
        let floor_guids = collect_group_leaf_guids(&state.session, "FloorModel");
        for guid in &floor_guids {
            state.gpu_session.set_flag(guid, InstanceData::FLAG_GLYPHS_HIDDEN, true, &state.queue);
            state.glyphs_hidden_guids.insert(guid.clone());
        }
        // Auto-hide endpoint glyphs for FloorPolylines (too many endpoints)
        let floor_poly_guids = collect_group_leaf_guids(&state.session, "FloorPolylines");
        for guid in &floor_poly_guids {
            state.gpu_session.set_flag(guid, InstanceData::FLAG_GLYPHS_HIDDEN, true, &state.queue);
            state.glyphs_hidden_guids.insert(guid.clone());
        }
        Ok(state)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.camera.aspect = width as f32 / height as f32;
            let (depth_tex_raw, depth_view) = create_depth_texture(&self.device, width, height, 4);
            self.depth_tex_raw = depth_tex_raw;
            self.depth_view = depth_view;
            self.msaa_view = create_msaa_texture(&self.device, width, height, self.config.format, 4);
        }
    }
}

include!("state_update.rs");     // select_by_guid, set_selection, update
include!("state_pick.rs");       // process_pick, selected_centroid
include!("state_cmd.rs");        // apply_thickness, execute_command
include!("state_ui.rs");         // build_ui
include!("state_render.rs");     // render
include!("state_interaction.rs"); // reapply_visibility, commit_transform, handle_*, fit_view


// ============================================================
// APP
// ============================================================
pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(event_loop: &EventLoop<State>) -> Self {
        Self {
            proxy: Some(event_loop.create_proxy()),
            state: None,
        }
    }

    pub fn run() -> anyhow::Result<()> {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
        let event_loop = EventLoop::with_user_event().build()?;
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
        Ok(())
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        const CANVAS_ID: &str = "canvas";

        let window = wgpu::web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();

        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            let cb = Closure::<dyn Fn(web_sys::Event)>::new(|e: web_sys::Event| {
                e.prevent_default();
            });
            canvas
                .add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref())
                .unwrap_throw();
            cb.forget();
        }

        let html_canvas_element = canvas.unchecked_into();
        let window_attributes = Window::default_attributes().with_canvas(Some(html_canvas_element));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {
                assert!(
                    proxy
                        .send_event(State::new(window).await.expect("Unable to create canvas!"))
                        .is_ok()
                );
            });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        event.window.request_redraw();
        event.resize(
            event.window.inner_size().width,
            event.window.inner_size().height,
        );
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        // Route event to egui first; skip 3D handling if egui consumed it
        let window = Arc::clone(&state.window);
        let egui_resp = state.egui_state.on_window_event(&window, &event);
        if egui_resp.consumed {
            match &event {
                WindowEvent::Resized(s) => state.resize(s.width, s.height),
                WindowEvent::RedrawRequested => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(e) => { log::error!("{e}"); event_loop.exit(); }
                    }
                }
                _ => {}
            }
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::CursorMoved { position, .. } => {
                state.handle_mouse_moved(position.x, position.y);
            }
            WindowEvent::ModifiersChanged(mods) => {
                state.controller.process_modifiers(mods.state());
            }
            WindowEvent::MouseInput { state: btn_state, button, .. } => {
                state.handle_mouse_button(button, btn_state.is_pressed());
            }
            WindowEvent::MouseWheel { delta, .. } => {
                state.handle_scroll(&delta);
            }
            _ => {}
        }
    }
}

// ============================================================
// WASM ENTRY POINT
// ============================================================
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}
