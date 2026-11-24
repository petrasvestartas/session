use super::encode::{BLACK, pack_rgba};
use session_rust::{Mesh, Tolerance};

/// Vertex key to its position in the sorted key list.
pub struct SlotMap {
    dense: Vec<u32>,                               // key indexes directly
    sparse: std::collections::HashMap<usize, u32>, // used for sparse keys
}

impl SlotMap {
    /// Build from the sorted vertex keys.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);

        // dense when keys are not too spread out
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

/// Edges, faces and normals of one mesh.
pub struct MeshTopo {
    pub edges: Vec<(usize, usize, u32)>, // (low key, high key, pen colour)
    pub edge_faces: Vec<[u32; 2]>,       // two faces per edge, u32::MAX = none
    pub opposed: Vec<bool>,              // per edge: faces wind opposite ways
    pub normals: Vec<Option<[f64; 3]>>,  // per face, None when degenerate
    pub closed: bool,                    // every edge has two faces
}

/// Unit normal of one face, None when degenerate.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }

    // Newell normal: sums over every edge
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

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}

/// Collect unique edges, their faces and the face normals.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());

    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }

    faces.sort_unstable_by_key(face_key);
    let cols = m.get_linecolors();

    let normals: Vec<_> = faces
        .iter()
        .map(|(_, vs)| face_normal(vs, vpos, slots))
        .collect();
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut dir0: Vec<u8> = Vec::new(); // direction the first face walked each edge
    let mut opposed: Vec<bool> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()]; // first edge at each low vertex
    let mut next: Vec<u32> = Vec::new(); // next edge at the same low vertex

    // Keep the kernel's outer-edge ordering for colors, widths and picking ids.
    for holes in [false, true] {
        for (fs, (key, vs)) in faces.iter().enumerate() {
            let rings: &[Vec<usize>] = if holes {
                m.face_holes.get(key).map_or(&[], Vec::as_slice)
            } else {
                std::slice::from_ref(*vs)
            };
            for ring in rings {
                // Hole rings may be stored in either direction; a face walks them opposite its border.
                let reverse = holes
                    && normals[fs]
                        .zip(face_normal(ring, vpos, slots))
                        .is_some_and(|(a, b)| {
                            a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>() > 0.0
                        });
                let n = ring.len();

                for i in 0..n {
                    let (u, v) = (ring[i], ring[(i + 1) % n]);
                    let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) }; // edge key
                    let dir = dir ^ u8::from(reverse);
                    let ls = slots.slot(lo);
                    let mut ei = head[ls];

                    // look for the edge among those at `lo`
                    while ei != u32::MAX && edges[ei as usize].1 != hi {
                        ei = next[ei as usize];
                    }

                    // new edge
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

                    // record this face on the edge, first free slot
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
        }
    }

    // closed when no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    MeshTopo {
        edges,
        edge_faces,
        opposed,
        normals,
        closed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Point, Polyline};

    fn topology(mesh: &Mesh) -> MeshTopo {
        let keys = mesh.vertices();
        let positions: Vec<_> = keys
            .iter()
            .map(|key| {
                let p = &mesh.vertex[key];
                [p.x, p.y, p.z]
            })
            .collect();
        mesh_topology(mesh, &keys, &positions, &SlotMap::new(&keys))
    }

    #[test]
    fn hole_rims_carry_both_faces_without_reordering_source_edges() {
        let loops = |z| {
            [10.0, 3.0].map(|r| {
                Polyline::new(vec![
                    Point::new(-r, -r, z),
                    Point::new(r, -r, z),
                    Point::new(r, r, z),
                    Point::new(-r, r, z),
                    Point::new(-r, -r, z),
                ])
            })
        };
        let mesh = Mesh::loft(&loops(0.0), &loops(2.0), true, false);
        assert_eq!(mesh.face_holes.len(), 2);
        let topo = topology(&mesh);
        assert!(topo.closed);
        assert_eq!(topo.edges.len(), 24);
        assert!(
            topo.edge_faces
                .iter()
                .all(|faces| !faces.contains(&u32::MAX))
        );
        assert!(topo.opposed.iter().all(|opposed| *opposed));
        let original = mesh.edges_with_colors();
        for (i, (a, b, _)) in original.iter().enumerate() {
            assert_eq!((topo.edges[i].0, topo.edges[i].1), (*a, *b));
        }
        for (face, holes) in &mesh.face_holes {
            let face_slot = mesh.faces().iter().position(|key| key == face).unwrap() as u32;
            for ring in holes {
                for i in 0..ring.len() {
                    let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                    let edge = topo
                        .edges
                        .iter()
                        .position(|(u, v, _)| (*u, *v) == (a.min(b), a.max(b)))
                        .unwrap();
                    assert!(topo.edge_faces[edge].contains(&face_slot));
                }
            }
        }
    }

    #[test]
    fn open_face_draws_its_hole_boundary() {
        let points = vec![
            Point::new(-10.0, -10.0, 0.0),
            Point::new(10.0, -10.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(-10.0, 10.0, 0.0),
            Point::new(-3.0, -3.0, 0.0),
            Point::new(3.0, -3.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(-3.0, 3.0, 0.0),
        ];
        let mut mesh = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3]]);
        mesh.set_face_holes(0, vec![vec![4, 5, 6, 7]]);
        let topo = topology(&mesh);
        assert_eq!(topo.edges.len(), 8);
        assert!(!topo.closed);
    }
}
