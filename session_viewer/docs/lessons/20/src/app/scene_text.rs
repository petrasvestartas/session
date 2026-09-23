//! Text objects belonging to the document itself, not to the interface around it.
use super::Scene;
use crate::engine::gpu::Gpu;
use crate::engine::text::{TextLabel, TextObject, TextPlacement};
use session_rust::Xform;

/// One text object and its scene row.
pub struct SceneText {
    pub label: TextLabel, // what to draw
    pub row: u32,
    pub(super) key: String,
    pub(super) active: bool,
}

impl Scene {
    /// Replace the manifest's texts, keeping their rows.
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

    /// Reuse the row for this key, else append one.
    pub(super) fn register_text(&mut self, key: String, mut label: TextLabel, active: bool) {
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

        let row = self.push_row(usize::MAX, &key, Xform::identity(), 0);
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

    /// Hide inactive and hidden texts on the GPU.
    pub(super) fn restore_text_visibility(&self, gpu: &mut Gpu) {
        for text in &self.texts {
            let hidden = self
                .identity_of(text.row)
                .is_some_and(|id| self.hidden.contains(&id));
            gpu.set_hidden(text.row, !text.active || hidden);
        }
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
