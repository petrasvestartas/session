
/// Pool sizing and grid tests.
#[cfg(test)]
mod tests {
    use super::*;

    /// Each tile shader carries what it includes and validates on its own.
    #[test]
    fn tile_shaders_validate() {
        for (name, source) in [
            ("project_triangles.wgsl", shader!("project_triangles.wgsl")),
            ("triangle_tiles.wgsl", shader!("triangle_tiles.wgsl")),
            (
                "scan_triangle_tiles.wgsl",
                shader!("scan_triangle_tiles.wgsl"),
            ),
        ] {
            let module = naga::front::wgsl::parse_str(source)
                .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(source)));
        }
    }

    /// Past 65535 workgroups the projection wraps into rows, covering every triangle once.
    #[test]
    fn projection_dispatch_stays_in_limits() {
        assert_eq!(project_groups(0), (0, 0));
        assert_eq!(project_groups(65), (2, 1));
        assert_eq!(project_groups(4_194_304), (32_768, 2));
        let (x, y) = project_groups(u32::MAX);
        assert!(x <= 65_535 && y <= 65_535);
        assert!(u64::from(x) * u64::from(y) * 64 >= u64::from(u32::MAX));
    }

    /// One more triangle reuses the table; a load jump is exact; an emptied table shrinks.
    #[test]
    fn projected_grows_by_capacity() {
        let most = u64::MAX;
        assert_eq!(
            projected_records(1_000, 1, most),
            Some(1_000),
            "a load is exact"
        );
        assert_eq!(
            projected_records(1_001, 1_000, most),
            Some(1_126),
            "an edit spares an eighth"
        );
        assert_eq!(
            projected_records(1_002, 1_126, most),
            None,
            "the next one fits"
        );
        assert_eq!(projected_records(900, 1_126, most), None, "fewer still fit");
        assert_eq!(
            projected_records(200, 1_126, most),
            Some(225),
            "a quarter full shrinks"
        );
        assert_eq!(
            projected_records(2_000, 1_000, 2_050),
            Some(2_000),
            "within the device limit"
        );
        assert_eq!(projected_records(1_010, 1_000, 1_050), Some(1_050));
    }

    /// Past or at capacity doubles; below it keeps the pool.
    #[test]
    fn a_saturated_report_grows_the_pool() {
        let capacity = 1_000;
        let pool = 800;
        assert_eq!(
            next_pool_words(pool, 0, capacity, Some(capacity + 64), u64::MAX),
            1_600,
            "a report past capacity doubles"
        );
        assert_eq!(
            next_pool_words(pool, 0, capacity, Some(capacity), u64::MAX),
            1_600,
            "a pool exactly filled has also run out"
        );
        assert_eq!(
            next_pool_words(pool, 0, capacity, Some(capacity - 1), u64::MAX),
            pool,
            "a report below capacity measured lists that fitted"
        );
        assert_eq!(
            next_pool_words(pool, 0, capacity, None, u64::MAX),
            pool,
            "no report, no change"
        );
    }

    /// Growth stops at the ceiling.
    #[test]
    fn growth_stops_at_the_ceiling() {
        let ceiling = 2_048;
        let mut pool = 512;

        for _ in 0..8 {
            pool = next_pool_words(pool, 0, pool, Some(pool), ceiling);
        }

        assert_eq!(pool, ceiling);
        assert_eq!(next_pool_words(pool, 0, pool, Some(pool), ceiling), ceiling);
    }

    /// Doubling reaches ten times the size in four frames.
    #[test]
    fn doubling_converges_in_a_few_frames() {
        let ceiling = u64::MAX;
        let mut pool = 1_000;
        let mut frames = 0;

        while pool < 10_000 {
            pool = next_pool_words(pool, 0, pool, Some(pool), ceiling);
            frames += 1;
        }

        assert_eq!(frames, 4);
    }

    /// The floor wins when the scene grew.
    #[test]
    fn the_initial_pool_is_a_floor() {
        assert_eq!(next_pool_words(100, 4_096, 100, None, u64::MAX), 4_096);
        assert_eq!(next_pool_words(8_192, 4_096, 8_192, None, u64::MAX), 8_192);
    }

    #[test]
    /// Every canvas size gets a grid under the tile and memory limits.
    fn viewport_grid_fits_core_storage_limits_without_fixed_per_tile_caps() {
        for size in [
            (1, 1),
            (1400, 900),
            (1800, 1400),
            (2800, 1800),
            (7680, 4320),
            (16384, 16384),
        ] {
            let layout = TileLayout::new(size);
            assert!(layout.count() <= MAX_TILES);
            assert!(layout.buffer_bytes(layout.max_pool_words()) < 128 * 1024 * 1024);
            assert!(layout.width * layout.span >= size.0 && layout.height * layout.span >= size.1);
        }

        assert_eq!(TileLayout::new((1800, 1400)).span, 4);
        assert_eq!(TileLayout::new((2800, 1800)).span, 8);
    }

    /// A small scene starts with a small pool.
    #[test]
    fn pool_starts_small_and_is_capped() {
        let layout = TileLayout::new((3200, 2000));
        let small = layout.initial_pool_words(6_000);
        assert!(small < layout.max_pool_words() / 8);
        assert_eq!(layout.initial_pool_words(u32::MAX), layout.max_pool_words());
        assert!(layout.buffer_bytes(small) < 4 * 1024 * 1024);
        assert_eq!(
            layout.buffer_bytes(layout.max_pool_words()),
            layout.header_records() * 16 + layout.count() as u64 * REFERENCES_PER_TILE * 8
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Lists are reused until camera, hiding or scene change.
    fn projection_cache_tracks_camera_hidden_state_replacement_and_release() {
        use crate::engine::gpu::{CylinderSegment, FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::{RenderVertex, Xform};
        let mut gpu = pollster::block_on(Gpu::new_headless(128, 128)).unwrap();
        gpu.view.show_grid = false;
        let initial = gpu.arena.tiles.allocated_bytes();
        // placeholders: tiles, one record, count, report
        assert_eq!(initial, (16 + PROJECTED_BYTES + 16 + 16, 0));
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));

        for position in [[-0.7, -0.7, 0.5], [0.7, -0.7, 0.5], [0.0, 0.7, 0.5]] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.5; 4],
            });
            upload.arena.vids.push(0);
        }

        upload.arena.idx.extend([0, 1, 2]);
        gpu.set_scene(&upload);
        let mut input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        gpu.render_offscreen(&input);
        assert!(
            gpu.arena.tiles.key.is_none(),
            "no stroke and no ambient occlusion: nothing is projected"
        );
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial, "nor allocated");
        // Arctic reads geometry directly and needs no projected table or tile list.
        gpu.view.ssao = true;
        let visible = gpu.render_offscreen(&input);
        assert!(gpu.arena.tiles.key.is_none());
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial);
        assert_eq!(visible, gpu.render_offscreen(&input));
        gpu.set_hidden(0, true);
        assert_ne!(gpu.render_offscreen(&input), visible);
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial);
        gpu.set_hidden(0, false);
        input.view_proj.m[12] = 0.1;
        gpu.render_offscreen(&input);
        gpu.resize(160, 96);
        gpu.render_offscreen(&input);
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial);

        // a pick of faces alone reads no tables either
        gpu.render_ids_offscreen(&input);
        assert!(
            gpu.arena.tiles.key.is_none(),
            "a face pick projects nothing"
        );
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial, "nor allocates");
        // a line over the triangle: its ink and its pick read the tables, with the usual invalidation
        upload.seg.ribbons.push(CylinderSegment {
            p0: [-0.5, 0.0, 0.6],
            radius: 0.0,
            p1: [0.5, 0.0, 0.6],
            instance_id: 0,
            color: 0xff00_0000,
            facing: 0,
        });
        gpu.reset();
        gpu.set_scene(&upload);
        gpu.render_ids_offscreen(&input);
        let key = gpu
            .arena
            .tiles
            .key
            .expect("picking a line prepares visibility");
        gpu.set_selected(0, true);
        gpu.render_ids_offscreen(&input);
        assert_eq!(
            gpu.arena.tiles.key,
            Some(key),
            "highlighting does not reproject"
        );
        gpu.set_hidden(0, true);
        gpu.render_ids_offscreen(&input);
        assert_ne!(gpu.arena.tiles.key, Some(key));
        let hidden_key = gpu.arena.tiles.key;
        gpu.set_hidden(0, false);
        input.view_proj.m[12] = 0.2;
        gpu.render_ids_offscreen(&input);
        assert_ne!(gpu.arena.tiles.key, hidden_key);
        assert_eq!(gpu.arena.tiles.layout, Some(TileLayout::new((160, 96))));
        gpu.reset();
        gpu.set_scene(&upload);
        assert!(
            gpu.arena.tiles.key.is_none(),
            "same-count replacement invalidates projected positions"
        );
        gpu.view.ssao = false;
        gpu.render_ids_offscreen(&input);
        assert!(
            gpu.arena.tiles.key.is_some() && gpu.arena.tiles.binned == gpu.arena.tiles.key,
            "ID-only rendering prepares its own current geometry and lists"
        );
        // lines off: nothing reads the tables, so they are freed
        gpu.view.show_lines = false;
        gpu.render_offscreen(&input);
        assert_eq!(
            gpu.arena.tiles.allocated_bytes(),
            initial,
            "unread tables are freed"
        );
        gpu.render_ids_offscreen(&input);
        assert!(
            gpu.arena.tiles.key.is_none(),
            "and a face pick does not bring them back"
        );
        gpu.view.show_lines = true;
        gpu.render_offscreen(&input);
        assert!(gpu.arena.tiles.key.is_some(), "lines back on project again");
        gpu.release();
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial);
        assert!(gpu.arena.tiles.key.is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A slow drag drops the lists, also the ones a scene change left in the buffer.
    fn a_slow_drag_drops_lists_a_scene_change_left_behind() {
        use crate::engine::gpu::{CylinderSegment, FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::{RenderVertex, Xform};
        let mut gpu = pollster::block_on(Gpu::new_headless(128, 128)).unwrap();
        gpu.view.show_grid = false;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));

        for position in [[-0.7, -0.7, 0.5], [0.7, -0.7, 0.5], [0.0, 0.7, 0.5]] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.5; 4],
            });
            upload.arena.vids.push(0);
        }

        upload.arena.idx.extend([0, 1, 2]);
        // a line over the triangle: its ink reads the lists
        upload.seg.ribbons.push(CylinderSegment {
            p0: [-0.5, 0.0, 0.6],
            radius: 0.0,
            p1: [0.5, 0.0, 0.6],
            instance_id: 0,
            color: 0xff00_0000,
            facing: 0,
        });
        gpu.set_scene(&upload);
        let input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        gpu.render_offscreen(&input);
        assert_eq!(first_record(&gpu)[0], 1, "a line at rest reads the lists");
        // the scene changes, then a slow drag draws
        gpu.arena.tiles.invalidate();
        gpu.performance.interacting = true;

        for frame in 0..3 {
            gpu.performance.frame(0, 0, 100.0 * f64::from(frame), false);
        }

        assert_eq!(gpu.performance.drag_tier(), 1);
        gpu.render_offscreen(&input);
        assert_eq!(first_record(&gpu)[0], 0, "a slow drag has no lists");
    }

    /// The first tile record, read back.
    #[cfg(not(target_arch = "wasm32"))]
    fn first_record(gpu: &crate::engine::gpu::Gpu) -> [u32; 4] {
        let readback = gpu.ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("test.tiles.readback"),
            size: 16,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = gpu.ctx.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&gpu.arena.tiles.buffer, 0, &readback, 0, 16);
        gpu.ctx.queue.submit([encoder.finish()]);
        readback.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = gpu.ctx.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        let bytes = readback.slice(..).get_mapped_range();
        bytemuck::pod_read_unaligned(&bytes[..16])
    }
}
