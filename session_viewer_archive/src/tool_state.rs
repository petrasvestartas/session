//! Interactive drawing tool state (the "getpoint" loop). Held on `State` as `tool`.

use session_rust::Point;
use crate::snap::{SnapKind, SnapModes, SnapResult};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DrawTool {
    Idle,
    Point,
    Line,
    Polyline,
    NurbsCurve { degree: usize },
    /// Move captured objects base→target (osnap point-to-point, or typed distance along cursor).
    Move,
}

pub struct ToolState {
    pub tool: DrawTool,
    /// Committed-so-far world points for the active tool.
    pub points: Vec<Point>,
    /// Current cursor snap resolve (drives the rubber-band end and the marker).
    pub cursor_snap: Option<SnapResult>,
    pub snap_modes: SnapModes,
    /// Set when the preview line geometry needs rebuilding.
    pub preview_dirty: bool,
    /// Move tool: objects to move + their GPU model at move-start (`(guid, model)`).
    pub move_origins: Vec<(String, [[f32; 4]; 4])>,
    /// Move tool targets the active edit selection (an edge / CVs) instead of whole objects.
    pub move_edit: bool,
    /// Cached static snap points `(world, kind, guid)` for the whole scene, built once per tool
    /// session; per-frame snapping only projects these (no `point_at`). `false` ⇒ rebuild.
    pub snap_cache: Vec<(Point, SnapKind, String)>,
    /// Cached edge polylines `(guid, world points)` for along-edge "Near" snap + the move ghost.
    pub snap_edges: Vec<(String, Vec<Point>)>,
    pub snap_cache_ready: bool,
}

impl ToolState {
    pub fn new() -> Self {
        Self {
            tool: DrawTool::Idle,
            points: Vec::new(),
            cursor_snap: None,
            snap_modes: SnapModes::default_on(),
            preview_dirty: false,
            move_origins: Vec::new(),
            move_edit: false,
            snap_cache: Vec::new(),
            snap_edges: Vec::new(),
            snap_cache_ready: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.tool != DrawTool::Idle
    }

    pub fn start(&mut self, tool: DrawTool) {
        self.tool = tool;
        self.points.clear();
        self.cursor_snap = None;
        self.preview_dirty = true;
        self.snap_cache_ready = false; // rebuild for the new session (scene may have changed)
    }

    pub fn reset(&mut self) {
        self.tool = DrawTool::Idle;
        self.points.clear();
        self.cursor_snap = None;
        self.preview_dirty = true;
        self.move_origins.clear();
        self.move_edit = false;
        self.snap_cache.clear();
        self.snap_edges.clear();
        self.snap_cache_ready = false;
    }
}
