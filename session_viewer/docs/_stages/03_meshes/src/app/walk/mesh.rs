//! One mesh into the tables: its faces into the arena, its local box. Nothing here reads
//! the GPU.

use super::bounds::mesh_thickness;
use session_rust::RenderVertex;
use session_rust::Mesh;
#[cfg(not(target_arch = "wasm32"))]
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::math::Aabb;
use super::{Row, WalkCx};

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
    pub cx: &'a WalkCx,
    pub opts: &'a MeshOpts,
}

/// Faces into the arena and the mesh-local box.
pub fn walk_mesh(arena: &mut ArenaRows, m: &Mesh, mc: &MeshCx) -> Row {
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
    Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags: 0, faces: true, thickness }
}

/// The positions of a render mesh's vertices, for the thickness measure.
fn positions(verts: &[RenderVertex]) -> Vec<[f32; 3]> {
    let mut out = Vec::with_capacity(verts.len());
    for v in verts {
        out.push(v.position);
    }
    out
}
