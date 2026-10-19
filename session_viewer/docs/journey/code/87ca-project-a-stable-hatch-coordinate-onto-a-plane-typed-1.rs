 &Xform) -> Option<Xform> {
    let scales: [f64; 4] = std::array::from_fn(|row| {
        (0..4)
            .map(|col| m.m[col * 4 + row].abs())
            .fold(0.0, f64::max)
    });

    if scales.iter().any(|scale| !scale.is_normal()) {
        return None;
    }

    let mut inverse =
        Xform::from_matrix(std::array::from_fn(|i| m.m[i] / scales[i % 4])).inverse()?;

    for (i, value) in inverse.m.iter_mut().enumerate() {
        *value /= scales[i / 4];
    }

    Some(inverse)
}

fn hatch_projection(u: &mut ClipUniform, i: usize, plane: &ClipPlane, s: [f64; 4], inverse: &Xform, v: &ClipView, period: f64) {
        // a plane point over canvas (x, y) is inverse · (x, y, depth(x, y), 1), homogeneous
        let columns = [
            [1.0, 0.0, -s[0] / s[2], 0.0],
            [0.0, 1.0, -s[1] / s[2], 0.0],
            [0.0, 0.0, -s[3] / s[2], 1.0],
        ];
        let from = [
            v.anchor[0] - plane.origin[0],
            v.anchor[1] - plane.origin[1],
            v.anchor[2] - plane.origin[2],
        ];
        let phase = dot(plane.hatch, from);
        let phase = if period.is_normal() {
            phase.rem_euclid(period)
        } else {
            phase
        };

        for (c, column) in columns.iter().enumerate() {
            let h: [f64; 4] =
                std::array::from_fn(|r| (0..4).map(|q| inverse.m[q * 4 + r] * column[q]).sum());
            u.hatch[i][c] = (dot(plane.hatch, [h[0], h[1], h[2]]) + phase * h[3]) as f32;
            u.hatch_w[i][c] = h[3] as f32;
        }
}
