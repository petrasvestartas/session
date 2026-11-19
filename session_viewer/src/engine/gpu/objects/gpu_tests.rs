use super::ObjectRow;
use session_rust::{AABB, Xform};

/// One row moves, its neighbour stays, the widget row stays last.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "requires a native GPU adapter"]
fn one_row_moves_and_its_neighbour_does_not() {
    use crate::engine::gpu::{Gpu, Upload};
    let mut gpu = pollster::block_on(Gpu::new_headless(64, 64)).unwrap();
    let mut upload = Upload::default();

    for x in [0.0, 100.0] {
        let mut row = ObjectRow::new(Xform::translation(x, 0.0, 0.0), 0);
        row.bounds = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        row.faces = true;
        upload.obj.rows.push(row);
    }

    gpu.set_scene(&upload);
    assert_eq!(gpu.objects.len(), 2);
    let radius = gpu.objects.row(0).unwrap().ao_radius;

    let moved = Xform::translation(0.0, 50.0, 0.0);
    assert!(gpu.objects.set_placement(&gpu.ctx, 0, &moved));
    assert_eq!(gpu.objects.row(0).unwrap().ao_radius, radius);

    let first = gpu.objects.row_bounds(0).expect("row 0 has a box");
    let second = gpu.objects.row_bounds(1).expect("row 1 has a box");
    assert_eq!(first.min_point()[1], 49.0);
    assert_eq!(second.min_point()[0], 99.0);
    assert_eq!(second.min_point()[1], -1.0, "the neighbour did not move");

    gpu.objects
        .set_placement(&gpu.ctx, 0, &Xform::scale_xyz(2.0, 2.0, 2.0));
    assert_eq!(gpu.objects.row(0).unwrap().ao_radius, 2.0 * radius);
    assert_eq!(gpu.objects.row(1).unwrap().ao_radius, radius);

    // widget row comes after every object row
    let (widget, _) = gpu.objects.widget_row(&gpu.ctx, &gpu.layouts);
    assert_eq!(widget, 2);
    assert_eq!(gpu.objects.widget_row(&gpu.ctx, &gpu.layouts).0, widget);

    // a later upload drops and remints the widget row
    let mut more = Upload::default();
    more.obj
        .rows
        .push(ObjectRow::new(Xform::translation(200.0, 0.0, 0.0), 0));
    gpu.set_scene(&more);
    assert_eq!(gpu.objects.len(), 3, "two rows, one file's row, no widget");
    assert_eq!(gpu.objects.widget_row(&gpu.ctx, &gpu.layouts).0, 3);
}

/// A fill mesh blends over the paper with its own alpha; recolouring preserves that alpha.
#[test]
fn fill_mesh_uses_per_object_opacity() {
    use crate::app::walk::{Walk, WalkCx, walk_geometry};
    use crate::camera::Camera;
    use crate::engine::gpu::{FrameInput, Gpu, Upload};
    use session_rust::{Color, Geometry, Mesh, Point};
    use std::rc::Rc;

    let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
        eprintln!("no GPU adapter; skipped");
        return;
    };
    gpu.view.show_grid = false;
    gpu.view.opacity = 1.0;
    let p = |x, y| Point::new(x, y, 0.0);
    let mut mesh = Mesh::from_vertices_and_faces(
        vec![p(-1.0, -1.0), p(1.0, -1.0), p(1.0, 1.0), p(-1.0, 1.0)],
        vec![vec![0, 1, 2, 3]],
    );
    mesh.set_objectcolor(Color::new(0.0, 0.48, 0.77, 1.0));
    mesh.set_linecolors(Vec::new(), vec![0.0]);
    let mut up = Upload::default();
    let cx = WalkCx {
        quality: Default::default(),
        vert_base: 0,
        cloud_px: 0.0,
        row: 0,
        interactions: false,
        attributes: false,
        arrowhead_pens: 0.0,
    };
    let geometry = Geometry::Mesh(Rc::new(mesh));
    let walked = walk_geometry(&mut Walk::of(&mut up), &cx, &geometry);
    let mut object = ObjectRow::new(Xform::identity(), walked.flags);
    object.bounds = walked.bounds;
    object.faces = walked.faces;
    up.obj.rows.push(object);
    up.bounds = walked.bounds;
    gpu.set_scene(&up);
    let mut camera = Camera::new();
    camera.fit(&up.bounds, 1.0);
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
    let input = FrameInput {
        view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };
    let centre = (128 * 256 + 128) * 4;
    let opaque = gpu.render_offscreen(&input);
    gpu.set_face_opacity(0, 0.18);
    let transparent = gpu.render_offscreen(&input);
    assert!(
        transparent[centre] > opaque[centre] + 100 && transparent[centre] < 250,
        "the fill blends over white: {} -> {}",
        opaque[centre],
        transparent[centre]
    );
    gpu.set_object_color(0, false, Some([0, 123, 196]));
    let recoloured = gpu.render_offscreen(&input);
    assert!(recoloured[centre].abs_diff(transparent[centre]) <= 2);
}
