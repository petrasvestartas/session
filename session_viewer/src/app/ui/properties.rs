use super::{Control, Output, record};
use crate::State;
use crate::app::gizmo::GizmoAlign;
use session_rust::Geometry;
use std::cell::RefCell;

/// Snapshot of selection and display properties shown in the Properties panel.
#[derive(Default)]
pub(crate) struct PropertiesModel {
    pub(crate) open: bool,
    rect: Option<egui::Rect>,
    selected_count: usize,
    name: String,
    guid: String,
    kind: &'static str,
    doc_layer: String,
    counts: String,
    dimensions: Option<[f64; 3]>,
    has_faces: bool,
    face_color: Option<[u8; 3]>,
    edge_color: Option<[u8; 3]>,
    alpha: Option<f32>,
    gizmo_align: GizmoAlign,
    wireframe: bool,
    arctic: bool,
    outlines: bool,
    lit: bool,
    backface: bool,
    cull_backfaces: bool,
    show_edges: bool,
    show_lines: bool,
    show_points: bool,
    show_grid: bool,
    opacity: f32,
    pending_face_color: Option<Option<[u8; 3]>>,
    pending_edge_color: Option<Option<[u8; 3]>>,
    pending_alpha: Option<Option<f32>>,
    pending_opacity: Option<f32>,
}

thread_local! { pub(crate) static STATE: RefCell<PropertiesModel> = RefCell::default(); }

pub(super) struct Hooks;

impl super::Panel for Hooks {
    fn fill(&self, state: &mut State) {
        STATE.with_borrow_mut(|m| {
            if !m.open {
                m.rect = None;
                return;
            }
            let rows = state.selected_rows();
            m.selected_count = rows.len();
            m.gizmo_align = state.features.gizmo_align;
            let v = &state.gpu.view;
            m.wireframe = v.wireframe;
            m.arctic = v.ssao;
            m.outlines = v.show_outlines;
            m.lit = v.lit;
            m.backface = v.backface;
            m.cull_backfaces = v.cull_backfaces;
            m.show_edges = v.show_mesh_edges;
            m.show_lines = v.show_lines;
            m.show_points = v.show_points;
            m.show_grid = v.show_grid;
            m.opacity = v.opacity;

            let Some(&row) = rows.first() else {
                m.name.clear();
                m.guid.clear();
                m.kind = "";
                m.doc_layer.clear();
                m.counts.clear();
                m.dimensions = None;
                m.has_faces = false;
                m.face_color = None;
                m.edge_color = None;
                m.alpha = None;
                return;
            };

            m.name = state.scene.object_name(row).to_string();
            if let Some(id) = state.scene.identity_of(row) {
                m.guid = id.1.to_string();
                let doc_name = state
                    .scene
                    .docs
                    .get(id.0)
                    .map(|d| d.name.as_str())
                    .unwrap_or("scene");
                let layer_name = state
                    .scene
                    .parent_of(id.0, &id.1)
                    .map(|p| p.borrow().name.clone())
                    .unwrap_or_else(|| "default".into());
                m.doc_layer = format!("{doc_name} / {layer_name}");
                m.face_color = state.scene.colors.get(&id).copied();
                m.edge_color = state.scene.edge_colors.get(&id).copied();
                m.alpha = state.scene.alphas.get(&id).copied();
            } else {
                m.guid.clear();
                m.doc_layer.clear();
                m.face_color = None;
                m.edge_color = None;
                m.alpha = None;
            }

            m.has_faces = state
                .gpu
                .objects
                .row(row)
                .is_some_and(|r| r.flags & crate::engine::gpu::Instance::FLAG_HAS_FACES != 0);

            let (kind, counts) = match state.scene.geometry(row) {
                Some(Geometry::Mesh(mesh)) => (
                    "Mesh",
                    format!(
                        "{} verts · {} faces",
                        mesh.number_of_vertices(),
                        mesh.number_of_faces()
                    ),
                ),
                Some(Geometry::BRep(brep)) => (
                    "BRep",
                    format!("{} faces · {} edges", brep.face_count(), brep.edge_count()),
                ),
                Some(Geometry::NurbsSurface(s)) => (
                    "NurbsSurface",
                    format!("{}×{} CVs", s.cv_count(0), s.cv_count(1)),
                ),
                Some(Geometry::NurbsCurve(c)) => ("NurbsCurve", format!("{} CVs", c.cv_count())),
                Some(Geometry::Polyline(p)) => ("Polyline", format!("{} pts", p.point_count())),
                Some(Geometry::Line(_)) => ("Line", "2 pts".into()),
                Some(Geometry::Point(_)) => ("Point", "1 pt".into()),
                Some(Geometry::PointCloud(pc)) => {
                    ("PointCloud", format!("{} pts", pc.point_count()))
                }
                Some(Geometry::Plane(_)) => ("Plane", String::new()),
                Some(Geometry::OBB(_)) => ("Box", String::new()),
                Some(Geometry::Element(_)) => ("Element", String::new()),
                None => ("Display", String::new()),
            };
            m.kind = kind;
            m.counts = counts;

            let mut box_ = session_rust::AABB::empty();
            for &r in &rows {
                if let Some(b) = state.gpu.objects.row_bounds(r) {
                    box_.union_with(&b);
                }
            }
            m.dimensions = box_
                .is_valid()
                .then_some([box_.hx * 2.0, box_.hy * 2.0, box_.hz * 2.0]);
        });
    }

    fn show(&self, root: &mut egui::Ui, controls: &mut Option<Vec<Control>>, out: &mut Output) {
        STATE.with_borrow_mut(|m| draw(root, m, controls, out));
    }

    fn apply(&self, state: &mut State) -> bool {
        STATE.with_borrow_mut(|m| {
            let mut changed = false;
            let rows = state.selected_rows();
            if let Some(color) = m.pending_face_color.take()
                && !rows.is_empty()
            {
                let touched = state.scene.set_rows_color(&rows, false, color);
                state.scene.appearance_rows.clear();
                for r in touched {
                    state.gpu.set_object_color(r, false, color);
                }
                state.refresh_layers();
                state.touch();
                changed = true;
            }
            if let Some(color) = m.pending_edge_color.take()
                && !rows.is_empty()
            {
                let touched = state.scene.set_rows_color(&rows, true, color);
                state.scene.appearance_rows.clear();
                for r in touched {
                    state.gpu.set_object_color(r, true, color);
                }
                state.refresh_layers();
                state.touch();
                changed = true;
            }
            if let Some(alpha) = m.pending_alpha.take()
                && !rows.is_empty()
            {
                let touched = state.scene.set_rows_alpha(&rows, alpha);
                state.scene.appearance_rows.clear();
                for r in touched {
                    state.gpu.set_object_alpha(r, alpha);
                }
                state.touch();
                changed = true;
            }
            if let Some(opacity) = m.pending_opacity.take() {
                state.set_opacity(opacity);
                changed = true;
            }
            changed
        })
    }

    fn hit(&self, point: egui::Pos2) -> (bool, bool) {
        let on_popup = STATE.with_borrow(|m| m.rect.is_some_and(|r| r.contains(point)));
        (false, on_popup)
    }

    fn snapshot(&self, json: &mut serde_json::Map<String, serde_json::Value>) {
        STATE.with_borrow(|m| {
            json.insert("properties_open".into(), m.open.into());
        });
    }
}

fn draw(
    root: &mut egui::Ui,
    m: &mut PropertiesModel,
    controls: &mut Option<Vec<Control>>,
    out: &mut Output,
) {
    m.rect = None;
    if !m.open {
        return;
    }

    let mut open = m.open;
    let shown = egui::Window::new("Properties")
        .id(egui::Id::new("session-properties"))
        .open(&mut open)
        .default_pos(egui::pos2(12.0, 12.0))
        .default_width(250.0)
        .resizable(false)
        .collapsible(true)
        .show(root.ctx(), |ui| {
            if m.selected_count > 0 {
                ui.strong(if m.selected_count == 1 {
                    format!("{} ({})", m.name, m.kind)
                } else {
                    format!("{} objects selected", m.selected_count)
                });
                if !m.doc_layer.is_empty() {
                    ui.small(&m.doc_layer);
                }
                if !m.guid.is_empty() {
                    let short: String = m.guid.chars().take(12).collect();
                    ui.small(format!("GUID: {short}…")).on_hover_text(&m.guid);
                }
                if !m.counts.is_empty() {
                    ui.small(&m.counts);
                }
                if let Some([dx, dy, dz]) = m.dimensions {
                    ui.small(format!("Size: {dx:.1} × {dy:.1} × {dz:.1}"));
                }

                ui.horizontal(|ui| {
                    let hide_btn = ui.button("Hide");
                    record(controls, "properties/hide", "Hide selection", &hide_btn);
                    if hide_btn.clicked() {
                        out.command = Some("Hide".into());
                    }
                    let lock_btn = ui.button("Lock");
                    record(controls, "properties/lock", "Lock selection", &lock_btn);
                    if lock_btn.clicked() {
                        out.command = Some("Lock".into());
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Face");
                    let mut rgb = m.face_color.unwrap_or([200, 200, 200]);
                    if ui.color_edit_button_srgb(&mut rgb).changed() {
                        m.face_color = Some(rgb);
                        m.pending_face_color = Some(Some(rgb));
                    }
                    if m.face_color.is_some() && ui.small_button("Reset").clicked() {
                        m.face_color = None;
                        m.pending_face_color = Some(None);
                    }
                    if m.has_faces {
                        ui.separator();
                        ui.label("Edge");
                        let mut ergb = m.edge_color.unwrap_or([35, 35, 35]);
                        if ui.color_edit_button_srgb(&mut ergb).changed() {
                            m.edge_color = Some(ergb);
                            m.pending_edge_color = Some(Some(ergb));
                        }
                        if m.edge_color.is_some() && ui.small_button("×").clicked() {
                            m.edge_color = None;
                            m.pending_edge_color = Some(None);
                        }
                    }
                });

                ui.horizontal(|ui| {
                    let mut a = m.alpha.unwrap_or(m.opacity);
                    if ui
                        .add(egui::Slider::new(&mut a, 0.0..=1.0).text("Alpha"))
                        .changed()
                    {
                        m.alpha = Some(a);
                        m.pending_alpha = Some(Some(a));
                    }
                    if m.alpha.is_some() && ui.small_button("×").clicked() {
                        m.alpha = None;
                        m.pending_alpha = Some(None);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Gumball:");
                    for (label, align, cmd) in [
                        ("World", GizmoAlign::World, "Gumball World"),
                        ("Object", GizmoAlign::Object, "Gumball Object"),
                        ("CPlane", GizmoAlign::CPlane, "Gumball CPlane"),
                    ] {
                        let resp = ui.selectable_label(m.gizmo_align == align, label);
                        record(controls, &format!("properties/{cmd}"), cmd, &resp);
                        if resp.clicked() {
                            out.command = Some(cmd.into());
                        }
                    }
                });

                ui.separator();
            } else {
                ui.small("No selection · click an object to inspect");
                ui.separator();
            }

            ui.strong("Display");
            ui.horizontal(|ui| {
                let shaded =
                    ui.selectable_label(!m.wireframe && !m.arctic && m.opacity > 0.0, "Shaded");
                record(controls, "properties/shaded", "Shaded", &shaded);
                if shaded.clicked() {
                    out.command = Some("View Shaded".into());
                }
                let wire = ui.selectable_label(m.wireframe, "Wireframe");
                record(controls, "properties/wireframe", "Wireframe", &wire);
                if wire.clicked() {
                    out.command = Some(if m.wireframe {
                        "View Wireframe Off".into()
                    } else {
                        "View Wireframe On".into()
                    });
                }
                let arctic = ui.selectable_label(m.arctic, "Arctic");
                record(controls, "properties/arctic", "Arctic", &arctic);
                if arctic.clicked() {
                    out.command = Some("Arctic".into());
                }
            });

            ui.horizontal_wrapped(|ui| {
                for (mut flag, label, on_cmd, off_cmd) in [
                    (m.show_edges, "Edges", "View Show Edges", "View Hide Edges"),
                    (m.show_lines, "Lines", "View Show Lines", "View Hide Lines"),
                    (
                        m.show_points,
                        "Points",
                        "View Show Points",
                        "View Hide Points",
                    ),
                    (
                        m.outlines,
                        "Outlines",
                        "View Outline On",
                        "View Outline Off",
                    ),
                    (m.lit, "Lit", "View Lighting On", "View Lighting Off"),
                    (
                        m.backface,
                        "Backface Red",
                        "View Backfaces On",
                        "View Backfaces Off",
                    ),
                    (
                        m.cull_backfaces,
                        "Cull Back",
                        "View Cull Backfaces On",
                        "View Cull Backfaces Off",
                    ),
                    (m.show_grid, "Grid", "Grid On", "Grid Off"),
                ] {
                    let resp = ui.checkbox(&mut flag, label);
                    record(controls, &format!("properties/{label}"), label, &resp);
                    if resp.changed() {
                        out.command = Some(if flag { on_cmd } else { off_cmd }.into());
                    }
                }
            });

            let mut op = m.opacity;
            if ui
                .add(egui::Slider::new(&mut op, 0.0..=1.0).text("Opacity"))
                .changed()
            {
                m.opacity = op;
                m.pending_opacity = Some(op);
            }
        });

    m.open = open;
    if let Some(inner) = shown {
        m.rect = Some(inner.response.rect);
    }
}
