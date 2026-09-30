
#[cfg(test)]
mod shell_tests {
    use super::tests::arrow;
    use super::*;

    /// Draws, highlights, hides, picks and releases one vector on a headless device.
    #[test]
    fn vector_renders_selects_hides_and_picks() {
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow};
        use session_rust::{AABB, Xform};

        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        let mut up = Upload::default();
        let mut bounds = AABB::empty();
        bounds.union_with_point(0.0, 0.0, 0.0);
        bounds.union_with_point(100.0, 100.0, 0.0);
        let mut row = ObjectRow::new(Xform::identity(), 0);
        row.bounds = bounds;
        up.obj.rows.push(row);
        up.bounds = bounds;
        let vector = arrow(
            0,
            [0.0, 0.0, 0.0],
            [100.0, 100.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        up.lanes.get_mut::<VectorRows>().rows.push(vector);
        gpu.set_scene(&up);

        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 1.0);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let ink = |rgba: &[u8]| {
            rgba.chunks_exact(4)
                .filter(|p| p[0] < 128 && p[1] < 128)
                .count()
        };
        let yellow = |rgba: &[u8]| {
            rgba.chunks_exact(4)
                .filter(|p| p[0] > 200 && p[1] > 200 && p[2] < 100)
                .count()
        };

        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(256, 256);
            let plain = gpu.render_offscreen(&input);
            let drawn = ink(&plain);
            assert!(drawn > 60, "{samples}x: the arrow draws ink: {drawn}");

            gpu.set_selected(0, true);
            let selected = gpu.render_offscreen(&input);
            assert!(
                yellow(&selected) > 60,
                "{samples}x: a selected arrow is yellow"
            );
            gpu.set_selected(0, false);

            gpu.set_hidden(0, true);
            let hidden = gpu.render_offscreen(&input);
            assert_eq!(ink(&hidden), 0, "{samples}x: a hidden arrow draws nothing");
            gpu.set_hidden(0, false);
        }

        let ids = gpu.render_ids_offscreen(&input);
        let hits = ids.iter().filter(|id| id[0] == 1).count();
        assert!(hits > 30, "the id pass answers row 0: {hits}");

        let tagged = ids
            .iter()
            .filter(|id| id[0] == 1 && id[1] == 0x4000_0000)
            .count();
        assert_eq!(tagged, hits, "vector picks carry the marker tag");

        // a stub head draws less ink than the default one
        let with_head = ink(&gpu.render_offscreen(&input));
        gpu.reset();
        up.lanes.get_mut::<VectorRows>().rows[0].head = 0.1;
        gpu.set_scene(&up);
        let stub = ink(&gpu.render_offscreen(&input));
        assert!(
            with_head > stub + 15,
            "the head adds ink: {with_head} vs {stub}"
        );

        gpu.release();
        let empty = gpu.render_offscreen(&input);
        assert_eq!(ink(&empty), 0, "released vectors are gone");
    }
}
