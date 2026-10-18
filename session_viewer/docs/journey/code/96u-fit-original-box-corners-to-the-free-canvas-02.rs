use crate::{bounds::Bounds, camera::{Camera, Projection, View}};
use session_rust::Point;

#[test]
fn fit_frames_all_box_corners_inside_the_panel_free_rectangle() {
    for projection in [Projection::Perspective, Projection::Orthographic] {
        for view in [View::Front, View::Top, View::Iso, View::IsoBack] {
            let mut camera = Camera::default();
            camera.set_view(view);
            camera.projection = projection;
            camera.aspect = 0.75;
            camera.frame([0.25, 0.1, 0.95, 0.85]);
            let bounds = Bounds { min: [1_000_000.125, -30.0, 10.0], max: [1_000_600.125, 10.0, 15.0] };
            camera.fit(&bounds);
            let matrix = camera.view_projection();
            for corner in 0..8 {
                let p: [f64; 3] = std::array::from_fn(|i| if corner & (1 << i) == 0 { bounds.min[i] } else { bounds.max[i] });
                let screen = matrix.transform_point(&Point::new(p[0], p[1], p[2]));
                assert!(screen[0] >= 2.0 * camera.frame[0] - 1.0 - 1e-8);
                assert!(screen[0] <= 2.0 * camera.frame[2] - 1.0 + 1e-8);
                assert!(screen[1] >= 1.0 - 2.0 * camera.frame[3] - 1e-8);
                assert!(screen[1] <= 1.0 - 2.0 * camera.frame[1] + 1e-8);
                assert!((0.0..=1.0).contains(&screen[2]));
            }
            assert_eq!(bounds.min[0], 1_000_000.125);
        }
    }
}

#[test]
fn thin_boxes_fill_the_frame_without_a_sphere_margin() {
    let mut camera = Camera::default();
    camera.set_view(View::Front);
    let bounds = Bounds { min: [-100.0, -0.1, -0.1], max: [100.0, 0.1, 0.1] };
    camera.fit(&bounds);
    let a = camera.view_projection().transform_point(&Point::new(-100.0, 0.0, 0.0));
    let b = camera.view_projection().transform_point(&Point::new(100.0, 0.0, 0.0));
    assert!(b[0] - a[0] > 1.8);
    let before = (camera.target, camera.radius, camera.distance);
    camera.fit(&Bounds { min: [f64::NAN, 0.0, 0.0], max: [1.0; 3] });
    assert_eq!((camera.target, camera.radius, camera.distance), before);
    camera.fit(&Bounds::point([7.0; 3]));
    assert_eq!(camera.target, [7.0; 3]);
    assert_eq!((camera.radius, camera.distance), (before.1, before.2));
}
