// BRep probe: a grey plate with a BRep cylinder and a BRep cone standing on it, edges black.
// BREP_PROBE_FLIPPED=1 reverses the orientation of the cylinder's first two face uses, so a
// render pair proves the ink does not depend on face orientation.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_brep_probe -- <out.pb>
use session_rust::brep::{brep_reverse, BRep};
use session_rust::{Color, Mesh, Session, Xform};

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/brep_probe.pb".into());
    let mut plate = Mesh::create_box(1600.0, 1000.0, 40.0);
    plate.transform(&Xform::translation(0.0, 0.0, -20.0));
    plate.set_objectcolor(Color::grey());
    let count = plate.edges_with_colors().len();
    plate.set_linecolors(vec![Color::black(); count], vec![0.0; count]);

    let mut cylinder = BRep::create_cylinder(150.0, 400.0);
    cylinder.transform(&Xform::translation(-350.0, 0.0, 0.0));
    if std::env::var("BREP_PROBE_FLIPPED").is_ok() {
        for face in cylinder.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = brep_reverse(face.orientation);
        }
    }
    let mut cone = BRep::create_cone(150.0, 400.0);
    cone.transform(&Xform::translation(350.0, 0.0, 0.0));

    let mut s = Session::new("brep_probe");
    s.add_mesh(plate, None);
    s.add_brep(cylinder, None);
    s.add_brep(cone, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
