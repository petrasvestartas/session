        bind,
    }
}


fn append_quad(
    vertices: &mut Vec<[f32; 16]>,
    label: &TextLabel,
    extent: [f32; 4],
    frame: &TextFrame,
    srgb: bool,
) {
    let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
    } = label.placement
    else {
        return;
    };
    let unit = world_height / f64::from(label.font_size);
    let mut color = [0.0; 4];

    for (index, component) in color.iter_mut().enumerate() {
        let value = f32::from(label.ink_color()[index]) / 255.0;
        *component = if srgb && index < 3 {
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        } else {
            value
        };
    }

    let scale = frame.framebuffer[0] as f32 / frame.logical[0] as f32;
    let mut bounds = [
        0.0,
        0.0,
        frame.framebuffer[0] as f32,
        frame.framebuffer[1] as f32,
    ];

    if let Some(clip) = label.clip {
        for (index, value) in bounds.iter_mut().enumerate() {
            *value = clip[index] * scale;
        }
    }

    bounds = [
        2.0 * bounds[0] / frame.framebuffer[0] as f32 - 1.0,
        1.0 - 2.0 * bounds[3] / frame.framebuffer[1] as f32,
        2.0 * bounds[2] / frame.framebuffer[0] as f32 - 1.0,
        1.0 - 2.0 * bounds[1] / frame.framebuffer[1] as f32,
    ];
    for [u, v] in [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [1.0, 0.0],
    ] {
        let x = f64::from(extent[0] + (extent[2] - extent[0]) * u) * unit;
        let y = f64::from(extent[1] + (extent[3] - extent[1]) * v) * unit;
        let mut point = world;

        for axis in 0..3 {
            point[axis] += right[axis] * x - up[axis] * y;
        }

        let clip = project(point, frame);
        vertices.push([
            clip[0],
            clip[1],
            clip[2],
            clip[3],
            u,
            v,
            color[0],
            color[1],
            color[2],
            color[3],
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],
            f32::from_bits(label.object.map_or(0, |object| object.row.wrapping_add(1))),
            if label.object.is_some_and(|object| object.selected) {
                1.0
            } else {
                0.0
            },
        ]);
    }
}
