rom(reverse);
                    table.add(slots.slot(lo), lo, hi, dir, fs, cols);
                }
            }
        }
    }

    // closed when no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in table.edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    MeshTopo {
        edges: table.edges,
        edge_faces: table.edge_faces,
        opposed: table.opposed,
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
