}


#[cfg(test)]
mod tests{use super::*;use crate::camera::{Camera,View};
#[test]
    fn the_text_reads_from_every_standard_view() {
        let x = Vector::x_axis();
        let y = Vector::y_axis();
        let z = Vector::z_axis();

        for (view, a, b, right, up) in [
            (View::Top, &x, &y, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            (View::Front, &x, &z, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            (View::Right, &y, &z, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
            (View::Back, &x, &z, [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            (View::Bottom, &x, &y, [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
        ] {
            let mut camera = Camera::default();
            camera.set_view(view);
            let turn = &camera.orientation;
            let found = reading_axes(
                a,
                b,
                &turn.rotate_vector(Vector::x_axis()),
                &turn.rotate_vector(Vector::z_axis()),
            );
            let close = |p: [f64; 3], q: [f64; 3]| (0..3).all(|i| (p[i] - q[i]).abs() < 1e-9);
            assert!(
                close(found.0, right) && close(found.1, up),
                "{found:?} should be {right:?} {up:?}"
            );
        }
    }

    /// The line box stands on the point, and the label shapes.
    #[test]
    fn the_line_box_stands_on_the_insertion_point() {
        let label = label(
            "Hello",
            &Point::new(1.0, 2.0, 3.0),
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            18.0,
        );
        let TextPlacement::WorldPlane { world, .. } = label.placement else {
            panic!("a plane label");
        };
        assert_eq!(world, [1.0, 28.0, 3.0]);
        assert!(
            crate::text::TextDocument::new()
                .set_labels(vec![label])
                .is_ok()
        );
    }

    /// Heights round down to 1, 2 or 5 times a power of ten.
    #[test]
    fn nice_heights_round_down() {
        assert_eq!(nice(137.0), 100.0);
        assert!((nice(0.37) - 0.2).abs() < 1e-12);
        assert_eq!(nice(5.0), 5.0);
        assert_eq!(nice(0.0), 1.0);
        assert_eq!(nice(f64::NAN), 1.0);
    }
}
