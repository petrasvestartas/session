// cargo run --example probe_mesh --target x86_64-unknown-linux-gnu --release -- <file.pb>...
// What is in a .pb, and which meshes are "print fills" (broadcast width 0)?
fn main() {
    use session_rust::Geometry;
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        let session = session_rust::Session::pb_loads(&bytes).expect("parse");
        println!("{path}:");
        let (mut nmesh, mut npoly, mut nline, mut npoint, mut nother) = (0, 0, 0, 0, 0);
        let mut width_hist: std::collections::HashMap<usize, usize> = Default::default();
        let mut red_meshes = 0;
        let mut print_fills = 0;
        let mut empty_widths = 0;
        let mut alpha_hist: std::collections::HashMap<String, usize> = Default::default();
        for guid in session.order() {
            match session.lookup.get(&guid) {
                Some(Geometry::Mesh(m)) => {
                    nmesh += 1;
                    let wl = m.widths().len();
                    *width_hist.entry(wl).or_default() += 1;
                    if wl == 0 { empty_widths += 1; }
                    if wl == 1 && m.widths()[0] == 0.0 { print_fills += 1; }
                    let oc = m.objectcolor();
                    *alpha_hist.entry(format!("{:.2}", oc.a)).or_default() += 1;
                    if oc.r > 0.5 && oc.g < 0.4 && oc.b < 0.4 { red_meshes += 1; }
                }
                Some(Geometry::Polyline(_)) => npoly += 1,
                Some(Geometry::Line(_)) => nline += 1,
                Some(Geometry::Point(_)) => npoint += 1,
                _ => nother += 1,
            }
        }
        println!("  meshes={nmesh} polylines={npoly} lines={nline} points={npoint} other={nother}");
        println!("  mesh widths_len histogram: {width_hist:?}");
        println!("  print_fills(broadcast 0)={print_fills} empty_widths={empty_widths} reddish_meshes={red_meshes}");
        println!("  mesh objectcolor alpha histogram: {alpha_hist:?}");
    }
}
