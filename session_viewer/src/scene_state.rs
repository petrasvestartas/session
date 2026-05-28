use std::collections::{HashMap, HashSet};
use winit::keyboard::ModifiersState;
use crate::camera::{Camera, CameraController};
use crate::gpu_session::GpuSession;
use session_rust::Session;

pub struct SceneState {
    pub gpu_session: GpuSession,
    pub session: Session,
    pub selected_guids: HashSet<String>,
    pub hidden_guids: HashSet<String>,
    pub glyphs_hidden_guids: HashSet<String>,
    pub group_locked: HashSet<String>,
    pub geom_guid_set: HashSet<String>,
    pub leaf_guid_cache: HashMap<String, Vec<String>>,
    pub leaf_cache_dirty: bool,
    pub camera: Camera,
    pub controller: CameraController,
    pub key_mods: ModifiersState,
    pub line_thickness: f32,
    pub shading_enabled: bool,
    pub backface_highlight: bool,
    pub pending_pick: Option<(f64, f64)>,
    pub mouse_position: (f64, f64),
    pub text_labels: Vec<crate::text::TextLabel>,
}
