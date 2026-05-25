// ============================================================
// IMPORTS
// ============================================================
use std::collections::{HashMap, HashSet};
use std::iter;
use std::rc::Rc;
use std::cell::RefCell;
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
use session_rust::{Session, TreeNode, Xform};

mod demo;
mod text;

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


fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, view)
}


// ============================================================
// EGUI TREE HELPERS
// ============================================================
fn collect_leaf_guids(node: &Rc<RefCell<TreeNode>>, vmap: &HashMap<String, String>) -> Vec<String> {
    let borrowed = node.borrow();
    if vmap.contains_key(&borrowed.name) {
        return vec![borrowed.name.clone()];
    }
    let children = borrowed.children();
    drop(borrowed);
    let mut result = Vec::new();
    for child in &children {
        result.extend(collect_leaf_guids(child, vmap));
    }
    result
}

fn render_tree_node(
    ui: &mut egui::Ui,
    node: &Rc<RefCell<TreeNode>>,
    vmap: &HashMap<String, String>,
    selected: &HashSet<String>,
    hidden: &HashSet<String>,
    new_sel: &mut Option<(Vec<String>, bool)>,
    vis_chg: &mut Vec<(String, bool)>,
) {
    let name = node.borrow().name.clone();
    if vmap.contains_key(&name) {
        let label = vmap.get(&name).cloned().unwrap_or_else(|| name.clone());
        let is_sel = selected.contains(&name);
        let mut vis = !hidden.contains(&name);
        ui.horizontal(|ui| {
            let resp = ui.selectable_label(is_sel, &label);
            if resp.clicked() {
                let shift = ui.ctx().input(|i| i.modifiers.shift);
                *new_sel = Some((vec![name.clone()], shift));
            }
            if ui.checkbox(&mut vis, "").changed() {
                vis_chg.push((name.clone(), !vis));
            }
        });
    } else {
        let children = node.borrow().children();
        let leaf_guids = collect_leaf_guids(node, vmap);
        let group_vis = leaf_guids.iter().all(|g| !hidden.contains(g));
        let id = ui.make_persistent_id(&name);
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.label(&*name);
                let mut gv = group_vis;
                if ui.checkbox(&mut gv, "").changed() {
                    for g in &leaf_guids {
                        vis_chg.push((g.clone(), !gv));
                    }
                }
            })
            .body(|ui| {
                for child in &children {
                    render_tree_node(ui, child, vmap, selected, hidden, new_sel, vis_chg);
                }
            });
    }
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
    egui_ctx: egui::Context,
    egui_renderer: egui_wgpu::Renderer,
    egui_state: egui_winit::State,
    text_labels: Vec<text::TextLabel>,
    #[allow(dead_code)]
    font_atlas_view: wgpu::TextureView,
    #[allow(dead_code)]
    font_sampler: wgpu::Sampler,
    glyph_bind_group: wgpu::BindGroup,
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
        let pipelines = Pipelines::new(&device, config.format, Some(depth_format));

        let aspect = size.width as f32 / size.height.max(1) as f32;
        let camera = Camera::new(aspect);
        let controller = CameraController::new();
        let (depth_tex_raw, depth_view) = create_depth_texture(&device, config.width.max(1), config.height.max(1));

        let session = demo::make_demo_session();
        let mut gpu_session = GpuSession::new(&device, &pipelines.geom_bgl);
        gpu_session.rebuild_from(&session, &device, &queue);
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

        let text_labels = labels_from_session(&session);
        let (font_atlas_view, font_sampler) = text::create_font_atlas(&device, &queue);
        let glyph_bind_group = create_glyph_bind_group(
            &device,
            &pipelines.glyph_bgl,
            &font_atlas_view,
            &font_sampler,
        );

        Ok(Self {
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
            egui_ctx,
            egui_renderer,
            egui_state,
            text_labels,
            font_atlas_view,
            font_sampler,
            glyph_bind_group,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.camera.aspect = width as f32 / height as f32;
            let (depth_tex_raw, depth_view) = create_depth_texture(&self.device, width, height);
            self.depth_tex_raw = depth_tex_raw;
            self.depth_view = depth_view;
        }
    }

    #[allow(dead_code)]
    fn select_by_guid(&mut self, guid: &str) {
        let prev: Vec<String> = self.selected_guids.drain().collect();
        for p in &prev {
            self.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.queue);
        }
        if self.gpu_session.pick.instance_id(guid).is_some() {
            self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
            self.selected_guids.insert(guid.to_string());
            let origin = self.selected_centroid();
            self.gumball = Some(Gumball::new(origin));
        }
    }

    fn set_selection(&mut self, guids: &[&str]) {
        let prev: Vec<String> = self.selected_guids.drain().collect();
        for p in &prev {
            self.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.queue);
        }
        for guid in guids {
            if self.gpu_session.pick.instance_id(guid).is_some() {
                self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                self.selected_guids.insert(guid.to_string());
            }
        }
        if !self.selected_guids.is_empty() {
            let origin = self.selected_centroid();
            self.gumball = Some(Gumball::new(origin));
        } else {
            self.gumball = None;
        }
    }

    pub fn update(&mut self) {
        self.controller.update_camera(&mut self.camera);
        let v = self.camera.view_matrix();
        let norm3 = |x: f32, y: f32, z: f32| -> [f32; 3] {
            let l = (x*x + y*y + z*z).sqrt().max(1e-30);
            [x/l, y/l, z/l]
        };
        let right   = norm3(v[0][0], v[1][0], v[2][0]);
        let up      = norm3(v[0][1], v[1][1], v[2][1]);
        let forward = norm3(v[0][2], v[1][2], v[2][2]);
        let cam_to_ws = |r: f32, u: f32, f: f32| -> [f32; 4] {
            let x = r*right[0] + u*up[0] + f*forward[0];
            let y = r*right[1] + u*up[1] + f*forward[1];
            let z = r*right[2] + u*up[2] + f*forward[2];
            let l = (x*x + y*y + z*z).sqrt().max(1e-30);
            [x/l, y/l, z/l, 0.0]
        };
        let cam = CameraUniform {
            view_proj:    self.camera.view_proj(),
            key_light_ws: cam_to_ws(-0.3, 0.8, 0.6),
            fill_light_ws:cam_to_ws( 0.8,-0.2, 0.5),
            screen_size:  [self.config.width as f32, self.config.height as f32],
            _pad2:        [0.0, 0.0],
        };
        self.queue.write_buffer(&self.camera_buf, 0, bytemuck::bytes_of(&cam));

        if let Some((cx, cy)) = self.pending_pick.take() {
            self.process_pick(cx as f32, cy as f32);
        }

        // Compute gumball scale after pick so a newly created gumball gets the
        // correct size on its first frame.
        if let Some(gb) = &self.gumball {
            const VIEWER_TO_MM: f32 = 1000.0;
            let vp_h = self.config.height as f32;
            self.gumball_scale = match self.camera.proj_mode {
                ProjMode::Perspective => {
                    // Use view-space Z depth of the gumball origin for accuracy
                    // when the orbit target and gumball are not at the same position.
                    let vm = self.camera.view_matrix();
                    let [ox, oy, oz] = gb.origin; // mm; view_matrix includes MM_TO_UNIT
                    let vz = vm[0][2]*ox + vm[1][2]*oy + vm[2][2]*oz + vm[3][2];
                    // vz is in viewer units, negative for objects in front of camera
                    let depth_mm = (-vz).max(0.001) * VIEWER_TO_MM;
                    use session_rust::tolerance::Tolerance;
                    let mm_per_px = 2.0 * depth_mm * (Tolerance::PI / 6.0).tan() / vp_h;
                    gumball::SCREEN_PX * mm_per_px / gumball::ARC_RADIUS
                }
                ProjMode::Ortho => {
                    // ortho_scale is the half-height of the frustum in viewer units
                    let ortho_h_mm = self.camera.ortho_scale * 2.0 * VIEWER_TO_MM;
                    let mm_per_px = ortho_h_mm / vp_h;
                    gumball::SCREEN_PX * mm_per_px / gumball::ARC_RADIUS
                }
            };
        }
    }

    fn process_pick(&mut self, cx: f32, cy: f32) {
        let view = self.camera.view_matrix();
        let proj = self.camera.proj_matrix();
        let viewport = (self.config.width as f32, self.config.height as f32);
        let is_ortho = self.camera.proj_mode == ProjMode::Ortho;
        let ray = screen_to_world_ray(&view, &proj, viewport, (cx, cy), is_ortho);

        let gumball_hit = self.gumball.as_ref()
            .and_then(|gb| gb.hit_test(ray, self.gumball_scale));
        if let Some(handle) = gumball_hit {
            self.drag_origins.clear();
            for guid in &self.selected_guids {
                if let Some(iid) = self.gpu_session.pick.instance_id(guid) {
                    let model = self.gpu_session.instances_cpu[iid as usize].model;
                    self.drag_origins.insert(guid.clone(), model);
                }
            }
            let origin = self.gumball.as_ref().unwrap().origin;
            let ds = gumball::begin_drag(handle, ray, origin, self.gumball_scale);
            self.gumball.as_mut().unwrap().drag = Some(ds);
            return;
        }

        log::info!("PICK ray origin=({:.1},{:.1},{:.1}) dir=({:.4},{:.4},{:.4}) vp={}x{}",
            ray.origin[0], ray.origin[1], ray.origin[2],
            ray.direction[0], ray.direction[1], ray.direction[2],
            self.config.width, self.config.height);
        let pick_radius = self.camera.pick_radius_mm(self.config.height as f32, 8.0)
            .max(crate::gpu_adapters::SPHERE_RADIUS);
        let hits     = pick::pick_by_ray(&mut self.session, ray, pick_radius);
        log::info!("PICK hits={} guids={:?}", hits.len(),
            hits.iter().map(|h| h.guid()).collect::<Vec<_>>());
        let new_guid = hits.iter()
            .find(|h| !self.hidden_guids.contains(h.guid()))
            .map(|h| h.guid().to_string());
        let shift    = self.controller.select_add();

        if shift {
            if let Some(guid) = new_guid.clone() {
                if self.selected_guids.contains(&guid) {
                    self.gpu_session.set_flag(&guid, InstanceData::FLAG_SELECTED, false, &self.queue);
                    self.selected_guids.remove(&guid);
                } else {
                    self.gpu_session.set_flag(&guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                    self.selected_guids.insert(guid.clone());
                }
            }
        } else {
            let prev: Vec<String> = self.selected_guids.drain().collect();
            for p in &prev {
                self.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.queue);
            }
            if let Some(guid) = new_guid.clone() {
                let was_only = prev.len() == 1 && prev[0] == guid;
                if !was_only {
                    self.gpu_session.set_flag(&guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                    self.selected_guids.insert(guid.clone());
                }
            }
        }

        if !self.selected_guids.is_empty() {
            let origin = self.selected_centroid();
            match &mut self.gumball {
                Some(gb) => gb.set_origin(origin),
                None     => self.gumball = Some(Gumball::new(origin)),
            }
        } else {
            self.gumball = None;
        }

    }


    /// Center of the AABB union over all selected objects.
    fn selected_centroid(&self) -> [f32; 3] {
        let mut mn = [f32::MAX;  3];
        let mut mx = [f32::MIN; 3];
        let mut found = false;
        for guid in &self.selected_guids {
            if let Some(idx) = self.session.cached_guids.iter().position(|g| g == guid) {
                if idx < self.session.cached_boxes.len() {
                    for corner in &self.session.cached_boxes[idx].corners() {
                        for i in 0..3 {
                            let v = corner[i] as f32;
                            if v < mn[i] { mn[i] = v; }
                            if v > mx[i] { mx[i] = v; }
                        }
                    }
                    found = true;
                }
            }
        }
        if found { [(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5] } else { [0.0; 3] }
    }

    fn build_ui(&mut self) -> egui::FullOutput {
        let egui_ctx = self.egui_ctx.clone();
        let window = Arc::clone(&self.window);
        let raw_input = self.egui_state.take_egui_input(&window);

        let tree_root = self.session.tree.root();
        let vmap: HashMap<String, String> = self.session.graph
            .get_vertices()
            .into_iter()
            .map(|v| {
                let label = if v.attribute.is_empty() { v.name.clone() } else { v.attribute.clone() };
                (v.name, label)
            })
            .collect();
        let edges = self.session.graph.get_edges();
        let selected = self.selected_guids.clone();
        let hidden = self.hidden_guids.clone();

        let mut new_sel: Option<(Vec<String>, bool)> = None;
        let mut vis_chg: Vec<(String, bool)> = Vec::new();

        let full_output = egui_ctx.run_ui(raw_input, |ui| {
            egui::Panel::right("panel")
                .default_size(240.0)
                .show_inside(ui, |ui| {
                    egui::CollapsingHeader::new("Tree")
                        .default_open(true)
                        .show(ui, |ui| {
                            if let Some(root) = &tree_root {
                                for child in &root.borrow().children() {
                                    render_tree_node(ui, child, &vmap, &selected, &hidden, &mut new_sel, &mut vis_chg);
                                }
                            }
                        });
                    egui::CollapsingHeader::new("Graph")
                        .default_open(false)
                        .show(ui, |ui| {
                            for (v0, v1) in &edges {
                                let l0 = vmap.get(v0).map(|s| s.as_str()).unwrap_or(v0.as_str());
                                let l1 = vmap.get(v1).map(|s| s.as_str()).unwrap_or(v1.as_str());
                                let both_sel = selected.contains(v0) && selected.contains(v1);
                                let resp = ui.selectable_label(both_sel, format!("{l0} — {l1}"));
                                if resp.clicked() {
                                    let shift = ui.ctx().input(|i| i.modifiers.shift);
                                    new_sel = Some((vec![v0.clone(), v1.clone()], shift));
                                }
                            }
                        });
                });
        });

        if let Some((guids, shift)) = new_sel {
            if shift {
                for guid in &guids {
                    if self.selected_guids.contains(guid) {
                        self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, false, &self.queue);
                        self.selected_guids.remove(guid);
                    } else if self.gpu_session.pick.instance_id(guid).is_some() {
                        self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                        self.selected_guids.insert(guid.clone());
                    }
                }
            } else {
                let refs: Vec<&str> = guids.iter().map(|s| s.as_str()).collect();
                self.set_selection(&refs);
            }
            if !self.selected_guids.is_empty() {
                let origin = self.selected_centroid();
                match &mut self.gumball {
                    Some(gb) => gb.set_origin(origin),
                    None => self.gumball = Some(Gumball::new(origin)),
                }
            } else {
                self.gumball = None;
            }
        }

        for (guid, should_hide) in vis_chg {
            self.gpu_session.set_flag(&guid, InstanceData::FLAG_HIDDEN, should_hide, &self.queue);
            if should_hide { self.hidden_guids.insert(guid); } else { self.hidden_guids.remove(&guid); }
        }

        full_output
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                self.surface.configure(&self.device, &self.config);
                texture
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost GPU device");
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.gpu_session.flush_geometry(&self.device, &self.queue, &self.pipelines.geom_bgl);

        // Build text/glyph vertex buffers before opening the geometry pass
        // (buffers must outlive the render pass that borrows them).
        let visible_labels: Vec<&text::TextLabel> = self.text_labels.iter()
            .filter(|l| !self.hidden_guids.contains(&l.guid))
            .collect();
        let quad_verts = text::build_label_quads(&visible_labels);
        let quad_buf = if !quad_verts.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text.quads"),
                contents: bytemuck::cast_slice(&quad_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };
        let glyph_verts = text::build_glyph_quads(&visible_labels);
        let glyph_buf = if !glyph_verts.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text.glyphs"),
                contents: bytemuck::cast_slice(&glyph_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        // Geometry pass → swapchain
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Geometry Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_pipeline(&self.pipelines.grid);
            render_pass.draw(0..298, 0..1);

            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_pipeline(&self.pipelines.mesh);
            self.gpu_session.draw_meshes(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.line);
            self.gpu_session.draw_lines(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.point);
            self.gpu_session.draw_points(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.cylinder);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_bind_group(1, &self.gpu_session.segment_bg, &[]);
            self.gpu_session.draw_cylinders(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.sphere);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_bind_group(1, &self.gpu_session.glyph_sphere_bg, &[]);
            self.gpu_session.draw_spheres(&mut render_pass);

            // Text background quads — depth-tested, drawn after opaque geometry.
            if let Some(buf) = &quad_buf {
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_pipeline(&self.pipelines.text);
                render_pass.set_vertex_buffer(0, buf.slice(..));
                render_pass.draw(0..quad_verts.len() as u32, 0..1);
            }

            // Glyph characters — depth-tested, font atlas in group 1.
            if let Some(buf) = &glyph_buf {
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_bind_group(1, &self.glyph_bind_group, &[]);
                render_pass.set_pipeline(&self.pipelines.glyph);
                render_pass.set_vertex_buffer(0, buf.slice(..));
                render_pass.draw(0..glyph_verts.len() as u32, 0..1);
            }
        }

        // Gumball overlay — cylinders (shafts+arcs), cones (arrowheads), spheres (handles).
        if let Some(gb) = &self.gumball {
            let lines   = gumball::build_lines(gb.origin, self.gumball_scale, gb.hovered);
            let cones   = gumball::build_cones(gb.origin, self.gumball_scale, gb.hovered);
            let spheres = gumball::build_spheres(gb.origin, self.gumball_scale, gb.hovered);
            let segs: Vec<gpu_session::CylinderSegment> = lines.iter().map(|l| {
                let c = l.color;
                gpu_session::CylinderSegment {
                    p0: l.a, radius: l.radius, p1: l.b, instance_id: 0,
                    color: [c[0] as f32/255.0, c[1] as f32/255.0, c[2] as f32/255.0, c[3] as f32/255.0],
                }
            }).collect();
            let cone_segs: Vec<gpu_session::CylinderSegment> = cones.iter().map(|cn| {
                let c = cn.color;
                gpu_session::CylinderSegment {
                    p0: cn.base, radius: cn.radius, p1: cn.tip, instance_id: 0,
                    color: [c[0] as f32/255.0, c[1] as f32/255.0, c[2] as f32/255.0, c[3] as f32/255.0],
                }
            }).collect();
            let glyph_pts: Vec<gpu_session::GlyphPoint> = spheres.iter().map(|s| {
                let c = s.color;
                gpu_session::GlyphPoint {
                    center: s.center, radius: s.radius,
                    color: [c[0] as f32/255.0, c[1] as f32/255.0, c[2] as f32/255.0, c[3] as f32/255.0],
                    instance_id: 0, _pad: [0; 3],
                }
            }).collect();
            // Upload segment buffer
            let seg_bytes = bytemuck::cast_slice::<_, u8>(&segs);
            if seg_bytes.len() as u64 > self.gumball_seg_buf.size() {
                self.gumball_seg_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gumball.segments"),
                    size: seg_bytes.len() as u64 * 2,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.gumball_seg_bg = gpu_session::make_geom_bind_group(
                    &self.device, &self.pipelines.geom_bgl, &self.gumball_seg_buf,
                );
            }
            self.queue.write_buffer(&self.gumball_seg_buf, 0, seg_bytes);
            // Upload cone buffer
            let cone_bytes = bytemuck::cast_slice::<_, u8>(&cone_segs);
            if cone_bytes.len() as u64 > self.gumball_cone_buf.size() {
                self.gumball_cone_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gumball.cones"),
                    size: cone_bytes.len() as u64 * 2,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.gumball_cone_bg = gpu_session::make_geom_bind_group(
                    &self.device, &self.pipelines.geom_bgl, &self.gumball_cone_buf,
                );
            }
            self.queue.write_buffer(&self.gumball_cone_buf, 0, cone_bytes);
            // Upload glyph buffer
            let glyph_bytes = bytemuck::cast_slice::<_, u8>(&glyph_pts);
            if glyph_bytes.len() as u64 > self.gumball_glyph_buf.size() {
                self.gumball_glyph_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gumball.glyphs"),
                    size: glyph_bytes.len() as u64 * 2,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.gumball_glyph_bg = gpu_session::make_geom_bind_group(
                    &self.device, &self.pipelines.geom_bgl, &self.gumball_glyph_buf,
                );
            }
            self.queue.write_buffer(&self.gumball_glyph_buf, 0, glyph_bytes);
            {
                let mut gpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Gumball Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
                gpass.set_bind_group(0, &self.gumball_bind_group, &[]);
                // Draw cylinder shafts + arcs
                if !segs.is_empty() {
                    gpass.set_pipeline(&self.pipelines.cylinder);
                    gpass.set_bind_group(1, &self.gumball_seg_bg, &[]);
                    gpass.set_vertex_buffer(0, self.gpu_session.cylinder_vbo.slice(..));
                    gpass.set_index_buffer(self.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..segs.len() as u32);
                }
                // Draw cone arrowheads
                if !cone_segs.is_empty() {
                    gpass.set_pipeline(&self.pipelines.cone);
                    gpass.set_bind_group(1, &self.gumball_cone_bg, &[]);
                    gpass.set_vertex_buffer(0, self.gpu_session.cylinder_vbo.slice(..));
                    gpass.set_index_buffer(self.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..cone_segs.len() as u32);
                }
                // Draw sphere handles
                if !glyph_pts.is_empty() {
                    gpass.set_pipeline(&self.pipelines.sphere);
                    gpass.set_bind_group(1, &self.gumball_glyph_bg, &[]);
                    gpass.set_vertex_buffer(0, self.gpu_session.sphere_vbo.slice(..));
                    gpass.set_index_buffer(self.gpu_session.sphere_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_SPHERE_INDICES, 0, 0..glyph_pts.len() as u32);
                }
            }
        }

        // egui pass
        let full_out = self.build_ui();
        self.egui_state.handle_platform_output(&self.window, full_out.platform_output);
        let tris = self.egui_ctx.tessellate(full_out.shapes, full_out.pixels_per_point);
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: full_out.pixels_per_point,
        };
        for (id, delta) in &full_out.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, delta);
        }
        let extra_cmds = self.egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &tris, &screen);
        {
            let epass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            self.egui_renderer.render(&mut epass.forget_lifetime(), &tris, &screen);
        }
        for id in &full_out.textures_delta.free { self.egui_renderer.free_texture(id); }

        self.queue.submit(extra_cmds.into_iter().chain(iter::once(encoder.finish())));
        output.present();

        Ok(())
    }

    fn commit_object_transform(&mut self, guid: &str, model: [[f32; 4]; 4]) {
        let flat = [
            model[0][0], model[0][1], model[0][2], model[0][3],
            model[1][0], model[1][1], model[1][2], model[1][3],
            model[2][0], model[2][1], model[2][2], model[2][3],
            model[3][0], model[3][1], model[3][2], model[3][3],
        ];
        let xf = Xform::from_matrix(flat);
        if let Some(geom) = self.session.lookup.get_mut(guid) {
            match geom {
                Geometry::Mesh(m)        => { m.transform(Some(&xf)); }
                Geometry::Point(p)       => { p.xform = xf.clone(); p.transform(); }
                Geometry::Line(l)        => { l.xform = xf.clone(); l.transform(); }
                Geometry::Polyline(pl)   => { pl.xform = xf.clone(); pl.transform(); }
                Geometry::Plane(pl)      => { pl.xform = xf.clone(); pl.transform(); }
                Geometry::PointCloud(pc) => { pc.xform = xf.clone(); pc.transform(); }
                Geometry::OBB(o)         => { o.xform = xf.clone(); o.transform(); }
                _ => {}
            }
        }
        self.session.cached_boxes.clear();
        self.session.cached_guids.clear();
        self.session.invalidate_bvh_cache();
        let was_selected = self.gpu_session.pick.instance_id(guid)
            .and_then(|iid| self.gpu_session.instances_cpu.get(iid as usize))
            .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
        self.gpu_session.remove(guid);
        if let Some(geom) = self.session.lookup.remove(guid) {
            self.gpu_session.add_geometry(guid, &geom, &self.device, &self.queue);
            self.session.lookup.insert(guid.to_string(), geom);
        }
        if was_selected {
            self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if !pressed {
                let was_dragging = self.gumball.as_ref().map_or(false, |gb| gb.drag.is_some());
                if was_dragging {
                    if let Some(gb) = &mut self.gumball {
                        gb.drag = None;
                    }
                    let to_commit: Vec<(String, [[f32; 4]; 4])> = self.drag_origins.keys()
                        .filter_map(|guid| {
                            self.gpu_session.pick.instance_id(guid).map(|iid| {
                                (guid.clone(), self.gpu_session.instances_cpu[iid as usize].model)
                            })
                        })
                        .collect();
                    for (guid, model) in to_commit {
                        self.commit_object_transform(&guid, model);
                    }
                    self.drag_origins.clear();
                    return;
                }
            }
            if pressed {
                self.pending_pick = Some(self.mouse_position);
            }
        }
        self.controller.process_mouse_button(button, pressed);
    }

    pub fn handle_mouse_moved(&mut self, x: f64, y: f64) {
        let (px, py) = self.mouse_position;
        self.mouse_position = (x, y);

        let drag_info = self.gumball.as_ref().and_then(|gb| {
            gb.drag.as_ref().map(|ds| (ds.clone(), gb.origin))
        });
        if let Some((ds, origin)) = drag_info {
            let view = self.camera.view_matrix();
            let proj = self.camera.proj_matrix();
            let vp = (self.config.width as f32, self.config.height as f32);
            let is_ortho = self.camera.proj_mode == ProjMode::Ortho;
            let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
            let scale = self.gumball_scale;
            if let Some(delta) = gumball::update_drag(&ds, ray, origin, scale) {
                for (guid, orig) in &self.drag_origins {
                    let new_model = mat4_mul_cm(&delta, orig);
                    self.gpu_session.update_transform(guid, new_model, &self.queue);
                }
                if matches!(ds.handle, HandleKind::TranslateX | HandleKind::TranslateY | HandleKind::TranslateZ) {
                    if let Some(gb) = &mut self.gumball {
                        gb.origin = [
                            ds.drag_start_origin[0] + delta[3][0],
                            ds.drag_start_origin[1] + delta[3][1],
                            ds.drag_start_origin[2] + delta[3][2],
                        ];
                    }
                }
            }
            return;
        }

        if self.gumball.is_some() {
            let view = self.camera.view_matrix();
            let proj = self.camera.proj_matrix();
            let vp = (self.config.width as f32, self.config.height as f32);
            let is_ortho = self.camera.proj_mode == ProjMode::Ortho;
            let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
            let scale = self.gumball_scale;
            if let Some(gb) = &mut self.gumball {
                gb.hovered = gb.hit_test(ray, scale);
            }
        }

        self.controller.process_mouse_move((x - px) as f32, (y - py) as f32);
    }

    pub fn handle_scroll(&mut self, delta: &MouseScrollDelta) {
        self.controller.process_scroll(delta);
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        } else if code == KeyCode::KeyF && is_pressed {
            self.fit_view();
        } else {
            self.controller.process_key(code, is_pressed);
        }
    }

    fn fit_view(&mut self) {
        if self.selected_guids.is_empty() {
            self.camera.reset();
            return;
        }
        let mut mn = [f32::MAX; 3];
        let mut mx = [f32::MIN; 3];
        let mut found = false;
        for guid in &self.selected_guids {
            if let Some(idx) = self.session.cached_guids.iter().position(|g| g == guid) {
                if idx < self.session.cached_boxes.len() {
                    for corner in &self.session.cached_boxes[idx].corners() {
                        for i in 0..3 {
                            let v = corner[i] as f32;
                            if v < mn[i] { mn[i] = v; }
                            if v > mx[i] { mx[i] = v; }
                        }
                    }
                    found = true;
                }
            }
        }
        if !found { self.camera.reset(); return; }
        let center = [(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5];
        let half_diag = (
            (mx[0]-mn[0]).powi(2) +
            (mx[1]-mn[1]).powi(2) +
            (mx[2]-mn[2]).powi(2)
        ).sqrt() * 0.5;
        self.camera.fit_to_box(center, half_diag.max(50.0));
    }
}

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
