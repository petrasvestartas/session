use super::Scene;
use crate::engine::gpu::Gpu;
use crate::engine::text::{TextLabel, TextObject, TextPlacement};
use session_rust::Xform;

/// Key prefix of a text made with the Text command.
pub(crate) const CREATED_TEXT: &str = "created-text/";

/// One text object and its scene row.
pub struct SceneText {
    pub label: TextLabel,    // what to draw
    pub row: u32,            // its object row
    pub(crate) key: String,  // stable name, e.g. manifest-text/3
    pub(crate) name: String, // optional independent name for linked highlighting
    pub(crate) active: bool, // false once replaced or its presentation step is hidden
}

impl Scene {
    /// Replace the manifest's texts, keeping their rows.
    pub fn set_texts(&mut self, texts: Vec<super::super::manifest::TextItem>, gpu: &mut Gpu) {
        for text in &mut self.texts {
            if text.key.starts_with("manifest-text/") {
                text.active = false;
            }
        }
        self.append_texts(texts, "manifest-text", gpu);
    }

    /// Add a named group of texts without replacing earlier groups.
    pub fn append_texts(
        &mut self,
        texts: Vec<super::super::manifest::TextItem>,
        namespace: &str,
        gpu: &mut Gpu,
    ) {
        for (index, text) in texts.into_iter().enumerate() {
            let fixed_spatial = text.spatial && text.camera_facing && text.fixed;
            let placement = if fixed_spatial {
                TextPlacement::Anchor {
                    world: text.at,
                    offset: text.offset,
                }
            } else if text.camera_facing {
                TextPlacement::WorldBillboard {
                    world: text.at,
                    world_height: text.height,
                }
            } else {
                TextPlacement::WorldPlane {
                    world: text.at,
                    right: text.right,
                    up: text.up,
                    world_height: text.height,
                    plate: text.plate,
                }
            };
            let font_size = if fixed_spatial {
                (text.height * crate::app::math::SPATIAL_FONT_SCALE) as f32
            } else {
                18.0
            };
            let key = format!("{namespace}/{index}");
            self.register_named_text(
                key.clone(),
                TextLabel {
                    id: (if text.serif {
                        crate::engine::text::MATH_SERIF_ID
                    } else {
                        0
                    }) | (if text.spatial {
                        crate::engine::text::SPATIAL_TEXT_ID
                    } else {
                        0
                    }),
                    object: None,
                    text: text.text,
                    font_size,
                    line_height: font_size * 26.0 / 18.0,
                    color: text
                        .color
                        .unwrap_or(if text.plate { [255; 4] } else { [0, 0, 0, 255] }),
                    outline: text.outline,
                    placement,
                    clip: None,
                },
                true,
                text.name,
            );
        }

        self.upload_to(gpu);
        self.restore_text_visibility(gpu);
    }

    /// Add a document title as a text object.
    pub fn set_document_title(&mut self, label: TextLabel, gpu: &mut Gpu) {
        self.register_text(
            format!("document-title/{}", self.docs.len() - 1),
            label,
            true,
        );
        self.upload_to(gpu);
        self.restore_text_visibility(gpu);
    }

    /// Give the titles of the matrices just walked their text rows.
    pub fn add_matrix_titles(&mut self, gpu: &mut Gpu) {
        let titles = std::mem::take(&mut self.matrix_titles);

        if titles.is_empty() {
            return;
        }

        for (key, label) in titles {
            self.register_text(key, label, true);
        }

        self.upload_to(gpu);
        self.restore_text_visibility(gpu);
    }

    /// Reuse the row for `key`, or add a new one.
    pub(crate) fn register_text(&mut self, key: String, label: TextLabel, active: bool) {
        self.register_named_text(key, label, active, String::new());
    }

    /// Register text with an optional link name.
    fn register_named_text(
        &mut self,
        key: String,
        mut label: TextLabel,
        active: bool,
        name: String,
    ) {
        for text in &mut self.texts {
            if text.key == key {
                label.id = text.row + 1 | (label.id & crate::engine::text::TEXT_ID_FLAGS);
                label.object = Some(TextObject {
                    row: text.row,
                    selected: false,
                });
                text.label = label;
                text.active = active;
                text.name = name;
                return;
            }
        }

        let row = self.push_row(super::rows::TEXT, &key, Xform::identity(), 0);
        label.id = row + 1 | (label.id & crate::engine::text::TEXT_ID_FLAGS);
        label.object = Some(TextObject {
            row,
            selected: false,
        });
        self.texts.push(SceneText {
            label,
            row,
            key,
            name,
            active,
        });
    }

    /// Add a text object as one undo step; returns its row.
    pub fn add_text(&mut self, label: TextLabel) -> u32 {
        let next = self
            .texts
            .iter()
            .filter_map(|text| text.key.strip_prefix(CREATED_TEXT)?.parse::<u64>().ok())
            .max()
            .map_or(0, |index| index + 1);
        let key = format!("{CREATED_TEXT}{next}");
        let loaded = self.loaded; // an edit keeps the GPU anchor, only a load re-centres it
        self.register_text(key.clone(), label, true);
        self.loaded = loaded;
        self.text_edited(format!("+{key}")); // register:editing
        let row = self.texts[self.texts.len() - 1].row;
        self.text_rows.push(row);
        row
    }

    /// Retire the active text on `row` as one undo step; false when no text is there.
    pub(crate) fn delete_text(&mut self, row: u32) -> bool {
        let Some(label) = self.retire_text(row) else {
            return false;
        };
        self.text_edited(label); // register:editing
        true
    }

    /// Retire the active text on `row`; its undo label, None when no text is there.
    pub(crate) fn retire_text(&mut self, row: u32) -> Option<String> {
        let text = self
            .texts
            .iter_mut()
            .find(|text| text.active && text.row == row)?;
        text.active = false;
        let label = format!("-{}", text.key);
        self.text_rows.push(row);
        self.selected = None;
        Some(label)
    }

    /// Show or hide the text an undo step `label` made or deleted.
    pub(crate) fn step_text(&mut self, label: &str, back: bool) {
        let (sign, key) = label.split_at(1);
        let shown = (sign == "+") != back;

        let Some(text) = self.texts.iter_mut().find(|text| text.key == key) else {
            return;
        };
        text.active = shown;
        let row = text.row;
        self.text_rows.push(row);

        if !shown && self.selected == Some(row) {
            self.selected = None;
        }
    }

    /// Hide or show on the GPU the texts undo, redo or delete changed.
    pub(crate) fn flag_texts(&mut self, gpu: &mut Gpu) {
        for row in std::mem::take(&mut self.text_rows) {
            let hidden = self
                .identity_of(row)
                .is_some_and(|id| self.hidden.contains(&id));
            gpu.set_hidden(row, self.text_at(row).is_none() || hidden);
        }
    }

    /// Hide inactive and hidden texts on the GPU.
    pub(crate) fn restore_text_visibility(&self, gpu: &mut Gpu) {
        for text in &self.texts {
            let hidden = self
                .identity_of(text.row)
                .is_some_and(|id| self.hidden.contains(&id));
            gpu.set_hidden(text.row, !text.active || hidden);
        }
    }

    /// The words a text row shows.
    pub fn text_name(&self, row: u32) -> Option<&str> {
        self.text_at(row).map(|text| text.label.text.as_str())
    }

    /// The active text on `row`.
    #[expect(
        clippy::manual_find,
        reason = "Scene identity lookup deliberately uses explicit control flow"
    )]
    pub fn text_at(&self, row: u32) -> Option<&SceneText> {
        for text in &self.texts {
            if text.active && text.row == row {
                return Some(text);
            }
        }
        None
    }

    /// A text's independent link name, falling back to its words.
    pub fn text_link_name(&self, row: u32) -> Option<&str> {
        let text = self.texts.iter().find(|text| text.row == row)?;
        Some(if text.name.is_empty() {
            &text.label.text
        } else {
            &text.name
        })
    }

    /// True for a text's scene row, even if its step is currently hidden.
    pub fn is_text(&self, row: u32) -> bool {
        self.texts.iter().any(|text| text.row == row)
    }

    /// Change an existing label without changing its row, name or selection state.
    pub fn update_text_label(&mut self, row: u32, label: TextLabel) -> bool {
        let Some(text) = self.texts.iter_mut().find(|text| text.row == row) else {
            return false;
        };
        text.label = label;
        true
    }

    /// Set a presentation text's reveal scale without reshaping its glyphs.
    pub fn set_text_scale(&mut self, row: u32, scale: f32) {
        self.text_scales.insert(row, scale.max(0.0));
    }

    /// Fade a presentation text without rerasterizing its glyphs.
    pub fn set_text_opacity(&mut self, row: u32, opacity: f32) {
        self.text_opacities.insert(row, opacity.clamp(0.0, 1.0));
    }

    /// Every visible text label, with its selection flag and presentation effect.
    pub fn visible_texts(&self) -> Vec<TextLabel> {
        let mut labels = Vec::new();

        for text in &self.texts {
            if !text.active
                || self
                    .identity_of(text.row)
                    .is_some_and(|id| self.hidden.contains(&id))
            {
                continue;
            }

            let scale = self.text_scales.get(&text.row).copied().unwrap_or(1.0);
            let opacity = self.text_opacities.get(&text.row).copied().unwrap_or(1.0);
            if scale * opacity <= 0.0001 {
                continue;
            }
            let mut label = text.label.clone();
            let linked = self.selected == Some(text.row) || self.linked.contains(&text.row);
            if linked && text.name.contains('[') {
                label.color = [255; 4];
            }
            label.object = Some(TextObject {
                row: text.row,
                selected: linked && !text.name.contains('['),
            });
            label.color[3] = (f32::from(label.color[3]) * opacity).round() as u8;
            if scale != 1.0 {
                match &mut label.placement {
                    TextPlacement::WorldPlane { world_height, .. }
                    | TextPlacement::WorldBillboard { world_height, .. } => {
                        *world_height *= f64::from(scale)
                    }
                    TextPlacement::Anchor { offset, .. } => {
                        label.font_size *= scale;
                        label.line_height *= scale;
                        offset[0] *= scale;
                        offset[1] *= scale;
                    }
                    TextPlacement::Screen { .. } | TextPlacement::Nameplate { .. } => {}
                }
            }
            if label.font_size < 1.0 {
                continue;
            }
            labels.push(label);
        }

        labels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_math_text_preserves_its_font_on_first_registration() {
        let mut scene = Scene::new();
        let label = TextLabel {
            id: crate::engine::text::MATH_SERIF_ID,
            object: None,
            text: "-1".into(),
            font_size: 18.0,
            line_height: 26.0,
            color: [255; 4],
            outline: 0.0,
            placement: TextPlacement::WorldPlane {
                world: [0.0; 3],
                right: [1.0, 0.0, 0.0],
                up: [0.0, 1.0, 0.0],
                world_height: 1.0,
                plate: false,
            },
            clip: None,
        };
        scene.register_text("math-entry".into(), label.clone(), true);
        assert_ne!(
            scene.texts[0].label.id & crate::engine::text::TEXT_ID_FLAGS,
            0
        );
        scene.register_text("math-entry".into(), label, true);
        assert_ne!(
            scene.texts[0].label.id & crate::engine::text::TEXT_ID_FLAGS,
            0
        );
    }

    #[test]
    fn fixed_spatial_math_uses_anchor_offsets_and_pixel_sizes() {
        let item = crate::app::manifest::TextItem {
            text: "n_0".into(),
            math: true,
            serif: false,
            center: true,
            center_ink: true,
            fixed: true,
            max_width: 0.0,
            name: "n0-label".into(),
            outline: 0.0,
            at: [3.3, 3.0, 0.57],
            offset: [0.0; 2],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            height: 18.0,
            camera_facing: true,
            spatial: true,
            plate: false,
            color: Some([255, 77, 77, 255]),
        };
        let output = crate::app::math::typeset(&[item]);
        assert!(
            output
                .texts
                .iter()
                .all(|text| text.spatial && text.camera_facing)
        );
        let Ok(mut gpu) = pollster::block_on(crate::engine::gpu::Gpu::new_headless(64, 64)) else {
            return;
        };
        let mut scene = Scene::new();
        scene.append_texts(output.texts, "test", &mut gpu);
        let labels = scene.visible_texts();
        assert!(!labels.is_empty());
        for label in &labels {
            assert_ne!(label.id & crate::engine::text::SPATIAL_TEXT_ID, 0);
            assert!(matches!(label.placement, TextPlacement::Anchor { .. }));
        }
        let main = labels.iter().find(|label| label.text == "n").unwrap();
        let subscript = labels.iter().find(|label| label.text == "0").unwrap();
        assert_eq!(main.font_size, 18.0);
        assert!((subscript.font_size - 12.6).abs() < 1e-5);
        assert!(labels.iter().any(|label| {
            let TextPlacement::Anchor { offset, .. } = label.placement else {
                return false;
            };
            offset[0].abs() > 1e-5 || offset[1].abs() > 1e-5
        }));
        let row = scene.texts[0].row;
        scene.set_text_scale(row, 0.01);
        let labels = scene.visible_texts();
        assert!(labels.iter().all(|label| label.font_size >= 1.0));
        assert!(
            labels
                .iter()
                .all(|label| { label.object.is_none_or(|object| object.row != row) })
        );
        scene.set_text_scale(row, 1.0);
        scene.set_text_opacity(row, 0.0);
        assert!(
            scene
                .visible_texts()
                .iter()
                .all(|label| label.object.is_none_or(|object| object.row != row))
        );
        scene.set_text_opacity(row, 0.5);
        assert!(
            scene
                .visible_texts()
                .iter()
                .filter(|label| label.object.is_some_and(|object| object.row == row))
                .all(|label| label.color[3] == 128)
        );
    }
}
