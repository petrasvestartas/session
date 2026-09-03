//! One mesh into the tables: its faces into the arena, its local box, then the ink pass
//! (`mesh_ink`) unless the mesh is dense, a print fill, or edges are switched off. The gates,
//! the thresholds and the walk's profiling clock live here. Nothing here reads the GPU.

use session_rust::Mesh;
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::Instance;
use crate::math::grow_bounds;
use super::{Row, WalkCx};
use super::mesh_ink::{edges_and_dots, Ink, InkCx};
use super::mesh_topology::{mesh_topology, SlotMap};

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no per-edge cylinder, no
/// per-vertex sphere. At 200k the bunny (69k tri) keeps its wireframe, the armadillo and the
/// dragon do not - the honest line until an impostor makes the decoration cheap.
pub const MESH_RAW_MIN: usize = 200_000;

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says: at scan
/// density a pen is a property of the tessellation. 104,288 on the bunny; 12 on a box, whose
/// authored red pen always survives.
pub const WIREFRAME_BLACK_MIN: usize = 10_000;

/// Typical distance between a mesh's vertices, world units: the AABB diagonal over the square
/// root of the vertex count (a surface spreads its vertices over an AREA). The ink lanes drop
/// their markers once it projects below a few pixels - see WIRE_MIN_PX in ribbon.wgsl.
fn mesh_spacing(bounds: Option<([f32; 3], [f32; 3])>, verts: usize) -> f32 {
    let Some((lo, hi)) = bounds else { return 0.0 };
    if verts < 2 {
        return 0.0;
    }
    let d = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    d / (verts as f32).sqrt()
}

/// A fill (every PDF glyph, every poche region) broadcasts a single width of 0 - print, not
/// surface. One test drives the wireframe skip, the index run AND `FLAG_PRINT` (flat lighting,
/// so the sheet reads the same from the back), so the three cannot drift apart.
pub fn is_print_fill(m: &Mesh) -> bool {
    m.widths().len() == 1 && m.widths()[0] == 0.0
}

/// Two faces count as one flat region above this normal dot, so the edge between them is
/// interior tessellation. EXACT coplanarity: 0.9999 (0.81 deg) silently ate 14,644 of the
/// bunny's 104,288 edges - curvature is not tessellation, and same-plane normals agree to ULPs.
pub const COPLANAR_DOT: f64 = 1.0 - 1e-9;

/// How one mesh is walked: its object row, the arena rows already on the GPU, and the two
/// choices the caller makes - whether a print fill takes the sheet index runs (and `FLAG_PRINT`),
/// and whether an open mesh may raise `FLAG_OPEN`.
pub struct MeshOpts {
    pub row: u32,
    pub base_off: u32,
    pub sheet_lanes: bool,
    pub allow_open: bool,
}

impl MeshOpts {
    /// A `Mesh` object: print fills take the sheet runs; `allow_open` is the caller's call.
    pub fn sheet(cx: &WalkCx, allow_open: bool) -> Self {
        Self { row: cx.row, base_off: cx.vert_base, sheet_lanes: true, allow_open }
    }

    /// A tessellated BRep or surface: always the depth-tested run, never FLAG_OPEN.
    pub fn model(cx: &WalkCx) -> Self {
        Self { row: cx.row, base_off: cx.vert_base, sheet_lanes: false, allow_open: false }
    }
}

/// The clock behind VIEWER_PROFILE: `mark` prints the lap since the previous mark. A no-op on
/// wasm32, where `Instant::now()` PANICS ("time not implemented") - and this runs per mesh.
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

/// The object row a mesh reports: its local box, its vertex spacing and the flags it earned.
fn mesh_row(bounds: Option<([f32; 3], [f32; 3])>, m: &Mesh, flags: u32) -> Row {
    Row::solid(bounds, mesh_spacing(bounds, m.number_of_vertices()), flags)
}

/// Which index run a mesh's triangles join decides WHEN it is drawn: sheet fills composite in
/// document order with no depth arbitration, and lettering ("text", named by the PDF importer)
/// goes last of all, after the ink lanes. Everything else takes the depth-tested `idx` run.
fn index_run<'a>(arena: &'a mut ArenaRows, m: &Mesh, sheet: bool) -> &'a mut Vec<u32> {
    if !sheet {
        return &mut arena.idx;
    }
    if m.name == "text" { &mut arena.idx_text } else { &mut arena.idx_print }
}

/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no. A dense
/// mesh, a print fill and VIEWER_NO_EDGES stop after the faces and report NO box: they emit
/// no ink, so nothing would read `FLAG_INSIDE` - and a print fill still gets `FLAG_PRINT`.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, o: &MeshOpts) -> Row {
    let base = o.base_off + arena.verts.len() as u32; // GPU rows already uploaded + rows pending in this delta
    let mut lap = Lap::start("push_mesh");
    let rm = m.to_render();
    lap.mark("to_render");

    // The mesh-local AABB rides the object row, so the edge lanes can be told "the eye is inside
    // this solid" (FLAG_INSIDE) - the facing cull's premise, both faces away = hidden, holds
    // only for an eye OUTSIDE. Reported only when the mesh actually draws ink (see the gates).
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
    for v in &rm.vertices{
        grow_bounds(&mut lo, &mut hi, v.position);
        arena.verts.push(*v);
        arena.vids.push(o.row);
    }
    let local_bounds = if lo[0] <= hi[0] { Some((lo, hi)) } else { None };
    let print = is_print_fill(m);
    let idx = index_run(arena, m, o.sheet_lanes && print);
    idx.reserve(rm.indices.len());
    for &i in &rm.indices{
        idx.push(base+i);
    }
    lap.mark("vert+idx push");
    let flags = if o.sheet_lanes && print { Instance::FLAG_PRINT } else { 0 };

    // A DENSE mesh gets no wireframe and no vertex dots: on the Stanford ladder (1.29M tris)
    // the cylinders and spheres were 90x the geometry they decorated, 118 MB of tables and a
    // 12.4 s walk. Picking reads the kernel Mesh, never these rows, so selection is unaffected.
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return mesh_row(None, m, flags);
    }

    // A fill (every PDF glyph, every poche region) asks for no wireframe at all. Leave before
    // topology: for sheets of hundreds of thousands of tiny fills that pass was the walk's
    // biggest cost, and every edge it produced was then skipped.
    if print { return mesh_row(None, m, flags) }

    if knobs::no_edges() { return mesh_row(None, m, flags) }

    // Positions by slot from the KERNEL's vertex map, not `rm.vertices` (to_render DUPLICATES
    // vertices for per-face colors), and kept in f64: the face normals come from these, and
    // rounding first would flip a near-degenerate cross product's sign, i.e. a `facing` word.
    let keys = m.vertices();
    let slots = SlotMap::new(&keys);
    let vpos64: Vec<[f64; 3]> = keys.iter().map(|&k| { let v = &m.vertex[&k]; [v.x, v.y, v.z] }).collect();
    let vpos: Vec<[f32; 3]> = vpos64.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect();

    let topo = mesh_topology(m, &keys, &vpos64, &slots);
    lap.mark("topology");

    let mut cx = InkCx { row: o.row, vpos: &vpos, slots: &slots, lap: &mut lap };
    edges_and_dots(ink, m, &topo, &mut cx);

    // An open mesh (border edges) is not a solid: the facing cull would strip the wireframe off
    // interior surface visible through the hole. The topology pass already knows - an edge
    // walked in one direction IS a border - where `Mesh::is_closed()` was a second full sweep.
    let open = o.allow_open && local_bounds.is_some() && !topo.closed;
    mesh_row(local_bounds, m, if open { flags | Instance::FLAG_OPEN } else { flags })
}
