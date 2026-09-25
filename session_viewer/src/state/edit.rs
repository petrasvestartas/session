use crate::app::cplane::CPlane;
use crate::app::gizmo::{Axis, Drag, Gizmo, Handle};
use crate::app::layers::{self, Layer};
use crate::app::selection::ControlId;
use crate::app::snap::{self, Snap, SnapKind};
use crate::state::{SelectionMode, State};
use session_rust::{Point, Vector, Xform};

/// A gizmo drag in progress.
pub struct GizmoDrag {
    row: u32,                                                // the main selected row
    group: Vec<(u32, Xform)>, // every selected row and where it started
    base_place: Xform,        // the main row's placement at the grab
    drag: Drag,               // the handle and where it was grabbed
    target: Option<crate::app::deform::Target>, // a face, edge or control point being moved
    source: Option<session_rust::Geometry>, // the geometry before the drag
    origin: Point,            // the gizmo center at the grab
    mesh_preview: Option<crate::app::mesh_preview::Gesture>, // a GPU-side mesh preview
}

impl State {
    /// Put the gizmo at the center of the selection, or remove it.
    pub fn place_gizmo(&mut self, row: Option<u32>) {
        let row = row.filter(|_| !self.tool_running()); // hidden while a tool asks for points
        // no box, no gizmo
        let Some(box_) = row.and_then(|r| self.gpu.objects.row_bounds(r)) else {
            self.gizmo = None;
            self.upload_gizmo();
            return;
        };
        // the box around every selected row
        let mut bounds = box_;
        if row.is_some() {
            for selected in &self.hierarchy.selected {
                if let Some(b) = self.gpu.objects.row_bounds(*selected) {
                    bounds.union_with(&b);
                }
            }
        }
        let mut origin = bounds.center();

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

    /// Grab a gizmo handle within `radius` CSS pixels; false when the press missed it.
    pub(crate) fn begin_gizmo_with_radius(&mut self, x: f64, y: f64, radius: f64) -> bool {
        let Some(row) = self.scene.selected else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        // every selected row with where it is now
        let group = self
            .selected_rows()
            .into_iter()
            .filter_map(|r| Some((r, self.scene.placement_of(r)?)))
            .collect();
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
        let Some(base_place) = self.scene.placement_of(row) else {
            return false;
        };
        let target = crate::app::deform::Target::selected(&self.selection);

        // a part of the object moves: its previews come from a walk of this row alone
        if target.is_some() {
            self.scene.capture_preview(row);
        }

        // a mesh vertex drag previews on the GPU
        let mesh_preview = target.and_then(|target| {
            self.scene
                .mesh_preview(row)?
                .begin(self.scene.geometry(row)?, target)
        });

        self.dragging = Some(GizmoDrag {
            group,
            row,
            base_place,
            drag,
            target: crate::app::deform::Target::selected(&self.selection),
            source: self.scene.geometry(row).cloned(),
            origin: gizmo.origin.clone(),
            mesh_preview,
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

            if let Some(preview) = active.mesh_preview.as_ref() {
                preview.apply(&mut self.gpu, &local, false);
                let origin = active.origin.transformed(&delta);
                self.gpu
                    .bounds
                    .union_with_point(origin[0], origin[1], origin[2]);

                if let Some(gizmo) = self.gizmo.as_mut() {
                    gizmo.origin = origin;
                }

                self.upload_gizmo();
                self.touch();
                return true;
            }

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

            self.update_label();

            if let Some(gizmo) = self.gizmo.as_mut() {
                gizmo.origin = origin;
            }

            self.upload_gizmo();
            self.touch();
            return true;
        }

        // whole objects: move every row's placement
        for (row, base) in &active.group {
            self.gpu
                .objects
                .set_placement(&self.gpu.ctx, *row, &(&delta * base));
            self.gpu.grew_bounds(*row);
        }
        self.place_gizmo(Some(active.row));
        self.update_label();
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
        for (row, base) in &active.group {
            self.gpu.objects.set_placement(&self.gpu.ctx, *row, base);
            self.gpu.grew_bounds(*row);
        }
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

            // the document draws the committed geometry; a refused edit puts the source back
            match &result {
                Ok(()) => self.commit_rows(),
                Err(_) => self.restore_source_render(active.row),
            }

            self.scene.drop_preview();
            self.restore_edit_selection(active.row);

            if let Err(error) = result {
                self.status(&error);
                return false;
            }

            self.refresh_layers();
            self.touch();
            return true;
        }

        // the undo label
        let label = match active.drag.handle {
            Handle::Translate(_) => "move",
            Handle::Rotate(_) => "rotate",
            Handle::Scale(_) | Handle::ScaleUniform => "scale",
        };
        if let Err(error) = self.apply(delta, label) {
            self.status(&error);
            return false;
        }

        self.touch();
        true
    }

    /// A click on a handle: nothing moves, its number box opens.
    pub fn click_gizmo(&mut self) -> bool {
        let Some(handle) = self.dragging.as_ref().map(|active| active.drag.handle) else {
            return false;
        };
        self.cancel_gesture();
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        gizmo.typing = Some(handle);
        self.upload_gizmo();
        let (_, _, unit) = handle.labels();
        self.status(&format!(
            "{}: type a value in {unit}, Enter applies, Esc closes",
            handle.title()
        ));
        self.touch();
        true
    }

    /// Drop a drag that will never be released; everything goes back.
    pub fn cancel_gesture(&mut self) {
        self.cancel_object_drag();
        self.tool_abandon(); // a running tool's drag, e.g. a lasso loop

        if let Some(active) = self.dragging.take() {
            if let Some(preview) = active.mesh_preview.as_ref() {
                preview.apply(&mut self.gpu, &Xform::identity(), true);
                self.scene.drop_preview();
                self.restore_edit_selection(active.row);
            } else if active.target.is_some() {
                self.restore_source_render(active.row);
                self.scene.drop_preview();
                self.restore_edit_selection(active.row);
            }

            self.gpu
                .objects
                .set_placement(&self.gpu.ctx, active.row, &active.base_place);

            if let Some(gizmo) = self.gizmo.as_mut() {
                gizmo.drag = None;
            }

            for (row, base) in &active.group {
                self.gpu.objects.set_placement(&self.gpu.ctx, *row, base);
                self.gpu.grew_bounds(*row);
            }
            self.place_gizmo(Some(active.row));
            self.update_label();
            self.touch();
        }

        if let Some(active) = self.control_drag.take() {
            self.restore_source_render(active.parent);
            self.scene.drop_preview();
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

        if let Some(reason) = self.locked_reason(&[row]) {
            self.status(&reason);
            return;
        }

        if !self.scene.delete_row(row) {
            self.status("This object cannot be deleted");
            return;
        }

        self.after_history();
    }

    /// Ctrl+Z: undo the last edit.
    pub fn undo(&mut self) {
        if self.scene.undo() {
            self.after_history();
        }
    }

    /// Ctrl+Y: redo it.
    pub fn redo(&mut self) {
        if self.scene.redo() {
            self.after_history();
        }
    }

    /// After an undo, redo or delete: sync the rows, drop the selection.
    pub(crate) fn after_history(&mut self) {
        // a tool's preview and bases belong to the documents as they were
        if self.tool_running() {
            self.cancel_drawing();
        }

        self.hierarchy.page = 0;
        self.selection = SelectionMode::Object;
        self.select(None);
        self.scene.flag_texts(&mut self.gpu);
        self.commit_rows();
        self.place_gizmo(None);
    }

    /// Bring the rows in line with the documents after an edit; costs what the edit changed.
    pub(crate) fn commit_rows(&mut self) {
        self.scene.sync();
        self.scene.upload_to(&mut self.gpu);
        self.purge_idle();
        let gesture = self.dragging.is_some() || self.control_drag.is_some();

        // dead rows outweigh the live ones: walk the lanes again, ids stay
        if !gesture && self.scene.compaction_due() {
            if self.scene.rewalk_editable(&mut self.gpu) {
                self.reselect_face();
            } else {
                self.resume_after(super::hydrate::Resume::Rewalk);
            }
        }

        if self.scene.cloud_compaction_due(&self.gpu) {
            self.scene.compact_clouds(&mut self.gpu);
        }

        // a selected, controlled or split row is gone
        let scene = &self.scene;
        let gone = |row: &u32| scene.identity_of(*row).is_none();
        let split = self
            .pending_split
            .as_ref()
            .is_some_and(|split| gone(&split.target) || split.cutters.iter().any(gone));
        let lost = self.selected_rows().iter().any(gone)
            || self.selection.parent().is_some_and(|row| gone(&row));
        let rows: Vec<u32> = self
            .selected_rows()
            .into_iter()
            .filter(|row| !gone(row))
            .collect();

        if split {
            self.cancel_split();
        }

        // the survivors stay selected; a freed id keeps no controls
        if lost {
            self.select_rows(rows, false);
        }

        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// Select the chosen source face again after its rows moved.
    pub(super) fn reselect_face(&mut self) {
        if let SelectionMode::Face { parent, face } = self.selection {
            let address = self.gpu.arena.source_faces.address(parent, face);
            self.gpu.arena.source_faces.select(&self.gpu.ctx, address);
        }
    }

    /// Scene length of one CSS pixel at the gizmo.
    pub(super) fn world_per_px(&self) -> f64 {
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

    /// Device pixels per CSS pixel: 1 on a monitor, 2 or more on a phone.
    pub(crate) fn pixel_scale(&self) -> f64 {
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
        // the dragged handle, else the one being typed for, else the hovered one
        let handle = self
            .dragging
            .as_ref()
            .map(|drag| drag.drag.handle)
            .or(gizmo.typing)
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
        let line = &crate::app::command::canonical(line); // `poly line` runs Polyline
        self.cancel_gesture();
        self.mark = None;
        // while drawing, points and Enter go to the draft
        if let Some(result) = self.drawing_command(line) {
            return result;
        }
        let action = crate::app::command::parse(line)?;

        if !action.keeps_draft() {
            self.cancel_drawing();
        }

        if !action.keeps_split() {
            self.cancel_split();
        }

        // Rhino-like: pick the objects first, Enter runs the command
        if action.needs_selection() && self.scene.selected.is_none() {
            return self.ask_for_objects(line);
        }

        action.run(self)
    }

    /// Apply one transform to the selection and record it.
    pub(crate) fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String> {
        let Some(row) = self.scene.selected else {
            return Err("nothing is selected".into());
        };

        if let Some(reason) = self.locked_reason(&self.selected_rows()) {
            return Err(reason);
        }

        // a face, edge or control point moves inside the object
        if let Some(target) = crate::app::deform::Target::selected(&self.selection) {
            self.scene.edit_subobject(row, target, &delta, label)?;
            self.commit_rows();
            self.restore_edit_selection(row);
            self.touch();
            return Ok(label.into());
        }

        // whole objects: the document moves them and every object below, the rows follow
        let rows = self.selected_rows();
        self.scene
            .transform_rows(&rows, &delta, label)
            .ok_or("this selection cannot be edited")?;
        self.commit_rows();
        self.place_gizmo(Some(row));
        self.update_label();
        self.touch();
        Ok(label.into())
    }
}

/// A rotation about a point.
pub(crate) fn rotation_about(axis: Axis, degrees: f64, about: Option<&Point>) -> Xform {
    let turn = match axis {
        Axis::X => Xform::rotation_x(degrees, true),
        Axis::Y => Xform::rotation_y(degrees, true),
        Axis::Z => Xform::rotation_z(degrees, true),
    };
    centred(turn, about)
}

/// A uniform scale about a point.
pub(crate) fn scaling_about(factor: f64, about: Option<&Point>) -> Xform {
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
        let mut rows = Vec::new();
        self.hierarchy_labels(&mut rows);
        crate::app::feedback::layers_panel(&rows);
        let selected = self.selected_rows(); // sorted
        let guid = |row| {
            self.scene
                .identity_of(row)
                .map(|id| id.1)
                .unwrap_or_default()
        };
        // rows only while the table is unfolded
        let open = crate::app::feedback::graph_open();
        let edges = self
            .hierarchy
            .edges
            .iter()
            .take(if open { MAX_EDGE_ROWS } else { 0 })
            .map(|&[from, to]| crate::app::feedback::EdgeRow {
                key: format!("pair/{from}/{to}"),
                from: edge_label(&self.scene, from),
                to: edge_label(&self.scene, to),
                guids: format!("From {}\nTo {}", guid(from), guid(to)), // the panel font has no arrow
                selected: selected.binary_search(&from).is_ok()
                    && selected.binary_search(&to).is_ok(),
            })
            .collect();
        crate::app::feedback::graph_panel(edges, self.hierarchy.edges.len());
    }
}

/// An edge end in the graph table: the object's own name, else the start of its guid.
fn edge_label(scene: &crate::app::scene::Scene, row: u32) -> String {
    match scene.geometry(row).map(session_rust::Geometry::name) {
        Some(name) if !name.trim().is_empty() => name.to_string(),
        _ => scene
            .identity_of(row)
            .map(|id| id.1.chars().take(8).collect())
            .unwrap_or_default(),
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
        self.scene.capture_preview(parent);
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
        let parent = active.parent;
        let id = active.id;
        // preview the geometry with the point moved
        if let Some(source) = self.scene.geometry(parent) {
            let target = crate::app::deform::Target::Control(id);
            if let Ok(points) = crate::app::deform::points(source, target)
                && let Some(from) = points.first()
                && let Ok(edited) = crate::app::deform::transform(
                    source,
                    target,
                    &Xform::translation(point[0] - from[0], point[1] - from[1], point[2] - from[2]),
                )
            {
                let _ = self.scene.preview_geometry(parent, edited, &mut self.gpu);
            }
        }
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
        let result = match self.control_target(&active, x, y) {
            Some(point) => self
                .scene
                .set_source_control(active.parent, active.id, &point),
            None => Err(String::new()),
        };

        // the document draws the committed geometry; a refused edit puts the source back
        match &result {
            Ok(()) => self.commit_rows(),
            Err(_) => self.restore_source_render(active.parent),
        }

        self.scene.drop_preview();

        if let Some(geometry) = self.scene.geometry(active.parent) {
            self.controls = crate::app::selection::Controls::from_geometry(geometry);
        }

        self.upload_controls();
        self.touch();

        if let Err(error) = result {
            if !error.is_empty() {
                self.status(&error);
            }

            return false;
        }

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
        if !self.snap_enabled {
            return Some(free);
        }
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
            self.snap_modes,
            (x, y),
            SNAP_APERTURE_PX * self.pixel_scale(),
            project,
        ) {
            Some(hit) => Some(hit.point),
            None => Some(free),
        }
    }

    /// A scene point in device pixels, or `None` behind the eye.
    pub(super) fn project(&self, at: [f64; 3]) -> Option<(f64, f64)> {
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

const MAX_EDGE_ROWS: usize = 5000; // graph edges listed in the panel

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

    /// The graph table shows an object's name as the tree does, and a short guid when it has none.
    #[test]
    fn an_edge_end_is_named_like_its_tree_row() {
        use crate::app::scene::{FileDoc, Scene};
        use session_rust::Session;
        use std::rc::Rc;

        let mut session = Session::new("site");
        session.add_point(Point::new(0.0, 0.0, 0.0), None);
        let mut blank = Point::new(1.0, 0.0, 0.0);
        blank.name = String::new();
        session.add_point(blank, None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        assert_eq!(edge_label(&scene, 0), "my_point");
        let guid = scene.identity_of(1).unwrap().1;
        assert_eq!(edge_label(&scene, 1), guid[..8]);
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
    /// Reselect `row` after its rows were redrawn, keeping the face, edge or control mode.
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

impl State {
    /// Draw `row` from its document geometry again, dropping any preview.
    fn restore_source_render(&mut self, row: u32) {
        if let Some(geometry) = self.scene.geometry(row).cloned() {
            self.scene.redraw(row, &geometry, false);
            self.scene.upload_to(&mut self.gpu);
        }
    }
}
