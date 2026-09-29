//! Probe scene: a plate with a BRep cylinder, a cone and a NURBS surface.
use session_rust::brep::{BRep, brep_reverse};
use session_rust::{Color, Mesh, NurbsSurface, Point, Session, Xform};

/// Write the scene to `argv[1]`; BREP_PROBE_FLIPPED reverses two cylinder faces.
fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/brep_probe.pb".into());
    let mut plate = Mesh::create_box(2800.0, 1000.0, 40.0);
    plate.transform(&Xform::translation(0.0, 0.0, -20.0));
    plate.set_objectcolor(Color::grey());
    let count = plate.edges_with_colors().len();
    plate.set_linecolors(vec![Color::black(); count], vec![0.0; count]);

    let mut cylinder = BRep::create_cylinder(150.0, 400.0);
    cylinder.transform(&Xform::translation(-900.0, 0.0, 0.0));
    cylinder.surfacecolor = Color::grey();
    if std::env::var("BREP_PROBE_FLIPPED").is_ok() {
        for face in cylinder.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = brep_reverse(face.orientation);
        }
    }
    let mut cone = BRep::create_cone(150.0, 400.0);
    cone.transform(&Xform::translation(900.0, 0.0, 0.0));
    cone.surfacecolor = Color::grey();

    let coord: [f64; 4] = [-450.0, -150.0, 150.0, 450.0];
    let mut points = Vec::with_capacity(16);
    for &x in &coord {
        for &y in &coord {
            let z = 250.0 + 150.0 * (x / 200.0).sin() * (y / 200.0).cos();
            points.push(Point::new(x, y, z));
        }
    }
    let mut surface =
        NurbsSurface::create(false, false, 3, 3, 4, 4, &points).expect("probe surface");
    surface.facecolors = vec![Color::grey()];

    let mut s = Session::new("brep_probe");
    s.add_mesh(plate, None);
    s.add_brep(cylinder, None);
    s.add_brep(cone, None);
    s.add_nurbssurface(surface, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
