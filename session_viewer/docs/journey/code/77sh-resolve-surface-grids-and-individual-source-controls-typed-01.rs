         let (a, b) = (keys[i], keys[(i + 1) % keys.len()]);
                    let pair = (a.min(b), a.max(b));

                    if seen.insert(pair) {
                        if at == index {
                            return Ok(vec![pair.0, pair.1]);
                        }

                        at += 1;
                    }
                }
            }

            Err("Unknown mesh edge".into())
        }
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}

/// The surface control (u, v) pairs a target covers.
fn surface_keys(surface: &NurbsSurface, target: Target) -> Result<Vec<(usize, usize)>, String> {
    let [nu, nv] = surface.m_cv_count; // control grid size
    let all = || (0..nu).flat_map(|u| (0..nv).map(move |v| (u, v))).collect();

    match target {
        Target::Control(ControlId::Surface { surface: 0, u, v }) if u < nu && v < nv => {
            Ok(vec![(u, v)])
        }
        Target::Face(0) => Ok(all()),
        // edge 0..3: the four sides of the control grid
        Target::Edge(edge) if edge < 4 => Ok((0..nu)
            .flat_map(|u| (0..nv).map(move |v| (u, v)))
            .filter(|&(u, v)| match edge {
                0 => u == 0,
                1 => u + 1 == nu,
                2 => v == 0,
                _ => v + 1 == nv,
            })
            .collect()),
        _ => Err("Unknown surface control, boundary or face".into()),
    }
}

/// One control point of any geometry.
fn control_point(geometry: &Geometry, id: ControlId) -> Result<Point, String> {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[0], p.position[1], p.position[2]))
        .ok_or("Unknown source control".into())
}
