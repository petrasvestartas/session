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
    /// Ctrl/Shift held, tracked directly from key events (ModifiersChanged is
    /// unreliable on the web backend). Read at pick time for Ctrl+Shift sub-object
    /// selection in edit mode.
    pub ctrl_down: bool,
    pub shift_down: bool,
    pub line_thickness: f32,
    pub shading_enabled: bool,
    pub backface_highlight: bool,
    /// `m` toggles drawing NURBS / trimmed tessellation as wireframe lines (diagnostic).
    pub show_tess: bool,
    pub pending_pick: Option<(f64, f64)>,
    /// One-shot: a viewport pick changed the selection; the tree should expand
    /// ancestor groups and scroll the selected row into view on the next frame.
    pub reveal_in_tree: bool,
    pub box_select_start: Option<(f64, f64)>,
    pub box_select: Option<((f64, f64), (f64, f64))>,
    /// Left mouse button currently held. Tracked from raw events (even when egui
    /// consumes them) so a leaked gumball press can never start a no-button drag.
    pub lmb_down: bool,
    pub mouse_position: (f64, f64),
    pub text_labels: Vec<crate::text::TextLabel>,
    /// Frustum-cull mesh draws when true (perf). Toggled from the perf HUD.
    pub frustum_cull: bool,
    /// Render counters from the last frame, shown in the perf HUD.
    pub draw_stats: crate::gpu_session::DrawStats,
    /// Arctic display mode: white material, ambient only + SSAO, ground plane.
    pub arctic: bool,
    pub arctic_gradient: bool,
    /// 0 = SSAO, 1 = HBAO, 2 = GTAO.
    pub ao_mode: u32,
    pub ssao_intensity: f32,
    /// AO radius as % of scene bbox diagonal.
    pub ssao_radius_pct: f32,
    /// Last non-empty scene bounds (mm) — survives the cached_boxes clear that
    /// follows every transform/edit commit, so arctic AO sizing never pops.
    pub last_arctic_bounds: Option<([f32; 3], [f32; 3])>,
    /// Screen-space boundary around surface geometry only (meshes/BReps/NURBS);
    /// polylines, lines and points are excluded by construction (mask pass).
    pub outline: bool,
    /// Outline width in pixels.
    pub outline_px: f32,
    /// FXAA on the resolved image (smooths thin-line stair-stepping beyond MSAA).
    pub fxaa: bool,
    /// Visible viewport in physical px [x, y, w, h]: the window area NOT covered
    /// by egui panels (reported by egui each frame). Drives camera aspect and the
    /// off-center projection so the perspective is never cut by the UI.
    pub viewport_px: [f32; 4],
}
