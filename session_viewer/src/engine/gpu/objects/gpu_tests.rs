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
