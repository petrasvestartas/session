use super::State;
use crate::app::clipping::Mode as ClipMode;
use crate::app::command::host::{CommandHost, ViewFlag};
use crate::app::command::tool::Tool;
use crate::app::command::verbs::measure::Mark;
use crate::app::cplane::CPlane;
use crate::app::scene::Scene;
use crate::app::selection::{SelectionMode, SelectionTool};
use crate::app::snap::{Screen, Snapping};
use crate::camera::Camera;
use crate::rows::TextLabel;
use session_rust::{AABB, Geometry, Plane, Point, Xform};

impl CommandHost for State {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    fn selection(&self) -> &SelectionMode {
        &self.selection.mode
    }

    fn set_selection_tool(&mut self, tool: SelectionTool) {
        self.selection.tool = tool;
    }

    fn selected_rows(&self) -> Vec<u32> {
        State::selected_rows(self)
    }

    fn ordered_rows(&self) -> Vec<u32> {
        State::ordered_rows(self)
    }

    fn select(&mut self, row: Option<u32>) {
        State::select(self, row);
    }

    fn select_rows(&mut self, rows: Vec<u32>, additive: bool) {
        State::select_rows(self, rows, additive);
    }

    fn escape_selection(&mut self) {
        State::escape_selection(self);
    }

    fn enable_controls(&mut self) {
        State::enable_controls(self);
    }

    fn snap(&self) -> &Snapping {
        &self.features.snap
    }

    fn snap_mut(&mut self) -> &mut Snapping {
        &mut self.features.snap
    }

    fn gizmo_origin(&self) -> Option<Point> {
        self.features.gizmo.as_ref().map(|g| g.origin.clone())
    }

    fn place_gizmo(&mut self, row: Option<u32>) {
        State::place_gizmo(self, row);
    }

    fn viewport(&self) -> (f64, f64) {
        State::viewport(self)
    }

    fn aspect(&self) -> f64 {
        State::aspect(self)
    }

    fn pixel_scale(&self) -> f64 {
        State::pixel_scale(self)
    }

    fn view_screen(&self) -> Screen {
        State::view_screen(self)
    }

    fn refresh_bounds(&mut self) {
        State::refresh_bounds(self);
    }

    fn scene_bounds(&self) -> AABB {
        self.gpu.bounds
    }

    fn row_bounds(&self, row: u32) -> Option<AABB> {
        self.gpu.objects.row_bounds(row)
    }

    fn row_flags(&self, row: u32) -> Option<u32> {
        self.gpu.objects.row(row).map(|o| o.flags)
    }

    fn locked_reason(&mut self, rows: &[u32]) -> Option<String> {
        State::locked_reason(self, rows)
    }

    fn after_history(&mut self) {
        State::after_history(self);
    }

    fn commit_rows(&mut self) {
        State::commit_rows(self);
    }

    fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String> {
        State::apply(self, delta, label)
    }

    fn copy_selection(&mut self, delta: &Xform) -> Result<String, String> {
        State::copy_selection(self, delta)
    }

    fn delete_selection(&mut self) -> Result<String, String> {
        State::delete_selection(self)
    }

    fn hide_selected(&mut self) {
        State::hide_selected(self);
    }

    fn show_all(&mut self) {
        State::show_all(self);
    }

    fn fit_selected_or_all(&mut self) {
        State::fit_selected_or_all(self);
    }

    fn start_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String> {
        State::start_tool(self, tool)
    }

    fn open_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String> {
        State::open_tool(self, tool)
    }

    fn tool_running(&self) -> bool {
        State::tool_running(self)
    }

    fn drafting(&self) -> bool {
        State::drafting(self)
    }

    fn cancel_drawing(&mut self) {
        State::cancel_drawing(self);
    }

    fn close_drawing(&mut self) -> Result<String, String> {
        State::close_drawing(self)
    }

    fn drawing_cursor(&self) -> Option<&Point> {
        State::drawing_cursor(self)
    }

    fn run_command(&mut self, line: &str) -> Result<String, String> {
        State::run_command(self, line)
    }

    fn split_command(&mut self) -> Result<String, String> {
        State::split_command(self)
    }

    fn set_row_selected(&mut self, row: u32, selected: bool) {
        self.gpu.set_selected(row, selected);
    }

    fn set_row_hidden(&mut self, row: u32, hidden: bool) {
        self.gpu.set_hidden(row, hidden);
    }

    fn restore_source_render(&mut self, row: u32) {
        if let Some(geometry) = self.scene.geometry(row).cloned() {
            self.scene.redraw(row, &geometry, false);
            self.scene.upload_to(&mut self.gpu);
        }
    }

    fn preview_geometry(&mut self, row: u32, geometry: Geometry) -> Result<(), String> {
        self.scene.preview_geometry(row, geometry, &mut self.gpu)
    }

    fn arctic(&self) -> bool {
        self.gpu.view.ssao
    }

    fn set_arctic(&mut self, on: bool) {
        self.gpu.view.set_arctic(on);
        self.touch();
    }

    fn outlines(&self) -> bool {
        self.gpu.view.show_outlines
    }

    fn set_outlines(&mut self, on: bool) {
        self.gpu.view.show_outlines = on;
        self.touch();
    }

    fn view_flag(&self, flag: ViewFlag) -> bool {
        match flag {
            ViewFlag::Points => self.gpu.view.show_points,
            ViewFlag::Lines => self.gpu.view.show_lines,
            ViewFlag::Edges => self.gpu.view.show_mesh_edges,
            ViewFlag::Outline => self.gpu.view.show_outlines,
            ViewFlag::Lighting => self.gpu.view.lit,
            ViewFlag::Backfaces => self.gpu.view.backface,
            ViewFlag::Wireframe => self.gpu.view.wireframe,
            ViewFlag::CullBackfaces => self.gpu.view.cull_backfaces,
        }
    }

    fn set_view_flag(&mut self, flag: ViewFlag, on: bool) {
        match flag {
            ViewFlag::Points => self.gpu.view.show_points = on,
            ViewFlag::Lines => self.gpu.view.show_lines = on,
            ViewFlag::Edges => self.gpu.view.show_mesh_edges = on,
            ViewFlag::Outline => self.gpu.view.show_outlines = on,
            ViewFlag::Lighting => self.gpu.view.lit = on,
            ViewFlag::Backfaces => self.gpu.view.backface = on,
            ViewFlag::CullBackfaces => self.gpu.view.cull_backfaces = on,
            ViewFlag::Wireframe => {
                self.gpu.view.wireframe = on;
                if on {
                    self.gpu.view.show_mesh_edges = true;
                }
                if self.scene.wireframe != on {
                    self.scene.wireframe = on;
                    if on && self.scene.has_large_meshes() {
                        if !self.scene.rewalk_editable(&mut self.gpu) {
                            self.resume_after(super::hydrate::Resume::Rewalk);
                        }
                    }
                }
            }
        }
        self.touch();
    }

    fn opacity(&self) -> f32 {
        self.gpu.view.opacity
    }

    fn set_opacity(&mut self, value: f32) {
        State::set_opacity(self, value);
    }

    fn toggle_xray(&mut self) {
        State::toggle_xray(self);
    }

    fn set_cloud_size(&mut self, size: f32) {
        State::set_cloud_size(self, size);
    }

    fn set_plane_size(&mut self, size: f64) {
        State::set_plane_size(self, size);
    }

    fn toggle_selected_names(&mut self) {
        State::toggle_selected_names(self);
    }

    fn show_attributes(&mut self, value: Option<bool>) -> bool {
        State::show_attributes(self, value)
    }

    fn show_interactions(&mut self, value: Option<bool>) -> bool {
        State::show_interactions(self, value)
    }

    fn set_construction_plane(&mut self, plane: Option<CPlane>) {
        State::set_construction_plane(self, plane);
    }

    fn construction_plane_status(&self) -> serde_json::Value {
        State::construction_plane_status(self)
    }

    fn clipping_half_size(&mut self) -> f64 {
        State::clipping_half_size(self)
    }

    fn pick_clipping_plane(&mut self, mode: ClipMode) -> String {
        State::pick_clipping_plane(self, mode)
    }

    fn create_clipping_plane(&mut self, plane: Plane) -> Result<String, String> {
        State::create_clipping_plane(self, plane)
    }

    fn flip_clipping_planes(&mut self) -> Result<String, String> {
        State::flip_clipping_planes(self)
    }

    fn set_clipping(&mut self, on: bool) -> String {
        State::set_clipping(self, on)
    }

    fn set_clipping_fill(&mut self, solid: Option<bool>) -> String {
        State::set_clipping_fill(self, solid)
    }

    fn add_text(&mut self, label: TextLabel) {
        State::add_text(self, label);
    }

    fn set_mark(&mut self, points: Vec<Point>, label: String) {
        self.features.mark = Some(Mark {
            points,
            label,
            revision: self.scene.row_revision,
        });
        self.touch();
    }

    fn mark_selection(&mut self, label: String) {
        State::mark_selection(self, label);
    }

    fn refresh_layers(&mut self) {
        State::refresh_layers(self);
    }

    fn expand_layers(&mut self) {
        State::expand_layers(self);
    }

    fn reveal_layer(&mut self, doc: usize, name: &str) {
        State::reveal_layer(self, doc, name);
    }

    fn set_layers_panel(&mut self, open: Option<bool>, expand_all: bool) {
        let show = open.unwrap_or(!crate::app::feedback::layers_open());
        crate::app::feedback::layers_visible(show);
        self.refresh_layers();
        if expand_all {
            self.expand_layers();
            self.refresh_layers();
        }
    }

    fn fetch_wanted(&mut self) {
        State::fetch_wanted(self);
    }

    fn save_session(&mut self) -> Result<String, String> {
        if self.save_when_back() {
            return Ok(
                "Loading the display-only documents; the save downloads when they are back".into(),
            );
        }
        let (cam, view) = self.saved_view_state();
        let bytes = crate::app::session_io::save_with_state(&self.scene, Some(cam), Some(view))?;
        #[cfg(target_arch = "wasm32")]
        crate::app::session_io::download(&bytes).map_err(|e| format!("Save failed: {e:?}"))?;
        Ok(format!("Saved complete session ({} bytes)", bytes.len()))
    }

    fn open_session_prompt(&mut self) -> Result<String, String> {
        #[cfg(target_arch = "wasm32")]
        crate::app::session_io::pick();
        Ok("Choose a .session file".into())
    }

    fn download_report(&mut self) -> Result<String, String> {
        crate::app::feedback::download_report();
        Ok("Report downloaded".into())
    }

    fn lock_selected(&mut self, lock: bool) -> Result<String, String> {
        if lock {
            let rows = self.selected_rows();
            if rows.is_empty() {
                return Err("Select an object to lock".into());
            }
            self.select(None);
            let n = self.scene.set_rows_locked(&rows, true);
            self.refresh_layers();
            self.touch();
            Ok(format!("Locked {n} object(s)"))
        } else {
            let n = self.scene.unlock_all_identities();
            self.refresh_layers();
            self.touch();
            Ok(format!("Unlocked {n} object(s)"))
        }
    }

    fn set_object_alpha(&mut self, alpha: Option<f32>) -> Result<String, String> {
        let rows = self.selected_rows();
        if rows.is_empty() {
            return Err("Select an object first".into());
        }
        let changed = self.scene.set_rows_alpha(&rows, alpha);
        self.scene.appearance_rows.clear();
        for row in changed {
            self.gpu.set_object_alpha(row, alpha);
        }
        self.touch();
        Ok(match alpha {
            Some(a) => format!("Object alpha {:.2}", a.clamp(0.0, 1.0)),
            None => "Object alpha reset".into(),
        })
    }

    fn set_gizmo_align(&mut self, align: crate::app::gizmo::GizmoAlign) -> String {
        self.features.gizmo_align = align;
        self.place_gizmo(self.scene.selected);
        self.touch();
        format!("Gumball aligned to {align:?}")
    }

    fn set_gizmo_pivot(&mut self, pivot: Option<Point>) -> String {
        self.features.gizmo_pivot = pivot.clone();
        self.place_gizmo(self.scene.selected);
        self.touch();
        match pivot {
            Some(p) => format!("Gumball pivot at ({:.2}, {:.2}, {:.2})", p[0], p[1], p[2]),
            None => "Gumball pivot reset".into(),
        }
    }

    fn set_properties_panel(&mut self, open: Option<bool>) -> String {
        let show = open.unwrap_or(!crate::app::feedback::properties_open());
        crate::app::feedback::properties_visible(show);
        self.touch();
        if show {
            "Properties panel open".into()
        } else {
            "Properties panel closed".into()
        }
    }

    fn export_obj(&mut self, path: Option<&str>) -> Result<String, String> {
        let selected = self.selected_rows();
        let filter = if selected.is_empty() {
            None
        } else {
            Some(selected.as_slice())
        };
        let text = crate::app::session_io::export_obj(&self.scene, filter)?;
        let count = text.lines().filter(|l| l.starts_with("o ")).count().max(1);
        #[cfg(target_arch = "wasm32")]
        {
            let name = path
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("export.obj");
            crate::app::session_io::download_text(name, &text)
                .map_err(|e| format!("Export failed: {e:?}"))?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(p) = path.filter(|s| !s.trim().is_empty()) {
            std::fs::write(p, text.as_bytes()).map_err(|e| format!("Cannot write {p}: {e}"))?;
        }
        Ok(format!("Exported {count} object(s) ({} bytes)", text.len()))
    }

    fn import_file(&mut self, path: Option<&str>) -> Result<String, String> {
        #[cfg(target_arch = "wasm32")]
        if path.is_none_or(|s| s.trim().is_empty()) {
            crate::app::session_io::pick_import();
            return Ok("Choose a .pb, .obj or .session file".into());
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(p) = path.filter(|s| !s.trim().is_empty()) {
            let bytes = std::fs::read(p).map_err(|e| format!("Cannot read {p}: {e}"))?;
            if p.to_ascii_lowercase().ends_with(".session") {
                let opened = crate::app::session_io::open_with_state(&bytes)?;
                self.clear();
                self.scene = opened.scene;
                self.scene.upload_to(&mut self.gpu);
                self.scene.restore_text_visibility(&mut self.gpu);
                self.update_label();
                self.apply_saved_view(opened.camera, opened.view);
                self.refresh_layers();
                self.touch();
                return Ok("Session opened".into());
            }
            let doc = crate::app::session_io::import_bytes(&mut self.scene, p, &bytes)?;
            let count = self.scene.docs[doc].session.lookup.len();
            self.scene.upload_to(&mut self.gpu);
            self.camera.grow_extent(&self.gpu.bounds);
            self.refresh_layers();
            self.update_label();
            self.touch();
            return Ok(format!("Imported {count} object(s) from {p}"));
        }
        Err("Import requires a file path or browser file picker".into())
    }

    fn saved_view(&self) -> crate::app::session_io::SavedView {
        self.saved_view_state().1
    }

    fn install_opened_session(&mut self, opened: crate::app::session_io::OpenedSession) {
        self.clear();
        self.scene = opened.scene;
        self.scene.upload_to(&mut self.gpu);
        self.scene.restore_text_visibility(&mut self.gpu);
        self.update_label();
        self.apply_saved_view(opened.camera, opened.view);
        self.refresh_layers();
        self.touch();
    }

    fn clear_scene(&mut self) {
        self.clear();
    }

    fn append_file(&mut self, doc: crate::app::scene::FileDoc) {
        self.append(doc, None);
    }

    fn perf_stats(&self) -> serde_json::Value {
        let p = &self.gpu.performance;
        let [w, h] = self.logical_size();
        serde_json::json!({
            "mode": "browser",
            "frames": p.frames,
            "draws": p.draws,
            "interacting": p.interacting,
            "cpu": p.cpu,
            "causes": p.causes,
            "stats": p.stats(),
            "documents": self.scene.docs.len(),
            "objects": self.scene.object_count(),
            "rows": self.scene.row_count(),
            "viewport": [w, h],
        })
    }

    fn note_status(&mut self, message: &str) {
        crate::app::feedback::status(message);
    }

    fn ensure_graph_open(&mut self) {
        if !crate::app::feedback::graph_open() {
            crate::app::feedback::toggle_graph();
        }
    }

    fn touch(&mut self) {
        State::touch(self);
    }
}
