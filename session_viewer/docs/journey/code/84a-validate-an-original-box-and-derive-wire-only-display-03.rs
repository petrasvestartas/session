use session_rust::{Mesh, OBB, Point, Xform};
use std::rc::Rc;
pub fn valid(source: &OBB) -> bool {
    let axes = [&source.x_axis, &source.y_axis, &source.z_axis];
    (0..3).all(|i| {
        source.center[i].is_finite() && source.half_size[i].is_finite() && source.half_size[i] >= 0.
    }) && axes
        .iter()
        .all(|a| (0..3).all(|i| a[i].is_finite()) && (a.magnitude() - 1.).abs() < 1e-9)
        && axes[0].dot(axes[1]).abs() < 1e-9
        && axes[0].dot(axes[2]).abs() < 1e-9
        && axes[1].dot(axes[2]).abs() < 1e-9
}
pub fn prepared(source: &OBB) -> Result<crate::prepared::PreparedMesh, &'static str> {
    if !valid(source) {
        return Err("Invalid original box frame");
    }
    let points = source.corners().to_vec();
    let mut mesh = Mesh::from_vertices_and_faces(
        points,
        vec![
            vec![0, 3, 2, 1],
            vec![4, 5, 6, 7],
            vec![0, 1, 5, 4],
            vec![1, 2, 6, 5],
            vec![2, 3, 7, 6],
            vec![3, 0, 4, 7],
        ],
    );
    mesh.set_guid(source.guid().to_owned());
    mesh.name = source.name.clone();
    mesh.is_visible = source.is_visible;
    mesh.is_locked = source.is_locked;
    let points = source.corners();
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    let endpoints = edges
        .iter()
        .map(|&(a, b)| [&points[a], &points[b]].map(|p| [p[0], p[1], p[2]]))
        .collect();
    let mut display = crate::mesh::Mesh::new(
        points
            .iter()
            .map(|p| [p[0] as f32, p[1] as f32, p[2] as f32, 0., 0., 0.])
            .collect(),
        Vec::new(),
    )?;
    display.parts = crate::mesh_parts::Parts {
        faces: Vec::new(),
        triangles: Vec::new(),
        edges,
        endpoints,
    };
    Ok(crate::prepared::PreparedMesh {
        geometry: Rc::new(mesh),
        display,
        source: None,
        model: Xform::identity(),
    })
}
