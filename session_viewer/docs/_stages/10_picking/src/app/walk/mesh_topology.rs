//! One pass over a mesh's faces for everything the ink lanes need: the unique edges with
//! their pen colours, each edge's two faces, the face normals, and whether the mesh is
//! closed. Byte-identical to the kernel's four separate passes, without their hash tables.

use session_rust::{Mesh, Tolerance};
use super::encode::{pack_rgba, BLACK};

/// Vertex key -> slot (the key's position in the sorted key list). Dense keys index a Vec;
/// a sparse key space falls back to a map.
pub struct SlotMap {
    dense: Vec<u32>,
    sparse: std::collections::HashMap<usize, u32>,
}

impl SlotMap {
    /// From the sorted keys `Mesh::vertices()` emits.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);
        if max_key < 4 * keys.len().max(1) {
            let mut dense = vec![u32::MAX; max_key + 1];
            for (s, &k) in keys.iter().enumerate() {
                dense[k] = s as u32;
            }
            return Self { dense, sparse: std::collections::HashMap::new() };
        }
        let mut sparse = std::collections::HashMap::with_capacity(keys.len());
        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }
        Self { dense: Vec::new(), sparse }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() { self.sparse[&k] as usize } else { self.dense[k] as usize }
    }
}

/// The fused topology of one mesh.
pub struct MeshTopo {
    /// Unique edges as (low key, high key, packed pen colour), in first-seen order.
    pub edges: Vec<(usize, usize, u32)>,
    /// Per edge: the face slots walking (low, high) and (high, low); u32::MAX = none. A lone
    /// face always sits in slot 0.
    pub edge_faces: Vec<[u32; 2]>,
    /// Per face slot, in sorted-face-key order; None for a degenerate face.
    pub normals: Vec<Option<[f64; 3]>>,
    /// Every edge walked in both directions: no border.
    pub closed: bool,
}

/// One face's normal from the by-slot position table, the kernel's arithmetic and cut-off.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }
    let (p0, p1, p2) = (vpos[slots.slot(vs[0])], vpos[slots.slot(vs[1])], vpos[slots.slot(vs[2])]);
    let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > Tolerance::ZERO_TOLERANCE { Some([n[0] / len, n[1] / len, n[2] / len]) } else { None }
}

/// The fused pass. Edges hang off their LOW vertex on an intrusive chain (`head` per slot,
/// `next` per edge), so finding an existing (lo, hi) is a walk of two or three entries.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());
    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }
    faces.sort_unstable_by_key(|f| f.0);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()];
    let mut next: Vec<u32> = Vec::new();

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal(vs, vpos, slots));
        let n = vs.len();
        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
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
            // First face wins, like the kernel's `or_insert`.
            let f = &mut edge_faces[ei as usize][dir];
            if *f == u32::MAX {
                *f = fs as u32;
            }
        }
    }

    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter_mut() {
        if f[0] == u32::MAX || f[1] == u32::MAX {
            closed = false;
        }
        if f[0] == u32::MAX {
            f[0] = f[1];
            f[1] = u32::MAX;
        }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's.
    if !closed && !m.face_holes.is_empty() {
        closed = m.is_closed();
    }

    MeshTopo { edges, edge_faces, normals, closed }
}
