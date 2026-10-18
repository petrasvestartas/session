/// A kernel pyramid on the base, apex along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [edge, height] = part.sizes[..] else {
        return Err("Pyramid needs an edge and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    Ok(shape::solid(
        BRep::create_pyramid(edge, height),
        option,
        &frame.to_xform(),
        "pyramid",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::tests::{XY, build, p, plane};

    /// Five faces; a clicked corner lands a base corner on the click; the apex tops it.
    #[test]
    fn a_clicked_corner_lands_on_the_click() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(13.0, 4.0, 0.0)),
            Answer::Number(15.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &top, &answers, "Brep").unwrap() else {
            panic!()
        };
        assert_eq!(brep.face_count(), 5);
        assert!(brep.is_solid());
        let points = brep.vertex_points();
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(13.0, 4.0, 0.0), None) < 1e-9)
        );
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(10.0, 0.0, 15.0), None) < 1e-9)
        );
        let typed = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(10.0),
            Answer::Number(15.0),
        ];
        let Geometry::BRep(square) = build(&SHAPE, &top, &typed, "Brep").unwrap() else {
            panic!()
        };
        assert!(
            square
                .vertex_points()
                .iter()
                .any(|q| q.distance(&p(5.0, 5.0, 0.0), None) < 1e-9)
        );
    }
}
