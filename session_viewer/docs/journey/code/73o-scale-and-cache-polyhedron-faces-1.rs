    mesh.name = name.into();
    Geometry::Mesh(Rc::new(mesh))
}

/// The largest distance of a vertex from the origin.
pub fn circumradius(mesh: &Mesh) -> f64 {
    mesh.vertex
        .values()
        .map(|vertex| vertex.position().distance(&Point::new(0.0, 0.0, 0.0), None))
        .fold(0.0, f64::max)
}

/// A polyhedron of circumradius 1 as face outlines, built on first use.
pub fn unit_faces(
    cache: &'static OnceLock<Vec<Vec<[f64; 3]>>>,
    make: fn(f64) -> Mesh,
) -> &'static [Vec<[f64; 3]>] {
    cache.get_or_init(|| {
        let mesh = make(1.0);
        let scale = 1.0 / circumradius(&mesh);
        mesh.face_outlines()
            .iter()
            .map(|outline| {
                outline
                    .get_points()
                    .iter()
                    .map(|p| [p[0] * scale, p[1] * scale, p[2] * scale])
                    .collect()
            })
            .collect()
    })
}

/// The unit faces at the part's radius.
pub fn polyhedron_outline(part: &Part, faces: &[Vec<[f64; 3]>]) -> Vec<Vec<Point>> {
    let [radius] = part.sizes[..] else {
        return Vec::new();
    };
    faces
        .iter()
        .map(|face| {
            face.iter()
                .map(|q| part.frame.at(q[0] * radius, q[1] * radius, q[2] * radius))
                .collect()
        })
        .collect()
}

/// The polyhedron `make` with its corners at the part's radius.
pub fn polyhedron_of(
    part: &Part,
    option: &str,
    make: fn(f64) -> Mesh,
    name: &str,
) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err(format!("The {name} needs a radius"));
    };

    // the Brep's faces join within 1e-6
    if option == "Brep" && radius < 1e-3 {
        return Err("The radius must be at least 0.001 for a Brep".into());
    }

    let mesh = make(radius / circumradius(&make(1.0)));
    Ok(polyhedron(mesh, option, &part.frame.to_xform(), name))
}


