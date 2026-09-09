//! Scene-text presentation: source labels, the derived selection name, and fit bounds.

use super::State;
use crate::app::selection::SelectionMode;
use crate::engine::text::{TextLabel, TextPlacement};

impl State {
    /// Keep one readable source-document title above each loaded CAD group.
    /// Only the newly appended rows are visited; large scenes are not scanned on selection.
    pub(super) fn annotate_document(&mut self, first_row: usize) {
        let mut bounds = crate::math::Aabb::empty();
        for index in first_row..self.scene.object_count() {
            let row = index as u32;
            if matches!(
                self.scene.geometry(row),
                Some(session_rust::Geometry::BRep(_) | session_rust::Geometry::NurbsSurface(_))
            ) && let Some(object) = self.gpu.objects.row_bounds(row)
            {
                bounds.union(&object);
            }
        }
        if !bounds.is_finite() {
            return;
        }
        let Some(document) = self.scene.docs.last() else {
            return;
        };
        let mut anchor = label_center(&bounds);
        anchor[2] = f64::from(bounds.max[2]) + f64::from(bounds.diagonal()) * 0.08;
        let label = nameplate(1, document.name.clone(), anchor);
        self.scene.set_document_title(label, &mut self.gpu);
    }

    /// Source-document text and the selected object's centered name share a fixed white style.
    pub(super) fn update_label(&mut self) {
        let mut labels = self.scene.visible_texts();
        if self.show_selected_names
            && !matches!(self.selection, SelectionMode::Controls { .. })
            && let Some(row) = self.scene.selected
            && let Some(bounds) = self.gpu.objects.row_bounds(row)
        {
            // Only this derived annotation has no selectable source row.
            labels.push(nameplate(
                0,
                self.scene.object_name(row).to_string(),
                label_center(&bounds),
            ));
        }
        if let Err(error) = self.gpu.text.set_labels(labels) {
            self.status(&format!("Text: {error}"));
        }
        self.include_text_bounds();
    }

    /// Include authored text planes in camera fitting using shaped extents and their fixed frame.
    pub(super) fn include_text_bounds(&mut self) {
        for run in &self.gpu.text.document.runs {
            let (world, right, up, world_height) = match run.label.placement {
                TextPlacement::WorldPlane {
                    world,
                    right,
                    up,
                    world_height,
                } => (world, right, up, world_height),
                TextPlacement::WorldBillboard {
                    world,
                    world_height,
                } => (world, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], world_height),
                TextPlacement::Nameplate { world, .. } | TextPlacement::Anchor { world, .. } => (
                    world,
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    f64::from(self.gpu.bounds.diagonal()).max(1.0) * 0.002,
                ),
                TextPlacement::Screen { .. } => continue,
            };
            if run.label.object.is_none() {
                continue;
            }
            let unit = world_height / f64::from(run.label.font_size);
            let mut width = 0.0f64;
            let mut height = 0.0f64;
            for line in run.buffer.layout_runs() {
                width = width.max(f64::from(line.line_w) * unit);
                height = height.max(f64::from(line.line_top + line.line_height) * unit);
            }
            let padding = world_height * 0.125;
            let mut bounds = crate::math::Aabb::empty();
            for x in [-padding, width + padding] {
                for y in [-padding, height + padding] {
                    let mut point = [0.0f32; 3];
                    for axis in 0..3 {
                        point[axis] = (world[axis] + right[axis] * x - up[axis] * y) as f32;
                    }
                    if point.into_iter().all(f32::is_finite) {
                        if matches!(
                            run.label.placement,
                            TextPlacement::WorldPlane { .. } | TextPlacement::WorldBillboard { .. }
                        ) {
                            self.gpu.bounds.grow(point);
                        }
                        bounds.grow(point);
                    }
                }
            }
            if let Some(object) = run.label.object {
                self.gpu.objects.set_text_bounds(object.row, bounds);
            }
        }
    }
}

/// Center a viewer annotation in the object's world-space bounds before camera projection.
fn label_center(bounds: &crate::math::Aabb) -> [f64; 3] {
    let mut center = [0.0; 3];
    for (axis, coordinate) in center.iter_mut().enumerate() {
        *coordinate = (f64::from(bounds.min[axis]) + f64::from(bounds.max[axis])) * 0.5;
    }
    center
}

/// White-on-black annotations; selection ID 0 uses 75% of the document title size.
fn nameplate(id: u32, text: String, world: [f64; 3]) -> TextLabel {
    let scale = if id == 0 { 0.75 } else { 1.0 };
    let line_height = 26.0 * scale;
    let vertical_padding = 4.0 * scale;
    // A full cap radius at each end keeps the entire text line inside the straight section.
    let horizontal_padding = if id == 0 {
        line_height * 0.5 + vertical_padding
    } else {
        6.0 * scale
    };
    TextLabel {
        object: None,
        id,
        text,
        font_size: 18.0 * scale,
        line_height,
        color: [255; 4],
        placement: TextPlacement::Nameplate {
            world,
            padding: [horizontal_padding, vertical_padding],
            rounded: id == 0,
        },
        clip: None,
    }
}
