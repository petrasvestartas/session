//  session_viewer layer stack
//
//  ┌──────────────────────────────────────────────────┐
//  │  ShellState  (shell)   egui + CLI                │
//  ├──────────────────────────────────────────────────┤
//  │  UndoState   (hist)    undo/redo action stacks   │
//  ├──────────────────────────────────────────────────┤
//  │  GumballState (gb)     select gizmo + GPU bufs   │
//  ├──────────────────────────────────────────────────┤
//  │  SceneState  (scene)   geometry + camera + pick  │
//  ├──────────────────────────────────────────────────┤
//  │  GpuCtx      (gpu)     device/queue/tex/pipelines│
//  └──────────────────────────────────────────────────┘
//
//  Render order each frame:
//    1. scene.gpu_session.flush → geometry pass (MSAA)
//    2. gb.gumball.is_some()   → gumball pass (MSAA)
//    3. shell.build_ui()       → egui pass (swapchain)
//
//  To isolate a broken layer: comment out its render step — the others survive.

// ============================================================
// IMPORTS
// ============================================================
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::Window,
};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys};

mod camera;
mod engine;
mod gumball;
mod pick;
mod cad_plane;
mod coord_parser;
mod snap;
mod tool_state;
use engine::gpu as gpu_session;
use engine::gpu::adapters as gpu_adapters;
use engine::gpu::arena as gpu_arena;
use engine::gpu::instance_groups as gpu_instance_groups;
use camera::{Camera, CameraController};
use engine::pipelines::{
    self, build_bind_group, create_camera_buffer, create_glyph_bind_group, Pipelines,
};

use wgpu::util::DeviceExt;
use gpu_session::{GpuSession, InstanceData};
use session_rust::session::Geometry;
use session_rust::Session;

mod demo;
mod text;
mod tree_ui;
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;   // headless native pick/selection self-test (no browser)
use tree_ui::{auto_lock_leaf_groups, collect_group_leaf_guids};

mod gpu_ctx;
mod scene_state;
mod gumball_state;
mod edit_state;
mod edit_points;
mod undo_state;
mod shell_state;
use gpu_ctx::{GpuCtx, create_arctic_targets, create_depth_texture, create_ground_bind_group, create_msaa_texture};
use scene_state::SceneState;
use gumball_state::GumballState;
use edit_state::EditState;
use undo_state::UndoState;
use shell_state::ShellState;
use tool_state::ToolState;

pub(crate) fn labels_from_session(session: &Session) -> Vec<text::TextLabel> {
    session.lookup.iter()
        .filter_map(|(guid, geom)| {
            if let Geometry::Point(p) = geom {
                if !p.name.is_empty() && p.name != "my_point" {
                    return Some(text::TextLabel {
                        guid: guid.clone(),
                        position: [p[0] as f32, p[1] as f32, p[2] as f32],
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
pub(crate) fn mat4_mul_cm(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
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



// ============================================================
// STATE
// ============================================================
pub struct State {
    window: Arc<Window>,
    pub gpu: GpuCtx,
    pub scene: SceneState,
    pub gb: GumballState,
    pub edit: EditState,
    pub hist: UndoState,
    pub shell: ShellState,
    pub tool: ToolState,
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
        // SSAO samples the MSAA depth buffer as a texture — the GL fallback backend
        // cannot bind multisampled depth, so Arctic degrades there (white shading only).
        let backend = format!("{:?}", adapter.get_info().backend);
        let ssao_supported = adapter.get_info().backend != wgpu::Backend::Gl;
        log::info!("wgpu backend: {backend}, ssao_supported: {ssao_supported}");
        let pipelines = Pipelines::new(&device, config.format, Some(depth_format), MSAA_SAMPLES, ssao_supported);

        let aspect = size.width as f32 / size.height.max(1) as f32;
        let camera = Camera::new(aspect);
        let controller = CameraController::new();
        let w = config.width.max(1);
        let h = config.height.max(1);
        let (depth_tex_raw, depth_view) = create_depth_texture(&device, w, h, MSAA_SAMPLES, ssao_supported);
        let msaa_view = create_msaa_texture(&device, w, h, config.format, MSAA_SAMPLES);
        let arctic_buf = pipelines::create_arctic_buffer(&device);
        let ground_bg = create_ground_bind_group(&device, &pipelines.ground_bgl, &arctic_buf);
        let arctic_targets = create_arctic_targets(
            &device, w, h, config.format, &pipelines, &arctic_buf, &depth_view,
        );

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

        let edit = EditState::new(&device, &pipelines.geom_bgl);

        let egui_ctx = egui::Context::default();
        {
            let mut vis = egui::Visuals::light();
            vis.selection.bg_fill = egui::Color32::BLACK;
            vis.selection.stroke  = egui::Stroke::new(1.0, egui::Color32::WHITE);
            vis.override_text_color = Some(egui::Color32::BLACK);
            vis.indent_has_left_vline = false;
            egui_ctx.set_visuals(vis);
        }
        egui_extras::install_image_loaders(&egui_ctx);
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
        for ts in &session.objects.nurbssurfacetrimmeds { geom_guid_set.insert(ts.guid().to_string()); }
        for nc in &session.objects.nurbscurves  { geom_guid_set.insert(nc.guid().to_string()); }

        let text_labels = labels_from_session(&session);
        let (font_atlas_view, font_sampler) = text::create_font_atlas(&device, &queue);
        let glyph_bind_group = create_glyph_bind_group(
            &device,
            &pipelines.glyph_bgl,
            &font_atlas_view,
            &font_sampler,
        );

        // ── GpuCtx ────────────────────────────────────────────
        let gpu = GpuCtx {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            clear_color: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 },
            pipelines,
            camera_buf,
            bind_group,
            depth_tex_raw,
            depth_view,
            msaa_view,
            ssao_supported,
            backend,
            arctic_buf,
            ground_bg,
            arctic_targets,
            font_atlas_view,
            font_sampler,
            glyph_bind_group,
        };

        // ── SceneState ────────────────────────────────────────
        let scene = SceneState {
            gpu_session,
            session,
            selected_guids: HashSet::new(),
            hidden_guids: HashSet::new(),
            glyphs_hidden_guids: HashSet::new(),
            group_locked: HashSet::new(),
            transform_locked: HashSet::new(),
            geom_guid_set,
            leaf_guid_cache: HashMap::new(),
            leaf_cache_dirty: false,
            face_color_overrides: HashMap::new(),
            point_color_overrides: HashMap::new(),
            camera,
            controller,
            key_mods: winit::keyboard::ModifiersState::empty(),
            ctrl_down: false,
            shift_down: false,
            line_thickness: 2.0,
            shading_enabled: true,
            // Off by default: backfaces are two-sided-shaded (the fs flips the normal), so a
            // trimmed cut that exposes the inner wall (cylinder/torus bite) reads as a clean
            // surface. Pressing `E` toggles the red backface-highlight diagnostic back on.
            backface_highlight: false,
            show_tess: false,
            pending_pick: None,
            reveal_in_tree: false,
            box_select_start: None,
            box_select: None,
            lmb_down: false,
            mouse_position: (0.0, 0.0),
            text_labels,
            frustum_cull: true,
            draw_stats: gpu_session::DrawStats::default(),
            arctic: false,
            arctic_gradient: true,
            ao_mode: 0, // classic SSAO (learnopengl) — user-requested default
            ssao_intensity: 0.9,
            ssao_radius_pct: 0.5,
            last_arctic_bounds: None,
            outline: true,
            outline_px: 2.0,
        };

        // ── GumballState ──────────────────────────────────────
        let gb = GumballState {
            gumball: None,
            gumball_scale: 1.0,
            gumball_press: None,
            gumball_input: None,
            gumball_dragged: false,
            drag_origins: HashMap::new(),
            drag_geom_snapshots: HashMap::new(),
            drag_nurbs_snapshots: HashMap::new(),
            gumball_instance_buf,
            gumball_bind_group,
            gumball_seg_buf,
            gumball_seg_bg,
            gumball_cone_buf,
            gumball_cone_bg,
            gumball_glyph_buf,
            gumball_glyph_bg,
        };

        // ── ShellState ────────────────────────────────────────
        let shell = ShellState {
            egui_ctx,
            egui_renderer,
            egui_state,
            cmd_input: String::new(),
            cmd_log: Vec::new(),
            cmd_counter: 0,
            cmd_history: Vec::new(),
            cmd_history_idx: None,
            cmd_history_saved: String::new(),
            tree_search: String::new(),
        };

        let mut state = Self { window, gpu, scene, gb, edit, hist: UndoState::new(), shell, tool: ToolState::new() };
        state.apply_thickness();
        // Auto-lock atomic element groups (mesh + polylines) for joint movement —
        // only under FloorModel. Other scenes (e.g. CDT) stay unlocked so every
        // object selects individually.
        if let Some(root) = state.scene.session.tree.root() {
            let children = root.borrow().children();
            for child in &children {
                if child.borrow().name == "FloorModel" {
                    auto_lock_leaf_groups(child, &state.scene.geom_guid_set, &mut state.scene.group_locked);
                }
            }
        }
        // Auto-hide edges and vertex glyphs for FloorModel meshes
        let floor_guids = collect_group_leaf_guids(&state.scene.session, "FloorModel");
        for guid in &floor_guids {
            state.scene.gpu_session.set_flag(guid, InstanceData::FLAG_GLYPHS_HIDDEN, true, &state.gpu.queue);
            state.scene.glyphs_hidden_guids.insert(guid.clone());
        }
        // Auto-hide endpoint glyphs for FloorPolylines (too many endpoints)
        let floor_poly_guids = collect_group_leaf_guids(&state.scene.session, "FloorPolylines");
        for guid in &floor_poly_guids {
            state.scene.gpu_session.set_flag(guid, InstanceData::FLAG_GLYPHS_HIDDEN, true, &state.gpu.queue);
            state.scene.glyphs_hidden_guids.insert(guid.clone());
        }
        Ok(state)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu.resize(width, height);
            self.scene.camera.aspect = width as f32 / height as f32;
        }
    }
}

mod state_update;                // select_by_guid, set_selection, update
mod state_pick;                  // process_pick, selected_centroid
mod state_cmd;                   // apply_thickness, execute_command
mod state_ui;                    // build_ui
mod state_render;                // render
mod state_interaction;           // reapply_visibility, commit_transform, handle_*, fit_view
mod state_undo;                  // undo, redo
mod state_edit;                  // F10 control-point editing: overlay, sub-pick, commit
mod state_tool;                  // interactive draw tools: click/typed input, osnap, preview


// ============================================================
// APP  (web event loop — wasm only; native uses the headless harness instead)
// ============================================================
#[cfg(target_arch = "wasm32")]
pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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
        // NOTE: the adopted canvas backing store follows its CSS size (devicePixelRatio
        // is NOT applied; winit 0.30 ignores request_inner_size for adopted canvases).
        // On displays with OS scaling > 100% the viewer renders at CSS resolution and
        // is upscaled. Fixing this requires scaling egui pixels_per_point and cursor
        // coordinates together — tracked as a follow-up; harmless at 100% scaling.
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

        // Undo/redo are driven solely by the toolbar arrow buttons (state_ui.rs);
        // no keyboard shortcuts (Ctrl+Z/Y were unreliable in the browser).

        // Track the left button from raw events — even when egui consumes the
        // release (e.g. over a panel) — so a stale gumball press can never start
        // a no-button drag.
        if let WindowEvent::MouseInput { state: btn_state, button: MouseButton::Left, .. } = &event {
            state.scene.lmb_down = btn_state.is_pressed();
        }

        // Route event to egui first; skip 3D handling if egui consumed it
        let window = Arc::clone(&state.window);
        let egui_resp = state.shell.egui_state.on_window_event(&window, &event);
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
                state.scene.controller.process_modifiers(mods.state());
                state.scene.key_mods = mods.state();
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
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}
