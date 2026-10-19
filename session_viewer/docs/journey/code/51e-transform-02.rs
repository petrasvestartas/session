use session_rust::{Point,Vector};
    use crate::gizmo::*;

    /// A ray a few pixels off the arm still grabs it.
    #[test]
    fn arm_grab_accepts_a_ray_within_the_screen_aperture() {
        let gizmo = Gizmo::new(Point::new(5.0, 45.0, 0.0));
        let from = Point::new(25.0, 44.5, 100.0);
        assert_eq!(
            gizmo.hit(&from, &Vector::new(0.0, 0.0, -1.0), 0.25),
            Some(Handle::Translate(Axis::X))
        );
    }

    const SCALE: f64 = 1.0; // one world unit per pixel

    /// A gizmo at the origin.
    fn at_origin() -> Gizmo {
        Gizmo::new(Point::new(0.0, 0.0, 0.0))
    }

    /// A ray straight down through (x, y).
    fn down(x: f64, y: f64) -> (Point, Vector) {
        (Point::new(x, y, 500.0), Vector::new(0.0, 0.0, -1.0))
    }

    /// Hub, ball, arm and arc each answer at their own place.
    #[test]
    fn the_handles_do_not_shadow_each_other() {
        let g = at_origin();
        let (f, d) = down(0.0, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::ScaleUniform));
        let (f, d) = down(BALL_AT, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Scale(Axis::X)));
        let (f, d) = down(ARM * 0.8, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Translate(Axis::X)));
        // the arcs live in the quadrant the arms do not
        let r = ARM / 2.0_f64.sqrt();
        let (f, d) = down(-r, -r);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Rotate(Axis::Z)));
        let (f, d) = down(ARM, 0.0);
        assert_eq!(
            g.hit(&f, &d, SCALE),
            Some(Handle::Translate(Axis::X)),
            "the arm tip is not an arc"
        );
        let (f, d) = down(ARM * 3.0, ARM * 3.0);
        assert_eq!(g.hit(&f, &d, SCALE), None);
    }

    /// An axis pointing at the eye cannot be grabbed.
    #[test]
    fn an_axis_seen_end_on_is_not_grabbable() {
        let g = at_origin();
        // just past the hub, where only the Z ball could answer
        let (f, d) = down(-7.0, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), None);
        let (f, d) = down(BALL_AT, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Scale(Axis::X)));
    }

    /// Handles keep their pixel size at any zoom.
    #[test]
    fn the_widget_is_screen_constant() {
        let g = at_origin();
        let (f, d) = down(BALL_AT * 2.0, 0.0);
        assert_eq!(g.hit(&f, &d, 2.0), Some(Handle::Scale(Axis::X)));
        assert_ne!(g.hit(&f, &d, 1.0), Some(Handle::Scale(Axis::X)));
    }

    /// A translate drag moves only along its axis.
    #[test]
    fn a_translate_drag_moves_along_its_axis_only() {
        let mut g = at_origin();
        let (f0, d0) = down(30.0, 0.0);
        let drag = g
            .begin(Handle::Translate(Axis::X), &f0, &d0)
            .expect("grabbed");
        let (f1, d1) = down(42.0, 0.0);
        let m = g.update(&drag, &f1, &d1, false).expect("a transform");
        assert!((m.m[12] - 12.0).abs() < 1e-9);
        assert!(m.m[13].abs() < 1e-9 && m.m[14].abs() < 1e-9);
    }

    /// A drag along the viewing direction is refused.
    #[test]
    fn a_drag_down_its_own_axis_refuses() {
        let mut g = at_origin();
        let from = Point::new(-500.0, 0.0, 0.0);
        let dir = Vector::new(1.0, 0.0, 0.0);
        assert!(g.begin(Handle::Translate(Axis::X), &from, &dir).is_none());
    }

    /// A quarter turn about Z takes the x axis onto the y axis.
    #[test]
    fn a_rotate_drag_turns_by_the_angle_swept() {
        let mut g = at_origin();
        let (f0, d0) = down(ARM, 0.0);
        let drag = g.begin(Handle::Rotate(Axis::Z), &f0, &d0).expect("grabbed");
        let (f1, d1) = down(0.0, ARM);
        let m = g.update(&drag, &f1, &d1, false).expect("a transform");
        // column 0 is the image of the x axis
        assert!((m.m[0]).abs() < 1e-9, "x.x");
        assert!((m.m[1] - 1.0).abs() < 1e-9, "x.y");
    }

    /// With Shift a turn of 60 degrees snaps to the nearest quarter turn, 90.
    #[test]
    fn a_snapped_rotate_turns_by_quarter_turns() {
        let mut g = at_origin();
        let (f0, d0) = down(ARM, 0.0);
        let drag = g.begin(Handle::Rotate(Axis::Z), &f0, &d0).expect("grabbed");
        let (f1, d1) = down(ARM * 0.5, ARM * 0.866);
        let free = g.update(&drag, &f1, &d1, false).expect("a transform");
        let snapped = g.update(&drag, &f1, &d1, true).expect("a transform");
        assert!((free.m[1] - 0.866).abs() < 1e-3, "60 degrees free");
        assert!(
            snapped.m[0].abs() < 1e-9 && (snapped.m[1] - 1.0).abs() < 1e-9,
            "90 degrees snapped"
        );
    }

    /// A scale starts at 1 and never flips.
    #[test]
    fn a_scale_is_relative_to_the_grab_and_never_collapses() {
        let mut g = at_origin();
        let (f0, d0) = down(BALL_AT, 0.0);
        let drag = g.begin(Handle::Scale(Axis::X), &f0, &d0).expect("grabbed");
        let m = g.update(&drag, &f0, &d0, false).expect("a transform");
        assert!((m.m[0] - 1.0).abs() < 1e-9, "no movement is no change");

        let (f1, d1) = down(BALL_AT * 4.0, 0.0);
        let grown = g.update(&drag, &f1, &d1, false).expect("a transform");
        assert!(
            grown.m[0] > 1.0 && grown.m[5] == 1.0 && grown.m[10] == 1.0,
            "one axis only"
        );

        let (f2, d2) = down(-BALL_AT * 4.0, 0.0);
        let flipped = g.update(&drag, &f2, &d2, false).expect("a transform");
        assert!(flipped.m[0] >= MIN_SCALE, "never mirrors");
    }

    /// A point at the gizmo centre does not move.
    #[test]
    fn transforms_act_about_the_gumball() {
        let g = Gizmo::new(Point::new(100.0, 200.0, 300.0));

        for m in [
            g.typed(Handle::Rotate(Axis::Z), 90.0),
            g.typed(Handle::ScaleUniform, 3.0),
        ] {
            let p = Point::new(100.0, 200.0, 300.0);
            let moved = m.transform_point(&p);

            for i in 0..3 {
                assert!((moved[i] - p[i]).abs() < 1e-9, "axis {i}");
            }
        }
    }

    /// Typed values give the same transforms as drags.
    #[test]
    fn typed_values_match_the_drag_and_are_clamped() {
        let g = at_origin();
        let m = g.typed(Handle::Translate(Axis::Y), 12.5);
        assert_eq!([m.m[12], m.m[13], m.m[14]], [0.0, 12.5, 0.0]);
        let z = g.typed(Handle::ScaleUniform, 0.0);
        assert!(z.m[0] >= MIN_SCALE && z.m[5] >= MIN_SCALE && z.m[10] >= MIN_SCALE);
        let fine = g.typed(Handle::Scale(Axis::X), 0.001);
        assert!(
            (fine.m[0] - 0.001).abs() < 1e-15,
            "a typed factor below the drag minimum"
        );
    }

    /// Every number box is pinned on its own handle.
    #[test]
    fn every_handle_anchor_lies_on_its_handle() {
        let g = at_origin();
        let top = Vector::new(0.0, 0.0, -1.0);
        let front = Vector::new(0.0, 1.0, 0.0);
        let side = Vector::new(1.0, 0.0, 0.0);
        let views = [
            (Handle::Translate(Axis::X), top.clone()),
            (Handle::Translate(Axis::Y), top.clone()),
            (Handle::Translate(Axis::Z), front.clone()),
            (Handle::Scale(Axis::X), top.clone()),
            (Handle::Scale(Axis::Y), top.clone()),
            (Handle::Scale(Axis::Z), front.clone()),
            (Handle::ScaleUniform, top.clone()),
            (Handle::Rotate(Axis::X), side),
            (Handle::Rotate(Axis::Y), front),
            (Handle::Rotate(Axis::Z), top),
        ];

        for (handle, dir) in views {
            // a ray through the anchor, from 500 units back
            let from = &g.handle_point(handle, SCALE) - &(&dir * 500.0);
            assert_eq!(g.hit(&from, &dir, SCALE), Some(handle), "{handle:?}");
        }
    }

    /// Typed text becomes a value, nothing, or a reason.
    #[test]
    fn typed_values_parse_per_handle() {
        let arm = Handle::Translate(Axis::X);
        let ball = Handle::Scale(Axis::Y);
        assert_eq!(typed_value(arm, " 250 "), Ok(Some(250.0)));
        assert_eq!(typed_value(Handle::Rotate(Axis::Z), "-90"), Ok(Some(-90.0)));
        assert_eq!(typed_value(arm, ""), Ok(None));
        assert_eq!(typed_value(arm, "0"), Ok(None), "no move");
        assert_eq!(typed_value(ball, "1"), Ok(None), "no scale");

        for text in ["abc", "1e999", "NaN"] {
            assert_eq!(
                typed_value(arm, text),
                Err(format!("`{text}` is not a number"))
            );
        }

        for (handle, text) in [(ball, "0"), (Handle::ScaleUniform, "-2")] {
            assert_eq!(
                typed_value(handle, text),
                Err("scale wants a factor above zero".into())
            );
        }
    }

    /// Moving the gumball closes its number box.
    #[test]
    fn a_moved_gizmo_closes_its_number_box() {
        let mut g = at_origin();
        assert_eq!(g.typing, None);
        g.typing = Some(Handle::Rotate(Axis::Z));
        g.set_origin(Point::new(1.0, 2.0, 3.0));
        assert_eq!(g.typing, None);
        assert_eq!(Handle::ScaleUniform.title(), "Scale");
        assert_eq!(Handle::Translate(Axis::X).title(), "Move X");
    }
