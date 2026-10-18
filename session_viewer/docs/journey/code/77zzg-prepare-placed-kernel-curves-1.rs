/// A line, polyline or NURBS curve moved into the world by `place`; None for anything else.
pub fn picked(geometry: &Geometry, place: &Xform) -> Option<Picked> {
    let corners = match geometry {
        Geometry::Line(line) => Some(vec![line.start(), line.end()]),
        Geometry::Polyline(polyline) => {
            let mut polyline = (**polyline).clone();
            polyline.remove_consecutive_duplicates(1e-12);
            Some(polyline.get_points())
        }
        Geometry::NurbsCurve(_) => None,
        _ => return None,
    };
    let corners: Option<Vec<Point>> =
        corners.map(|points| points.iter().map(|p| p.transformed(place)).collect());
    let mut curve = match (&corners, geometry) {
        (Some(points), _) if points.len() >= 2 => NurbsCurve::create(false, 1, points),
        (None, Geometry::NurbsCurve(curve)) => {
            let mut curve = (**curve).clone();

            // loft and sweep cannot mix periodic and clamped knots
            if curve.is_periodic() {
                curve.clamp_end(2);
            }

            curve.transform(place);
            curve
        }
        _ => return None,
    };

    if !curve.is_valid() || !curve.set_domain(0.0, 1.0) {
        return None;
    }

    Some(Picked { curve, corners })
}

/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}

impl Picked {
    /// Points along the curve: its corners, else even samples.
    pub fn outline(&self) -> Vec<Point> {
        match &self.corners {
            Some(corners) => corners.clone(),
            None => self.curve.divide_by_count(SAMPLES, true).0,
        }
    }
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
pub fn no_axis(_input:&Input)->(Point,Vector){(Point::new(0.,0.,0.),Vector::new(0.,0.,1.))}
