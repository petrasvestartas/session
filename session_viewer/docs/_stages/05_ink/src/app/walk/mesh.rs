//! One mesh into the tables: its faces into the arena, its local box, then the ink pass
//! (`mesh_ink`) unless the mesh is dense or edges are switched off. The gates
//! and thresholds live here. Nothing here reads the GPU.

use super::bounds::mesh_thickness;
use session_rust::RenderVertex;
use session_rust::Mesh;
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::math::Aabb;
use super::{Row, WalkCx};
use super::mesh_ink::{edges_and_dots, Ink, InkCx};
use super::mesh_topology::{mesh_topology, SlotMap};

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no edges, no markers: on a
/// scan the decoration is 90x the geometry. The bunny (69k tri) keeps its wireframe.
pub const MESH_RAW_MIN: usize = 200_000;

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says.
pub const WIREFRAME_BLACK_MIN: usize = 10_000;

/// Two faces count as one flat region above this normal dot: EXACT coplanarity, so curvature
/// on a dense scan is never mistaken for tessellation.
pub const COPLANAR_DOT: f64 = 1.0 - 1e-9;

/// Typical distance between a mesh's vertices: the diagonal over the square root of the
/// vertex count (a surface spreads its vertices over an area). The markers thin below it.
fn mesh_spacing(bounds: &Aabb, verts: usize) -> f32 {
    if verts < 2 {
        return 0.0;
    }
    bounds.diagonal() / (verts as f32).sqrt()
}

/// How one mesh is walked.
pub struct MeshOpts {}

impl MeshOpts {
    /// A `Mesh` object.
    pub const OBJECT: MeshOpts = MeshOpts {};
}

/// The clock behind VIEWER_PROFILE: `mark` prints the lap since the previous mark. A no-op
/// on wasm32, where `Instant::now()` panics.
#[cfg(not(target_arch = "wasm32"))]
pub struct Lap {
    on: bool,
    at: std::time::Instant,
    prefix: &'static str,
}

/// The browser build's clock: nothing to read, nothing to print.
#[cfg(target_arch = "wasm32")]
pub struct Lap;

#[cfg(not(target_arch = "wasm32"))]
impl Lap {
    /// Start the clock; `prefix` names the caller in every printed line.
    pub fn start(prefix: &'static str) -> Self {
        Self { on: knobs::profile(), at: std::time::Instant::now(), prefix }
    }

    /// Print the lap since the previous mark under `name`, then restart.
    pub fn mark(&mut self, name: &str) {
        if self.on {
            eprintln!("  {} {name:<20} {:?}", self.prefix, self.at.elapsed());
            self.at = std::time::Instant::now();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Lap {
    /// No clock on wasm32.
    pub fn start(_prefix: &'static str) -> Self {
        Self
    }

    /// No clock on wasm32.
    pub fn mark(&mut self, _name: &str) {}
}

/// The walk context and the options one mesh is walked with.
pub struct MeshCx<'a> {
    pub cx: &'a WalkCx<'a>,
    pub opts: &'a MeshOpts,
}

/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, mc: &MeshCx) -> Row {
    let cx = mc.cx;
    let base = cx.vert_base + arena.verts.len() as u32;
    let mut lap = Lap::start("walk_mesh");
    let rm = m.to_render();
    lap.mark("to_render");

    let mut bounds = Aabb::empty();
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
    for v in &rm.vertices {
        bounds.grow(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }
    let idx = &mut arena.idx;
    idx.reserve(rm.indices.len());
    for &i in &rm.indices {
        idx.push(base + i);
    }
    lap.mark("vert+idx push");
    let thickness = mesh_thickness(&positions(&rm.vertices), &rm.indices);
    let row = Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags: 0, faces: true, thickness };

    if rm.indices.len() / 3 > MESH_RAW_MIN || knobs::no_edges() {
        return row;
    }

    // Positions by slot from the KERNEL's vertex map, kept in f64 for the face normals.
    let keys = m.vertices();
    let slots = SlotMap::new(&keys);
    let mut vpos64: Vec<[f64; 3]> = Vec::with_capacity(keys.len());
    for &k in &keys {
        let v = &m.vertex[&k];
        vpos64.push([v.x, v.y, v.z]);
    }
    let mut vpos: Vec<[f32; 3]> = Vec::with_capacity(keys.len());
    for p in &vpos64 {
        vpos.push([p[0] as f32, p[1] as f32, p[2] as f32]);
    }
    let topo = mesh_topology(m, &keys, &vpos64, &slots);
    lap.mark("topology");

    let mut icx = InkCx { row: cx.row, vpos: &vpos, slots: &slots, lap: &mut lap };
    edges_and_dots(ink, m, &topo, &mut icx);
    row
}

/// The positions of a render mesh's vertices, for the thickness measure.
fn positions(verts: &[RenderVertex]) -> Vec<[f32; 3]> {
    let mut out = Vec::with_capacity(verts.len());
    for v in verts {
        out.push(v.position);
    }
    out
}
