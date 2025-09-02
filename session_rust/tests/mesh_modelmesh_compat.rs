use session_rust::collections::Mesh;
use session_rust::primitives::Point;

fn compare_exports(mesh: &mut Mesh) {
    let (v_buf, i_buf, n_buf, c_buf, v_count, t_count) = mesh.to_model_mesh_buffers();

    // Basic counts and lengths
    assert_eq!(v_buf.len(), v_count * 3, "positions length vs count mismatch");
    assert_eq!(n_buf.len(), v_count * 3, "normals length vs count mismatch");
    assert_eq!(c_buf.len(), v_count * 3, "colors length vs count mismatch");
    assert_eq!(i_buf.len(), t_count * 3, "indices length vs triangle_count mismatch");

    // Interleaved must agree exactly with separated buffers
    let (interleaved, indices2, vc2, tc2) = mesh.to_model_mesh_interleaved();
    assert_eq!(indices2, i_buf, "interleaved indices differ from indices buffer");
    assert_eq!(vc2, v_count, "interleaved vertex_count mismatch");
    assert_eq!(tc2, t_count, "interleaved triangle_count mismatch");
    assert_eq!(interleaved.len(), v_count * 9, "interleaved length mismatch");

    let stride = 9usize; // 3 pos, 3 normal, 3 color
    for i in 0..v_count {
        let b = i * stride;
        assert_eq!(interleaved[b + 0], v_buf[i * 3 + 0], "x mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 1], v_buf[i * 3 + 1], "y mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 2], v_buf[i * 3 + 2], "z mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 3], n_buf[i * 3 + 0], "nx mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 4], n_buf[i * 3 + 1], "ny mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 5], n_buf[i * 3 + 2], "nz mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 6], c_buf[i * 3 + 0], "r mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 7], c_buf[i * 3 + 1], "g mismatch at vertex {}", i);
        assert_eq!(interleaved[b + 8], c_buf[i * 3 + 2], "b mismatch at vertex {}", i);
    }
}

#[test]
fn test_mesh_export_vs_modelmesh_cube() {
    // Quad faces for a unit cube (consistent with existing tests)
    let cube_polygons = vec![
        vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(0.0, 1.0, 0.0)],
        vec![Point::new(1.0, 0.0, 1.0), Point::new(0.0, 0.0, 1.0), Point::new(0.0, 1.0, 1.0), Point::new(1.0, 1.0, 1.0)],
        vec![Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 1.0)],
        vec![Point::new(1.0, 0.0, 0.0), Point::new(1.0, 0.0, 1.0), Point::new(1.0, 1.0, 1.0), Point::new(1.0, 1.0, 0.0)],
        vec![Point::new(0.0, 0.0, 1.0), Point::new(1.0, 0.0, 1.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0)],
        vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(1.0, 1.0, 1.0), Point::new(0.0, 1.0, 1.0)],
    ];
    let mut mesh = Mesh::from_polygons_with_merge(cube_polygons, None);
    compare_exports(&mut mesh);
}

#[test]
fn test_mesh_export_vs_modelmesh_pentagon() {
    // Single convex pentagon in the XY plane (z=0)
    let pentagon = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.5, 0.8, 0.0),
        Point::new(0.5, 1.5, 0.0),
        Point::new(-0.5, 0.8, 0.0),
    ];
    let polygons = vec![pentagon];
    let mut mesh = Mesh::from_polygons_with_merge(polygons, None);
    compare_exports(&mut mesh);
}
