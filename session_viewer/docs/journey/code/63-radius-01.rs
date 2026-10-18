pub fn radius(size: (u32, u32), css_width: f64) -> f32 {
    let css_radius = 1.6875;
    (css_radius * f64::from(size.0) / css_width.max(1.0)).clamp(1.0, 12.0) as f32
}

pub fn block(radius: f32) -> u32 {
    ((radius + 0.5).ceil() as u32).next_power_of_two().max(4)
}

pub fn taps(radius: f32) -> Vec<[f32; 4]> {
    let extent = (radius + 0.5).ceil() as i32;
    let mut taps = Vec::new();

    for y in -extent..=extent {
        for x in -extent..=extent {
            let distance = ((x * x + y * y) as f32).sqrt();

            if distance >= radius + 0.5 {
                continue;
            }

            let t = (distance - (radius - 0.5)).clamp(0.0, 1.0);
            taps.push([x as f32, y as f32, 1.0 - t * t * (3.0 - 2.0 * t), distance]);
        }
    }

    taps.sort_by(|a, b| a[3].total_cmp(&b[3]));
    taps
}
