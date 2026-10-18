        self.vertices.buf.size()
    }
}

impl Plates {

    pub(super) fn prepare(&mut self, ctx: &GpuCtx, rectangles: &[Rectangle], size: [u32; 2]) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);
        self.physical_vertices = 0;

        for overlay in [false, true] {
            for rectangle in rectangles {
                if rectangle.depth.is_none() != overlay {
                    continue;
                }

                let [left, top, right, bottom] = rectangle.bounds;
                let half = [(right - left) * 0.5, (bottom - top) * 0.5];
                let center = [(left + right) * 0.5, (top + bottom) * 0.5];
                let radius = if rectangle.rounded {
                    half[0].min(half[1]).max(0.0)
                } else {
                    0.0
                };
                let left = left.max(rectangle.clip[0]);
                let top = top.max(rectangle.clip[1]);
                let right = right.min(rectangle.clip[2]);
                let bottom = bottom.min(rectangle.clip[3]);

                if right <= left || bottom <= top {
                    continue;
                }

                for [x, y] in [
                    [left, top],
                    [left, bottom],
                    [right, bottom],
                    [left, top],
                    [right, bottom],
                    [right, top],
                ] {
                    vertices.push([
                        2.0 * x / size[0] as f32 - 1.0,
                        1.0 - 2.0 * y / size[1] as f32,
                        rectangle.depth.unwrap_or(0.0),
                        x - center[0],
                        y - center[1],
                        half[0],
                        half[1],
                        radius,
                        f32::from_bits(rectangle.object.map_or(0, |object| object.row.wrapping_add(1))),
                        if rectangle.object.is_some_and(|object| object.selected) {
                            1.0
                        } else {
                            0.0
                        },
                    ]);
                }
            }

            if !overlay {
                self.physical_vertices = vertices.len() as u32;
            }
        }

        self.vertices.append(ctx, &vertices);
    }
}
