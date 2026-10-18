head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}

/// What is left of `source` once `removed` parts are gone: one geometry per unbroken run.
pub fn kept(source: &Geometry, parts: &Parts, removed: &[usize]) -> Result<Vec<Geometry>, String> {
    let count = parts.count();

    if removed.len() >= count {
        return Err("The last part stays; use Delete".into());
    }

    match parts {
        Parts::Curve { curve, cuts } => {
            let closed = curve.is_closed();
            // start after a removed part, so a run never breaks at a closed curve's seam
            let first = if closed {
                (0..count)
                    .find(|i| removed.contains(i))
                    .map_or(0, |i| (i + 1) % count)
            } else {
                0
            };
            let mut runs: Vec<(f64, f64)> = Vec::new();
            let mut open = false; // the last run can still grow

            for step in 0..count {
                let index = (first + step) % count;

                if removed.contains(&index) {
                    open = false;
                    continue;
                }

                let (a, b) = interval(curve, cuts, index);

                match runs.last_mut() {
                    Some(run) if open => run.1 = b,
                    _ => runs.push((a, b)),
                }

                open = true;
            }

            runs.iter()
                .map(|(a, b)| {
                    rebuilt(source, curve, *a, *b)
                        .ok_or_else(|| "Cannot cut this curve".to_string())
                })
                .collect()
        }
        Parts::Faces { brep, groups, .. } => {
            let faces: Vec<usize> = removed
                .iter()
                .flat_map(|part| groups[*part].iter().copied())
                .collect();
            Ok(vec![Geometry::BRep(Rc::new(without_faces(brep, &faces)?))])
        }
        Parts::Mesh { sides, .. } => {
            let side = (0..sides.len())
                .find(|i| !removed.contains(i))
                .ok_or("The last part stays")?;
            Ok(vec![Geometry::Mesh(Rc::new(sides[side].clone()))])
        }
    }
}

