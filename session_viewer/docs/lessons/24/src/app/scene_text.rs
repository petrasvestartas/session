// --8<-- [start:scene-text]
// Text object = a label the scene owns like any geometry: it has a row, so it can be picked, hidden and undone.
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
    pub(crate) active: bool, // false once replaced
}
// --8<-- [end:scene-text]

// --8<-- [start:manifest-texts]
impl Scene {
    /// Replace the manifest's texts, keeping their rows.
    // `super::super` climbs two modules: from scene::text up to scene, then up to app
    pub fn set_texts(&mut self, texts: Vec<super::super::manifest::TextItem>, gpu: &mut Gpu) {
        // retire the old manifest texts
        for text in &mut self.texts {
            if text.key.starts_with("manifest-text/") {
                text.active = false;
            }
        }

        for (index, text) in texts.into_iter().enumerate() {
            let placement = if text.camera_facing {
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
                }
            };
            self.register_text(
                format!("manifest-text/{index}"),
                TextLabel {
                    id: 0,
                    object: None,
                    text: text.text,
                    font_size: 18.0,
                    line_height: 26.0,
                    color: [255; 4],
                    placement,
                    clip: None,
                },
                true,
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
    // --8<-- [end:manifest-texts]

    // --8<-- [start:register-text]
    /// Reuse the row for `key`, or add a new one.
    pub(crate) fn register_text(&mut self, key: String, mut label: TextLabel, active: bool) {
        for text in &mut self.texts {
            if text.key == key {
                label.id = text.row + 1;
                label.object = Some(TextObject {
                    row: text.row,
                    selected: false,
                });
                text.label = label;
                text.active = active;
                return;
            }
        }

        let row = self.push_row(super::rows::TEXT, &key, Xform::identity(), 0);
        label.id = row + 1;
        label.object = Some(TextObject {
            row,
            selected: false,
        });
        self.texts.push(SceneText {
            label,
            row,
            key,
            active,
        });
    }
    // --8<-- [end:register-text]

    // --8<-- [start:text-undo]
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
        // --8<-- [start:21-text-edited-add]
        self.text_edited(format!("+{key}")); // register:editing
        // --8<-- [end:21-text-edited-add]
        let row = self.texts[self.texts.len() - 1].row;
        self.text_rows.push(row);
        row
    }

    /// Retire the active text on `row` as one undo step; false when no text is there.
    pub(crate) fn delete_text(&mut self, row: u32) -> bool {
        let Some(label) = self.retire_text(row) else {
            return false;
        };
        // --8<-- [start:21-text-edited-delete]
        self.text_edited(label); // register:editing
        // --8<-- [end:21-text-edited-delete]
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
        // an undo label is a sign and a key: "+created-text/0" made it, "-created-text/0" deleted it
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
    // --8<-- [end:text-undo]

    // --8<-- [start:text-lookup]
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

    /// Every visible text label, with its selection flag.
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

            let mut label = text.label.clone();
            label.object = Some(TextObject {
                row: text.row,
                selected: self.selected == Some(text.row),
            });
            labels.push(label);
        }

        labels
    }
}
// --8<-- [end:text-lookup]
