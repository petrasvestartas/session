mod tests {
    use crate::selection_region::{EDGE_STEPS,curve_samples,enclosed,Region};
    use session_rust::{Point,Plane};
    use session_rust::{AABB, BRep, Geometry, Line, Mesh, NurbsCurve, OBB, Polyline, Vector};
    use std::cell::Cell;

    /// Inside the square 0..10 in x and y.
    fn square(p: [f64; 3]) -> bool {
        (0.0..=10.0).contains(&p[0]) && (0.0..=10.0).contains(&p[1])
    }

    /// Even-odd by brute force: crossings left of `at` on its row's centre line.
    fn ray_cast(points: &[(f64, f64)], at: (f64, f64)) -> bool {
        let y = at.1.floor() + 0.5;
        let mut count = 0;

        for (index, a) in points.iter().enumerate() {
            let b = points[(index + 1) % points.len()];

            if (a.1 <= y && y < b.1) || (b.1 <= y && y < a.1) {
                let x = a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1);
                count += usize::from(x < at.0);
            }
        }

        count % 2 == 1
    }

    /// A square holds its centre; a U rejects its notch; a bow-tie follows even-odd; a flat loop holds nothing.
    #[test]
    fn a_loop_holds_what_it_surrounds() {
        let region = Region::new(&[(10.0, 10.0), (50.0, 10.0), (50.0, 50.0), (10.0, 50.0)]);
        assert!(region.contains((30.0, 30.0)));
        assert!(!region.contains((60.0, 30.0)));
        assert!(!region.contains((30.0, 5.0)));
        assert!(!region.contains((30.0, 55.0)));

        let u = Region::new(&[
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 30.0),
            (20.0, 30.0),
            (20.0, 0.0),
            (30.0, 0.0),
            (30.0, 40.0),
            (0.0, 40.0),
        ]);
        assert!(!u.contains((15.0, 10.0)), "the notch");
        assert!(u.contains((5.0, 10.0)));
        assert!(u.contains((15.0, 35.0)));

        let tie = Region::new(&[(0.0, 0.0), (10.0, 10.0), (10.0, 0.0), (0.0, 10.0)]);
        assert!(tie.contains((2.0, 5.0)));
        assert!(tie.contains((8.0, 5.0)));
        assert!(!tie.contains((5.0, 2.0)));

        let flat = Region::new(&[(0.0, 0.0), (5.0, 5.0), (10.0, 10.0)]);

        for at in [(5.0, 5.0), (5.5, 5.5), (2.0, 2.0), (7.0, 3.0)] {
            assert!(!flat.contains(at), "{at:?}");
        }
    }

    /// The row index answers like a ray cast over a twelve-point star.
    #[test]
    fn the_scanline_index_agrees_with_ray_casting() {
        let star: Vec<(f64, f64)> = (0..12)
            .map(|i| {
                let angle = i as f64 * std::f64::consts::PI / 6.0;
                let radius = if i % 2 == 0 { 100.0 } else { 40.0 };
                (150.0 + radius * angle.cos(), 150.0 + radius * angle.sin())
            })
            .collect();
        let region = Region::new(&star);
        let mut inside = 0;

        for x in 0..300 {
            for y in 0..300 {
                let at = (x as f64 + 0.25, y as f64 + 0.5);
                assert_eq!(region.contains(at), ray_cast(&star, at), "{at:?}");
                inside += usize::from(region.contains(at));
            }
        }

        assert!(inside > 10_000, "{inside}");
    }

    /// Points, lines, polylines, curves and meshes pass only with every sample inside.
    #[test]
    fn geometry_is_inside_only_when_every_sample_is() {
        let shape = |geometry: Geometry| enclosed(&geometry, &square);
        assert_eq!(
            shape(Geometry::Point(Point::new(5.0, 5.0, 0.0).into())),
            Some(true)
        );
        assert_eq!(
            shape(Geometry::Point(Point::new(15.0, 5.0, 0.0).into())),
            Some(false)
        );
        assert_eq!(
            shape(Geometry::Line(
                Line::new(1.0, 1.0, 0.0, 9.0, 9.0, 0.0).into()
            )),
            Some(true)
        );
        assert_eq!(
            shape(Geometry::Line(
                Line::new(1.0, 1.0, 0.0, 11.0, 9.0, 0.0).into()
            )),
            Some(false)
        );
        let corners = vec![
            Point::new(1.0, 1.0, 0.0),
            Point::new(9.0, 1.0, 0.0),
            Point::new(9.0, 9.0, 0.0),
        ];
        assert_eq!(
            shape(Geometry::Polyline(Polyline::new(corners.clone()).into())),
            Some(true)
        );
        let mut out = corners;
        out.push(Point::new(-1.0, 9.0, 0.0));
        assert_eq!(
            shape(Geometry::Polyline(Polyline::new(out).into())),
            Some(false)
        );
        assert_eq!(
            shape(Geometry::Polyline(Polyline::new(Vec::new()).into())),
            None
        );

        let bulge = [
            Point::new(1.0, 1.0, 0.0),
            Point::new(5.0, 30.0, 0.0),
            Point::new(9.0, 1.0, 0.0),
        ];
        assert_eq!(
            shape(Geometry::NurbsCurve(
                NurbsCurve::create(false, 2, &bulge).into()
            )),
            Some(false)
        );
        let low = [
            Point::new(1.0, 1.0, 0.0),
            Point::new(5.0, 9.0, 0.0),
            Point::new(9.0, 1.0, 0.0),
        ];
        assert_eq!(
            shape(Geometry::NurbsCurve(
                NurbsCurve::create(false, 2, &low).into()
            )),
            Some(true)
        );

        let shifted = |p: [f64; 3]| square([p[0] + 5.0, p[1] + 5.0, p[2]]);
        let small = Geometry::Mesh(Mesh::create_box(4.0, 4.0, 4.0).into());
        let large = Geometry::Mesh(Mesh::create_box(20.0, 20.0, 20.0).into());
        assert_eq!(enclosed(&small, &shifted), Some(true));
        assert_eq!(enclosed(&large, &shifted), Some(false));
    }

    /// A sphere's single face is judged by its surface, not by its seam; a box's flat faces add nothing.
    #[test]
    fn a_curved_face_is_judged_by_its_surface() {
        let sphere = BRep::create_sphere(5.0);
        let half = |p: [f64; 3]| p[0] >= -0.1;
        let seam: Vec<_> = sphere
            .m_curves_3d
            .iter()
            .flat_map(|curve| curve_samples(curve, EDGE_STEPS))
            .collect();
        assert!(
            !seam.is_empty() && seam.iter().all(|p| half(*p)),
            "the seam alone passes"
        );
        assert_eq!(enclosed(&Geometry::BRep(sphere.into()), &half), Some(false));

        let boxed = BRep::create_box(2.0, 2.0, 2.0);
        let calls = Cell::new(0);
        let wide = |p: [f64; 3]| {
            calls.set(calls.get() + 1);
            p.iter().all(|value| value.abs() < 100.0)
        };
        let straight = boxed.m_curves_3d.iter().all(|curve| curve.degree() == 1);
        assert_eq!(enclosed(&Geometry::BRep(boxed.into()), &wide), Some(true));
        assert!(straight, "a box's edges are straight");
        assert_eq!(
            calls.get(),
            8 + 12 * 2,
            "vertices and edge ends, no face samples"
        );
    }

    /// Planes and frames have no samples of their own.
    #[test]
    fn planes_and_frames_fall_back_to_their_box() {
        let plane =
            Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0), None);
        assert_eq!(enclosed(&Geometry::Plane(plane.into()), &square), None);
        let frame = OBB::from_aabb(&AABB::new(5.0, 5.0, 0.0, 1.0, 1.0, 1.0));
        assert_eq!(enclosed(&Geometry::OBB(frame.into()), &square), None);
    }

}
