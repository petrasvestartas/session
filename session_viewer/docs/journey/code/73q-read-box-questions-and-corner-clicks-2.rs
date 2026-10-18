    Ok(shape::solid(
        BRep::create_box(length, width, height.abs()),
        option,
        &place,
        "box",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::tests::{XY, XZ, bounds, build, corners, near, p, plane};

    /// Typed sizes and a corner click give the same box; a negative height goes below the plane.
    #[test]
    fn typed_sizes_and_a_corner_give_the_box() {
        let top = plane(XY.0, XY.1);
        let typed = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(50.0),
            Answer::Number(30.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &top, &typed, "Brep").unwrap() else {
            panic!()
        };
        assert_eq!(brep.face_count(), 6);
        assert!(brep.is_solid());
        assert!(near(
            bounds(&brep.vertex_points()),
            [[-50.0, -25.0, 0.0], [50.0, 25.0, 30.0]]
        ));
        let corner = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(50.0, 25.0, 0.0)),
            Answer::Number(30.0),
        ];
        let clicked = build(&SHAPE, &top, &corner, "Brep").unwrap();
        assert!(near(
            bounds(&corners(&clicked)),
            [[-50.0, -25.0, 0.0], [50.0, 25.0, 30.0]]
        ));
        let below = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(50.0),
            Answer::Number(-30.0),
        ];
        let low = build(&SHAPE, &top, &below, "Brep").unwrap();
        assert!(near(
            bounds(&corners(&low)),
            [[-50.0, -25.0, -30.0], [50.0, 25.0, 0.0]]
        ));
        let Geometry::Mesh(mesh) = build(&SHAPE, &top, &typed, "Mesh").unwrap() else {
            panic!()
        };
        assert_eq!(mesh.number_of_faces(), 6);
        assert!(mesh.is_closed());
    }

    /// On the Front plane the height runs along −Y.
    #[test]
    fn the_front_plane_stands_the_box_along_minus_y() {
        let front = plane(XZ.0, XZ.1);
        let typed = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(50.0),
            Answer::Number(30.0),
        ];
        let made = build(&SHAPE, &front, &typed, "Brep").unwrap();
        assert!(near(
            bounds(&corners(&made)),
            [[-50.0, -30.0, -25.0], [50.0, 0.0, 25.0]]
        ));
    }

    /// Zero sizes and a corner on an axis are refused.
    #[test]
    fn zero_sizes_are_refused() {
        let frame = Frame::new(&plane(XY.0, XY.1), p(0.0, 0.0, 0.0));
        let center = Answer::Point(p(0.0, 0.0, 0.0));
        assert!(read(&frame, &[center.clone(), Answer::Number(0.0)], "Brep").is_err());
        assert!(
            read(
                &frame,
                &[center.clone(), Answer::Point(p(10.0, 0.0, 0.0))],
                "Brep"
            )
            .is_err()
        );
        assert!(
            read(
                &frame,
                &[
                    center,
                    Answer::Number(10.0),
                    Answer::Number(5.0),
                    Answer::Number(0.0)
                ],
                "Brep"
            )
            .is_err()
        );
    }
}
