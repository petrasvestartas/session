//! The ink a mesh wears: one pipe per visible edge, one sphere per vertex - the SOLID lane.
//! Reads the fused topology (`mesh_topology`) and the positions by slot; writes `SegRows.pipes`
//! and `GlyphRows.spheres`, nothing else. The face pass and the gates are `mesh.rs`.

use session_rust::Mesh;
use session_rust::mesh::ColorMode;
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use super::encode::{encode_width, oct16, pack_facing, BLACK, FACING_UNKNOWN};
use super::mesh::{Lap, COPLANAR_DOT, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};

/// The two ink lanes a mesh reaches: pipes for its edges, spheres for its vertices.
pub struct Ink<'a> {
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
}

/// What the ink pass needs from the face pass: the object row, the f32 positions by slot, the
/// key -> slot map and the profiling clock.
pub struct InkCx<'a> {
    pub row: u32,
    pub vpos: &'a [[f32; 3]],
    pub slots: &'a SlotMap,
    pub lap: &'a mut Lap,
}

/// Edge `i`'s pen width. A single entry broadcasts to every edge (one number instead of one
/// per edge - lean .pb for thousands of glyph meshes); an absent entry is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 {
        w[0]
    } else {
        w.get(i).copied().unwrap_or(1.0)
    }
}

/// Width 0 = hidden: a triangulated PDF fill asks for no wireframe at all.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal in face slot `k` of an edge's pair, borrowed - a kernel `Vector` carries a
/// String and a guid, and cloning one here was two heap allocations per edge.
fn normal_of(topo: &MeshTopo, f: [u32; 2], k: usize) -> Option<&[f64; 3]> {
    if f[k] == u32::MAX { None } else { topo.normals[f[k] as usize].as_ref() }
}

/// Append edge `ei`'s faces to `fkeys`, deduped; the face lists are cached from the topology
/// pass instead of a kernel call per incident edge.
fn push_faces(edge_faces: &[[u32; 2]], ei: usize, fkeys: &mut Vec<usize>) {
    for &f in edge_faces[ei].iter() {
        if f == u32::MAX { continue }
        let fk = f as usize;
        if !fkeys.contains(&fk) { fkeys.push(fk); }
    }
}

/// Word `k` of a dot's facing triple, by pack_facing's rules: a lone normal is duplicated, none
/// at all is FACING_UNKNOWN, and a pair colliding with the sentinel collapses to it.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;
            if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// The pipe loop, the widest-edge and incidence passes, then the dots (unless VIEWER_NO_DOTS).
/// A dense wireframe (>= WIREFRAME_BLACK_MIN edges) draws BLACK whatever pens the file carries.
pub fn edges_and_dots(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    let edges = &topo.edges;
    let edge_faces = &topo.edge_faces;
    let w = m.widths();
    let black_wire = edges.len() >= WIREFRAME_BLACK_MIN;

    // Upper bounds from the topology: a segment per edge, a marker per vertex.
    ink.seg.pipes.reserve(edges.len());
    ink.glyph.spheres.reserve(cx.vpos.len());
    for (i, (a, b, col)) in edges.iter().enumerate(){
        let f = edge_faces[i];

        // The two faces sharing this edge, so the shader decides visibility from the geometry
        // instead of the depth buffer: a pen has width, and ink tested against the surface it
        // decorates is either cut by it or floats in front - no offset wins at every slant.
        let facing = pack_facing(normal_of(topo, f, 0), normal_of(topo, f, 1));

        if hidden(w, i) {
            continue
        }

        // Interior tessellation, not an edge of the shape: a diagonal across a flat region
        // shares two COPLANAR faces. A border has one face and a crease two that disagree, so
        // both survive. VIEWER_ALL_EDGES brings the full tessellation back.
        if let (Some(n0), Some(n1)) = (normal_of(topo, f, 0), normal_of(topo, f, 1)) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !knobs::all_edges() {
                continue
            }
        }
        ink.seg.pipes.push(
            CylinderSegment{
                p0: cx.vpos[cx.slots.slot(*a)],
                radius: encode_width(width_at(w, i)),
                p1: cx.vpos[cx.slots.slot(*b)],
                instance_id: cx.row,
                color: if black_wire { BLACK } else { *col },
                facing,
            }
        )
    }
    cx.lap.mark("pipe loop");

    // Dots follow user-set pointcolors only (`m.vertices()` is sorted, the order to_render
    // indexes them by); the auto-seeded white vec is filtered by the mode gate.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();

    // A vertex dot must be as fat as its pipes: the kernel has no per-vertex width, so take the
    // widest incident edge and remember WHICH - the dot leads that edge's `facing` adjacency.
    // Sentinel -inf, not 0: widths can be NEGATIVE world-mm radii. One slot per vertex key.
    let nv = cx.vpos.len();
    let mut vbest = vec![(f64::NEG_INFINITY, 0usize); nv];
    for (i, (a, b, _)) in edges.iter().enumerate(){
        if hidden(w, i){ // A vertex whose every edge is hidden gets no dot either
            continue;
        }
        let wi = width_at(w, i);
        for vk in [*a, *b] {
            let e = &mut vbest[cx.slots.slot(vk)];
            if wi > e.0 {
                *e = (wi, i);
            }
        }
    }

    // Incident EDGES per vertex as CSR (degree count, prefix sum, fill). Hidden edges count
    // too: their face can still carry a visible band, and the dot must hug it to stay in front.
    let mut vstart = vec![0u32; nv + 1];
    for (a, b, _) in edges.iter(){
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }
    for i in 0..nv{
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in edges.iter().enumerate(){
        for vk in [*a, *b] {
            let s = cx.slots.slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    cx.lap.mark("vbest+vedges");

    // VIEWER_NO_DOTS: the harness can then tell how much of a dense wireframe's ink is dots.
    if knobs::no_dots() { return }

    // The row carries up to SIX normals (3 words x oct16 pair): a trihedral corner needs three,
    // and hugging only the widest edge's two lets the third face's band bite a sector out of
    // the disc at grazing slants. Widest edge's pair first, then every other incident edge's.
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    for i in 0..nv{
        let (vw, ei) = vbest[i];
        if vw == f64::NEG_INFINITY { continue }

        fkeys.clear();
        push_faces(edge_faces, ei, &mut fkeys);
        for &j in &vinc[vstart[i] as usize..vstart[i + 1] as usize] {
            push_faces(edge_faces, j as usize, &mut fkeys);
        }
        codes.clear();
        codes.extend(
            fkeys.iter()
                .filter_map(|fk| topo.normals[*fk].as_ref())
                .filter_map(oct16)
                .take(6),
        );
        ink.glyph.spheres.push(
            GlyphPoint {
                center: cx.vpos[i],
                radius: encode_width(vw),
                // No pointcolors -> a fixed near-black marker whatever the pen: the dot must
                // read as a DOT (following the pen hid it on a black-penned cube).
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: cx.row,
                facing: facing_word(&codes, 0),
                facing_ext: [facing_word(&codes, 1), facing_word(&codes, 2)],
            }
        );
    }
    cx.lap.mark("dots loop");
}
