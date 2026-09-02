//! Mesh topology for the ink lanes, fused into one face walk: unique edges with their pen,
//! edge-to-face adjacency, face normals, closedness. Reads a kernel `Mesh`; writes no table.

use std::collections::HashMap;
use session_rust::{Mesh, Tolerance};
use super::encode::{pack_rgba, BLACK};

/// Vertex key -> slot (the key's position in the sorted `m.vertices()` order). Keys are
/// arbitrary usizes but in practice dense ids, so a Vec indexed BY KEY (u32::MAX = unused)
/// makes every lookup an array read; a sparse key space (a mesh after deletions) takes the map.
pub struct SlotMap {
    dense: Vec<u32>,
    sparse: HashMap<usize, u32>,
}

impl SlotMap {
    /// Dense when the largest key is under four times the vertex count.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);
        let dense = max_key < 4 * keys.len().max(1);
        let mut dense_vec: Vec<u32> = Vec::new();
        let mut sparse: HashMap<usize, u32> = HashMap::new();
        if dense {
            dense_vec = vec![u32::MAX; max_key + 1];
            for (s, &k) in keys.iter().enumerate() { dense_vec[k] = s as u32; }
        } else {
            sparse = keys.iter().enumerate().map(|(s, &k)| (k, s as u32)).collect();
        }
        Self { dense: dense_vec, sparse }
    }

    /// The slot of `key`. Dense path first: it is the one every CAD mesh takes.
    pub fn slot(&self, key: usize) -> usize {
        if !self.dense.is_empty() { self.dense[key] as usize } else { self.sparse[&key] as usize }
    }
}

/// Everything the ink lanes need from a mesh's faces, built in ONE pass: the unique edges with
/// their pen, each edge's two faces, the face normals, and whether the mesh is closed. The kernel
/// answers these in four passes (123 ms of the bunny's 137 ms walk); same order, same rules, same bytes.
pub struct MeshTopo {
    /// Unique edges as (low, high) vertex key + PACKED pen color, in `edges_with_colors` order -
    /// a kernel `Color` carries a String and a guid, and cloning one per edge was 104k allocations.
    pub edges: Vec<(usize, usize, u32)>,
    /// Per edge: the face walking (low, high) and the face walking (high, low), as SLOTS into
    /// `normals` (u32::MAX = none); a lone face always lands in slot 0.
    pub edge_faces: Vec<[u32; 2]>,
    /// Per face slot, in sorted-face-key order. `None` for a degenerate face.
    pub normals: Vec<Option<[f64; 3]>>,
    /// Every edge walked in BOTH directions, i.e. no border. Meshes with declared hole rings fall
    /// back to the kernel, which knows that a ring's own edges are not borders.
    pub closed: bool,
}

/// One face's normal, from the by-slot position table - no `Point`, no `Vector`, no allocation
/// and no map lookup. Same arithmetic and the same `ZERO_TOLERANCE` cut-off as `Mesh::face_normal`.
pub fn face_normal_raw(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 { return None }
    let (p0, p1, p2) = (vpos[slots.slot(vs[0])], vpos[slots.slot(vs[1])], vpos[slots.slot(vs[2])]);
    let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > Tolerance::ZERO_TOLERANCE { Some([n[0] / len, n[1] / len, n[2] / len]) } else { None }
}

/// The fused pass. No hash table: edges hang off their LOW vertex on an intrusive chain (`head`
/// per vertex slot, `next` per edge), so "does (lo, hi) exist" is a walk of the two or three
/// edges sharing `lo` - array reads, and deterministic where a HashMap's order is seeded.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    // SORTED (key, vertex list) pairs: `m.face` is a HashMap whose order changes between runs,
    // and the pen colors and packed `facing` words must come out reproducible.
    let mut faces: Vec<(usize, &Vec<usize>)> = m.face.iter().map(|(k, v)| (*k, v)).collect();
    faces.sort_unstable_by_key(|f| f.0);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()];
    let mut next: Vec<u32> = Vec::new();

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal_raw(vs, vpos, slots));
        let n = vs.len();
        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
            // dir 0 = this face walks the edge low -> high, dir 1 = high -> low. The two are the
            // two SIDES of the edge, which is exactly what the facing test needs.
            let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) };
            let ls = slots.slot(lo);
            let mut ei = head[ls];
            while ei != u32::MAX && edges[ei as usize].1 != hi {
                ei = next[ei as usize];
            }
            if ei == u32::MAX {
                ei = edges.len() as u32;
                let pen = cols.get(edges.len()).map_or(BLACK, |c| pack_rgba(c.to_f32()));
                edges.push((lo, hi, pen));
                edge_faces.push([u32::MAX; 2]);
                next.push(head[ls]);
                head[ls] = ei;
            }
            // FIRST face wins, like the kernel's `or_insert`: on a non-manifold patch two faces
            // walk the same directed edge, and last-wins would make `facing` visit-order dependent.
            let f = &mut edge_faces[ei as usize][dir];
            if *f == u32::MAX { *f = fs as u32; }
        }
    }

    // The chain is only ever used for lookup: `edges` was built in first-seen order above, which
    // is `edges_with_colors`' order and what the pen colors are indexed by. Nothing to re-sort.
    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter_mut() {
        if f[0] == u32::MAX || f[1] == u32::MAX { closed = false }
        // A lone face moves to slot 0, so a border edge's single normal is always `normal_of(0)`.
        if f[0] == u32::MAX { f[0] = f[1]; f[1] = u32::MAX; }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's, and only
    // the kernel knows the rings (rare: PDF poche fills, which return before this pass anyway).
    if !closed && !m.face_holes.is_empty() { closed = m.is_closed(); }

    MeshTopo { edges, edge_faces, normals, closed }
}
