//! Object snapping: the candidate points near the cursor, and which one wins.
//!
//! Ranked in SCREEN space, not world space. Two candidates a metre apart in world can be one
//! pixel apart on screen, and it is the pixel distance the user is aiming with. The caller
//! supplies the projection, which keeps this module free of the camera and the GPU.
//!
//! Kind beats distance. An endpoint three pixels away wins over a point-on-edge one pixel
//! away, because a user reaching for an end and getting "somewhere along it" has to undo,
//! while the reverse is a small correction.

use session_rust::Point;

/// What a candidate is, in the order it wins ties. Lower wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SnapKind {
    End,
    Vertex,
    Mid,
    Center,
    Near,
}

#[derive(Clone, Debug)]
pub struct Snap {
    pub point: Point,
    pub kind: SnapKind,
    /// Which object it came from, so a caller can say what it snapped to.
    pub owner: u32,
}

/// The ends, vertices and midpoints of one open or closed polyline.
///
/// A two-point line has one midpoint and two ends and no interior vertex; a closed loop has no
/// ends at all, which is why the caller says whether it is closed rather than this guessing
/// from coincident endpoints.
pub fn from_polyline(points: &[Point], closed: bool, owner: u32, out: &mut Vec<Snap>) {
    if points.is_empty() {
        return;
    }
    let last = points.len() - 1;
    for (i, p) in points.iter().enumerate() {
        let interior = i != 0 && i != last;
        let kind = if closed || interior {
            SnapKind::Vertex
        } else {
            SnapKind::End
        };
        out.push(Snap {
            point: p.clone(),
            kind,
            owner,
        });
    }
    let spans = if closed { points.len() } else { last };
    for i in 0..spans {
        let a = &points[i];
        let b = &points[(i + 1) % points.len()];
        out.push(Snap {
            point: Point::new(
                (a[0] + b[0]) * 0.5,
                (a[1] + b[1]) * 0.5,
                (a[2] + b[2]) * 0.5,
            ),
            kind: SnapKind::Mid,
            owner,
        });
    }
}

/// The point on a segment nearest a world position, as a `Near` candidate.
///
/// Clamped to the segment: the nearest point on an infinite line can be nowhere near the edge
/// the user is pointing at.
pub fn nearest_on_segment(a: &Point, b: &Point, to: &Point, owner: u32) -> Snap {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ap = [to[0] - a[0], to[1] - a[1], to[2] - a[2]];
    let len2 = ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2];
    let t = if len2 > 0.0 {
        ((ap[0] * ab[0] + ap[1] * ab[1] + ap[2] * ab[2]) / len2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    Snap {
        point: Point::new(a[0] + ab[0] * t, a[1] + ab[1] * t, a[2] + ab[2] * t),
        kind: SnapKind::Near,
        owner,
    }
}

/// The winning candidate within `aperture` pixels of the cursor, or none.
///
/// `project` returns the screen position of a world point, or `None` when it is behind the eye
/// or otherwise off the frustum - a candidate that cannot be seen cannot be snapped to.
pub fn best<F>(candidates: &[Snap], cursor: (f64, f64), aperture: f64, project: F) -> Option<Snap>
where
    F: Fn(&Point) -> Option<(f64, f64)>,
{
    let mut winner: Option<(SnapKind, f64, &Snap)> = None;
    for c in candidates {
        let Some((x, y)) = project(&c.point) else {
            continue;
        };
        let (dx, dy) = (x - cursor.0, y - cursor.1);
        let d = (dx * dx + dy * dy).sqrt();
        if d > aperture {
            continue;
        }
        let better = match winner {
            None => true,
            Some((kind, best_d, _)) => c.kind < kind || (c.kind == kind && d < best_d),
        };
        if better {
            winner = Some((c.kind, d, c));
        }
    }
    winner.map(|(_, _, c)| c.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f64, y: f64) -> Point {
        Point::new(x, y, 0.0)
    }

    /// Flat projection: x and y are already screen pixels, everything is visible.
    fn flat(point: &Point) -> Option<(f64, f64)> {
        Some((point[0], point[1]))
    }

    /// An open polyline has two ends, its interior points are vertices, and every span has a
    /// midpoint.
    #[test]
    fn an_open_polyline_offers_ends_vertices_and_midpoints() {
        let mut out = Vec::new();
        from_polyline(
            &[p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0)],
            false,
            7,
            &mut out,
        );
        let count = |k: SnapKind| out.iter().filter(|s| s.kind == k).count();
        assert_eq!(count(SnapKind::End), 2);
        assert_eq!(count(SnapKind::Vertex), 1);
        assert_eq!(count(SnapKind::Mid), 2);
        assert!(out.iter().all(|s| s.owner == 7));
    }

    /// A closed loop has no ends, and it has a midpoint for the closing span too.
    #[test]
    fn a_closed_loop_has_no_ends() {
        let mut out = Vec::new();
        from_polyline(
            &[p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0)],
            true,
            0,
            &mut out,
        );
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::End).count(), 0);
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::Vertex).count(), 3);
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::Mid).count(), 3);
    }

    /// Kind beats distance: a nearer point-on-edge loses to an endpoint inside the aperture.
    #[test]
    fn kind_wins_before_distance() {
        let candidates = vec![
            Snap {
                point: p(2.0, 0.0),
                kind: SnapKind::Near,
                owner: 0,
            },
            Snap {
                point: p(6.0, 0.0),
                kind: SnapKind::End,
                owner: 0,
            },
        ];
        let best = best(&candidates, (0.0, 0.0), 12.0, flat).expect("a snap");
        assert_eq!(best.kind, SnapKind::End);
    }

    /// Within one kind, the nearest wins.
    #[test]
    fn distance_decides_within_a_kind() {
        let candidates = vec![
            Snap {
                point: p(9.0, 0.0),
                kind: SnapKind::End,
                owner: 1,
            },
            Snap {
                point: p(3.0, 0.0),
                kind: SnapKind::End,
                owner: 2,
            },
        ];
        assert_eq!(best(&candidates, (0.0, 0.0), 12.0, flat).unwrap().owner, 2);
    }

    /// Nothing outside the aperture, and nothing the projection cannot see.
    #[test]
    fn out_of_reach_and_out_of_sight_do_not_snap() {
        let candidates = vec![Snap {
            point: p(40.0, 0.0),
            kind: SnapKind::End,
            owner: 0,
        }];
        assert!(
            best(&candidates, (0.0, 0.0), 12.0, flat).is_none(),
            "too far"
        );
        let near = vec![Snap {
            point: p(1.0, 0.0),
            kind: SnapKind::End,
            owner: 0,
        }];
        assert!(
            best(&near, (0.0, 0.0), 12.0, |_| None).is_none(),
            "not visible"
        );
    }

    /// The nearest point on a segment stays on it, at either end when the target is past it.
    #[test]
    fn nearest_on_a_segment_stays_on_the_segment() {
        let (a, b) = (p(0.0, 0.0), p(10.0, 0.0));
        let middle = nearest_on_segment(&a, &b, &p(4.0, 5.0), 0);
        assert!((middle.point[0] - 4.0).abs() < 1e-12 && middle.point[1].abs() < 1e-12);
        let past = nearest_on_segment(&a, &b, &p(50.0, 5.0), 0);
        assert!((past.point[0] - 10.0).abs() < 1e-12);
        let before = nearest_on_segment(&a, &b, &p(-50.0, 5.0), 0);
        assert!(before.point[0].abs() < 1e-12);
    }
}
