//! CAD fixture: each solid once at the source and once under an affine instance.
use session_rust::{BRep, Color, NurbsSurface, Point, Session, Xform};
use std::path::Path;

/// A mirrored, rotated, unevenly scaled placement.
fn affine_placement() -> Xform {
    let scale = Xform::from_matrix([
        -1.4, 0.0, 0.0, 0.0, 0.0, 0.65, 0.0, 0.0, 0.0, 0.0, 1.15, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);
    Xform::rotation_z(23.0, true) * Xform::rotation_x(17.0, true) * scale
}

/// A folded degree-one surface with a crease.
fn crease_surface() -> NurbsSurface {
    let points = [
        Point::new(-200.0, -100.0, 0.0),
        Point::new(-200.0, 100.0, 0.0),
        Point::new(0.0, -100.0, 0.0),
        Point::new(0.0, 100.0, 0.0),
        Point::new(200.0, -100.0, 160.0),
        Point::new(200.0, 100.0, 160.0),
    ];
    let mut surface = NurbsSurface::create(false, false, 1, 1, 3, 2, &points).unwrap();
    surface.facecolors = vec![Color::grey()];
    surface.name = "CAD C0 crease".into();
    surface
}

/// The named grey solid.
fn solid(kind: &str) -> BRep {
    let mut brep = match kind {
        "cylinder" => BRep::create_cylinder(120.0, 240.0),
        "sphere" => BRep::create_sphere(160.0),
        "hole" => BRep::create_block_with_hole(400.0, 300.0, 120.0, 70.0),
        _ => panic!("unknown CAD fixture"),
    };
    brep.name = format!("CAD {kind}");
    brep.surfacecolor = Color::grey();
    brep
}

/// Write one solid, source or affine, as its own scene.
fn write_case(directory: &Path, kind: &str, affine: bool) {
    let suffix = if affine { "affine" } else { "source" };
    let mut scene = Session::new(&format!("CAD {kind} {suffix}"));
    let guid = if kind == "crease" {
        let surface = crease_surface();
        let guid = surface.guid().to_string();
        scene.add_nurbssurface(surface, None);
        guid
    } else {
        let brep = solid(kind);
        let guid = brep.guid().to_string();
        scene.add_brep(brep, None);
        guid
    };
    if affine {
        scene.set_xform(&guid, affine_placement());
    }
    let path = directory.join(format!("{kind}-{suffix}.pb"));
    scene.pb_dump(path.to_str().unwrap());
    println!("{} {guid}", path.display());
}

/// Write all eight scenes into `argv[1]`.
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/cad-fixture".into());
    let directory = Path::new(&directory);
    std::fs::create_dir_all(directory).unwrap();
    for kind in ["cylinder", "sphere", "hole", "crease"] {
        write_case(directory, kind, false);
        write_case(directory, kind, true);
    }
}
