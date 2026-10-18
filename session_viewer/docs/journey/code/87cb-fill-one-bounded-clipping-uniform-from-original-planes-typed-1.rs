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

pub fn clip_uniform(planes: &[ClipPlane], fill: u32, v: &ClipView) -> ClipUniform {
    let mut u = ClipUniform::default();
    if planes.is_empty() {
        return u;
    }

    let Some(inverse) = inverse(v.view_proj) else {
        return u;
    };
    let m = &v.view_proj.m;
    let ortho = v.view_proj.ortho_half_height() > 0.0;
    let eye = v.view_proj.eye();
    let sign=if v.forward_depth{-1.0}else{1.0};
    let toward = [sign*m[2],sign*m[6],sign*m[10]];
    // scene units per pixel at the anchor; the hatch phase wraps far above that
    let rows = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
    let per_px = 2.0 * m[15].abs() / (rows * f64::from(v.height.max(1)));
    let spacing = HATCH_CSS_PX * v.pixel_scale;
    let period = (spacing * per_px * 65536.0).log2().ceil().exp2();
    u.count = planes.len().min(MAX_PLANES) as u32;
    u.samples = v.samples;
    u.fill = fill;
    u.spacing = spacing as f32;
    u.width = v.pixel_scale.max(1.5) as f32;
    u.outline = (2.0 * v.pixel_scale) as f32;

    for (i, plane) in planes.iter().take(MAX_PLANES).enumerate() {
        let k = plane.normal;
        let p = [k[0], k[1], k[2], plane.offset + dot(k, v.anchor)];
        let s: [f64; 4] =
            std::array::from_fn(|j| (0..4).map(|r| inverse.m[j * 4 + r] * p[r]).sum());
        let side = if ortho {
            dot(k, toward)
        } else {
            dot(k, [eye[0], eye[1], eye[2]]) + p[3]
        };
        u.planes[i] = p.map(|value| value as f32);
        u.screen[i] = s.map(|value| value as f32);
        u.sides[i / 4][i % 4] = if side > 0.0 {
            1.0
        } else if side < 0.0 {
            -1.0
        } else {
            0.0
        };

        // edge on: no cap to hatch
        if s[2] == 0.0 {
            continue;
        }

        hatch_projection(&mut u, i, plane, s, &inverse, v, period);
    }

    u
}
