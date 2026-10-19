at(t), direction));
        }
        _ => return None,
    };
    let mut direction = &tip - &inward;
    direction.normalize_self().then_some(())?;
    Some((tip, direction))
}

/// `geometry` grown at `end` by a distance or to the first boundary ahead.
pub fn extended(geometry: &Geometry, end: End, reach: &Reach) -> Result<Grown, String> {
    match geometry {
        Geometry::Line(line) => {
            let (from, direction) = tip(geometry, end).ok_or("This line has no length")?;
            let to = straight(&from, &direction, reach)?;
            let grow = from.distance(&to, None);
            let mut next = (**line).clone();

            match end {
                End::Start => next.extend(grow, 0.0),
                End::End => next.extend(0.0, grow),
            }

            Ok(Grown {
                geometry: Geometry::Line(Rc::new(next)),
                tail: vec![from, to],
            })
        }
        Geometry::Polyline(polyline) => {
            if polyline.is_closed() {
                return Err("no free end".into());
            }

            let (from, direction) = tip(geometry, end).ok_or("This polyline has no length")?;
            let to = straight(&from, &direction, reach)?;
            let last = polyline.point_count() - 1;
            let mut next: Polyline = (**polyline).clone();
            next.set_point(if end == End::Start { 0 } else { last }, &to);
            Ok(Grown {
                geometry: Geometry::Polyline(Rc::new(next)),
                tail: vec![from, to],
            })
        }
        Geometry::NurbsCurve(curve) => {
            if curve.is_closed() {
                return Err("no free end".into());
            }

            let grown = match reach {
                Reach::Distance(distance) => by_length(curve, end, *distance)?,
                Reach::Boundaries { cutters, .. } => to_boundary(curve, end, cutters)?,
            };
            let tail = tail(curve, &grown, end);
            let mut grown = grown;
            fit_colors(&mut grown);
            Ok(Grown {
                geometry: Geometry::NurbsCurve(Rc::new(grown)),
                tail,
            })
        }
        _ => Err("Extend grows lines, polylines and NURBS curves".into()),
    }
}

