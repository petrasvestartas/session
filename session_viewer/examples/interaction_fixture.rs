//! Deterministic source geometry for tests/interaction.cjs; writes outside tracked assets.
use session_rust::{
    BRep, Color, Line, Mesh, NurbsCurve, NurbsSurface, Point, PointCloud, Polyline, Session, Xform,
};

/// Write source geometry and original selection targets outside the tracked asset tree.
fn main() {
    let output = std::env::args()
        .nth(1)
        .unwrap_or("/tmp/viewer-interaction.pb".into());
    let mut scene = Session::new("interaction regression");
    let mut cases = Vec::new();
    let mut mesh = Mesh::create_box(2.0, 2.0, 0.4);
    mesh.name = "interaction mesh".into();
    let guid = mesh.guid().to_string();
    scene.add_mesh(mesh, None);
    scene.set_xform(&guid, Xform::translation(-9.0, 3.0, 0.0));
    cases.push(case(
        "mesh",
        &guid,
        [-9.0, 3.0, 0.2],
        Some([-9.0, 4.0, 0.2]),
        8,
    ));
    let mut line = Line::new(-4.0, 3.0, 0.0, -2.0, 3.0, 0.0);
    line.name = "interaction line".into();
    line.width = 0.02;
    let guid = line.guid().to_string();
    scene.add_line(line, None);
    cases.push(case(
        "line",
        &guid,
        [-3.0, 3.0, 0.0],
        Some([-3.0, 3.0, 0.0]),
        2,
    ));
    let mut polyline = Polyline::new(vec![p(2.0, 3.0), p(3.0, 4.0), p(4.0, 3.0)]);
    polyline.name = "interaction polyline".into();
    polyline.width = 0.02;
    let guid = polyline.guid().to_string();
    scene.add_polyline(polyline, None);
    cases.push(case(
        "polyline",
        &guid,
        [2.5, 3.5, 0.0],
        Some([2.5, 3.5, 0.0]),
        3,
    ));
    let mut curve =
        NurbsCurve::create_clamped_uniform(3, 3, &[p(8.0, 3.0), p(9.0, 4.0), p(10.0, 3.0)], 1.0);
    curve.name = "interaction curve".into();
    curve.width = 0.02;
    let guid = curve.guid().to_string();
    scene.add_nurbscurve(curve, None);
    cases.push(case(
        "curve",
        &guid,
        [9.0, 3.5, 0.0],
        Some([9.0, 3.5, 0.0]),
        3,
    ));
    let mut surface = NurbsSurface::create_simple(3, false, 2, 2, 2, 2).unwrap();
    for u in 0..2 {
        for v in 0..2 {
            surface.set_cv(u, v, &p(-10.0 + 2.0 * u as f64, -4.0 + 2.0 * v as f64));
        }
    }
    surface.name = "interaction surface".into();
    let guid = surface.guid().to_string();
    scene.add_nurbssurface(surface, None);
    cases.push(case(
        "surface",
        &guid,
        [-9.0, -3.0, 0.0],
        Some([-9.0, -2.0, 0.0]),
        4,
    ));
    let mut brep = BRep::create_box(2.0, 2.0, 0.4);
    brep.name = "interaction brep".into();
    let guid = brep.guid().to_string();
    scene.add_brep(brep, None);
    scene.set_xform(&guid, Xform::translation(-3.0, -3.0, 0.0));
    cases.push(case(
        "brep",
        &guid,
        [-3.0, -3.0, 0.2],
        Some([-3.0, -2.0, 0.2]),
        8,
    ));
    let mut points = Vec::with_capacity(25);
    for x in 0..5 {
        for y in 0..5 {
            points.push(p(2.0 + f64::from(x) * 0.5, -4.0 + f64::from(y) * 0.5));
        }
    }
    let mut cloud = PointCloud::new(points, vec![], vec![Color::black(); 25]);
    cloud.name = "interaction resident cloud".into();
    let guid = cloud.guid().to_string();
    scene.add_pointcloud(cloud, None);
    cases.push(case("cloud", &guid, [3.0, -3.0, 0.0], None, 0));
    scene.pb_dump(&output);
    std::fs::write(
        format!("{output}.json"),
        serde_json::to_vec_pretty(&cases).unwrap(),
    )
    .unwrap();
    println!("wrote {output} and {output}.json");
}
/// Keep planar source coordinates explicit in fixture construction.
fn p(x: f64, y: f64) -> Point {
    Point::new(x, y, 0.0)
}
/// Record original world-space targets independently of display tessellation.
fn case(
    kind: &str,
    guid: &str,
    pick: [f64; 3],
    edge: Option<[f64; 3]>,
    minimum_controls: usize,
) -> serde_json::Value {
    serde_json::json!({"kind":kind,"guid":guid,"pick":pick,"edge":edge,"minimum_controls":minimum_controls})
}
