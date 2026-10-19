use crate::clipping_uniform::*;
use crate::clipping_geometry::{ClipPlane,Mode,plane_from,clip_plane,dot,array};
use session_rust::{Xform,Point,Vector};
fn view(
    eye: [f64; 3],
    target: [f64; 3],
    anchor: [f64; 3],
    perspective: bool,
) -> Xform {
    let at = |p: [f64; 3]| Point::new(p[0] - anchor[0], p[1] - anchor[1], p[2] - anchor[2]);
    let d = [target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]];
    let distance = dot(d, d).sqrt();
    // straight up or down: y is up on screen
    let up = if d[0].abs() + d[1].abs() < 1e-9 * distance {
        Vector::new(0.0, 1.0, 0.0)
    } else {
        Vector::new(0.0, 0.0, 1.0)
    };
    let look = Xform::look_at_right_handed(&at(eye), &at(target), &up);
    let projection = if perspective {
        Xform::perspective(60f64.to_radians(), 1.0, distance * 10.0, distance * 1e-3)
    } else {
        let h = distance * 30f64.to_radians().tan();
        Xform::orthographic(-h, h, -h, h, distance * 3.0, -distance * 3.0)
    };
    &projection * &look
}

fn uniform_for(plane: &ClipPlane, view_proj: &Xform, anchor: [f64; 3]) -> ClipUniform {
    clip_uniform(
        std::slice::from_ref(plane),
        0,
        &ClipView {
            view_proj,
            anchor,
            height: 512,
            pixel_scale: 1.0,
            samples: 4,forward_depth:false,
        },
    )
}
#[test]
fn no_plane_is_all_zero() {
    let eye = view([0.0, -500.0, 300.0], [0.0; 3], [0.0; 3], true);
    let u = clip_uniform(
        &[],
        1,
        &ClipView {
            view_proj: &eye,
            anchor: [0.0; 3],
            height: 512,
            pixel_scale: 2.0,
            samples: 4,forward_depth:false,
        },
    );
    assert_eq!(u, ClipUniform::default());
}
#[test]
fn anchored_planes_match_the_world() {
    let anchor = [1.0e6, -3.0e5, 2.0e3];
    let origin = [anchor[0] + 3.0, anchor[1] - 2.0, anchor[2] + 1.0];
    let normal = [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 2.0];
    let plane = plane_from(Mode::Normal, &[origin, normal], 100.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let camera = view(
        [anchor[0] + 400.0, anchor[1] - 900.0, anchor[2] + 500.0],
        anchor,
        anchor,
        true,
    );
    let u = uniform_for(&clip, &camera, anchor);

    for i in 0..50 {
        let t = f64::from(i);
        let p = [
            anchor[0] + (t * 7.3).sin() * 50.0,
            anchor[1] + (t * 3.1).cos() * 50.0,
            anchor[2] + (t * 1.7).sin() * 50.0,
        ];
        let local = [
            (p[0] - anchor[0]) as f32,
            (p[1] - anchor[1]) as f32,
            (p[2] - anchor[2]) as f32,
        ];
        let k = u.planes[0];
        let gpu = k[0] * local[0] + k[1] * local[1] + k[2] * local[2] + k[3];
        assert!(
            (f64::from(gpu) - clip.distance(p)).abs() < 1e-3,
            "point {i}"
        );
    }
}
#[test]
fn screen_planes_vanish_on_the_plane() {
    let plane = plane_from(
        Mode::Normal,
        &[[10.0, 20.0, 30.0], [11.0, 21.0, 32.0]],
        100.0,
    )
    .unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let anchor = [5.0, 5.0, 5.0];

    for perspective in [true, false] {
        let camera = view(
            [300.0, -400.0, 250.0],
            [10.0, 20.0, 30.0],
            anchor,
            perspective,
        );
        let u = uniform_for(&clip, &camera, anchor);
        let s = u.screen[0].map(f64::from);
        let ndc = |p: [f64; 3]| {
            camera.transform_point(&Point::new(
                p[0] - anchor[0],
                p[1] - anchor[1],
                p[2] - anchor[2],
            ))
        };
        let value = |p: [f64; 3]| {
            let q = ndc(p);
            s[0] * q[0] + s[1] * q[1] + s[2] * q[2] + s[3]
        };
        let x = array(&plane.x_axis());
        let y = array(&plane.y_axis());

        for (a, b) in [(0.0, 0.0), (0.3, -0.7), (-0.9, 0.2)] {
            let on = [
                10.0 + a * x[0] + b * y[0],
                20.0 + a * x[1] + b * y[1],
                30.0 + a * x[2] + b * y[2],
            ];
            assert!(
                value(on).abs() < 1e-4,
                "on the plane, perspective {perspective}"
            );
        }

        assert!(
            value([10.0, 20.0, 29.0]) > 0.0,
            "kept below the normal point"
        );
        assert!(
            value([10.0, 20.0, 31.0]) < 0.0,
            "cut toward the normal point"
        );
    }
}
#[test]
fn hatch_coordinate_follows_the_plane() {
    let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 200.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let anchor = [30.0, -10.0, 0.0];
    let camera = view([250.0, -300.0, 400.0], [0.0, 0.0, 50.0], anchor, true);
    let inverse = camera.inverse().unwrap();
    let u = uniform_for(&clip, &camera, anchor);
    let s = u.screen[0].map(f64::from);
    // the scene point under canvas point (x, y), and the hatch coordinate there
    let point = |x: f64, y: f64| {
        let z = -(s[0] * x + s[1] * y + s[3]) / s[2];
        let p = inverse.transform_point(&Point::new(x, y, z));
        [p[0] + anchor[0], p[1] + anchor[1], p[2] + anchor[2]]
    };
    let coordinate = |x: f64, y: f64| {
        let h = [x, y, 1.0];
        let n: f64 = (0..3).map(|c| f64::from(u.hatch[0][c]) * h[c]).sum();
        let w: f64 = (0..3).map(|c| f64::from(u.hatch_w[0][c]) * h[c]).sum();
        n / w
    };
    let a = point(-0.2, 0.1);
    let b = point(0.3, -0.25);
    assert!((a[2] - 50.0).abs() < 1e-3 && (b[2] - 50.0).abs() < 1e-3);
    let across = |p: [f64; 3]| dot(clip.hatch, p);
    let expected = across(b) - across(a);
    let found = coordinate(0.3, -0.25) - coordinate(-0.2, 0.1);
    assert!((expected - found).abs() < 1e-2, "{expected} vs {found}");
}
#[test]
fn eye_sides_follow_the_eye() {
    let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 100.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();

    for (eye, side) in [([0.0, -300.0, 200.0], -1.0), ([0.0, -300.0, -200.0], 1.0)] {
        for perspective in [true, false] {
            let camera = view(eye, [0.0; 3], [0.0; 3], perspective);
            let u = uniform_for(&clip, &camera, [0.0; 3]);
            assert_eq!(u.sides[0][0], side, "{eye:?}, perspective {perspective}");
        }
    }
}
#[test]
fn hatch_spacing_follows_the_device_scale() {
    let plane = clip_plane(
        &plane_from(Mode::Xy, &[[0.0; 3]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let camera = view([0.0, -300.0, 200.0], [0.0; 3], [0.0; 3], true);
    let u = clip_uniform(
        &[plane],
        0,
        &ClipView {
            view_proj: &camera,
            anchor: [0.0; 3],
            height: 512,
            pixel_scale: 2.0,
            samples: 1,forward_depth:false,
        },
    );
    assert_eq!(
        (u.count, u.samples, u.spacing, u.width, u.outline),
        (1, 1, 16.0, 2.0, 4.0)
    );
}
