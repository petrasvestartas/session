use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    Pick([f32; 2]),
    AddBox,
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    history: History,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changing_the_owned_camera_keeps_document_identity() {
        let mut editor = Editor::default();
        let ids: Vec<_> = editor.scene.objects().iter().map(|object| object.id).collect();
        editor.camera.zoom(2.0);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.scene.objects().iter().map(|object| object.id).collect::<Vec<_>>(), ids);
        assert!(editor.selected.is_none());
    }
}
