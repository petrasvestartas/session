use crate::{
    pick_sources::{Mode, Part, Sources, Target},
    production_curve_walk as walk, production_joined as joined,
    scene::Scene,
};
use session_rust::{Arrowhead, Line, Point, Polyline, Xform};
use std::rc::Rc;
fn word(bytes: &[u8], offset: usize) -> u32 {
    u32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
fn polyline(points: &[[f64; 3]], heads: Arrowhead) -> Polyline {
    let mut p = Polyline::new(
        points
            .iter()
            .map(|p| Point::new(p[0], p[1], p[2]))
            .collect(),
    );
    p.arrowhead = heads;
    p.width = 3.0;
    p
}

#[test]
fn production_joined_neighbours_heads_and_degenerate_ends_are_exact() {
    let p = polyline(
        &[
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
        ],
        Arrowhead::BOTH,
    );
    let mut segments = walk::SegRows::default();
    let mut heads = walk::VectorRows::default();
    let flags = walk::walk_polyline(&mut segments, &mut heads, &p, 7).flags;
    assert_eq!(flags, 32768);
    assert_eq!(segments.ribbons.len(), 2);
    assert_eq!(heads.rows.len(), 2);
    assert_eq!(
        (heads.rows[0].start, heads.rows[0].end, heads.rows[0].heads),
        ([2.0, 0.0, 0.0], [0.0, 0.0, 0.0], 5)
    );
    assert_eq!(
        (heads.rows[1].start, heads.rows[1].end, heads.rows[1].heads),
        ([2.0, 0.0, 0.0], [2.0, 2.0, 0.0], 5)
    );
    let rows = walk::joined_rows(
        &segments.ribbons,
        &segments.ribbon_chains,
        &segments.ribbon_heads,
        10,
        &[],
    );
    assert_eq!(
        (
            rows[0].previous,
            rows[0].next,
            rows[1].previous,
            rows[1].next
        ),
        (walk::HEAD_MARK, 11, 10, walk::HEAD_MARK)
    );
    assert!(
        rows.iter()
            .all(|row| row.segment.instance_id == 7 && row.segment.radius == -1.5)
    );
    let closed = polyline(
        &[
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
        Arrowhead::START,
    );
    let mut s = walk::SegRows::default();
    let mut h = walk::VectorRows::default();
    walk::walk_polyline(&mut s, &mut h, &closed, 0);
    let r = walk::joined_rows(&s.ribbons, &s.ribbon_chains, &s.ribbon_heads, 0, &[]);
    assert_eq!(r[0].previous, walk::HEAD_MARK);
    assert_eq!(r[2].next, u32::MAX);
    assert_eq!(r[1].next, 2);
    let mut zero = Line::new(1.0, 1.0, 0.0, 1.0, 1.0, 0.0);
    zero.arrowhead = Arrowhead::BOTH;
    let mut s = walk::SegRows::default();
    let mut h = walk::VectorRows::default();
    assert_eq!(walk::walk_line(&mut s, &mut h, &zero, 0).flags, 0);
    assert!(h.rows.is_empty());
}

#[test]
fn compact_joined_rows_preserve_sparse_original_ids_placement_and_source_edges() {
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    let source = Rc::new(polyline(
        &[
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
        Arrowhead::BOTH,
    ));
    let _ = source.guid();
    let original = source.to_proto();
    let id = scene
        .insert_path(crate::chain::PreparedChain::polyline(source.clone()).unwrap())
        .unwrap();
    scene
        .place_path(id, Xform::translation(1e9 + 2.0, -1e9 + 3.0, 4.0))
        .unwrap();
    let sources = Sources::anchored(&scene, [1e9, -1e9, 0.0]).unwrap();
    let rows = joined::rows(&scene, &sources, [1e9, -1e9, 0.0], &[id], 1024).unwrap();
    assert_eq!(rows.objects.len(), 96);
    assert_eq!(rows.translations.len(), 16);
    assert_eq!(rows.segments.len(), 104);
    assert_eq!(rows.heads.len(), 96);
    assert_eq!(rows.object_ids, [id]);
    assert_eq!(rows.pick_owners, [id.value()]);
    assert_eq!(rows.edges, [0, 2]);
    assert_eq!(word(&rows.objects, 80), 32769);
    assert_eq!(word(&rows.segments, 28), 0);
    assert_eq!(word(&rows.segments, 52 + 28), 0);
    assert_eq!(
        rows.translations
            .chunks_exact(4)
            .map(|p| f32::from_ne_bytes(p.try_into().unwrap()))
            .collect::<Vec<_>>(),
        [2.0, 3.0, 4.0, 0.0]
    );
    for (i, edge) in [0, 2].into_iter().enumerate() {
        assert_eq!(
            sources.resolve(&scene, rows.ids[i], Mode::Edge),
            Some(Target {
                owner: id,
                part: Part::PathEdge(edge)
            })
        );
    }
    assert_eq!(source.to_proto(), original);
    assert!(joined::rows(&scene, &sources, [0.0; 3], &[], 95).is_err());
}

#[test]
fn ownership_adapters_leave_all_three_production_shader_algorithms_byte_exact() {
    use sha2::{Digest, Sha256};
    let p: serde_json::Value = serde_json::from_str(joined::PROVENANCE).unwrap();
    for (key, source) in [
        ("joined", joined::SOURCE),
        ("vector", crate::production_vector_gpu::SOURCE),
        ("sheet", crate::production_sheet_gpu::SOURCE),
    ] {
        let row = &p[key];
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            row["compiled_sha256"].as_str().unwrap()
        );
        let declaration = format!("\n{}\n", row["ownership_declaration"].as_str().unwrap());
        let mut pristine = source.strip_suffix(&declaration).unwrap().to_owned();
        for edit in row["ownership_edits"].as_array().unwrap() {
            let from = edit[0].as_str().unwrap();
            let to = edit[1].as_str().unwrap();
            pristine = pristine.replacen(to, from, 1);
        }
        assert_eq!(
            format!("{:x}", Sha256::digest(pristine.as_bytes())),
            row["pristine_sha256"].as_str().unwrap()
        );
        assert!(source.contains("fn ink_depth("));
        assert!(source.contains("fn clip_cut("));
        assert!(source.contains("fn face_normal("));
        assert!(source.contains("fn physical_primitive("));
    }
    assert_eq!(
        crate::production_sheet_gpu::chunks(10, 208).unwrap(),
        vec![0..4, 4..8, 8..10]
    );
    assert!(crate::production_sheet_gpu::chunks(1, 51).is_err());
}

fn dump(name: &str, image: &[u8]) {
    if let Ok(directory) = std::env::var("VECTOR_WITNESS_DIR") {
        let directory = std::path::Path::new(&directory);
        std::fs::create_dir_all(directory).unwrap();
        let mut ppm = b"P6\n320 240\n255\n".to_vec();
        for pixel in image.chunks_exact(4) {
            ppm.extend_from_slice(&pixel[..3]);
        }
        std::fs::write(directory.join(format!("{name}.ppm")), ppm).unwrap();
    }
}
fn hit(
    gpu: &mut crate::renderer::Renderer,
    scene: &Scene,
    position: [f32; 2],
    mode: Mode,
) -> Target {
    gpu.set_pick_mode(mode);
    gpu.request_pick(position, crate::cap_gpu_tests::matrix())
        .unwrap();
    crate::cap_gpu_tests::wait(gpu);
    let pixel = gpu
        .take_pick(crate::cap_gpu_tests::matrix())
        .unwrap()
        .unwrap()
        .unwrap();
    Sources::new(scene)
        .unwrap()
        .resolve(scene, pixel, mode)
        .unwrap()
}
#[test]
fn actual_production_joined_heads_sheets_and_matrix_keep_mixed_source_picks_and_selection() {
    use crate::cap_gpu_tests::{at, matrix, pixels_at, renderer};
    let mut scene = Scene::empty();
    let mut line = Line::new(-0.8, 0.75, -0.4, -0.1, 0.75, -0.4);
    line.width = 3.0;
    let ordinary = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let p = polyline(
        &[[-0.8, 0.3, -0.4], [-0.3, 0.3, -0.4], [-0.3, 0.55, -0.4]],
        Arrowhead::BOTH,
    );
    let path = scene
        .insert_path(crate::chain::PreparedChain::polyline(Rc::new(p)).unwrap())
        .unwrap();
    let mut headed = Line::new(0.15, 0.3, -0.4, 0.8, 0.3, -0.4);
    headed.width = 3.0;
    headed.arrowhead = Arrowhead::END;
    let vector = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(headed)).unwrap())
        .unwrap();
    let mut m = session_rust::Matrix::from_rows(&[vec![2.0]]);
    m.cell = 0.3;
    let matrix_id = scene.insert_matrix(Rc::new(m.to_component())).unwrap();
    scene
        .place_matrix(matrix_id, Xform::translation(0.5, -0.55, -0.4))
        .unwrap();
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::new(session_rust::Mesh::from_vertices_and_faces(
                vec![
                    Point::new(-0.9, -0.8, -0.4),
                    Point::new(-0.6, -0.8, -0.4),
                    Point::new(-0.75, -0.5, -0.4),
                ],
                vec![vec![0, 1, 2]],
            ))
            .unwrap(),
        )
        .unwrap();
    scene.next_id = 16_777_217;
    let mut sheet = crate::sheet_example::source();
    sheet.coords = vec![-0.8, -0.2, -0.4, 0.8, -0.2, -0.4];
    sheet.colors = vec![0xff00_0000];
    sheet.widths = vec![0.025];
    sheet.source_ids = vec![2];
    sheet.segment_count = 1;
    let source = Rc::new(sheet);
    let original = source.clone();
    let sheet_id = scene
        .insert_sheet(Rc::new(crate::sheet_source::Prepared::new(source).unwrap()))
        .unwrap();
    let copy = scene.copy_row(sheet_id, false).unwrap();
    scene
        .place_sheet(copy, Xform::translation(0.0, -0.1, 0.0))
        .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.sheet_usage()[..2], &[2, 1]);
    let uploads = gpu.sheet_usage()[3];
    let mut first = None;
    for samples in [1, 4, 1] {
        eprintln!("vector fixture: {samples}x set_samples begin");
        gpu.set_samples(samples);
        eprintln!("vector fixture: {samples}x set_samples complete; baseline readback begin");
        let image = pixels_at(&gpu, matrix());
        eprintln!("vector fixture: {samples}x baseline readback complete");
        dump(&format!("vector-{samples}x"), &image);
        for (x, y) in [
            (80, 30),
            (80, 84),
            (112, 70),
            (220, 84),
            (160, 144),
            (160, 156),
        ] {
            let p = at(&image, x, y);
            assert!(
                p[..3].iter().all(|v| *v < 180),
                "{samples}x source stroke ({x},{y}) {p:?}"
            );
        }
        // Production head_px gives max(10 pens, 6 * 1.5 shaft half-width) = 10 px.
        // Tip (288,84), base x=278, half-base=4: these probes are inside both
        // sides of the head and outside the three-pixel shaft's AA footprint.
        for (x,y) in [(279,81),(279,86)] {
            let pixel=at(&image,x,y);
            assert!(pixel[..3].iter().all(|value|*value<50),
                "{samples}x actual head interior ({x},{y}): {pixel:?}");
        }
        for (x,y) in [(276,81),(276,86),(284,80),(284,87),(279,78),(279,89),(290,83),(290,84)] {
            let pixel=at(&image,x,y);
            assert!(pixel[..3].iter().all(|value|*value>245),
                "{samples}x outside actual head support ({x},{y}): {pixel:?}");
        }
        for (position, id) in [
            ([-0.5, 0.75], ordinary),
            ([-0.5, 0.3], path),
            ([0.5, 0.3], vector),
            ([0.0, -0.2], sheet_id),
            ([0.0, -0.3], copy),
            ([0.5, -0.55], matrix_id),
            ([-0.75, -0.7], mesh),
        ] {
            assert_eq!(hit(&mut gpu, &scene, position, Mode::Object).owner, id);
        }
        assert_eq!(
            hit(&mut gpu, &scene, [-0.5, 0.3], Mode::Edge),
            Target {
                owner: path,
                part: Part::PathEdge(0)
            }
        );
        assert_eq!(
            hit(&mut gpu, &scene, [0.0, -0.2], Mode::Edge),
            Target {
                owner: sheet_id,
                part: Part::SheetEntity(2)
            }
        );
        gpu.set_highlight(
            &scene,
            Some(Target {
                owner: path,
                part: Part::PathEdge(0),
            }),
        )
        .unwrap();
        eprintln!("vector fixture: {samples}x path selection readback begin");
        let selected = pixels_at(&gpu, matrix());
        eprintln!("vector fixture: {samples}x path selection readback complete");
        dump(&format!("vector-edge-{samples}x"), &selected);
        let p = at(&selected, 80, 84);
        assert!(
            p[0] > 200 && p[1] > 200 && p[2] < 100,
            "Selected original path edge: {p:?}"
        );
        assert_eq!(at(&selected, 112, 70), at(&image, 112, 70));
        gpu.set_highlight(
            &scene,
            Some(Target {
                owner: sheet_id,
                part: Part::SheetEntity(2),
            }),
        )
        .unwrap();
        eprintln!("vector fixture: {samples}x sheet selection readback begin");
        let selected = pixels_at(&gpu, matrix());
        eprintln!("vector fixture: {samples}x sheet selection readback complete");
        dump(&format!("vector-sheet-{samples}x"), &selected);
        let p = at(&selected, 160, 144);
        assert!(
            p[0] > 200 && p[1] > 200 && p[2] < 100,
            "Selected original sheet entity: {p:?}"
        );
        assert_eq!(at(&selected, 160, 156), at(&image, 160, 156));
        gpu.set_highlight(&scene, None).unwrap();
        eprintln!("vector fixture: {samples}x selection cleared; sample iteration complete");
        assert_eq!(gpu.sheet_usage()[3], uploads);
        if samples == 1 {
            if let Some(first) = &first {
                assert_eq!(&image, first);
            } else {
                first = Some(image);
            }
        }
    }
    assert!(Rc::ptr_eq(&scene.sheets()[0].prepared.original, &original));
    scene.clear();
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.sheet_usage()[..2], &[0, 0]);
}
