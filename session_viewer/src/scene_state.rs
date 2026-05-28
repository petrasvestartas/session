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
    pub transform_locked: HashSet<String>,
    pub geom_guid_set: HashSet<String>,
    pub leaf_guid_cache: HashMap<String, Vec<String>>,
    pub leaf_cache_dirty: bool,
    pub face_color_overrides: HashMap<String, [f32; 4]>,
    pub point_color_overrides: HashMap<String, [f32; 4]>,
    pub camera: Camera,
    pub controller: CameraController,
    pub key_mods: ModifiersState,
    /// Ctrl held, tracked from key-up/down events directly. WindowEvent::ModifiersChanged
    /// is unreliable on the winit web backend, which left Ctrl+Z/Ctrl+U dead.
    pub ctrl_down: bool,
    pub line_thickness: f32,
    pub shading_enabled: bool,
    pub backface_highlight: bool,
    pub pending_pick: Option<(f64, f64)>,
    pub box_select_start: Option<(f64, f64)>,
    pub box_select: Option<((f64, f64), (f64, f64))>,
    pub mouse_position: (f64, f64),
    pub text_labels: Vec<crate::text::TextLabel>,
}
