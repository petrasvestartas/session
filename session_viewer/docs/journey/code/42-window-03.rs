use crate::pick_window::Window;

#[test]
fn finite_pick_windows_match_density_and_remain_inside_the_canvas() {
    for density in [0.125, 1.0, 2.0, 1000.0] {
        for full in [[1, 1], [3, 2], [960, 640], [u32::MAX, u32::MAX]] {
            for screen in [[-1.0, -1.0], [1.0, 1.0], [0.0, 0.0]] {
                let w = Window::new(screen, full, density).unwrap();
                assert!(w.radius <= 128);
                assert!(w.copy[2] <= 257 && w.copy[3] <= 257);
                assert!(w.draw[2] <= 263 && w.draw[3] <= 263);
                for i in 0..2 {
                    assert!(w.cursor[i] < w.copy[i + 2]);
                    assert!(u64::from(w.draw[i]) + u64::from(w.draw[i + 2]) <= u64::from(full[i]));
                    assert!(w.copy[i] >= w.draw[i]);
                }
                assert_eq!(w.row_bytes() % 256, 0);
                assert!(w.crop().iter().all(|v| v.is_finite()));
            }
        }
    }
    assert_eq!(Window::new([0.0; 2], [960, 640], 1.0).unwrap().radius, 6);
    assert_eq!(Window::new([0.0; 2], [960, 640], 2.0).unwrap().radius, 12);
    for screen in [[f32::NAN, 0.0], [0.0, f32::INFINITY], [1.01, 0.0]] {
        assert!(Window::new(screen, [960, 640], 1.0).is_none());
    }
    for density in [0.0, -1.0, f32::NAN, f32::INFINITY] { assert!(Window::new([0.0; 2], [960, 640], density).is_none()); }
    assert!(Window::new([0.0; 2], [0, 640], 1.0).is_none());
}

#[test]
fn cropped_pixel_centres_preserve_homogeneous_depth_in_both_projections() {
    for screen in [[-1.0, -1.0], [0.0, 0.0], [0.9, -0.8], [1.0, 1.0]] {
        let w = Window::new(screen, [960, 640], 2.0).unwrap(); let m = w.crop();
        for weight in [1.0, 0.25, 7.0] {
            for [x, y] in [[w.draw[0], w.draw[1]], [w.copy[0] + w.cursor[0], w.copy[1] + w.cursor[1]]] {
                let p = [(2.0 * (f64::from(x) + 0.5) / 960.0 - 1.0) * weight,
                    (1.0 - 2.0 * (f64::from(y) + 0.5) / 640.0) * weight, 0.4 * weight, weight];
                let actual: [f64; 4] = std::array::from_fn(|r| (0..4).map(|c| f64::from(m[c * 4 + r]) * p[c]).sum());
                let expected = [2.0 * (f64::from(x - w.draw[0]) + 0.5) / f64::from(w.draw[2]) - 1.0,
                    1.0 - 2.0 * (f64::from(y - w.draw[1]) + 0.5) / f64::from(w.draw[3])];
                for i in 0..2 { assert!((actual[i] / actual[3] - expected[i]).abs() < 0.00001); }
                assert_eq!([actual[2], actual[3]], [p[2], p[3]]);
            }
        }
    }
}

#[test]
fn pick_decoding_ignores_padding_and_prefers_visible_ink_within_the_tolerance() {
    let w = Window::new([0.0; 2], [960, 640], 1.0).unwrap();
    let mut bytes = vec![0; w.bytes() as usize];
    let at = |x: u32, y: u32| (y * w.row_bytes() + x * 8) as usize;
    let mut put = |x: u32, y: u32, row: u32, sub: u32| {
        let offset = at(x, y); bytes[offset..offset + 4].copy_from_slice(&row.to_le_bytes());
        bytes[offset + 4..offset + 8].copy_from_slice(&sub.to_le_bytes());
    };
    put(6, 6, 16_777_217, 0); put(8, 6, 16_777_218, 1); put(0, 0, 1, 1);
    bytes[(w.copy[2] * 8) as usize..w.row_bytes() as usize].fill(255);
    assert_eq!(w.closest(&bytes).unwrap(), Some([16_777_218, 1]));
    bytes[at(8, 6)..at(8, 6) + 8].fill(0);
    assert_eq!(w.closest(&bytes).unwrap(), Some([16_777_217, 0]));
    bytes[at(6, 6)..at(6, 6) + 8].fill(0);
    assert_eq!(w.closest(&bytes).unwrap(), None);
    assert!(w.closest(&bytes[..bytes.len() - 1]).is_err());
}
