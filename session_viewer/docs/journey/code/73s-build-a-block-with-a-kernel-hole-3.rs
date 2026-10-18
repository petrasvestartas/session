/// A kernel block centered on the axis, standing on the plane, the hole along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [length, width, height, hole] = part.sizes[..] else {
        return Err("Block With Hole needs its sides and a hole radius".into());
    };
    let place = &part.frame.to_xform() * &Xform::translation(0.0, 0.0, height * 0.5);
    let brep = BRep::create_block_with_hole(length, width, height.abs(), hole);
    Ok(shape::solid(brep, option, &place, "block with hole"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::tests::{XY, bounds, build, corners, near, p, plane};

    /// Seven faces standing on the plane; a hole as wide as the block is refused.
    #[test]
    fn the_block_stands_on_the_plane() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(600.0),
            Answer::Number(400.0),
            Answer::Number(200.0),
            Answer::Number(100.0),
        ];
        let made = build(&SHAPE, &top, &answers, "Brep").unwrap();
        let Geometry::BRep(brep) = &made else {
            panic!()
        };
        assert_eq!(brep.face_count(), 7);
        assert!(brep.is_solid());
        assert!(near(
            bounds(&corners(&made)),
            [[-300.0, -200.0, 0.0], [300.0, 200.0, 200.0]]
        ));
        let corner = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(300.0, 200.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(100.0),
        ];
        assert!(build(&SHAPE, &top, &corner, "Mesh").is_ok());
        let mut wide = answers.clone();
        wide[4] = Answer::Number(200.0);
        assert!(build(&SHAPE, &top, &wide, "Brep").is_err());
    }
}
