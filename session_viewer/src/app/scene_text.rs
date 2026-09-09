//! Source text registration. Placement never changes object identity or hide/select behavior.
//! Document titles and manifest text use this same path; the selected-name annotation does not.

use super::Scene;
use crate::engine::gpu::{Gpu, ObjectRow};
use crate::engine::text::{TextLabel, TextObject, TextPlacement};
use session_rust::Xform;
use std::rc::Rc;

/// Retained source text and its ordinary scene row. Inactive slots survive replacement so
/// a manifest reload cannot accidentally reassign another text's hidden identity.
pub struct SceneText {
    pub label: TextLabel,
    pub row: u32,
    pub(super) key: String,
    pub(super) active: bool,
}

impl Scene {
    /// Replace manifest-owned text while preserving document titles and stable object rows.
    pub fn set_texts(&mut self, texts: Vec<super::super::manifest::TextItem>, gpu: &mut Gpu) {
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

    /// A camera-facing title is a source object, independent of the geometry it describes.
    pub fn set_document_title(&mut self, label: TextLabel, gpu: &mut Gpu) {
        self.register_text(
            format!("document-title/{}", self.docs.len() - 1),
            label,
            true,
        );
        self.upload_to(gpu);
        self.restore_text_visibility(gpu);
    }

    /// Reuse the row for this source key, or append one ordinary identity/instance pair.
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
        let row = self.bases.obj + self.tables.obj.rows.len() as u32;
        let guid: Rc<str> = Rc::from(key.as_str());
        self.order.push(Rc::clone(&guid));
        self.owners.push(usize::MAX);
        self.guid_to_row.insert((usize::MAX, guid), row);
        self.ribbon_ranges.push(None);
        self.tables
            .obj
            .rows
            .push(ObjectRow::new(Xform::identity().m, 0));
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

    /// Apply source visibility after the instance rows have reached the renderer.
    pub(super) fn restore_text_visibility(&self, gpu: &mut Gpu) {
        for text in &self.texts {
            let hidden = self
                .identity_of(text.row)
                .is_some_and(|id| self.hidden.contains(&id));
            gpu.set_hidden(text.row, !text.active || hidden);
        }
    }

    /// Current manifest row, retained for source lookup independently of title insertion order.
    pub fn text_row(&self, index: usize) -> Option<u32> {
        let key = format!("manifest-text/{index}");
        for text in &self.texts {
            if text.active && text.key == key {
                return Some(text.row);
            }
        }
        None
    }

    /// Resolve a source text independently of geometry document indices.
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

    /// Build the text lane's visible submission with the current ordinary selection flags.
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
