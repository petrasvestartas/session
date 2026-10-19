use crate::outline_math::*;
#[test]
fn current_outline_radius_and_sorted_taps_keep_css_width_independent_of_geometry() {
    for density in [1.0_f32, 1.25, 1.5, 2., 3., 4., 8., 20.] {
        let pixels = (800. * density) as u32;
        let r = radius((pixels, 600), 800.);
        assert_eq!(r, (1.6875 * density).clamp(1., 12.));
        let table = taps(r);
        assert!(table.len() <= 512);
        assert_eq!(table[0], [0., 0., 1., 0.]);
        assert!(
            table
                .windows(2)
                .all(|p| p[0][3] <= p[1][3] && p[0][2] >= p[1][2])
        );
        for tap in table {
            assert!(tap[2] > 0. && tap[2] <= 1.);
            assert!(tap[3] < r + 0.5);
            assert!(tap[0].abs() < block(r) as f32 && tap[1].abs() < block(r) as f32);
        }
    }
}
