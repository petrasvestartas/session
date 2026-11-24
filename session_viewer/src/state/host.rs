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
        }
    }

    fn set_view_flag(&mut self, flag: ViewFlag, on: bool) {
        let slot = match flag {
            ViewFlag::Points => &mut self.gpu.view.show_points,
            ViewFlag::Lines => &mut self.gpu.view.show_lines,
            ViewFlag::Edges => &mut self.gpu.view.show_mesh_edges,
            ViewFlag::Outline => &mut self.gpu.view.show_outlines,
            ViewFlag::Lighting => &mut self.gpu.view.lit,
            ViewFlag::Backfaces => &mut self.gpu.view.backface,
        };
        *slot = on;
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
        let bytes = crate::app::session_io::save(&self.scene)?;
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
