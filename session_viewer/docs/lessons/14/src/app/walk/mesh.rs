use super::mesh_ink::{Ink, InkCx, edges_and_dots};
use super::mesh_topology::{SlotMap, mesh_topology};
use super::{Row, WalkCx};
use crate::app::knobs;
use crate::engine::gpu::Instance;
use crate::engine::gpu::arena::ArenaRows;
use session_rust::AABB;
use session_rust::Mesh;
use session_rust::RenderVertex;

/// Above this many triangles a mesh gets no edges or dots.
pub const MESH_RAW_MIN: usize = 200_000;

/// From this many edges the wireframe is black.
pub const WIREFRAME_BLACK_MIN: usize = 10_000;

/// Normal dot above which two faces are one flat region.
pub const COPLANAR_DOT: f64 = 1.0 - 1e-9;

/// Normal dot below which a smooth seam is a crease, cos 25°.
pub const CREASE_COS: f64 = 0.906_307_787;

/// Typical distance between vertices.
pub(super) fn mesh_spacing(bounds: &AABB, verts: usize) -> f32 {
    if verts < 2 {
        return 0.0;
    }

    bounds.diagonal() as f32 / (verts as f32).sqrt()
}

/// A mesh with one zero width is a printed fill.
pub fn is_print_fill(m: &Mesh) -> bool {
    m.widths().len() == 1 && m.widths()[0] == 0.0
}

/// How one mesh is walked.
pub struct MeshOpts {
    pub sheet_lanes: bool, // print fills go to the sheet runs
    pub allow_open: bool,  // an open mesh may be flagged open
    pub smooth: bool,      // a sampled surface, seams are not edges
}

impl MeshOpts {
    /// A plain mesh object.
    pub const OBJECT: MeshOpts = MeshOpts {
        sheet_lanes: true,
        allow_open: true,
        smooth: false,
    };

    /// A sampled NURBS surface.
    pub const SURFACE: MeshOpts = MeshOpts {
        sheet_lanes: false,
        allow_open: true,
        smooth: true,
    };

    /// An element's mesh, never flagged open.
    pub const ELEMENT: MeshOpts = MeshOpts {
        sheet_lanes: true,
        allow_open: false,
        smooth: false,
    };
}

/// A lap timer printing when profiling is on.
#[cfg(not(target_arch = "wasm32"))]
pub struct Lap {
    on: bool,               // profiling enabled
    at: std::time::Instant, // last mark
    prefix: &'static str,   // caller name in each line
}

/// No timer in the browser.
#[cfg(target_arch = "wasm32")]
pub struct Lap;

#[cfg(not(target_arch = "wasm32"))]
impl Lap {
    /// Start the timer.
    pub fn start(prefix: &'static str) -> Self {
        Self {
            on: knobs::profile(),
            at: std::time::Instant::now(),
            prefix,
        }
    }

    /// Print the time since the last mark.
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

/// The index list this mesh's triangles join.
fn index_run<'a>(arena: &'a mut ArenaRows, m: &Mesh, sheet: bool) -> &'a mut Vec<u32> {
    if !sheet {
        return &mut arena.idx;
    }

    if m.name == "text" {
        &mut arena.idx_text // lettering draws last
    } else {
        &mut arena.idx_print
    }
}

/// Context and options for one mesh.
pub struct MeshCx<'a> {
    pub cx: &'a WalkCx,     // where rows go
    pub opts: &'a MeshOpts, // how to walk
}

/// A mesh: triangles, then edges and dots.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, mc: &MeshCx) -> Row {
    let (cx, o) = (mc.cx, mc.opts);
    let base = cx.vert_base + arena.verts.len() as u32; // first vertex index
    let mut lap = Lap::start("walk_mesh");
    let rm = m.to_render(); // triangles from the kernel
    lap.mark("to_render");

    let print = is_print_fill(m);
    let decorated = rm.indices.len() / 3 <= MESH_RAW_MIN && !print; // gets edges and dots
    let keys = if decorated { m.vertices() } else { Vec::new() }; // sorted vertex keys
    let slots = SlotMap::new(&keys);
    let mut vpos64 = Vec::with_capacity(keys.len()); // positions by slot
    let mut vpos = Vec::with_capacity(keys.len()); // same in f32

    for &key in &keys {
        let point = &m.vertex[&key];
        vpos64.push([point.x, point.y, point.z]);
        vpos.push([point.x as f32, point.y as f32, point.z as f32]);
    }

    let topo = if decorated {
        Some(mesh_topology(m, &keys, &vpos64, &slots))
    } else {
        None
    };
    let mut bounds = AABB::empty();
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());

    // triangles into the arena
    for v in &rm.vertices {
        bounds.union_with_point(
            v.position[0] as f64,
            v.position[1] as f64,
            v.position[2] as f64,
        );
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }

    let idx = index_run(arena, m, o.sheet_lanes && print);
    idx.reserve(rm.indices.len());

    for &i in &rm.indices {
        idx.push(base + i);
    }

    lap.mark("vert+idx push");
    let mut flags = if o.sheet_lanes && print {
        Instance::FLAG_PRINT
    } else {
        0
    };
    let smooth = o.smooth && !knobs::seams(); // knob shows seams

    if smooth {
        flags |= Instance::FLAG_SMOOTH;
    }

    if m.number_of_faces() == 1 {
        flags |= Instance::FLAG_SINGLE;
    }

    let row = Row {
        bounds,
        spacing: mesh_spacing(&bounds, m.number_of_vertices()),
        flags,
        faces: true,
    };

    if !decorated || knobs::no_edges() {
        return row; // triangles only
    }

    let topo = topo.expect("decorated mesh has topology");
    lap.mark("topology");
    let mut icx = InkCx {
        row: cx.row,
        vpos: &vpos,
        slots: &slots,
        smooth,
        lap: &mut lap,
    };
    edges_and_dots(ink, m, &topo, &mut icx);

    // an open mesh keeps its back faces
    let open = o.allow_open && !topo.closed;
    Row {
        flags: if open {
            row.flags | Instance::FLAG_OPEN
        } else {
            row.flags
        },
        ..row
    }
}

/// The positions of a render mesh's vertices, for the thickness measure.
fn positions(verts: &[RenderVertex]) -> Vec<[f32; 3]> {
    let mut out = Vec::with_capacity(verts.len());

    for v in verts {
        out.push(v.position);
    }

    out
}
