//! Load each file twice and report any difference between the two loads.
use session_rust::{Session, Xform};
use session_viewer::app::scene::{FileDoc, Scene};

/// The GPU tables of one load.
fn tables(bytes: &[u8]) -> Scene {
    let s = Session::pb_loads(bytes).expect("pb_loads");
    let mut sc = Scene::new();
    sc.add_file(FileDoc {
        name: "d".into(),
        session: std::rc::Rc::new(s),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    sc
}

/// Check every file in the arguments; exit 1 if any differs.
fn main() {
    let mut bad = 0;
    for path in std::env::args().skip(1) {
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let name = path.rsplit('/').next().unwrap_or(&path).to_string();
        let mut fails: Vec<String> = Vec::new();

        // 1. GPU tables
        let (a, b) = (tables(&bytes), tables(&bytes));
        macro_rules! same {
            ($l:ident.$f:ident) => {
                if bytemuck::cast_slice::<_, u8>(&a.tables.$l.$f)
                    != bytemuck::cast_slice::<_, u8>(&b.tables.$l.$f)
                {
                    fails.push(format!("tables.{}.{}", stringify!($l), stringify!($f)));
                }
            };
        }
        same!(arena.verts);
        same!(arena.idx);
        same!(seg.ribbons);
        same!(seg.pipes);
        same!(glyph.spheres);
        same!(glyph.dots);
        same!(cloud.pos);
        same!(cloud.col);
        same!(cloud.nrm);
        if a.tables.bounds != b.tables.bounds {
            fails.push("tables.bounds".into())
        }

        // 2. mesh readers
        let (sa, sb) = (
            Session::pb_loads(&bytes).unwrap(),
            Session::pb_loads(&bytes).unwrap(),
        );
        for (i, (ma, mb)) in sa.objects.meshes.iter().zip(&sb.objects.meshes).enumerate() {
            let mut m = |what: &str| fails.push(format!("mesh[{i}].{what}"));
            if ma.area().to_bits() != mb.area().to_bits() {
                m("area")
            }
            if ma.volume().to_bits() != mb.volume().to_bits() {
                m("volume")
            }
            let (ca, cb) = (ma.centroid(), mb.centroid());
            if ca.to_f32() != cb.to_f32() {
                m("centroid")
            }
            if ma.is_closed() != mb.is_closed() {
                m("is_closed")
            }
            if ma
                .edges_with_colors()
                .iter()
                .map(|e| (e.0, e.1))
                .ne(mb.edges_with_colors().iter().map(|e| (e.0, e.1)))
            {
                m("edges_with_colors")
            }
            if ma.edge_face_map() != mb.edge_face_map() {
                m("edge_face_map")
            }
            if ma.to_vertices_and_faces().1 != mb.to_vertices_and_faces().1 {
                m("to_vertices_and_faces")
            }
            if ma.jsondump().unwrap() != mb.jsondump().unwrap() {
                m("jsondump");
                if std::env::var("DETAIL").is_ok() {
                    let (x, y) = (ma.jsondump().unwrap(), mb.jsondump().unwrap());
                    let at = x
                        .bytes()
                        .zip(y.bytes())
                        .position(|(p, q)| p != q)
                        .unwrap_or(x.len().min(y.len()));
                    let lo = at.saturating_sub(120);
                    println!(
                        "    jsondump differs at {at}:\n      A: ...{}\n      B: ...{}",
                        &x[lo..(at + 60).min(x.len())],
                        &y[lo..(at + 60).min(y.len())]
                    );
                }
            }
            // encoded bytes differ by map order; PB_BYTES=1 checks them anyway
            if std::env::var("PB_BYTES").is_ok() && ma.pb_dumps() != mb.pb_dumps() {
                m("pb_dumps")
            }
            let (wa, wb) = (ma.weld(0.001), mb.weld(0.001));
            if wa.to_vertices_and_faces().1 != wb.to_vertices_and_faces().1 {
                m("weld")
            }
            let (mut ua, mut ub) = ((**ma).clone(), (**mb).clone());
            ua.unify_winding();
            ub.unify_winding();
            if ua.to_vertices_and_faces().1 != ub.to_vertices_and_faces().1 {
                m("unify_winding")
            }
            if fails.len() > 40 {
                break;
            }
        }

        if fails.is_empty() {
            println!("{name}: DETERMINISTIC");
        } else {
            bad += 1;
            let mut seen: Vec<String> = Vec::new();
            for f in &fails {
                let kind = f.split('.').next_back().unwrap_or(f).to_string();
                if !seen.contains(&kind) {
                    seen.push(kind)
                }
            }
            println!("{name}: FLAKY -> {}", seen.join(", "));
        }
    }
    if bad > 0 {
        std::process::exit(1)
    }
}
