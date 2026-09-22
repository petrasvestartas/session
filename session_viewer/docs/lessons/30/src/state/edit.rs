use crate::app::command::Command;
use crate::app::cplane::CPlane;
use crate::app::gizmo::{Axis, Drag, Gizmo, Handle};
use crate::app::layers::{self, Layer};
use crate::app::selection::ControlId;
use crate::app::snap::{self, Snap, SnapKind};
use crate::state::{SelectionMode, State};
use session_rust::{Point, Vector, Xform};

/// A gizmo drag in progress.
pub struct GizmoDrag {
    row: u32,                                             // the main selected row
    base_local: Xform, // the object's local transform at the grab
    base_place: Xform,                                    // the main row's placement at the grab
    drag: Drag,                                           // the handle and where it was grabbed
    target: Option<crate::app::deform::Target>,           // a face, edge or control point being moved
    source: Option<session_rust::Geometry>,               // the geometry before the drag
    origin: Point,                                        // the gizmo center at the grab
}

impl State {
    /// Put the gizmo at the center of the selection, or remove it.
    pub fn place_gizmo(&mut self, row: Option<u32>) {
        // no box, no gizmo
        let Some(box_) = row.and_then(|r| self.gpu.objects.row_bounds(r)) else {
            self.gizmo = None;
            self.upload_gizmo();
            return;
        };
        let mut origin = box_.center();

        // a selected face, edge or control point: center on it instead
        if let Some(row) = row
            && let Some(target) = crate::app::deform::Target::selected(&self.selection)
            && let Some(geometry) = self.scene.geometry(row)
            && let Ok(points) = crate::app::deform::points(geometry, target)
            && !points.is_empty()
            && let Some(place) = self.scene.placement_of(row)
        {
            let n = points.len() as f64;
            origin = Point::new(
                points.iter().map(|p| p[0]).sum::<f64>() / n,
                points.iter().map(|p| p[1]).sum::<f64>() / n,
                points.iter().map(|p| p[2]).sum::<f64>() / n,
            )
            .transformed(&place);
        }

        match self.gizmo.as_mut() {
            Some(gizmo) => gizmo.set_origin(origin),
            None => self.gizmo = Some(Gizmo::new(origin)),
        }

        self.upload_gizmo();
    }

    /// Grab a gizmo handle under the mouse; false when the click missed it.
    pub fn begin_gizmo(&mut self, x: f64, y: f64) -> bool {
        self.begin_gizmo_with_radius(x, y, 8.0)
    }

    /// Grab a gizmo handle under a finger, with a wider reach.
    pub fn begin_gizmo_touch(&mut self, x: f64, y: f64) -> bool {
        self.begin_gizmo_with_radius(x, y, 18.0)
    }

    /// Grab a gizmo handle within `radius` CSS pixels.
    fn begin_gizmo_with_radius(&mut self, x: f64, y: f64, radius: f64) -> bool {
        let Some(row) = self.scene.selected else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let per_px = self.world_per_px(); // scene length of one pixel at the gizmo
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        let Some(handle) = gizmo.hit_with_radius(&from, &dir, per_px, radius) else {
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
            target: crate::app::deform::Target::selected(&self.selection),
            source: self.scene.geometry(row).cloned(),
            origin: gizmo.origin.clone(),
        });
        true
    }

    /// Move the selection with the pointer; a preview, the document is untouched.
    pub fn drag_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.dragging.as_ref() else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        // the transform the gesture means so far
        let Some(delta) = Gizmo::new(active.origin.clone()).update(&active.drag, &from, &dir)
        else {
            return false;
        };

        // a face, edge or control point moves inside the object
        if let (Some(target), Some(source)) = (active.target, active.source.as_ref()) {
            let row = active.row;
            let place = active.base_place.clone();
            let Some(back) = place.inverse() else {
                return false;
            };
            let local = &(&back * &delta) * &place; // the delta in the object's own frame
            let edited = match crate::app::deform::transform(source, target, &local) {
                Ok(value) => value,
                Err(error) => {
                    self.status(&error);
                    return false;
                }
            };
            let origin = active.origin.transformed(&delta);

            if let Err(error) = self.scene.preview_geometry(row, edited, &mut self.gpu) {
                self.status(&error);
                return false;
            }

            if let Some(gizmo) = self.gizmo.as_mut() {
                gizmo.origin = origin;
            }

            self.upload_gizmo();
            self.touch();
            return true;
        }

        let place = &delta * &active.base_place;
        self.gpu
            .objects
            .set_placement(&self.gpu.ctx, active.row, &place);
        self.gpu.grew_bounds(active.row);
        self.place_gizmo(Some(active.row));
        self.touch();
        true
    }

    /// Release: the document records the whole gesture as one undo step.
    pub fn end_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.dragging.take() else {
            return false;
        };
        // put the preview back, the document applies the real move
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
        let Some(delta) = Gizmo::new(active.origin.clone()).update(&active.drag, &from, &dir)
        else {
            return false;
        };

        if let Some(target) = active.target {
            let result =
                self.scene
                    .edit_subobject(active.row, target, &delta, "transform subobject");
            // --8<-- [start:step-30b]
            self.restore_source_render(active.row);
            // --8<-- [end:step-30b]
            self.restore_edit_selection(active.row);

            if let Err(error) = result {
                self.status(&error);
                return false;
            }

            self.refresh_layers();
            self.touch();
            return true;
        }

        // world delta to local: conjugate by the parent
        let Some(local) = self
            .scene
            .local_for_world_delta(active.row, &delta, &active.base_local)
        else {
            return false;
        };
        // the undo label
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

    /// Drop a drag that will never be released; everything goes back.
    pub fn cancel_gesture(&mut self) {
        if let Some(active) = self.dragging.take() {
            if active.target.is_some() {
                // --8<-- [start:step-30c]
                self.restore_source_render(active.row);
                // --8<-- [end:step-30c]
                self.restore_edit_selection(active.row);
            }

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

    /// Delete the selection.
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

    /// Ctrl+Z: undo the last edit.
    pub fn undo(&mut self) {
        if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
            self.status("Undo requires a scene without streamed sources");
            return;
        }

        if self.scene.undo() {
            self.after_history();
        }
    }

    /// Ctrl+Y: redo it.
    pub fn redo(&mut self) {
        if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
            self.status("Redo requires a scene without streamed sources");
            return;
        }

        if self.scene.redo() {
            self.after_history();
        }
    }

    /// Rebuild the rows after an undo.
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

    /// Scene length of one CSS pixel at the gizmo.
    fn world_per_px(&self) -> f64 {
        // at the gizmo: from its projected depth
        if let Some(gizmo) = self.gizmo.as_ref() {
            let anchor = self.camera.origin();
            let m = self.camera.view_proj_anchored(self.aspect(), &anchor).m;
            let p = [
                gizmo.origin[0] - anchor[0],
                gizmo.origin[1] - anchor[1],
                gizmo.origin[2] - anchor[2],
            ];
            let w = m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15]; // clip w: the depth
            let vertical = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt(); // clip units per scene unit, vertical
            return 2.0 * w.abs() / (vertical * self.logical_size()[1]).max(1e-12);
        }

        world_per_css_px(
            self.camera.distance_world(),
            self.viewport().1,
            self.pixel_scale(),
        )
    }

    /// Physical pixels per CSS pixel.
    fn pixel_scale(&self) -> f64 {
        let logical = self.logical_size()[0];

        if logical <= 0.0 {
            return 1.0;
        }

        f64::from(self.gpu.config.width) / logical
    }
}

/// Scene length of one CSS pixel at `world_distance` from the eye.
fn world_per_css_px(world_distance: f64, physical_height: f64, physical_per_css: f64) -> f64 {
    if physical_height <= 0.0 {
        return 1.0;
    }

    // view height at that distance, over the pixels it covers
    let per_physical =
        2.0 * world_distance * (crate::camera::FOVY_DEG * 0.5).to_radians().tan() / physical_height;
    per_physical * physical_per_css
}

impl State {
    /// Tell the GPU where the gizmo is and which handle lights up.
    pub fn upload_gizmo(&mut self) {
        let Some(gizmo) = self.gizmo.as_ref() else {
            self.gpu.widget.clear();
            return;
        };
        self.gpu.widget.placement = Some((
            [gizmo.origin[0], gizmo.origin[1], gizmo.origin[2]],
            self.world_per_px(), // keeps the widget the same pixel size
        ));
        // the dragged handle, else the hovered one
        let handle = self
            .dragging
            .as_ref()
            .map(|drag| drag.drag.handle)
            .or(gizmo.hovered);
        // handle index for the shader: 0-2 move, 3-5 rotate, 6-8 scale, 9 uniform
        self.gpu.widget.active = match handle {
            Some(Handle::Translate(axis)) => axis as u32 as f32,
            Some(Handle::Rotate(axis)) => axis as u32 as f32 + 3.0,
            Some(Handle::Scale(axis)) => axis as u32 as f32 + 6.0,
            Some(Handle::ScaleUniform) => 9.0,
            None => -1.0,
        };
    }

    /// Light the gizmo handle under the pointer; true when it changed.
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
    /// Run one command line; the answer is what to show the person.
    pub fn run_command(&mut self, line: &str) -> Result<String, String> {
        self.cancel_gesture();
        let command = crate::app::command::parse(line)?;

        if matches!(command, Command::Delete | Command::Undo | Command::Redo)
            && (!self.scene.streamed.is_empty() || !self.scene.sheets.is_empty())
        {
            return Err("this command requires a scene without streamed sources".into());
        }

        // commands that act on the selection
        let needs_selection = matches!(
            command,
            Command::Move(_) | Command::Rotate { .. } | Command::Scale(_) | Command::Delete
        );

        if needs_selection && self.scene.selected.is_none() {
            return Err("nothing is selected".into());
        }

        match command {
            Command::Save => {
                let bytes = crate::app::session_io::save(&self.scene)?;
                #[cfg(target_arch = "wasm32")]
                crate::app::session_io::download(&bytes)
                    .map_err(|e| format!("Save failed: {e:?}"))?;
                Ok(format!("Saved complete session ({} bytes)", bytes.len()))
            }
            Command::Open => {
                #[cfg(target_arch = "wasm32")]
                crate::app::session_io::pick();
                Ok("Choose a .session file".into())
            }
            Command::Model(command) => {
                // --8<-- [start:step-30d]
                use crate::app::modeling::Modeling;
                // a new object, as opposed to an edit
                let created = matches!(
                    command,
                    Modeling::Point(_) | Modeling::Line(..) | Modeling::Polyline(_)
                );
                self.scene.model(&command)?;
                self.after_history();

                if created {
                    // select the new object: the last row of its document
                    if let Some(doc) = self.scene.created_doc {
                        let row = (0..self.gpu.objects.len())
                            .rev()
                            .find(|&row| self.scene.identity_of(row).is_some_and(|id| id.0 == doc));
                        self.select(row);
                    }

                    let name = match command {
                        Modeling::Point(_) => "point",
                        Modeling::Line(..) => "line",
                        _ => "polyline",
                    };
                    Ok(format!(
                        "Created and selected {name}. Type Fit to locate it; Undo to remove it."
                    ))
                } else {
                    Ok("geometry updated".into())
                }
                // --8<-- [end:step-30d]
            }
            Command::Move(d) => self.apply(Xform::translation(d[0], d[1], d[2]), "move"),
            Command::Rotate { axis, degrees } => {
                let about = self.gizmo.as_ref().map(|g| g.origin.clone()); // turn about the gizmo
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

    /// Apply one transform to the selection and record it.
    fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String> {
        let Some(row) = self.scene.selected else {
            return Err("nothing is selected".into());
        };

        // a face, edge or control point moves inside the object
        if let Some(target) = crate::app::deform::Target::selected(&self.selection) {
            self.scene.edit_subobject(row, target, &delta, label)?;
            self.scene.rebuild(&mut self.gpu);
            self.restore_edit_selection(row);
            self.touch();
            return Ok(label.into());
        }

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

/// A rotation about a point.
fn rotation_about(axis: Axis, degrees: f64, about: Option<&Point>) -> Xform {
    let turn = match axis {
        Axis::X => Xform::rotation_x(degrees, true),
        Axis::Y => Xform::rotation_y(degrees, true),
        Axis::Z => Xform::rotation_z(degrees, true),
    };
    centred(turn, about)
}

/// A uniform scale about a point.
fn scaling_about(factor: f64, about: Option<&Point>) -> Xform {
    match about {
        Some(p) => Xform::scale_uniform(p, factor),
        None => Xform::scale_xyz(factor, factor, factor),
    }
}

/// Move `about` to the origin, apply `inner`, move back.
fn centred(inner: Xform, about: Option<&Point>) -> Xform {
    let Some(p) = about else {
        return inner;
    };
    let to = Xform::translation(p[0], p[1], p[2]);
    let back = Xform::translation(-p[0], -p[1], -p[2]);
    &(&to * &inner) * &back
}

impl State {
    /// Hide a layer, or show it when it is fully hidden.
    pub fn toggle_layer(&mut self, layer: Layer) {
        let rows = layers::of_layer(&self.scene, layer);

        if rows.is_empty() {
            return;
        }

        // anything still visible: hide the whole layer
        let hide = rows.iter().any(|&row| {
            self.scene
                .identity_of(row)
                .is_some_and(|id| !self.scene.hidden.contains(&id))
        });
        self.set_rows_hidden(&rows, hide);
    }

    /// Refill the layers panel, when it is open.
    pub fn refresh_layers(&mut self) {
        if !crate::app::feedback::layers_open() {
            return;
        }

        self.hierarchy.refresh(&self.scene);
        // --8<-- [start:step-30e]
        let mut rows = Vec::new();
        // --8<-- [end:step-30e]
        self.hierarchy_labels(&mut rows);
        crate::app::feedback::layers_panel(&rows);
    }
}

impl State {
    /// L: open or close the layers panel.
    pub fn toggle_layers_panel(&mut self) {
        let open = !crate::app::feedback::layers_open();
        crate::app::feedback::layers_visible(open);

        if open {
            self.refresh_layers();
        }

        self.touch();
    }
}

/// A control point drag in progress.
pub struct ControlDrag {
    parent: u32,   // the object's row
    index: usize,  // which dot in `controls.points`
    id: ControlId, // which control in the geometry
    plane: CPlane, // the plane the point moves in
    origin: Point, // where the point was at the grab
}

impl State {
    /// Grab the selected control point, if the press is on it.
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
        let origin = Point::new(at[0], at[1], at[2]).transformed(&place);
        let Some((sx, sy)) = self.project([origin[0], origin[1], origin[2]]) else {
            return false;
        };
        let grab = GRAB_CSS * self.pixel_scale(); // grab radius in device pixels

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

    /// Move the control point with the pointer; a preview.
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
            .and_then(|place| place.inverse())
        else {
            return false;
        };
        let point = point.transformed(&back); // into the object's own frame
        let index = active.index;
        self.controls.points[index].position = [point[0], point[1], point[2]];
        self.upload_controls();
        self.touch();
        true
    }

    /// Release: the geometry takes the moved control point.
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

        if let Err(error) = self
            .scene
            .set_source_control(active.parent, active.id, &point)
        {
            self.status(&error);
            return false;
        }

        self.scene.rebuild(&mut self.gpu);
        self.selection = SelectionMode::Object;
        self.select(Some(active.parent));
        self.enable_controls();
        self.touch();
        true
    }

    /// Where the dragged control point lands: a snap, or the plane.
    fn control_target(&self, active: &ControlDrag, x: f64, y: f64) -> Option<Point> {
        let (from, dir) = self.camera.ray((x, y), self.viewport())?;
        let free = active.plane.hit(&active.origin, &from, &dir)?; // the plane point
        let place = self.scene.placement_of(active.parent)?;
        let mut candidates = Vec::new();

        // the other control points are snap targets
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

        // nearest on screen, within the aperture
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
        // anchored at the point, its clip position is the translation
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

/// Grab radius of a control dot, CSS pixels.
const GRAB_CSS: f64 = 10.0;

/// Snap reach, CSS pixels.
const SNAP_APERTURE_PX: f64 = 12.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// The widget draws three colored arms.
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
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));
        gpu.set_scene(&upload);

        let mut camera = Camera::new();
        camera.target = [0.0, 0.0, 0.0];
        camera.distance = 0.2; // meters: 200 scene units
        camera.update_position();
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::BLACK,
            now_ms: 0.0,
        };
        let before = gpu.render_offscreen(&input);

        // 96 pixels at 0.5 units per pixel fits the view
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

    /// A CSS pixel is the same scene length on a 1x and a 2x display.
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
        // the answer scales with the distance
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

impl State {
    /// Reselect `row` after a rebuild, keeping the face, edge or control mode.
    fn restore_edit_selection(&mut self, row: u32) {
        let selection = self.selection.clone();
        self.select(Some(row)); // resets the mode
        self.selection = selection; // put it back

        match self.selection {
            SelectionMode::Controls { .. } => {
                self.gpu.set_selected(row, false);

                if let Some(geometry) = self.scene.geometry(row) {
                    self.controls = crate::app::selection::Controls::from_geometry(geometry);
                }

                self.upload_controls();
            }
            SelectionMode::Face { face, .. } => {
                self.gpu.set_selected(row, false);
                let address = self.gpu.arena.source_faces.address(row, face);
                self.gpu.arena.source_faces.select(&self.gpu.ctx, address);
            }
            SelectionMode::Edge { edge, .. } => {
                self.gpu.set_selected(row, false);
                self.gpu.segments.set_edge(&self.gpu.ctx, Some((row, edge)));
            }
            SelectionMode::Object => {}
        }

        self.place_gizmo(Some(row));
        self.refresh_layers();
        self.touch();
    }
}
// --8<-- [start:step-30f]

impl State {
    /// Draw `row` from its document geometry again, dropping any preview.
    fn restore_source_render(&mut self, row: u32) {
        let geometry = self.scene.geometry(row).cloned();
        // patch in place when possible, else rebuild everything

        if !geometry.is_some_and(|geometry| self.scene.patch_preview(row, &geometry, &mut self.gpu))
        {
            self.scene.rebuild(&mut self.gpu);
        }
    }
}
// --8<-- [end:step-30f]
