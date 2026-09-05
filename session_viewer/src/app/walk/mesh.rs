//! One mesh into the tables: its faces into the arena, its local box, then the ink pass
//! (`mesh_ink`) unless the mesh is dense, a print fill, or edges are switched off. The gates
//! and thresholds live here. Nothing here reads the GPU.

use super::bounds::mesh_thickness;
use session_rust::RenderVertex;
use session_rust::Mesh;
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::Instance;
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

/// A fill (every PDF glyph, every poche region) broadcasts a single width of 0: print, not
/// surface. One test drives the wireframe skip, the index run and `FLAG_PRINT`.
pub fn is_print_fill(m: &Mesh) -> bool {
    m.widths().len() == 1 && m.widths()[0] == 0.0
}

/// How one mesh is walked: whether a print fill takes the sheet index runs (and
/// `FLAG_PRINT`), and whether an open mesh may raise `FLAG_OPEN`.
pub struct MeshOpts {
    pub sheet_lanes: bool,
    pub allow_open: bool,
}

impl MeshOpts {
    /// A `Mesh` object: print fills take the sheet runs; open meshes are flagged.
    pub const OBJECT: MeshOpts = MeshOpts { sheet_lanes: true, allow_open: true };
    /// A tessellated BRep or surface: always the depth-tested run, never `FLAG_OPEN`.
    pub const MODEL: MeshOpts = MeshOpts { sheet_lanes: false, allow_open: false };
    /// An element's mesh: sheet runs, but an element is never flagged open.
    pub const ELEMENT: MeshOpts = MeshOpts { sheet_lanes: true, allow_open: false };
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

/// Which index run a mesh's triangles join decides WHEN it is drawn: sheet fills composite
/// in document order, lettering ("text", named by the PDF importer) goes last of all.
fn index_run<'a>(arena: &'a mut ArenaRows, m: &Mesh, sheet: bool) -> &'a mut Vec<u32> {
    if !sheet {
        return &mut arena.idx;
    }
    if m.name == "text" { &mut arena.idx_text } else { &mut arena.idx_print }
}

/// The walk context and the options one mesh is walked with.
pub struct MeshCx<'a> {
    pub cx: &'a WalkCx,
    pub opts: &'a MeshOpts,
}

/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, mc: &MeshCx) -> Row {
    let (cx, o) = (mc.cx, mc.opts);
    let base = cx.vert_base + arena.verts.len() as u32;
    let mut lap = Lap::start("walk_mesh");
    let mut rm = m.to_render();
    lap.mark("to_render");

    let print = is_print_fill(m);
    let decorated = rm.indices.len() / 3 <= MESH_RAW_MIN && !print;
    let keys = if decorated { m.vertices() } else { Vec::new() };
    let slots = SlotMap::new(&keys);
    let mut vpos64 = Vec::with_capacity(keys.len());
    let mut vpos = Vec::with_capacity(keys.len());
    for &key in &keys {
        let point = &m.vertex[&key];
        vpos64.push([point.x, point.y, point.z]);
        vpos.push([point.x as f32, point.y as f32, point.z as f32]);
    }
    let topo = if decorated { Some(mesh_topology(m, &keys, &vpos64, &slots)) } else { None };
    let mut bounds = Aabb::empty();
    for vertex in &rm.vertices { bounds.grow(vertex.position); }
    let mut tokens = Vec::new();
    let mut host_faces = Vec::new();
    if let Some(topology) = &topo {
        let faces = super::mesh_faces::decorate(m, &rm, topology, &super::mesh_faces::FaceCx { base: cx.face_base + arena.face_planes.len() as u32, row: cx.row });
        rm = faces.render;
        arena.face_ids.extend(faces.ids);
        arena.face_planes.extend(faces.planes);
        tokens = faces.tokens;
        host_faces = faces.hosts;
    } else if !(o.sheet_lanes && print) {
        let faces = super::mesh_raw_faces::decorate(rm, &super::mesh_faces::FaceCx { base: cx.face_base + arena.face_planes.len() as u32, row: cx.row });
        rm = faces.render;
        arena.face_ids.extend(faces.ids);
        arena.face_planes.extend(faces.planes);
    } else {
        arena.face_ids.resize(arena.face_ids.len() + rm.vertices.len(), 0);
    }
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
    for v in &rm.vertices {
        bounds.grow(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }
    let idx = index_run(arena, m, o.sheet_lanes && print);
    idx.reserve(rm.indices.len());
    for &i in &rm.indices {
        idx.push(base + i);
    }
    lap.mark("vert+idx push");
    let flags = if o.sheet_lanes && print { Instance::FLAG_PRINT } else { 0 };
    let thickness = mesh_thickness(&positions(&rm.vertices), &rm.indices);
    let row = Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags, faces: true, thickness, host_faces };

    if rm.indices.len() / 3 > MESH_RAW_MIN || print || knobs::no_edges() {
        return row;
    }

    let topo = topo.expect("decorated mesh has topology");
    lap.mark("topology");
    let mut icx = InkCx { row: cx.row, vpos: &vpos, slots: &slots, lap: &mut lap, tokens: &tokens };
    edges_and_dots(ink, m, &topo, &mut icx);

    // An open mesh is not a solid: the facing cull would strip interior surface seen through
    // the hole, so the shaders skip it like FLAG_INSIDE.
    let open = o.allow_open && !topo.closed;
    Row { flags: if open { row.flags | Instance::FLAG_OPEN } else { row.flags }, ..row }
}

/// The positions of a render mesh's vertices, for the thickness measure.
fn positions(verts: &[RenderVertex]) -> Vec<[f32; 3]> {
    let mut out = Vec::with_capacity(verts.len());
    for v in verts {
        out.push(v.position);
    }
    out
}
