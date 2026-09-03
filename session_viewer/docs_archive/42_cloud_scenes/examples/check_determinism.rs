// Flake hunt: load each file TWICE in one process and compare everything a golden test could
// look at. Rust seeds every HashMap differently, so any place the kernel lets map iteration
// order reach an ORDERED result (a Vec, a float sum, a last-writer-wins insert) shows up here as
// a difference between two loads of identical bytes.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example check_determinism -- <file.pb>...
//
// Green means: two loads of the same bytes produce the same GPU tables, the same area/volume/
// centroid, the same edge and halfedge topology, the same JSON. `PB_BYTES=1` additionally
// requires the ENCODED .pb bytes to match - see the note at that check for why it is off.
use session_rust::{Session, Xform};
use session_viewer::app::scene::Scene;

fn tables(bytes: &[u8]) -> Scene {
    let s = Session::pb_loads(bytes).expect("pb_loads");
    let mut sc = Scene::new();
    sc.add_file("d".into(), s, Xform::identity(), 0.0, false);
    sc
}

fn main() {
    let mut bad = 0;
    for path in std::env::args().skip(1) {
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let name = path.rsplit('/').next().unwrap_or(&path).to_string();
        let mut fails: Vec<String> = Vec::new();

        // 1. the GPU tables, byte for byte
        let (a, b) = (tables(&bytes), tables(&bytes));
        macro_rules! same { ($f:ident) => {
            if bytemuck::cast_slice::<_, u8>(&a.tables.$f) != bytemuck::cast_slice::<_, u8>(&b.tables.$f) {
                fails.push(format!("tables.{}", stringify!($f)));
            }
        }; }
        same!(verts); same!(idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
        same!(cloud_pos); same!(cloud_col); same!(cloud_nrm);
        if a.tables.min != b.tables.min || a.tables.max != b.tables.max { fails.push("tables.bounds".into()) }

        // 2. per-mesh kernel readers a test or an exporter would call
        let (sa, sb) = (Session::pb_loads(&bytes).unwrap(), Session::pb_loads(&bytes).unwrap());
        for (i, (ma, mb)) in sa.objects.meshes.iter().zip(&sb.objects.meshes).enumerate() {
            let mut m = |what: &str| fails.push(format!("mesh[{i}].{what}"));
            if ma.area().to_bits() != mb.area().to_bits() { m("area") }
            if ma.volume().to_bits() != mb.volume().to_bits() { m("volume") }
            let (ca, cb) = (ma.centroid(), mb.centroid());
            if ca.to_f32() != cb.to_f32() { m("centroid") }
            if ma.is_closed() != mb.is_closed() { m("is_closed") }
            if ma.edges_with_colors().iter().map(|e| (e.0, e.1)).ne(
               mb.edges_with_colors().iter().map(|e| (e.0, e.1))) { m("edges_with_colors") }
            if ma.edge_face_map() != mb.edge_face_map() { m("edge_face_map") }
            if ma.to_vertices_and_faces().1 != mb.to_vertices_and_faces().1 { m("to_vertices_and_faces") }
            if ma.jsondump().to_string() != mb.jsondump().to_string() {
                m("jsondump");
                if std::env::var("DETAIL").is_ok() {
                    let (x, y) = (ma.jsondump().to_string(), mb.jsondump().to_string());
                    let at = x.bytes().zip(y.bytes()).position(|(p, q)| p != q).unwrap_or(x.len().min(y.len()));
                    let lo = at.saturating_sub(120);
                    println!("    jsondump differs at {at}:\n      A: ...{}\n      B: ...{}",
                        &x[lo..(at + 60).min(x.len())], &y[lo..(at + 60).min(y.len())]);
                }
            }
            // ACCEPTED EXCEPTION, not an oversight: prost writes a map field in iteration order,
            // and Mesh's four big maps (vertices, faces, halfedges, triangulation) stay HashMap
            // ON PURPOSE - a BTreeMap there cost 55% on every DECODE (216 -> 338 ms on one 52 MB
            // sheet) to fix an order that only matters when WRITING, and the two encodings are
            // the same file semantically. Nothing in the repo compares .pb bytes. PB_BYTES=1
            // re-enables the check if that ever changes.
            if std::env::var("PB_BYTES").is_ok() && ma.pb_dumps() != mb.pb_dumps() { m("pb_dumps") }
            let (wa, wb) = (ma.weld(0.001), mb.weld(0.001));
            if wa.to_vertices_and_faces().1 != wb.to_vertices_and_faces().1 { m("weld") }
            let (mut ua, mut ub) = ((**ma).clone(), (**mb).clone());
            ua.unify_winding(); ub.unify_winding();
            if ua.to_vertices_and_faces().1 != ub.to_vertices_and_faces().1 { m("unify_winding") }
            if fails.len() > 40 { break }
        }

        if fails.is_empty() {
            println!("{name}: DETERMINISTIC");
        } else {
            bad += 1;
            let mut seen: Vec<String> = Vec::new();
            for f in &fails {
                let kind = f.split('.').next_back().unwrap_or(f).to_string();
                if !seen.contains(&kind) { seen.push(kind) }
            }
            println!("{name}: FLAKY -> {}", seen.join(", "));
        }
    }
    if bad > 0 { std::process::exit(1) }
}
