//! PLY -> Session (.pb)
//!   ply_import <file.ply> <out_stem> [--points]
//!
//! A file carrying faces lands as a Mesh; one without lands as a PointCloud. `--points` forces the
//! cloud reading even when faces are present.
use session_io::ply;
use session_rust::Session;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("usage: ply_import <file.ply> <out_stem> [--points]");
        std::process::exit(2);
    }
    let (src, stem) = (&a[1], &a[2]);
    let force_points = a.iter().any(|x| x == "--points");

    let data = match ply::read_ply(src) {
        Ok(p) => p,
        Err(e) => { eprintln!("ply_import: {src}: {e}"); std::process::exit(1) }
    };
    let (np, nc, nf) = (data.points.len(), data.colors.len(), data.faces.len());

    let name = std::path::Path::new(stem)
        .file_name().and_then(|s| s.to_str()).unwrap_or("ply");
    let mut s = Session::new(name);

    let summary = if nf > 0 && !force_points {
        s.add_mesh(data.into_mesh(), None);
        format!("mesh {np} vertices, {nf} faces")
    } else {
        s.add_pointcloud(data.into_pointcloud(), None);
        format!("{np} points ({nc} coloured)")
    };

    let out = format!("{stem}.pb");
    s.pb_dump(&out);
    println!("{name}: {summary} -> {out}");
}
