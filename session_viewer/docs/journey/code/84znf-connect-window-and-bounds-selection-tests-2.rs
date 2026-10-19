/// Vertices, edges and curved faces; a flat face is bounded by its edges.
fn brep_samples(brep: &BRep) -> impl Iterator<Item = [f64; 3]> + '_ {
    let vertices = brep.m_vertices.iter().map(|vertex| xyz(&vertex.point));
    let edges = brep
        .m_curves_3d
        .iter()
        .flat_map(|curve| curve_samples(curve, EDGE_STEPS));
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Line;

    /// The rectangle 10..50 by 10..50, the plan seen from above one pixel per unit.
    fn rectangle() -> Region {
        Region::rectangle((50.0, 50.0), (10.0, 10.0))
    }

    fn plan(p: [f64; 3]) -> Option<(f64, f64)> {
        Some((p[0], p[1]))
    }

    /// Either corner first gives one rectangle; segments meet it inside, across or not at all.
    #[test]
    fn a_rectangle_holds_and_crosses() {
        let region = rectangle();
        assert!(region.contains((30.0, 30.0)) && !region.contains((60.0, 30.0)));
        assert!(
            region.crosses((0.0, 30.0), (60.0, 30.0)),
            "straight through"
        );
        assert!(region.crosses((20.0, 20.0), (25.0, 25.0)), "inside");
        assert!(!region.crosses((0.0, 0.0), (0.0, 60.0)), "beside");
        assert!(!region.crosses((0.0, 70.0), (70.0, 55.0)), "below");
        assert!(
            !Region::new(&[(0.0, 0.0), (9.0, 0.0), (9.0, 9.0)]).crosses((0.0, 5.0), (9.0, 5.0)),
            "a lasso has no rectangle"
        );
    }

    /// A line half inside: a crossing takes it, a window does not; a line through with both ends out is a crossing too.
    #[test]
    fn a_window_takes_what_is_inside_a_crossing_what_it_touches() {
        let region = rectangle();
        let window = |geometry: &Geometry| {
            enclosed(geometry, &|p| plan(p).is_some_and(|at| region.contains(at)))
        };
        let crossing = |geometry: &Geometry| touched(geometry, &plan, &region);

        let half = Geometry::Line(Line::new(30.0, 30.0, 0.0, 80.0, 30.0, 0.0).into());
        assert_eq!((window(&half), crossing(&half)), (Some(false), Some(true)));

        let through = Geometry::Line(Line::new(0.0, 30.0, 0.0, 80.0, 30.0, 0.0).into());
        assert_eq!(
            (window(&through), crossing(&through)),
            (Some(false), Some(true))
        );

        let inside = Geometry::Line(Line::new(20.0, 20.0, 0.0, 40.0, 40.0, 0.0).into());
        assert_eq!(
            (window(&inside), crossing(&inside)),
            (Some(true), Some(true))
        );

        let away = Geometry::Line(Line::new(60.0, 60.0, 0.0, 90.0, 90.0, 0.0).into());
        assert_eq!((window(&away), crossing(&away)), (Some(false), Some(false)));

        // a box bigger than the rectangle: its edges cross it
        let around = Geometry::Mesh(Mesh::create_box(100.0, 100.0, 1.0).into());
        let shifted = |p: [f64; 3]| plan([p[0] + 30.0, p[1] + 30.0, p[2]]);
        assert_eq!(
            touched(&around, &shifted, &region),
            Some(false),
            "only the outline; the rectangle sits inside the faces"
        );
        let crossing_box = |p: [f64; 3]| plan([p[0] + 80.0, p[1] + 30.0, p[2]]);
        assert_eq!(touched(&around, &crossing_box, &region), Some(true));
    }
}
