use super::State;
use crate::app::selection::SelectionMode;
use crate::engine::text::{TextLabel, TextPlacement};
use session_rust::AABB;

impl State {
    /// Put the document's name above its CAD objects.
    pub(super) fn annotate_document(&mut self, first_row: usize) {
        let mut bounds = AABB::empty();

        // the box around the new BReps and surfaces
        for index in first_row..self.scene.row_count() {
            let row = index as u32;
            if matches!(
                self.scene.geometry(row),
                Some(session_rust::Geometry::BRep(_) | session_rust::Geometry::NurbsSurface(_))
            ) && let Some(object) = self.gpu.objects.row_bounds(row)
            {
                bounds.union_with(&object);
            }
        }

        if !bounds.is_valid() {
            return;
        }

        let Some(document) = self.scene.docs.last() else {
            return;
        };
        let mut anchor = label_center(&bounds);
        anchor[2] = bounds.cz + bounds.hz + bounds.diagonal() * 0.08; // a little above the top
        let label = nameplate(1, document.name.clone(), anchor);
        self.scene.set_document_title(label, &mut self.gpu);
    }

    /// Draw the labels with the whole fonts, main font first.
    pub fn use_fonts(&mut self, faces: [&'static [u8]; 3]) {
        let sources = faces
            .into_iter()
            .map(|face| glyphon::fontdb::Source::Binary(std::sync::Arc::new(face)))
            .collect();

        if let Err(error) = self.gpu.text.document.replace_sources(sources) {
            self.status(&format!("Text: {error}"));
        }

        self.update_label();
        self.touch();
    }

    /// Send the scene texts plus the selection's name label to the GPU.
    pub(super) fn update_label(&mut self) {
        let mut labels = self.scene.visible_texts();

        // the name of the selected object, at its center
        if self.show_selected_names
            && !matches!(self.selection, SelectionMode::Controls { .. })
            && let Some(row) = self.scene.selected
            && let Some(bounds) = self.gpu.objects.row_bounds(row)
        {
            labels.push(nameplate(
                0,
                self.scene.object_name(row).to_string(),
                label_center(&bounds),
            ));
        }

        for label in &labels {
            crate::app::fonts::need(&label.text);
        }

        if let Err(error) = self.gpu.text.set_labels(labels) {
            self.status(&format!("Text: {error}"));
        }

        self.include_text_bounds();
    }

    /// Grow the scene box and each text row's box around the shaped text.
    pub(super) fn include_text_bounds(&mut self) {
        for run in &self.gpu.text.document.runs {
            // the text position and size in the scene
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
                    self.gpu.bounds.diagonal().max(1.0) * 0.002,
                ),
                TextPlacement::Screen { .. } => continue,
            };

            if run.label.object.is_none() {
                continue;
            }

            let unit = world_height / f64::from(run.label.font_size); // scene units per font pixel
            let mut width = 0.0f64;
            let mut height = 0.0f64;

            // the shaped text size
            for line in run.buffer.layout_runs() {
                width = width.max(f64::from(line.line_w) * unit);
                height = height.max(f64::from(line.line_top + line.line_height) * unit);
            }

            let vertical_padding = world_height * 2.0 / 9.0;
            let horizontal_padding = height * 0.5 + vertical_padding;
            let mut bounds = AABB::empty();

            // the four corners of the padded text
            for x in [-horizontal_padding, width + horizontal_padding] {
                for y in [-vertical_padding, height + vertical_padding] {
                    let mut point = [0.0f32; 3];

                    for axis in 0..3 {
                        point[axis] = (world[axis] + right[axis] * x - up[axis] * y) as f32;
                    }

                    if point.into_iter().all(f32::is_finite) {
                        if matches!(
                            run.label.placement,
                            TextPlacement::WorldPlane { .. } | TextPlacement::WorldBillboard { .. }
                        ) {
                            self.gpu.bounds.union_with_point(
                                f64::from(point[0]),
                                f64::from(point[1]),
                                f64::from(point[2]),
                            );
                        }

                        bounds.union_with_point(
                            f64::from(point[0]),
                            f64::from(point[1]),
                            f64::from(point[2]),
                        );
                    }
                }
            }

            if let Some(object) = run.label.object {
                self.gpu.objects.set_text_bounds(object.row, bounds);
            }
        }
    }
}

/// The center of a box.
fn label_center(bounds: &AABB) -> [f64; 3] {
    [bounds.cx, bounds.cy, bounds.cz]
}

/// A white-on-black label; id 0 is the smaller selection name.
fn nameplate(id: u32, text: String, world: [f64; 3]) -> TextLabel {
    let scale = if id == 0 { 0.75 } else { 1.0 };
    let line_height = 26.0 * scale;
    let vertical_padding = 4.0 * scale;
    // room for the rounded ends
    let horizontal_padding = line_height * 0.5 + vertical_padding;
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
            rounded: true,
        },
        clip: None,
    }
}
