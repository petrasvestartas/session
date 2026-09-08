//! One pass over a mesh's faces for everything the ink lanes need: the unique edges with
//! their pen colours, each edge's two faces, the face normals, and whether the mesh is
//! closed. Byte-identical to the kernel's four separate passes, without their hash tables.

use super::encode::{BLACK, pack_rgba};
use session_rust::{Mesh, Tolerance};

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
            return Self {
                dense,
                sparse: std::collections::HashMap::new(),
            };
        }
        let mut sparse = std::collections::HashMap::with_capacity(keys.len());
        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }
        Self {
            dense: Vec::new(),
            sparse,
        }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() {
            self.sparse[&k] as usize
        } else {
            self.dense[k] as usize
        }
    }
}

/// The fused topology of one mesh.
pub struct MeshTopo {
    /// Unique edges as (low key, high key, packed pen colour), in first-seen order.
    pub edges: Vec<(usize, usize, u32)>,
    /// Per edge: the two faces that meet there, in arrival order; u32::MAX = none. A lone face
    /// always sits in slot 0.
    pub edge_faces: Vec<[u32; 2]>,
    /// Per edge: the two faces walk it in OPPOSITE directions, which is what consistent
    /// winding means locally. False says the pair disagrees, so one of the two normals points
    /// into the solid and the facing test must negate it. A lone face is trivially true.
    pub opposed: Vec<bool>,
    /// Per face slot, in sorted-face-key order; None for a degenerate face.
    pub normals: Vec<Option<[f64; 3]>>,
    /// Every edge has two faces: no border. Deliberately NOT "walked in both directions" -
    /// that reading called a badly wound solid open and threw the facing cull away wholesale.
    pub closed: bool,
}

/// One face's normal from the by-slot position table, the kernel's arithmetic and cut-off.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }
    // Newell, not the cross product of the first three corners: that one inverts on a face
    // whose second corner is reflex, and the ink pass reads these normals to decide whether
    // two faces are coplanar - an inverted one turns a flat region into a crease.
    let mut n = [0.0f64; 3];
    for i in 0..vs.len() {
        let a = vpos[slots.slot(vs[i])];
        let b = vpos[slots.slot(vs[(i + 1) % vs.len()])];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}

/// Sort source faces by their stable key before assigning display face slots.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}

/// The fused pass. Edges hang off their LOW vertex on an intrusive chain (`head` per slot,
/// `next` per edge), so finding an existing (lo, hi) is a walk of two or three entries.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());
    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }
    faces.sort_unstable_by_key(face_key);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    // The direction slot 0's face walked the edge, and whether slot 1's face walked it back.
    let mut dir0: Vec<u8> = Vec::new();
    let mut opposed: Vec<bool> = Vec::new();
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
                let pen = match cols.get(edges.len()) {
                    Some(color) => pack_rgba(color.to_f32()),
                    None => BLACK,
                };
                edges.push((lo, hi, pen));
                edge_faces.push([u32::MAX; 2]);
                dir0.push(dir);
                opposed.push(true);
                next.push(head[ls]);
                head[ls] = ei;
            }
            // Slot by ARRIVAL, not by direction. Slotting by direction sent two same-wound
            // faces to the same slot and left the other empty, so an edge inside a badly wound
            // solid read as a border: the mesh was declared open and the facing cull dropped.
            // Here the pair survives and `opposed` records that its winding disagrees.
            let ef = &mut edge_faces[ei as usize];
            if ef[0] == u32::MAX {
                ef[0] = fs as u32;
                dir0[ei as usize] = dir;
            } else if ef[1] == u32::MAX && ef[0] != fs as u32 {
                ef[1] = fs as u32;
                opposed[ei as usize] = dir != dir0[ei as usize];
            }
        }
    }

    // Slot 0 is filled the moment an edge is created, so only slot 1 can be empty.
    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's.
    if !closed && !m.face_holes.is_empty() {
        closed = m.is_closed();
    }

    MeshTopo {
        edges,
        edge_faces,
        opposed,
        normals,
        closed,
    }
}
