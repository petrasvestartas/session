    let mesh = make(radius / circumradius(&make(1.0)));
    Ok(polyhedron(mesh, option, &part.frame.to_xform(), name))
}


#[cfg(test)]
pub mod tests {
    use super::*;

    /// A point from coordinates.
    pub fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// A drawing plane through the origin with axes `x` and `y`.
    pub fn plane(x: [f64; 3], y: [f64; 3]) -> Plane {
        Plane::new(
            p(0.0, 0.0, 0.0),
            Vector::new(x[0], x[1], x[2]),
            Vector::new(y[0], y[1], y[2]),
        )
    }

    pub const XY: ([f64; 3], [f64; 3]) = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]); // Top
    pub const XZ: ([f64; 3], [f64; 3]) = ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]); // Front
    pub const YZ: ([f64; 3], [f64; 3]) = ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0]); // Right

    /// Answer every question of `shape` in turn, then build it.
    pub fn build(
        shape: &Shape,
        plane: &Plane,
        answers: &[Answer],
        option: &str,
    ) -> Result<Geometry, String> {
        let frame = frame_of(plane, answers);

        for count in 1..=answers.len() {
            let part = (shape.read)(&frame, &answers[..count], option)?;
            let asked = (shape.ask)(&answers[..count], &part, option).is_some();
            assert_eq!(
                asked,
                count < answers.len(),
                "{shape:?} after {count} answers"
            );
        }

        let part = (shape.read)(&frame, answers, option)?;
        (shape.build)(&part, option)
    }

    /// Every point of a geometry: BRep vertices, mesh vertices or curve samples.
    pub fn corners(geometry: &Geometry) -> Vec<Point> {
        match geometry {
            Geometry::BRep(brep) => brep.vertex_points(),
            Geometry::Mesh(mesh) => mesh
                .vertex
                .values()
                .map(|vertex| vertex.position())
                .collect(),
            Geometry::NurbsCurve(curve) => curve.divide_by_count(RING, true).0,
            _ => Vec::new(),
        }
    }

    /// Low and high corners of the points.
    pub fn bounds(points: &[Point]) -> [[f64; 3]; 2] {
        let mut low = [f64::MAX; 3];
        let mut high = [f64::MIN; 3];

        for q in points {
            for i in 0..3 {
                low[i] = low[i].min(q[i]);
                high[i] = high[i].max(q[i]);
            }
        }

        [low, high]
    }

    /// Close within 1e-9 of the size.
    pub fn near(a: [[f64; 3]; 2], b: [[f64; 3]; 2]) -> bool {
        let scale = b.iter().flatten().fold(1.0_f64, |m, v| m.max(v.abs()));
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .all(|(x, y)| (x - y).abs() <= 1e-9 * scale)
    }

    /// Xy gives +Z, Xz gives −Y, Yz gives +X; the flipped frame is still a rotation.
    #[test]
    fn frames_are_right_handed_on_every_plane() {
        for ((x, y), z) in [
            (XY, [0.0, 0.0, 1.0]),
            (XZ, [0.0, -1.0, 0.0]),
            (YZ, [1.0, 0.0, 0.0]),
        ] {
            let frame = Frame::new(&plane(x, y), p(1.0, 2.0, 3.0));
            assert_eq!([frame.z[0], frame.z[1], frame.z[2]], z);
            let flipped = frame.flipped();
            let turn = flipped.x.cross(&flipped.y);
            assert!((turn.dot(&flipped.z) - 1.0).abs() < 1e-12);
            let rolled = frame.rolled();
            assert!((rolled.x.cross(&rolled.y).dot(&rolled.z) - 1.0).abs() < 1e-12);
            let moved = frame.to_xform().transform_point(&p(1.0, 0.0, 0.0));
            assert!(moved.distance(&frame.at(1.0, 0.0, 0.0), None) < 1e-12);
        }

        let turned = Frame::new(&plane(XY.0, XY.1), p(0.0, 0.0, 0.0)).turned(0.0, 2.0);
        assert!(turned.at(1.0, 0.0, 0.0).distance(&p(0.0, 1.0, 0.0), None) < 1e-12);
        assert!(turned.at(0.0, 1.0, 0.0).distance(&p(-1.0, 0.0, 0.0), None) < 1e-12);
    }

}
