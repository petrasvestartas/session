        }
    }
}


fn rasterize(
    run: &TextRun,
    fonts: &mut FontSystem,
    raster: &mut SwashCache,
    em_pixels: u32,
) -> anyhow::Result<(Vec<u8>, [u32; 2], [f32; 4])> {
    let scale = em_pixels as f32 / run.label.font_size;
    let mut glyphs = Vec::new();
    let mut bounds = [0i32; 4];

    for line in run.buffer.layout_runs() {
        bounds[2] = bounds[2].max((line.line_w * scale).ceil() as i32);
        bounds[3] = bounds[3].max(((line.line_top + line.line_height) * scale).ceil() as i32);
        anyhow::ensure!(
            bounds[2] <= 4092 && bounds[3] <= 4092,
            "world text layout exceeds 4096px extent"
        );

        for glyph in line.glyphs {
            let physical = glyph.physical((0.0, 0.0), scale);
            let Some(image) = raster.get_image(fonts, physical.cache_key) else {
                continue;
            };
            let x = physical.x + image.placement.left;
            let y = (line.line_y * scale).round() as i32 + physical.y - image.placement.top;
            bounds[0] = bounds[0].min(x);
            bounds[1] = bounds[1].min(y);
            bounds[2] = bounds[2].max(x + image.placement.width as i32);
            bounds[3] = bounds[3].max(y + image.placement.height as i32);
            glyphs.push((physical.cache_key, x, y));
        }
    }

    let vertical_padding = (run.label.font_size * (2.0 / 9.0) * scale).ceil() as i32;
    let horizontal_padding = (bounds[3] - bounds[1] + 2 * vertical_padding + 1) / 2;
    bounds[0] -= horizontal_padding;
    bounds[1] -= vertical_padding;
    bounds[2] += horizontal_padding;
    bounds[3] += vertical_padding;
    let size = [
        (bounds[2] - bounds[0]) as u32,
        (bounds[3] - bounds[1]) as u32,
    ];
    anyhow::ensure!(
        size[0] <= 4096 && size[1] <= 4096,
        "world text texture exceeds 4096px extent"
    );
    let mut pixels = vec![0u8; (size[0] * size[1]) as usize];

    for (key, x, y) in glyphs {
        let Some(image) = raster.get_image(fonts, key) else {
            continue;
        };

        for row in 0..image.placement.height {
            for column in 0..image.placement.width {
                let source = (row * image.placement.width + column) as usize;
                let coverage = coverage(&image.content, &image.data, source);
                let target = ((y - bounds[1] + row as i32) as u32 * size[0]
                    + (x - bounds[0] + column as i32) as u32) as usize;
                let previous = u16::from(pixels[target]);
                pixels[target] = (previous + u16::from(coverage) * (255 - previous) / 255) as u8;
            }
        }
    }

    Ok((
        pixels,
        size,
        [
            bounds[0] as f32 / scale,
            bounds[1] as f32 / scale,
            bounds[2] as f32 / scale,
            bounds[3] as f32 / scale,
        ],
    ))
}
