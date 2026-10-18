shes.get(*f)?)))
                })
                .collect(),
            Parts::Mesh { sides, .. } => sides.iter().enumerate().collect(),
        }
    }
}

/// Cut `target` (in its own frame) by `blades`; an error when nothing is crossed.
pub fn cut(target: &Geometry, blades: &[Blade]) -> Result<Parts, String> {
    match target {
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_) => {
            let curve = crate::source_cut::as_curve(target)
                .ok_or("This curve has no points")?;
            let cuts = curve_cuts(&curve, blades)?;
            let parts = Parts::Curve { curve, cuts };

            if parts.count() < 2 {
                return Err("not crossed".into());
            }

            Ok(parts)
        }
        Geometry::BRep(brep) => faces(brep, blades),
        Geometry::NurbsSurface(surface) => {
            let brep = face_of(surface)?;
            faces(&brep, blades)
        }
        Geometry::Mesh(mesh) => {
            let mut planes = blades
                .iter()
                .filter_map(|blade| match &blade.cutter {
                    Cutter::Plane(plane) => Some(plane),
                    Cutter::Curve(_) => blade.fence.as_ref(),
                })
                .peekable();

            if planes.peek().is_none() {
                return Err(
                    "A mesh is cut by a plane: a Plane, a planar surface or a straight line".into(),
                );
            }

            // the first plane that passes through it
            let mut answer = Err("not crossed".to_string());

            for plane in planes {
                answer = mesh_sides(mesh, plane);

                if answer.as_ref().is_ok()
                    || answer.as_ref().is_err_and(|error| error != "not crossed")
                {
                    break;
                }
            }

            let (sides, section) = answer?;
            Ok(Parts::Mesh { sides, section })
        }
        _ => Err("Trim cuts lines, polylines, curves, surfaces, BReps and meshes".into()),
    }
}

