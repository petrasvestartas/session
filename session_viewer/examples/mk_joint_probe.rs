// Joint probe: the strokes where solids touch must weigh the same as free edges, and ink
// inside another solid must stay hidden.
//   1. A box standing on a plate: its four bottom edges lie on the plate's top (red).
//   2. Two boxes sharing a face: the first box's four shared-face edges lie on the second
//      box's face (red); the second box's edges are black.
//   3. A box whose top 3 mm is inside a beam: its four top edges are 3 mm behind the beam's
//      bottom face (magenta = must never show). The beam is 600 mm across the 400 mm box.
//   4. A 400 mm wide beam 4 mm above a 260 mm wide plate outline polyline (magenta = must
//      never show from above).
// Every other edge of the red boxes is blue, the reference weight. Each beam is wider than the
// ink it must hide: an occluder narrower than its stroke leaves the ends of that stroke in
// plain sight, which is the probe failing, not the ink rule.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_joint_probe -- <out.pb>
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};

/// A grey slab with its zero-width edges, so it adds no ink of its own.
fn slab(size: [f64; 3], at: [f64; 3]) -> Mesh {
    let mut m = Mesh::create_box(size[0], size[1], size[2]);
    m.transform(&Xform::translation(at[0], at[1], at[2]));
    m.set_objectcolor(Color::grey());
    let n = m.edges_with_colors().len();
    m.set_linecolors(vec![Color::black(); n], vec![0.0; n]);
    m
}

/// A 400 mm box centred at `at` whose edges satisfying `special` (both ends) take `color`,
/// the rest blue; every edge at the default pen.
fn marked_box(at: [f64; 3], special: fn(&Point) -> bool, color: Color) -> Mesh {
    let mut m = Mesh::create_box(400.0, 400.0, 400.0);
    m.transform(&Xform::translation(at[0], at[1], at[2]));
    m.set_objectcolor(Color::grey());
    let edges = m.edges_with_colors();
    let mut colors = Vec::with_capacity(edges.len());
    for (a, b, _) in &edges {
        let (pa, pb) = (m.vertex_point(*a).unwrap(), m.vertex_point(*b).unwrap());
        colors.push(if special(&pa) && special(&pb) { color.clone() } else { Color::blue() });
    }
    m.set_linecolors(colors, vec![-1.0; edges.len()]);
    m
}

/// Write the four cases into one session at `argv[1]`, spaced 2000 mm apart along x so one
/// camera frames them all and no case occludes another.
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/joint_probe.pb".into());
    let mut s = Session::new("joint_probe");

    // 1. box on a plate: plate top at z = 0, box bottom edges at z = 0.
    s.add_mesh(slab([1200.0, 1200.0, 40.0], [0.0, 0.0, -20.0]), None);
    s.add_mesh(marked_box([0.0, 0.0, 200.0], |p| p[2].abs() < 1e-9, Color::red()), None);

    // 2. two boxes sharing the face x = 2200.
    s.add_mesh(marked_box([2000.0, 0.0, 200.0], |p| (p[0] - 2200.0).abs() < 1e-9, Color::red()), None);
    s.add_mesh(slab([400.0, 400.0, 400.0], [2400.0, 0.0, 200.0]), None);

    // 3. box top at z = 400 inside a beam whose bottom is at z = 397.
    s.add_mesh(marked_box([4000.0, 0.0, 200.0], |p| (p[2] - 400.0).abs() < 1e-9, Color::magenta()), None);
    s.add_mesh(slab([1200.0, 600.0, 200.0], [4000.0, 0.0, 497.0]), None);

    // 4. a plate whose top outline is 4 mm under a beam.
    s.add_mesh(slab([1200.0, 300.0, 40.0], [6000.0, 0.0, -20.0]), None);
    let mut outline = Polyline::new(vec![
        Point::new(5420.0, -130.0, 0.0), Point::new(6580.0, -130.0, 0.0), Point::new(6580.0, 130.0, 0.0),
        Point::new(5420.0, 130.0, 0.0), Point::new(5420.0, -130.0, 0.0),
    ]);
    outline.linecolor = Color::magenta();
    s.add_polyline(outline, None);
    s.add_mesh(slab([1400.0, 400.0, 200.0], [6000.0, 0.0, 104.0]), None);

    s.pb_dump(&out);
    println!("wrote {out}");
}
