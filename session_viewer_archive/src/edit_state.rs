//! EditState — the sub-object / control-point editing layer.
//!
//! When edit mode is active (toggled with F10) the currently targeted object's
//! editable parts are shown as a draggable overlay:
//!   * control points / mesh vertices → sphere handles (own glyph buffer)
//!   * control polygon / mesh edges    → thin cylinders (own segment buffer)
//!
//! Sub-objects are picked with Ctrl+Shift+LMB; the existing gumball is reused to
//! translate/rotate/scale the picked nodes. Group 0 (camera + identity instance)
//! is borrowed from the gumball; only the group-1 storage buffers live here.

use std::collections::HashSet;
use crate::gpu_session::{CylinderSegment, GlyphPoint, make_geom_bind_group};
use crate::undo_state::EditSnapshot;

/// Which kind of geometry the current edit target is. Drives where node positions
/// are read from and written back to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EditKind {
    Mesh,
    Polyline,
    NurbsCurve,
    NurbsSurface,
    BRep,
}

/// How one overlay node maps back into the source geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeAddr {
    MeshVertex(usize),           // mesh vertex key
    PolyPoint(usize),            // polyline point index
    CurveCv(usize),              // nurbs curve cv index
    SurfaceCv(usize, usize),     // nurbs surface (i, j)
    BRepCv(usize, usize, usize), // brep (surface_index, i, j)
    CurveEditPt(usize),          // nurbs curve edit (Greville) point index — on the curve
    SurfaceEditPt(usize, usize), // nurbs surface (boundary 0..3, edit-point index along that boundary)
}

/// One editable handle: a world-space position plus the address to write it back.
/// `world` is f64 (kernel precision) so edits round-trip into the f64 CVs without
/// quantizing; it is narrowed to f32 only at the GPU/gumball boundary (see `f32p`).
#[derive(Clone, Copy)]
pub struct EditNode {
    pub world: [f64; 3],
    pub addr: NodeAddr,
}

/// Narrow a kernel-precision world position to the f32 the GPU buffers and the
/// gumball use. The single, named f64→f32 boundary for edit node positions.
#[inline]
pub fn f32p(p: [f64; 3]) -> [f32; 3] {
    [p[0] as f32, p[1] as f32, p[2] as f32]
}

/// An overlay edge connecting two nodes (indices into `nodes`).
#[derive(Clone, Copy)]
pub struct EditEdge {
    pub a: usize,
    pub b: usize,
}

pub struct EditState {
    /// Sub-object edit mode on/off (F10).
    pub active: bool,
    /// Guid of the object whose sub-objects are being edited.
    pub target: Option<String>,
    pub kind: Option<EditKind>,

    pub nodes: Vec<EditNode>,
    pub edges: Vec<EditEdge>,
    pub selected_nodes: HashSet<usize>,
    pub selected_edges: HashSet<usize>,

    /// Centroid of all nodes — anchors the screen-constant handle scale.
    pub centroid: [f32; 3],
    /// Per-frame screen-constant handle scale (mirrors `gumball_scale`).
    pub handle_scale: f32,

    /// Geometry clone captured at drag/press start, for the undo "before" state.
    pub before_snapshot: Option<EditSnapshot>,
    /// World positions of the moving node set captured at drag/press start
    /// (node index → start world). Non-empty ⇒ an edit-gumball interaction is live.
    pub drag_start: Vec<(usize, [f64; 3])>,

    /// Transient boundary-edge edit entered without F10 (Ctrl+Shift+RMB). On exit we
    /// leave edit mode entirely instead of returning to a control-point session.
    pub edge_mode: bool,

    // ── Live-deform capture (frozen at drag start, fixed topology) ──────────────
    /// Frozen NurbsSurface tessellation (carries per-vertex u/v; nx/ny/nz stripped so
    /// normals recompute from the deformed positions). Re-deformed in place each move.
    pub live_surf_mesh: Option<session_rust::Mesh>,
    /// Basis-weight live deform (Part B): per tessellation vertex, its drag-start position
    /// and the influence weight of each moved CV (parallel to `drag_start`). A NURBS point
    /// is linear in its CVs, so a drag updates positions with cheap multiply-adds — no
    /// `point_at`/`normal_at` per move. `surf_base[v] = (vertex_key, base_pos)`,
    /// `surf_weights[v][k] = Nᵢ(u)Nⱼ(v)·Wᵢⱼ/denom` for moved CV `k`.
    pub surf_base: Vec<(usize, [f64; 3])>,
    pub surf_weights: Vec<Vec<f64>>,
    /// Frozen segment count for live NurbsCurve resampling (keeps the range stable).
    pub live_curve_segs: usize,

    // ── Edit-point (Greville) editing ───────────────────────────────────────────
    /// Overlay shows on-geometry edit points (Ctrl+Shift+LMB boundary path) rather than
    /// raw control points (F10). Drives both the overlay build and the refit on drag.
    pub use_edit_points: bool,
    /// Refit context captured at drag start: the affected control points as
    /// (write-back address, Euclidean start position, weight), in collocation column order.
    pub editpt_cv: Vec<(NodeAddr, [f64; 3], f64)>,
    /// Per moved edit point (parallel to `drag_start`): column `k` of `R⁻¹` over `editpt_cv`,
    /// i.e. the CV displacement field for a unit drag of that edit point.
    pub editpt_cols: Vec<Vec<f64>>,
    /// For a surface edit-point session: which boundary (0..3) the edit points sit on.
    pub editpt_boundary: Option<usize>,
    /// For a BRep edge session: the picked topology edge index (for drawing the true edge curve).
    pub brep_edge: Option<usize>,
    /// Render-only world-space polyline of the SELECTED edge sampled along the actual geometry
    /// (the iso-curve / 3D edge curve), so a circular edge reads as a circle instead of the
    /// chorded control polygon. Empty ⇒ fall back to drawing the chorded `edges`.
    pub edge_polyline: Vec<[f32; 3]>,
    /// `edge_polyline` snapshot at drag start. During a full-edge transform the live overlay is
    /// this base mapped by the gumball delta (cheap, exact) instead of re-evaluating geometry.
    pub edge_polyline_base: Vec<[f32; 3]>,
    /// Surface CVs coincident with a moved row CV but NOT in the row (a closed surface's opposite
    /// seam column): `((i, j), row_index)`. Written with the matching row CV's delta so the seam
    /// stays closed.
    pub editpt_mirror: Vec<((usize, usize), usize)>,
    /// BRep hole faces to deform into a collar: `(face_surface_index, hole_center_3d, r_in, r_out,
    /// [(cv_ij, original_pos)])`. The face is degree-elevated at selection; each move pushes CVs by
    /// a smooth falloff (1 within `r_in` of the hole → follows the move, 0 beyond `r_out` → outer
    /// boundary fixed), so the surface bends and the hole rim lifts while the outline stays put.
    pub editpt_face_deform: Vec<(usize, [f64; 3], f64, f64, Vec<((usize, usize), [f64; 3])>)>,

    // ── GPU overlay (group-1 storage buffers; group 0 reuses the gumball's) ──
    pub node_buf: wgpu::Buffer,
    pub node_bg: wgpu::BindGroup,
    pub edge_buf: wgpu::Buffer,
    pub edge_bg: wgpu::BindGroup,
}

impl EditState {
    pub fn new(device: &wgpu::Device, geom_bgl: &wgpu::BindGroupLayout) -> Self {
        let node_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("edit.nodes"),
            size: 256 * std::mem::size_of::<GlyphPoint>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let node_bg = make_geom_bind_group(device, geom_bgl, &node_buf);
        let edge_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("edit.edges"),
            size: 512 * std::mem::size_of::<CylinderSegment>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let edge_bg = make_geom_bind_group(device, geom_bgl, &edge_buf);
        Self {
            active: false,
            target: None,
            kind: None,
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            centroid: [0.0; 3],
            handle_scale: 1.0,
            before_snapshot: None,
            drag_start: Vec::new(),
            edge_mode: false,
            live_surf_mesh: None,
            surf_base: Vec::new(),
            surf_weights: Vec::new(),
            live_curve_segs: 0,
            use_edit_points: false,
            editpt_cv: Vec::new(),
            editpt_cols: Vec::new(),
            editpt_boundary: None,
            brep_edge: None,
            edge_polyline: Vec::new(),
            edge_polyline_base: Vec::new(),
            editpt_mirror: Vec::new(),
            editpt_face_deform: Vec::new(),
            node_buf,
            node_bg,
            edge_buf,
            edge_bg,
        }
    }

    /// Indices of every node that should move with the gumball: explicitly selected
    /// nodes plus the endpoints of every selected edge.
    pub fn move_node_set(&self) -> Vec<usize> {
        let mut set: HashSet<usize> = self.selected_nodes.clone();
        for &ei in &self.selected_edges {
            if let Some(e) = self.edges.get(ei) {
                set.insert(e.a);
                set.insert(e.b);
            }
        }
        let mut v: Vec<usize> = set.into_iter().collect();
        v.sort_unstable();
        v
    }

    /// Centroid of the moving node set (gumball origin). Falls back to all-node
    /// centroid when nothing is selected.
    pub fn selection_centroid(&self) -> Option<[f32; 3]> {
        let set = self.move_node_set();
        if set.is_empty() {
            return None;
        }
        let mut s = [0.0f64; 3];
        for &i in &set {
            let w = self.nodes[i].world;
            s[0] += w[0];
            s[1] += w[1];
            s[2] += w[2];
        }
        let n = set.len() as f64;
        Some([(s[0] / n) as f32, (s[1] / n) as f32, (s[2] / n) as f32])
    }

    /// Recompute `centroid` from all nodes (used to anchor the handle scale).
    pub fn recompute_centroid(&mut self) {
        if self.nodes.is_empty() {
            self.centroid = [0.0; 3];
            return;
        }
        let mut s = [0.0f64; 3];
        for n in &self.nodes {
            s[0] += n.world[0];
            s[1] += n.world[1];
            s[2] += n.world[2];
        }
        let c = self.nodes.len() as f64;
        self.centroid = [(s[0] / c) as f32, (s[1] / c) as f32, (s[2] / c) as f32];
    }

    pub fn clear_overlay(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.drag_start.clear();
        self.before_snapshot = None;
        self.use_edit_points = false;
        self.editpt_boundary = None;
        self.brep_edge = None;
        self.editpt_face_deform.clear();
        self.edge_polyline.clear();
        self.edge_polyline_base.clear();
        self.clear_live();
    }

    /// Drop the live-deform capture taken at drag start.
    pub fn clear_live(&mut self) {
        self.live_surf_mesh = None;
        self.surf_base.clear();
        self.surf_weights.clear();
        self.live_curve_segs = 0;
        self.editpt_cv.clear();
        self.editpt_cols.clear();
        self.edge_polyline_base.clear();
        self.editpt_mirror.clear();
    }

    /// Upload the current overlay geometry to the group-1 storage buffers,
    /// growing them when needed. Mirrors `GumballState::upload_geometry`.
    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        geom_bgl: &wgpu::BindGroupLayout,
        node_glyphs: &[GlyphPoint],
        edge_segs: &[CylinderSegment],
    ) {
        let nb = bytemuck::cast_slice::<_, u8>(node_glyphs);
        if !nb.is_empty() && nb.len() as u64 > self.node_buf.size() {
            self.node_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("edit.nodes"),
                size: nb.len() as u64 * 2,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.node_bg = make_geom_bind_group(device, geom_bgl, &self.node_buf);
        }
        if !nb.is_empty() {
            queue.write_buffer(&self.node_buf, 0, nb);
        }
        let eb = bytemuck::cast_slice::<_, u8>(edge_segs);
        if !eb.is_empty() && eb.len() as u64 > self.edge_buf.size() {
            self.edge_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("edit.edges"),
                size: eb.len() as u64 * 2,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.edge_bg = make_geom_bind_group(device, geom_bgl, &self.edge_buf);
        }
        if !eb.is_empty() {
            queue.write_buffer(&self.edge_buf, 0, eb);
        }
    }
}
