//! The editing gestures: what a pointer on a gizmo handle, a Delete and a Ctrl+Z do.
//!
//! State is the only place that knows the camera, the scene and the GPU at once, so the join
//! lives here and the four pure modules below it stay free of all three.
//!
//! A drag is three moments. It BEGINS by remembering the object's own transform and the
//! placement it was drawn with. It MOVES by writing a preview into the row's GPU placement -
//! two small writes, no document touched. It ENDS by writing the document once, which is what
//! makes one gesture one undo step.

use crate::app::command::Command;
use crate::app::cplane::CPlane;
use crate::app::gizmo::{Axis, Drag, Gizmo, Handle};
use crate::app::layers::{self, Layer};
use crate::app::selection::ControlId;
use crate::app::snap::{self, Snap, SnapKind};
use crate::state::{SelectionMode, State};
use session_rust::{Point, Vector, Xform};

/// A drag in progress: the row, its transform when the drag started, and the handle.
pub struct GizmoDrag {
    row: u32,
    /// The object's LOCAL transform at the grab. Every frame's preview is measured from this,
    /// never from the frame before, so a dropped frame changes nothing.
    base_local: Xform,
    /// The full placement at the grab, for the same reason, on the GPU side.
    base_place: [f64; 16],
    drag: Drag,
}

impl State {
    /// Put the gizmo on the selected row's box centre, or take it away.
    ///
    /// A row whose box is empty gets no widget rather than one at the world origin: a streamed
    /// cloud before its first slice lands, a sheet row, a row whose geometry produced no
    /// finite bounds. A single point is NOT one of those - `walk_point` grows a box around it,
    /// so min equals max and the widget sits on the point.
    pub fn place_gizmo(&mut self, row: Option<u32>) {
        let Some(box_) = row.and_then(|r| self.gpu.objects.row_bounds(r)) else {
            self.gizmo = None;
            self.upload_gizmo();
            return;
        };
        let origin = Point::new(
            f64::from(box_.min[0] + box_.max[0]) * 0.5,
            f64::from(box_.min[1] + box_.max[1]) * 0.5,
            f64::from(box_.min[2] + box_.max[2]) * 0.5,
        );
        match self.gizmo.as_mut() {
            Some(gizmo) => gizmo.set_origin(origin),
            None => self.gizmo = Some(Gizmo::new(origin)),
        }
        self.upload_gizmo();
    }

    /// Try to grab a gizmo handle under the cursor. False means the click was not on the
    /// widget and the caller should go on to pick.
    pub fn begin_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(row) = self.scene.selected else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let per_px = self.world_per_px();
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        let Some(handle) = gizmo.hit(&from, &dir, per_px) else {
            return false;
        };
        let Some(drag) = gizmo.begin(handle, &from, &dir) else {
            return false;
        };
        let Some(base_local) = self.scene.local_xform_of(row) else {
            return false;
        };
        let Some(base_place) = self.scene.placement_of(row) else {
            return false;
        };
        self.dragging = Some(GizmoDrag {
            row,
            base_local,
            base_place,
            drag,
        });
        true
    }

    /// Move the object with the pointer. The document is not touched: this is a preview.
    pub fn drag_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.dragging.as_ref() else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let Some(gizmo) = self.gizmo.as_ref() else {
            return false;
        };
        let Some(delta) = gizmo.update(&active.drag, &from, &dir) else {
            return false;
        };
        let place = crate::math::mat_mul(&delta, &active.base_place);
        self.gpu
            .objects
            .set_placement(&self.gpu.ctx, active.row, &place);
        self.gpu.grew_bounds(active.row);
        self.place_gizmo(Some(active.row));
        self.touch();
        true
    }

    /// Let go: the document records the whole gesture as one transaction.
    pub fn end_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.dragging.take() else {
            return false;
        };
        self.gpu
            .objects
            .set_placement(&self.gpu.ctx, active.row, &active.base_place);
        self.gpu.grew_bounds(active.row);
        self.touch();
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        gizmo.drag = None;
        let Some(delta) = gizmo.update(&active.drag, &from, &dir) else {
            return false;
        };
        // The delta is a WORLD matrix and the session stores a LOCAL one: the same conjugation
        // the command line goes through, or the object jumps to a different place than the last
        // preview frame drew it.
        let delta = Xform::from_matrix(delta);
        let Some(local) = self
            .scene
            .local_for_world_delta(active.row, &delta, &active.base_local)
        else {
            return false;
        };
        let label = match active.drag.handle {
            Handle::Translate(_) => "move",
            Handle::Rotate(_) => "rotate",
            Handle::Scale(_) | Handle::ScaleUniform => "scale",
        };
        if let Some(place) = self.scene.set_row_xform(active.row, local, label) {
            self.gpu
                .objects
                .set_placement(&self.gpu.ctx, active.row, &place);
            self.gpu.grew_bounds(active.row);
        }
        self.touch();
        true
    }

    /// Abandon a gesture that will never be released - a cancelled pointer, a lost focus.
    ///
    /// The preview lives only in the row's GPU placement, and nothing was recorded that an undo
    /// could take back, so the row is put back where the grab found it. Without this the object
    /// stays where the pointer left it, with the document still holding the old placement, and
    /// no key reaches that state.
    pub fn cancel_gesture(&mut self) {
        if let Some(active) = self.dragging.take() {
            self.gpu
                .objects
                .set_placement(&self.gpu.ctx, active.row, &active.base_place);
            if let Some(gizmo) = self.gizmo.as_mut() {
                gizmo.drag = None;
            }
            self.place_gizmo(Some(active.row));
            self.touch();
        }
        if let Some(active) = self.control_drag.take() {
            if let Some(geometry) = self.scene.geometry(active.parent) {
                self.controls = crate::app::selection::Controls::from_geometry(geometry);
            }
            self.upload_controls();
            self.touch();
        }
    }

    /// Delete the selection. The rows change, so every document is walked again.
    pub fn delete_selected(&mut self) {
        let Some(row) = self.scene.selected else {
            return;
        };
        if !self.scene.delete_row(row) {
            self.status("This object cannot be deleted; streamed scenes cannot be rebuilt");
            return;
        }
        self.after_history();
    }

    /// Undo the last edit in the document that was edited last - Ctrl+Z.
    pub fn undo(&mut self) {
        if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
            self.status("Undo requires a scene without streamed sources");
            return;
        }
        if self.scene.undo() {
            self.after_history();
        }
    }

    /// Redo it - Ctrl+Y, or Ctrl+Shift+Z.
    pub fn redo(&mut self) {
        if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
            self.status("Redo requires a scene without streamed sources");
            return;
        }
        if self.scene.redo() {
            self.after_history();
        }
    }

    /// An undo can bring an object back or take one away, so the rows are rebuilt rather than
    /// patched. The selection is dropped because the row it named may not exist any more.
    fn after_history(&mut self) {
        self.hierarchy.open.clear();
        self.hierarchy.page = 0;
        self.selection = SelectionMode::Object;
        self.select(None);
        self.scene.rebuild(&mut self.gpu);
        self.place_gizmo(None);
        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// World length of one CSS pixel at the gizmo, for a widget that must feel the same however
    /// far the camera is AND whatever the display's pixel ratio is.
    ///
    /// `viewport()` is the surface in PHYSICAL pixels, so the world-per-physical-pixel it gives
    /// is multiplied back up by the physical-per-CSS ratio. Without that the widget is half
    /// size on a 2x display - drawn half size, and grabbable only within half the radius.
    fn world_per_px(&self) -> f64 {
        if let Some(gizmo) = self.gizmo.as_ref() {
            let anchor = self.camera.origin();
            let m = self.camera.view_proj_anchored(self.aspect(), &anchor).m;
            let p = [
                gizmo.origin[0] - anchor[0],
                gizmo.origin[1] - anchor[1],
                gizmo.origin[2] - anchor[2],
            ];
            let w = m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15];
            let vertical = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
            return 2.0 * w.abs() / (vertical * self.logical_size()[1]).max(1e-12);
        }
        world_per_css_px(
            self.camera.distance_world(),
            self.viewport().1,
            self.pixel_scale(),
        )
    }

    /// Physical pixels per CSS pixel, from the surface and the canvas: 1 on a desktop monitor,
    /// 2 or more on a phone. The marker lane wants physical pixels and the widget's sizes are
    /// in CSS pixels, so this is the conversion between them.
    fn pixel_scale(&self) -> f64 {
        let logical = self.logical_size()[0];
        if logical <= 0.0 {
            return 1.0;
        }
        f64::from(self.gpu.config.width) / logical
    }
}

/// World length of one CSS pixel at `world_distance`, given the surface height in PHYSICAL
/// pixels and how many physical pixels one CSS pixel is.
///
/// Every parameter carries its unit in its name, because the two the camera offers differ by a
/// thousand: `Camera::distance_world`, never the `distance` field, which is internal metres. The lengths this scales are an arm and a ball in a millimetre
/// scene, so the metres would draw the widget a thousand times too small.
///
/// Split out because the units are the whole of it: the frustum arithmetic answers in physical
/// pixels, and every size a person sees - an arm, a ball, a grab radius - is in CSS pixels.
/// Forgetting the last multiply makes the widget half size on a 2x display, drawn half size
/// and grabbable only within half the radius.
fn world_per_css_px(world_distance: f64, physical_height: f64, physical_per_css: f64) -> f64 {
    if physical_height <= 0.0 {
        return 1.0;
    }
    let per_physical =
        2.0 * world_distance * (crate::math::FOVY_DEG * 0.5).to_radians().tan() / physical_height;
    per_physical * physical_per_css
}

impl State {
    /// Update the fixed mesh placement without rebuilding its vertices.
    pub fn upload_gizmo(&mut self) {
        let Some(gizmo) = self.gizmo.as_ref() else {
            self.gpu.widget.clear();
            return;
        };
        self.gpu.widget.placement = Some((
            [gizmo.origin[0], gizmo.origin[1], gizmo.origin[2]],
            self.world_per_px(),
        ));
        let handle = self
            .dragging
            .as_ref()
            .map(|drag| drag.drag.handle)
            .or(gizmo.hovered);
        self.gpu.widget.active = match handle {
            Some(Handle::Translate(axis)) => axis as u32 as f32,
            Some(Handle::Rotate(axis)) => axis as u32 as f32 + 3.0,
            Some(Handle::Scale(axis)) => axis as u32 as f32 + 6.0,
            Some(Handle::ScaleUniform) => 9.0,
            None => -1.0,
        };
    }

    pub fn hover_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let per_px = self.world_per_px();
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        let hovered = gizmo.hit(&from, &dir, per_px);
        if gizmo.hovered == hovered {
            return false;
        }
        gizmo.hovered = hovered;
        self.upload_gizmo();
        true
    }
}

impl State {
    /// Run one typed line. The string that comes back is what to show the person who typed it:
    /// what happened, or why nothing did.
    ///
    /// Every arm calls an action the viewer already has, so a command cannot drift from the
    /// key that does the same thing.
    pub fn run_command(&mut self, line: &str) -> Result<String, String> {
        self.cancel_gesture();
        let command = crate::app::command::parse(line)?;
        if matches!(command, Command::Delete | Command::Undo | Command::Redo)
            && (!self.scene.streamed.is_empty() || !self.scene.sheets.is_empty())
        {
            return Err("this command requires a scene without streamed sources".into());
        }
        let needs_selection = matches!(
            command,
            Command::Move(_) | Command::Rotate { .. } | Command::Scale(_) | Command::Delete
        );
        if needs_selection && self.scene.selected.is_none() {
            return Err("nothing is selected".into());
        }
        match command {
            Command::Model(command) => {
                self.scene.model(&command)?;
                self.after_history();
                Ok("geometry updated".into())
            }
            Command::Move(d) => self.apply(Xform::translation(d[0], d[1], d[2]), "move"),
            Command::Rotate { axis, degrees } => {
                let about = self.gizmo.as_ref().map(|g| g.origin.clone());
                let turn = rotation_about(axis, degrees, about.as_ref());
                self.apply(turn, "rotate")
            }
            Command::Scale(k) => {
                let about = self.gizmo.as_ref().map(|g| g.origin.clone());
                self.apply(scaling_about(k, about.as_ref()), "scale")
            }
            Command::Delete => {
                let row = self.scene.selected.ok_or("nothing is selected")?;
                if !self.scene.delete_row(row) {
                    return Err("this object cannot be deleted".into());
                }
                self.after_history();
                Ok("deleted".into())
            }
            Command::Undo => {
                if !self.scene.undo() {
                    return Err("nothing to undo".into());
                }
                self.after_history();
                Ok("undone".into())
            }
            Command::Redo => {
                if !self.scene.redo() {
                    return Err("nothing to redo".into());
                }
                self.after_history();
                Ok("redone".into())
            }
            Command::Hide => {
                self.hide_selected();
                Ok("hidden".into())
            }
            Command::ShowAll => {
                self.show_all();
                Ok("everything shown".into())
            }
            Command::Fit => {
                self.fit_selected_or_all();
                Ok("fitted".into())
            }
            Command::Escape => {
                self.escape_selection();
                Ok("selection cleared".into())
            }
        }
    }

    /// One recorded transform on the selection, with the row's placement written back.
    fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String> {
        let Some(row) = self.scene.selected else {
            return Err("nothing is selected".into());
        };
        let Some(place) = self.scene.transform_row(row, &delta, label) else {
            return Err("this row cannot be edited".into());
        };
        self.gpu.objects.set_placement(&self.gpu.ctx, row, &place);
        self.gpu.grew_bounds(row);
        self.place_gizmo(Some(row));
        self.touch();
        Ok(label.into())
    }
}

/// A turn about a point rather than about the world origin: translate the point to the origin,
/// turn, translate back. Without the centring, `rotate z 90` would swing the object around the
/// file's origin, which is rarely where it is.
fn rotation_about(axis: Axis, degrees: f64, about: Option<&Point>) -> Xform {
    let turn = match axis {
        Axis::X => Xform::rotation_x(degrees, true),
        Axis::Y => Xform::rotation_y(degrees, true),
        Axis::Z => Xform::rotation_z(degrees, true),
    };
    centred(turn, about)
}

/// The same centring for a scale. The kernel already has the centred form, so this only has to
/// choose a centre: the widget's origin, or the world origin when there is no widget.
fn scaling_about(factor: f64, about: Option<&Point>) -> Xform {
    match about {
        Some(p) => Xform::scale_uniform(p, factor),
        None => Xform::scale_xyz(factor, factor, factor),
    }
}

fn centred(inner: Xform, about: Option<&Point>) -> Xform {
    let Some(p) = about else {
        return inner;
    };
    let to = Xform::translation(p[0], p[1], p[2]);
    let back = Xform::translation(-p[0], -p[1], -p[2]);
    &(&to * &inner) * &back
}

impl State {
    /// Hide or show everything one panel row controls, through the same hide set `H` writes.
    ///
    /// A layer with anything still visible hides; a layer wholly hidden comes back. That rule
    /// makes one click enough on a half-hidden layer, which is what a person means by it.
    pub fn toggle_layer(&mut self, layer: Layer) {
        let rows = layers::of_layer(&self.scene, layer);
        if rows.is_empty() {
            return;
        }
        let hide = rows.iter().any(|&row| {
            self.scene
                .identity_of(row)
                .is_some_and(|id| !self.scene.hidden.contains(&id))
        });
        self.set_rows_hidden(&rows, hide);
    }

    /// Redraw the panel from the scene, when it is open.
    pub fn refresh_layers(&mut self) {
        if !crate::app::feedback::layers_open() {
            return;
        }
        self.hierarchy.refresh(&self.scene);
        let mut rows: Vec<crate::app::feedback::LayerRow> = layers::rows(&self.scene)
            .into_iter()
            .map(|row| crate::app::feedback::LayerRow {
                key: row.layer.key(),
                label: row.label,
                count: row.count,
                hidden: row.hidden,
            })
            .collect();
        self.hierarchy_labels(&mut rows);
        crate::app::feedback::layers_panel(&rows);
    }
}

impl State {
    /// Open or close the layers panel - `L`. Opening fills it; closing leaves it empty, so a
    /// scene change while it is shut costs nothing.
    pub fn toggle_layers_panel(&mut self) {
        let open = !crate::app::feedback::layers_open();
        crate::app::feedback::layers_visible(open);
        if open {
            self.refresh_layers();
        }
        self.touch();
    }
}

/// A control point being dragged: which row and which control, and the plane it moves in.
pub struct ControlDrag {
    parent: u32,
    /// Index into `State.controls.points`, which is what the preview moves.
    index: usize,
    /// Which kernel control it is, which is what the commit moves.
    id: ControlId,
    plane: CPlane,
    origin: Point,
}

impl State {
    /// Grab the selected control point, if the press is on it. A control drag is offered
    /// before an object drag because the two gestures are the same press: in control mode the
    /// object's own widget is not shown.
    pub fn begin_control_drag(&mut self, x: f64, y: f64) -> bool {
        let SelectionMode::Controls {
            parent,
            selected: Some(id),
            ..
        } = self.selection
        else {
            return false;
        };
        let Some(index) = self.controls.points.iter().position(|c| c.id == id) else {
            return false;
        };
        let at = self.controls.points[index].position;
        let Some(place) = self.scene.placement_of(parent) else {
            return false;
        };
        let origin = Point::new(at[0], at[1], at[2]).transformed(&Xform::from_matrix(place));
        let Some((sx, sy)) = self.project([origin[0], origin[1], origin[2]]) else {
            return false;
        };
        // `project` answers in physical pixels, so a CSS radius is converted the same way the
        // widget's own sizes are.
        let grab = GRAB_CSS * self.pixel_scale();
        if (sx - x).abs() > grab || (sy - y).abs() > grab {
            return false;
        }
        let forward = self.camera.orientation.rotate_vector(Vector::y_axis());
        self.control_drag = Some(ControlDrag {
            parent,
            index,
            id,
            plane: CPlane::facing(&forward),
            origin,
        });
        true
    }

    /// Move the preview dot. The document is untouched until the drag ends, for the same
    /// reason a gizmo drag does not touch it: one gesture is one undo step.
    pub fn drag_control(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.control_drag.as_ref() else {
            return false;
        };
        let Some(point) = self.control_target(active, x, y) else {
            return false;
        };
        let Some(back) = self
            .scene
            .placement_of(active.parent)
            .and_then(|m| Xform::from_matrix(m).inverse())
        else {
            return false;
        };
        let point = point.transformed(&back);
        let index = active.index;
        self.controls.points[index].position = [point[0], point[1], point[2]];
        self.upload_controls();
        self.touch();
        true
    }

    /// Commit: the kernel replaces the geometry, and the rows are walked again because the
    /// shape changed rather than its placement.
    pub fn end_control_drag(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.control_drag.take() else {
            return false;
        };
        if let Some(geometry) = self.scene.geometry(active.parent) {
            self.controls = crate::app::selection::Controls::from_geometry(geometry);
        }
        self.upload_controls();
        self.touch();
        let Some(point) = self.control_target(&active, x, y) else {
            return false;
        };
        let index = match active.id {
            ControlId::Curve { point, .. } => point,
            ControlId::Vertex(index) => index,
            _ => return false,
        };
        if !self.scene.set_control_point(active.parent, index, &point) {
            self.status("This geometry's control points cannot be edited");
            return false;
        }
        self.scene.rebuild(&mut self.gpu);
        self.selection = SelectionMode::Object;
        self.select(Some(active.parent));
        self.enable_controls();
        self.touch();
        true
    }

    /// Where the pointer is, in the world: the construction plane the view is facing, with the
    /// object's other control points offered as snaps.
    fn control_target(&self, active: &ControlDrag, x: f64, y: f64) -> Option<Point> {
        let (from, dir) = self.camera.ray((x, y), self.viewport())?;
        let free = active.plane.hit(&active.origin, &from, &dir)?;
        let place = Xform::from_matrix(self.scene.placement_of(active.parent)?);
        let mut candidates = Vec::new();
        for (i, control) in self.controls.points.iter().enumerate() {
            if i == active.index {
                continue;
            }
            candidates.push(Snap {
                point: Point::new(
                    control.position[0],
                    control.position[1],
                    control.position[2],
                )
                .transformed(&place),
                kind: SnapKind::Vertex,
                owner: active.parent,
            });
        }
        // The ranking is in SCREEN space, so the aperture means pixels wherever the camera is.
        let project = |p: &Point| self.project([p[0], p[1], p[2]]);
        match snap::best(
            &candidates,
            (x, y),
            SNAP_APERTURE_PX * self.pixel_scale(),
            project,
        ) {
            Some(hit) => Some(hit.point),
            None => Some(free),
        }
    }

    /// A world point in framebuffer pixels, or `None` when it is behind the eye.
    fn project(&self, at: [f64; 3]) -> Option<(f64, f64)> {
        let (w, h) = self.viewport();
        let anchor = Point::new(at[0], at[1], at[2]);
        let mvp = self.camera.view_proj_anchored(self.aspect(), &anchor);
        let clip = [mvp.m[12], mvp.m[13], mvp.m[14], mvp.m[15]];
        if clip[3] <= 0.0 {
            return None;
        }
        Some((
            (clip[0] / clip[3] * 0.5 + 0.5) * w,
            (0.5 - clip[1] / clip[3] * 0.5) * h,
        ))
    }
}

/// How near the pointer must be to grab a control dot, in CSS pixels: the dot is drawn at
/// 3.5 CSS px, and a grab radius smaller than the thing it grabs is a gesture people miss.
const GRAB_CSS: f64 = 10.0;

/// How far a snap reaches, in CSS pixels. Wide enough to catch what you meant, narrow enough
/// that a point a centimetre away on screen is not "what you meant".
const SNAP_APERTURE_PX: f64 = 12.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// GPU pixels contain all three axes and the widget owns no scene rows.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn the_widget_reaches_the_pixels() {
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::Xform;

        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
        gpu.view.show_grid = false;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity().m, 0));
        gpu.set_scene(&upload);

        let mut camera = Camera::new();
        camera.target = [0.0, 0.0, 0.0];
        camera.distance = 0.2; // metres: 200 world units in a millimetre scene
        camera.update_position();
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::BLACK,
            now_ms: 0.0,
        };
        let before = gpu.render_offscreen(&input);

        // 96 CSS px at 0.5 world units per pixel fits inside a 200-unit view.
        gpu.widget.placement = Some(([0.0; 3], 0.5));
        let after = gpu.render_offscreen(&input);

        let (mut red, mut green, mut blue, mut changed) = (0, 0, 0, 0);
        for (a, b) in before.chunks_exact(4).zip(after.chunks_exact(4)) {
            if a[..3] == b[..3] {
                continue;
            }
            changed += 1;
            let (r, g, bl) = (i32::from(b[0]), i32::from(b[1]), i32::from(b[2]));
            if r > g + 40 && r > bl + 40 {
                red += 1;
            } else if g > r + 40 && g > bl + 40 {
                green += 1;
            } else if bl > r + 40 && bl > g + 40 {
                blue += 1;
            }
        }
        assert!(
            changed > 100,
            "the widget changed the picture: {changed} pixels"
        );
        assert!(
            red > 10 && green > 10 && blue > 10,
            "three coloured arms: red {red}, green {green}, blue {blue}, of {changed} changed"
        );
    }

    /// The conversion the widget's size depends on. A 2x display has twice the physical pixels
    /// for the same CSS pixel, so one CSS pixel is twice as much world - and the arm that is
    /// 96 CSS pixels long stays 96 CSS pixels long.
    #[test]
    fn a_css_pixel_is_worth_more_world_on_a_denser_display() {
        let one_to_one = world_per_css_px(1000.0, 800.0, 1.0);
        let retina = world_per_css_px(1000.0, 1600.0, 2.0);
        assert!(
            (one_to_one - retina).abs() < 1e-9,
            "the same CSS pixel, either way"
        );

        let closer = world_per_css_px(500.0, 800.0, 1.0);
        assert!(closer < one_to_one, "nearer camera, less world in a pixel");
        // The unit trap: a metre distance where a millimetre one was meant shrinks every
        // length the widget draws by a thousand.
        assert!(
            world_per_css_px(1.0, 800.0, 1.0) * 1000.0 - world_per_css_px(1000.0, 800.0, 1.0)
                < 1e-9,
            "the answer scales with the distance, so the distance must be in world units"
        );
        assert_eq!(
            world_per_css_px(1000.0, 0.0, 1.0),
            1.0,
            "no surface, no answer"
        );
    }
}
