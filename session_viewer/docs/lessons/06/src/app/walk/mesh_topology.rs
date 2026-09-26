// --8<-- [start:step-5a]
use super::encode::{BLACK, pack_rgba};
use session_rust::{Mesh, Tolerance};

/// Kernel vertex keys can have gaps (3, 7, 250); a slot is the vertex's index 0..n in our own arrays.
pub struct SlotMap {
    dense: Vec<u32>,                               // dense[key] = slot, while keys are close together
    sparse: std::collections::HashMap<usize, u32>, // key -> slot, when they are spread out
}

impl SlotMap {
    /// `keys` must be sorted: the last one is the largest.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);

        // a plain array wins while fewer than 3 in 4 of its entries are unused
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

    /// Panics on an unknown key: every face corner must be a vertex of the mesh.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() {
            self.sparse[&k] as usize
        } else {
            self.dense[k] as usize
        }
    }
}

/// Each edge once, the faces beside it and one normal per face: what the ink and the shading need.
pub struct MeshTopo {
    pub edges: Vec<(usize, usize, u32)>, // (low key, high key, pen colour)
    pub edge_faces: Vec<[u32; 2]>,       // two faces per edge, u32::MAX = none
    pub opposed: Vec<bool>,              // the two faces walk the edge in opposite directions, as consistent winding requires
    pub normals: Vec<Option<[f64; 3]>>,  // per face, None for a zero-area face
    pub closed: bool,                    // watertight: every edge has two faces
    // --8<-- [end:step-5a]
// --8<-- [start:step-5b]
}

/// None when the polygon has no area, e.g. three corners on one line.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }

    // Newell's method: works for any polygon, even a slightly bent one, not just triangles
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
// --8<-- [end:step-5b]
// --8<-- [start:step-5c]

/// Faces sort by kernel key, so the same mesh always yields the same edge order.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}

/// Finds each edge once with a linked list per vertex, no HashMap of vertex pairs.
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
    let mut dir0: Vec<u8> = Vec::new(); // direction the first face walked each edge
    let mut opposed: Vec<bool> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()]; // head[slot] = first edge whose lower key is this vertex
    let mut next: Vec<u32> = Vec::new(); // next[edge] = the next one from the same vertex: a linked list in two arrays

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal(vs, vpos, slots));
        let n = vs.len();

        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
            let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) }; // (lo, hi) names the edge whichever way a face walks it
            let ls = slots.slot(lo);
            let mut ei = head[ls];

            // follow lo's list until an edge ends at hi
            while ei != u32::MAX && edges[ei as usize].1 != hi {
                ei = next[ei as usize];
            }

            // not found: append it and put it at the front of lo's list
            if ei == u32::MAX {
                ei = edges.len() as u32;
                let pen = match cols.get(edges.len()) { // the n-th new edge takes the n-th kernel line colour
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

            // the first two faces only; a third (a non-manifold edge) is ignored
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

    // closed = no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    // an edge of a hole ring has one face here, so ask the kernel instead
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
// --8<-- [end:step-5c]
