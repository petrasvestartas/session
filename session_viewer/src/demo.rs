use session_rust::{Color, Line, Mesh, Point, PointCloud, Polyline, Session, TreeNode};

fn make_box(cx: f32, cy: f32, cz: f32, r: f32, color: Color, name: &str) -> Mesh {
    let mut m = Mesh::new();
    m.name = name.to_string();
    let pts = [
        Point::new(cx-r, cy-r, cz-r), Point::new(cx+r, cy-r, cz-r),
        Point::new(cx+r, cy+r, cz-r), Point::new(cx-r, cy+r, cz-r),
        Point::new(cx-r, cy-r, cz+r), Point::new(cx+r, cy-r, cz+r),
        Point::new(cx+r, cy+r, cz+r), Point::new(cx-r, cy+r, cz+r),
    ];
    let v: Vec<_> = pts.iter().map(|p| m.add_vertex(p.clone(), None)).collect();
    for f in &[[0,3,2,1],[4,5,6,7],[0,1,5,4],[2,3,7,6],[0,4,7,3],[1,2,6,5]] {
        m.add_face(vec![v[f[0]], v[f[1]], v[f[2]], v[f[3]]], None);
    }
    m.set_objectcolor(color);
    m
}

fn make_tetra(cx: f32, cy: f32, cz: f32, r: f32, color: Color, name: &str) -> Mesh {
    let mut m = Mesh::new();
    m.name = name.to_string();
    let a = m.add_vertex(Point::new(cx,     cy+r,  cz),    None);
    let b = m.add_vertex(Point::new(cx-r,   cy-r,  cz-r),  None);
    let c = m.add_vertex(Point::new(cx+r,   cy-r,  cz-r),  None);
    let d = m.add_vertex(Point::new(cx,     cy-r,  cz+r),  None);
    m.add_face(vec![a,c,b], None);
    m.add_face(vec![a,d,c], None);
    m.add_face(vec![a,b,d], None);
    m.add_face(vec![b,c,d], None);
    m.set_objectcolor(color);
    m
}

fn named_point(x: f32, y: f32, z: f32, name: &str, color: Color) -> Point {
    let mut p = Point::with_name(x, y, z, name);
    p.pointcolor = color;
    p
}

pub fn make_demo_session() -> Session {
    let mut s = Session::new("demo");

    let ground     = s.add_group("ground");
    let boxes      = s.add_group("boxes");
    let tetras     = s.add_group("tetrahedra");
    let labels     = s.add_group("labels");
    let primitives = s.add_group("primitives");

    // Ground triangle
    let mut tri = Mesh::new();
    tri.name = "triangle".to_string();
    let a = tri.add_vertex(Point::new(    0.0,  400.0, 0.0), None);
    let b = tri.add_vertex(Point::new( -400.0, -267.0, 0.0), None);
    let c = tri.add_vertex(Point::new(  400.0, -267.0, 0.0), None);
    tri.add_face(vec![a, b, c], None);
    tri.set_objectcolor(Color::new(1.0, 0.549, 0.0, 1.0));
    s.add_mesh(tri, Some(&ground));

    // Sub-group: small boxes nested under boxes
    let small = TreeNode::new("small_boxes");
    s.tree.add(&small, Some(&boxes));
    s.add_mesh(make_box( -600.0,  600.0, 200.0, 120.0, Color::new(0.3, 0.9, 0.6, 1.0), "box_small_a"), Some(&small));
    s.add_mesh(make_box(  600.0,  600.0, 200.0, 120.0, Color::new(0.9, 0.3, 0.6, 1.0), "box_small_b"), Some(&small));
    s.add_mesh(make_box(    0.0, -600.0, 200.0, 120.0, Color::new(0.6, 0.6, 0.2, 1.0), "box_small_c"), Some(&small));

    // Boxes
    let n0 = s.add_mesh(make_box( 2000.0,  2000.0,  1000.0, 400.0, Color::new(0.2, 0.6, 1.0, 1.0), "box_blue"),   Some(&boxes));
    let n1 = s.add_mesh(make_box(-2500.0,  1000.0,  2000.0, 350.0, Color::new(0.2, 0.8, 0.3, 1.0), "box_green"),  Some(&boxes));
    let n2 = s.add_mesh(make_box( 1000.0, -2000.0,  1500.0, 500.0, Color::new(0.9, 0.2, 0.2, 1.0), "box_red"),    Some(&boxes));
    let n3 = s.add_mesh(make_box(-1500.0, -1500.0, -1000.0, 300.0, Color::new(0.8, 0.2, 0.8, 1.0), "box_purple"), Some(&boxes));
    let n4 = s.add_mesh(make_box(    0.0,  2500.0,  2500.0, 450.0, Color::new(0.9, 0.8, 0.1, 1.0), "box_yellow"), Some(&boxes));

    // Tetrahedra
    let n5 = s.add_mesh(make_tetra( 3000.0,     0.0,  500.0, 600.0, Color::new(0.1, 0.9, 0.9, 1.0), "tetra_cyan"),   Some(&tetras));
    let n6 = s.add_mesh(make_tetra(-3000.0,  1500.0, 1000.0, 500.0, Color::new(1.0, 0.4, 0.0, 1.0), "tetra_orange"), Some(&tetras));
    let n7 = s.add_mesh(make_tetra(    0.0, -3000.0, 2000.0, 550.0, Color::new(0.6, 0.9, 0.2, 1.0), "tetra_lime"),   Some(&tetras));

    // Graph edges
    let g0 = n0.borrow().name.clone();
    let g1 = n1.borrow().name.clone();
    let g2 = n2.borrow().name.clone();
    let g3 = n3.borrow().name.clone();
    let g4 = n4.borrow().name.clone();
    let g5 = n5.borrow().name.clone();
    let g6 = n6.borrow().name.clone();
    let g7 = n7.borrow().name.clone();

    s.add_edge(&g0, &g5, "neighbor"); // box_blue   — tetra_cyan
    s.add_edge(&g1, &g6, "neighbor"); // box_green  — tetra_orange
    s.add_edge(&g2, &g7, "neighbor"); // box_red    — tetra_lime
    s.add_edge(&g3, &g4, "adjacent"); // box_purple — box_yellow

    // Primitives group: line, polyline, point cloud, standalone point
    let mut ln = Line::from_points(
        &Point::new(-800.0, 1200.0, 600.0),
        &Point::new( 800.0, 1200.0, 600.0),
    );
    ln.name = "line_h".to_string();
    ln.linecolor = Color::new(1.0, 0.5, 0.0, 1.0);
    s.add_line(ln, Some(&primitives));

    let mut pl = Polyline::new(vec![
        Point::new(-400.0,  -800.0,  800.0),
        Point::new(   0.0,  -600.0, 1200.0),
        Point::new( 400.0,  -800.0,  800.0),
        Point::new(   0.0, -1000.0,  400.0),
    ]);
    pl.name = "polyline_z".to_string();
    pl.linecolor = Color::new(0.4, 0.8, 1.0, 1.0);
    s.add_polyline(pl, Some(&primitives));

    let mut pc = PointCloud::new(vec![], vec![], vec![]);
    pc.name = "cloud_a".to_string();
    pc.add_point(&Point::new(-200.0, 2800.0, 300.0));
    pc.add_point(&Point::new(   0.0, 2800.0, 500.0));
    pc.add_point(&Point::new( 200.0, 2800.0, 300.0));
    pc.add_point(&Point::new(   0.0, 2600.0, 500.0));
    pc.add_color(&Color::new(0.9, 0.3, 0.9, 1.0));
    pc.add_color(&Color::new(0.9, 0.3, 0.9, 1.0));
    pc.add_color(&Color::new(0.9, 0.3, 0.9, 1.0));
    pc.add_color(&Color::new(0.9, 0.3, 0.9, 1.0));
    s.add_pointcloud(pc, Some(&primitives));

    let mut pt = Point::with_name(-1800.0, -800.0, 600.0, "pt_standalone");
    pt.pointcolor = Color::new(0.3, 1.0, 0.5, 1.0);
    s.add_point(pt, Some(&primitives));

    // Named points — appear in tree under "labels", rendered as text labels
    s.add_point(named_point(    0.0,  400.0,    0.0, "A", Color::new(1.0, 0.784, 0.392, 1.0)), Some(&labels));
    s.add_point(named_point( -400.0, -267.0,    0.0, "B", Color::new(1.0, 0.784, 0.392, 1.0)), Some(&labels));
    s.add_point(named_point(  400.0, -267.0,    0.0, "C", Color::new(1.0, 0.784, 0.392, 1.0)), Some(&labels));
    s.add_point(named_point( 2000.0,  2000.0, 1000.0, "box_blue",    Color::new(0.2, 0.6, 1.0, 1.0)), Some(&labels));
    s.add_point(named_point(-2500.0,  1000.0, 2000.0, "box_green",   Color::new(0.2, 0.8, 0.3, 1.0)), Some(&labels));
    s.add_point(named_point( 1000.0, -2000.0, 1500.0, "box_red",     Color::new(0.9, 0.2, 0.2, 1.0)), Some(&labels));
    s.add_point(named_point(-1500.0, -1500.0,-1000.0, "box_purple",  Color::new(0.8, 0.2, 0.8, 1.0)), Some(&labels));
    s.add_point(named_point(    0.0,  2500.0, 2500.0, "box_yellow",  Color::new(0.9, 0.8, 0.1, 1.0)), Some(&labels));
    s.add_point(named_point( 3000.0,     0.0,  500.0, "tetra_cyan",   Color::new(0.1, 0.9, 0.9, 1.0)), Some(&labels));
    s.add_point(named_point(-3000.0,  1500.0, 1000.0, "tetra_orange", Color::new(1.0, 0.4, 0.0, 1.0)), Some(&labels));
    s.add_point(named_point(    0.0, -3000.0, 2000.0, "tetra_lime",   Color::new(0.6, 0.9, 0.2, 1.0)), Some(&labels));

    s
}
