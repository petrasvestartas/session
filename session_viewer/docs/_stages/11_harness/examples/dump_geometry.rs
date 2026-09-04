// Print every object of a .pb: type, name, and its box (meshes) or points (polylines/lines).
fn main() {
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        let s = session_rust::Session::pb_loads(&bytes).expect("parse");
        let world = s.world_xforms();
        for g in s.order() {
            if let Some(x) = world.get(&g) && x.m != session_rust::Xform::identity().m {
                println!("  xform t=({:.0},{:.0},{:.0})", x.m[12], x.m[13], x.m[14]);
            }
            match s.lookup.get(&g) {
                Some(session_rust::Geometry::Mesh(m)) => {
                    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
                    for v in m.vertex.values() {
                        for (k, c) in [v.x, v.y, v.z].iter().enumerate() {
                            lo[k] = lo[k].min(*c);
                            hi[k] = hi[k].max(*c);
                        }
                    }
                    println!("mesh {:?} box {:?} .. {:?} color {:?}", m.name, lo, hi, m.objectcolor());
                }
                Some(session_rust::Geometry::Polyline(p)) => {
                    let pts: Vec<[f64; 3]> = p.coords.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
                    println!("polyline {:?} width {} color {:?} points {:?}", p.name, p.width, p.linecolor, pts);
                }
                Some(session_rust::Geometry::Point(pt)) => println!("point {:?}", [pt[0], pt[1], pt[2]]),
                Some(other) => println!("{}", std::any::type_name_of_val(other)),
                None => {}
            }
        }
    }
}
