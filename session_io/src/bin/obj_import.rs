//! OBJ -> Session (.pb)
//!   obj_import <file.obj> <out_stem> [--polylines]
//!
//! Default reads the `v`/`f` mesh. `--polylines` reads `curv` runs instead, which is how the
//! joinery datasets store their outlines.
use session_io::obj;
use session_rust::Session;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("usage: obj_import <file.obj> <out_stem> [--polylines]");
        std::process::exit(2);
    }
    let (src, stem) = (&a[1], &a[2]);
    let polylines = a.iter().any(|x| x == "--polylines");

    let name = std::path::Path::new(stem)
        .file_name().and_then(|s| s.to_str()).unwrap_or("obj");
    let mut s = Session::new(name);

    let summary = if polylines {
        let pls = match obj::read_file_obj_polylines(src) {
            Ok(p) => p,
            Err(e) => { eprintln!("obj_import: {src}: {e}"); std::process::exit(1) }
        };
        let n = pls.len();
        for pl in pls { s.add_polyline(pl, None); }
        format!("{n} polylines")
    } else {
        let mesh = match obj::read_file_obj(src) {
            Ok(m) => m,
            Err(e) => { eprintln!("obj_import: {src}: {e}"); std::process::exit(1) }
        };
        let (v, f) = (mesh.number_of_vertices(), mesh.number_of_faces());
        s.add_mesh(mesh, None);
        format!("mesh {v} vertices, {f} faces")
    };

    let out = format!("{stem}.pb");
    s.pb_dump(&out);
    println!("{name}: {summary} -> {out}");
}
