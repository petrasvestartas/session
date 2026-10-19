/// Three great circles.
pub fn sphere_outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius] = part.sizes[..] else {
        return Vec::new();
    };
    let side = part.frame.rolled();
    vec![
        part.frame.ring(radius, 0.0),
        side.ring(radius, 0.0),
        side.rolled().ring(radius, 0.0),
    ]
}

/// One mesh of every part's faces; corners within a billionth of the size share a vertex.
pub fn to_mesh(parts: &[Mesh]) -> Mesh {
    let polygons: Vec<Vec<Point>> = parts
        .iter()
        .flat_map(|part| part.face_outlines())
        .map(|outline| {
            let mut points = outline.get_points();
            points.pop(); // the closing point
            points
        })
        .collect();
    let size = polygons.iter().flatten().fold(0.0_f64, |size, p| {
        size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs())
    });
    let step = (size * 1e-9).max(1e-12);
    let mut cells: HashMap<[i64; 3], Point> = HashMap::new();
    let welded = polygons
        .into_iter()
        .map(|polygon| {
            polygon
                .into_iter()
                .map(|p| weld(&mut cells, p, step))
                .collect()
        })
        .collect();
    Mesh::from_polylines(welded, None)
}

/// The point already stored within `step` of `p` in this or a neighbouring cell, else `p`.
fn weld(cells: &mut HashMap<[i64; 3], Point>, p: Point, step: f64) -> Point {
    let key = [0, 1, 2].map(|i| (p[i] / step).round() as i64);

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if let Some(q) = cells.get(&[key[0] + dx, key[1] + dy, key[2] + dz])
                    && q.distance(&p, None) <= step
                {
                    return q.clone();
                }
            }
        }
    }

    cells.insert(key, p.clone());
    p
}

/// The BRep placed by `place`, or its tessellation for the Mesh option.
pub fn solid(mut brep: BRep, option: &str, place: &Xform, name: &str) -> Geometry {
    brep.transform(place);

    if option == "Mesh" {
        let mut mesh = to_mesh(&brep.face_meshes_q(Some(MESH_QUALITY)));
        mesh.name = name.into();
        return Geometry::Mesh(Rc::new(mesh));
    }

    brep.name = name.into();
    Geometry::BRep(Rc::new(brep))
}

/// The mesh placed by `place`, or a BRep of its planar faces for the Brep option.
pub fn polyhedron(mut mesh: Mesh, option: &str, place: &Xform, name: &str) -> Geometry {
    mesh.transform(place);

    if option == "Brep" {
        let mut brep = BRep::from_polylines(&mesh.face_outlines(), &[]);
        brep.name = name.into();
        return Geometry::BRep(Rc::new(brep));
    }

    mesh.name = name.into();
    Geometry::Mesh(Rc::new(mesh))
}

