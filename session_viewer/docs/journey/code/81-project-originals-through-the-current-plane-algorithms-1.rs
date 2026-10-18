() <= 1e-12 {
        return Err("The three points lie on one line; pick a point off the line".into());
    }

    Ok(Plane::from_points(vec![a.clone(), b.clone(), c.clone()]))
}

/// Why an object is left as it is.
#[derive(Debug, PartialEq)]
pub enum Skip {
    Collapsed,   // every point would land on one
    Unsupported, // not a point, line, polyline, curve or mesh
}

/// The object's geometry projected onto a world plane, in its own frame under `place`.
pub fn projected(geometry: &Geometry, place: &Xform, plane: &Plane) -> Result<Geometry, Skip> {
    let back = place.inverse().ok_or(Skip::Unsupported)?;
    let local = &(&back * &Xform::project_to_plane(plane)) * place; // the projection in the object's frame
    let (before, after, geometry) = match geometry {
        Geometry::Point(point) => return Ok(Geometry::Point(Rc::new(point.transformed(&local)))),
        Geometry::Line(line) => {
            let out = line.transformed(&local);
            (
                vec![line.start(), line.end()],
                vec![out.start(), out.end()],
                Geometry::Line(Rc::new(out)),
            )
        }
        Geometry::Polyline(polyline) => {
            let out = polyline.transformed(&local);
            (
                polyline.get_points(),
                out.get_points(),
                Geometry::Polyline(Rc::new(out)),
            )
        }
        Geometry::NurbsCurve(curve) => {
            let out = curve.transformed(&local);
            let cvs = |curve: &session_rust::NurbsCurve| {
                (0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect()
            };
            (cvs(curve), cvs(&out), Geometry::NurbsCurve(Rc::new(out)))
        }
        Geometry::Mesh(mesh) => {
            let mut out = mesh.transformed(&local);

            // the old normals no longer fit the flat faces
            for vertex in out.vertex.values_mut() {
                for key in ["nx", "ny", "nz"] {
                    vertex.attributes.remove(key);
                }
            }

            let points = |mesh: &session_rust::Mesh| {
                mesh.vertex
                    .values()
                    .map(|vertex| vertex.position())
                    .collect()
            };
            (points(mesh), points(&out), Geometry::Mesh(Rc::new(out)))
        }
        _ => return Err(Skip::Unsupported),
    };

    if extent(&after) <= 1e-9 * extent(&before) {
        return Err(Skip::Collapsed);
    }

    Ok(geometry)
}

/// The largest distance from the first point to any other.
fn extent(points: &[Point]) -> f64 {
    let Some(first) = points.first() else {
        return 0.0;
    };
    points
        .iter()
        .map(|point| first.distance(point, None))
        .fold(0.0, f64::max)
}

