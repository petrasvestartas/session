    clip
}


fn raster_em(label: &TextLabel, frame: &TextFrame) -> u32 {
    let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
        ..
    } = label.placement
    else {
        return 32;
    };
    let a = project(world, frame);

    if a[3] <= 0.0 {
        return 32;
    }

    let mut projected_em = 0.0f32;

    for direction in [right, up] {
        let mut end = world;

        for axis in 0..3 {
            end[axis] += direction[axis] * world_height;
        }

        let b = project(end, frame);

        if b[3] <= 0.0 {
            continue;
        }

        let x = (a[0] / a[3] - b[0] / b[3]) * frame.framebuffer[0] as f32 * 0.5;
        let y = (a[1] / a[3] - b[1] / b[3]) * frame.framebuffer[1] as f32 * 0.5;
        projected_em = projected_em.max(x.hypot(y));
    }

    let needed = (projected_em * 2.0).clamp(32.0, 256.0);
    let mut bucket = 32;

    while (bucket as f32) < needed {
        bucket *= 2;
    }

    bucket
}

fn fitting_em(run: &TextRun) -> u32 {
    let mut size = 0.0f32;

    for line in run.buffer.layout_runs() {
        size = size.max(line.line_w).max(line.line_top + line.line_height);
    }

    let ems = size / run.label.font_size + 3.0;
    let mut em = 256;

    while em > 32 && ems * em as f32 > 4000.0 {
        em /= 2;
    }

    em
}

fn coverage(content: &SwashContent, data: &[u8], source: usize) -> u8 {
    match content {
        SwashContent::Mask => data[source],
        SwashContent::Color => data[source * 4 + 3],
        SwashContent::SubpixelMask => {
            ((u16::from(data[source * 4])
                + u16::from(data[source * 4 + 1])
                + u16::from(data[source * 4 + 2]))
                / 3) as u8
        }
    }
}
