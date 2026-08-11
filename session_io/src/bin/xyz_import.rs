//! XYZ point cloud -> Session (.pb)
//!   xyz_import <file.xyz> <out_stem>
use session_io::xyz;
use session_rust::Session;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("usage: xyz_import <file.xyz> <out_stem>");
        std::process::exit(2);
    }
    let (src, stem) = (&a[1], &a[2]);

    let cloud = match xyz::read_xyz(src) {
        Ok(c) => c,
        Err(e) => { eprintln!("xyz_import: {src}: {e}"); std::process::exit(1) }
    };
    let (n, nc) = (cloud.point_count(), cloud.color_count());

    let name = std::path::Path::new(stem)
        .file_name().and_then(|s| s.to_str()).unwrap_or("xyz");
    let mut s = Session::new(name);
    s.add_pointcloud(cloud, None);

    let out = format!("{stem}.pb");
    s.pb_dump(&out);
    println!("{name}: {n} points ({nc} coloured) -> {out}");
}
