//! Adversarial hidden ink: four covers with a 4 mm top-face clearance. Every magenta
//! edge, authored outline and vertex lies well inside its grey cover in top projection.
//! Blue outlines lie on the visible top, except the blue square in the concave cover's
//! notch, which intentionally remains visible. Use VIEWER_VIEW=top, then iso/orbit.
//!
//! cargo run --release --example mk_hidden_line_probe --target x86_64-unknown-linux-gnu -- /tmp/hidden_line_probe.pb

use session_rust::{Color, Mesh, NurbsCurve, Point, Polyline, Session, Xform};

/// Box with its top at `top` and its centre at the supplied horizontal location.
fn slab(size: [f64; 3], centre: [f64; 2], top: f64) -> Mesh {
    let mut mesh = Mesh::create_box(size[0], size[1], size[2]);
    mesh.transform(&Xform::translation(centre[0], centre[1], top - size[2] * 0.5));
    mesh
}

/// Opaque grey geometry whose own zero-width edges do not add diagnostic ink.
fn cover_style(mesh: &mut Mesh) {
    let count = mesh.edges_with_colors().len();
    mesh.set_objectcolor(Color::grey());
    mesh.set_linecolors(vec![Color::black(); count], vec![0.0; count]);
    assert!(mesh.widths().len() > 1, "cover must remain physical mesh geometry, not a print fill");
}

/// Magenta mesh edges and vertex markers; the whole small box is behind the cover.
fn hidden_style(mesh: &mut Mesh) {
    let count = mesh.edges_with_colors().len();
    mesh.set_linecolors(vec![Color::magenta(); count], vec![-1.0; count]);
    mesh.set_pointcolors(vec![Color::magenta(); mesh.number_of_vertices()]);
    mesh.color_mode = session_rust::mesh::ColorMode::POINTCOLORS;
}

/// A closed authored outline, inset into its supporting physical face.
fn outline(centre: [f64; 2], half: [f64; 2], height: f64, color: Color) -> Polyline {
    let [x, y] = centre;
    let [a, b] = half;
    let mut line = Polyline::new(vec![
        Point::new(x - a, y - b, height), Point::new(x + a, y - b, height),
        Point::new(x + a, y + b, height), Point::new(x - a, y + b, height),
        Point::new(x - a, y - b, height),
    ]);
    line.linecolor = color;
    line.width = -1.0;
    line
}

/// Share one logical object between nonadjacent cover and hidden faces.
fn combine(cover: &Mesh, hidden: &Mesh) -> Mesh {
    let (mut points, mut faces) = cover.to_vertices_and_faces();
    let (other, triangles) = hidden.to_vertices_and_faces();
    let base = points.len();
    points.extend(other);
    for face in triangles { faces.push(face.into_iter().map(|key| key + base).collect()); }
    let mut mesh = Mesh::from_vertices_and_faces(points, faces);
    let edges = mesh.edges_with_colors();
    let mut colors = Vec::with_capacity(edges.len());
    let mut widths = Vec::with_capacity(edges.len());
    for (a, b, _) in edges {
        let hidden_edge = a >= base && b >= base;
        colors.push(if hidden_edge { Color::magenta() } else { Color::black() });
        widths.push(if hidden_edge { -1.0 } else { 0.0 });
    }
    mesh.set_objectcolor(Color::grey());
    mesh.set_linecolors(colors, widths);
    let mut points = vec![Color::grey(); base];
    points.extend(vec![Color::magenta(); mesh.number_of_vertices() - base]);
    mesh.set_pointcolors(points);
    mesh.color_mode = session_rust::mesh::ColorMode::POINTCOLORS;
    mesh
}

/// L-shaped slab with an explicit cached triangulation across its concave top and bottom.
fn concave_cover() -> Mesh {
    let ring = [[-500.0, -400.0], [500.0, -400.0], [500.0, -100.0], [0.0, -100.0], [0.0, 400.0], [-500.0, 400.0]];
    let mut points = Vec::new();
    for z in [0.0, -40.0] {
        for p in ring { points.push(Point::new(p[0], p[1], z)); }
    }
    let mut faces = vec![vec![0, 1, 2, 3, 4, 5], vec![11, 10, 9, 8, 7, 6]];
    for i in 0..6 { let j = (i + 1) % 6; faces.push(vec![i, i + 6, j + 6, j]); }
    let mut mesh = Mesh::from_vertices_and_faces(points, faces);
    let triangles = vec![[0, 1, 2], [0, 2, 3], [0, 3, 5], [3, 4, 5]];
    mesh.triangulation.insert(0, triangles.clone());
    mesh.triangulation.insert(1, triangles.into_iter().map(|t| [t[2] + 6, t[1] + 6, t[0] + 6]).collect());
    mesh
}

/// Apply a genuine nonuniform instance transform, plus a sloping top, without baking it.
fn placement(tile: usize) -> Xform {
    let mut place = Xform::identity();
    place.m[2] = [0.45, -0.55, 0.35, -0.3][tile];
    if tile == 2 { place.m[0] = 1.4; place.m[5] = 0.7; place.m[10] = 1.8; }
    place.m[12] = if tile.is_multiple_of(2) { -850.0 } else { 850.0 };
    place.m[13] = if tile < 2 { -650.0 } else { 650.0 };
    place
}

/// Add one placed mesh while retaining a real scene-level transform.
fn add_mesh(session: &mut Session, mesh: Mesh, place: &Xform) {
    let guid = mesh.guid().to_owned();
    session.add_mesh(mesh, None);
    session.set_xform(&guid, place.clone());
}

/// Add one placed polyline using the same instance transform as its supporting face.
fn add_line(session: &mut Session, line: Polyline, place: &Xform) {
    let guid = line.guid().to_owned();
    session.add_polyline(line, None);
    session.set_xform(&guid, place.clone());
}

/// A folded roof represented by one warped polygon, whose second triangle rises steeply.
fn warped_roof(scale: f64, top: f64, bottom: f64) -> Mesh {
    let ring = [[-500.0, -400.0, 0.0], [500.0, -400.0, 0.0], [500.0, 400.0, 0.0], [-500.0, 400.0, 300.0]];
    let mut points = Vec::new();
    for p in ring { points.push(Point::new(p[0] * scale, p[1] * scale, p[2] * scale + top)); }
    for p in ring { points.push(Point::new(p[0] * scale, p[1] * scale, bottom)); }
    Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3], vec![7, 6, 5, 4], vec![0, 4, 5, 1], vec![1, 5, 6, 2], vec![2, 6, 7, 3], vec![3, 7, 4, 0]])
}

/// The warped-face case would leak its rising lower edge if the first face normal were reused.
fn warped_probe(out: &str) {
    let mut session = Session::new("warped_hidden_line_probe");
    let mut cover = warped_roof(1.0, 0.0, -100.0);
    let mut hidden = warped_roof(0.5, -4.0, -24.0);
    cover_style(&mut cover);
    hidden_style(&mut hidden);
    session.add_mesh(cover, None);
    session.add_mesh(hidden, None);
    let mut visible = Polyline::new(vec![Point::new(-250.0, -200.0, 0.0), Point::new(250.0, -200.0, 0.0), Point::new(250.0, 200.0, 0.0), Point::new(-250.0, 200.0, 150.0), Point::new(-250.0, -200.0, 0.0)]);
    visible.linecolor = Color::blue();
    session.add_polyline(visible, None);
    session.pb_dump(out);
    println!("wrote {out}: warped covering polygons, zero magenta expected from above");
}

/// A separate variant covers authored free dots and sampled NURBS without changing the
/// established regular/warped fixtures: green dots and blue curve visible, magenta hidden.
fn authored_probe(out: &str) {
    let mut session = Session::new("authored_hidden_line_probe");
    let place = placement(2);
    let height = 0.123456789;
    let mut cover = slab([1000.0, 800.0, 40.0], [0.0; 2], height);
    cover_style(&mut cover);
    add_mesh(&mut session, cover, &place);
    for hidden in [false, true] {
        let z = height - if hidden { 4.0 } else { 0.0 };
        let y = if hidden { -220.0 } else { 60.0 };
        let mut curve = NurbsCurve::create(false, 2, &[
            Point::new(-300.0, y, z), Point::new(0.0, y + 240.0, z), Point::new(300.0, y, z),
        ]);
        curve.linecolors = vec![if hidden { Color::magenta() } else { Color::blue() }];
        curve.width = -1.0;
        let guid = curve.guid().to_string();
        session.add_nurbscurve(curve, None);
        session.set_xform(&guid, place.clone());
        for x in [-280.0, -140.0, 0.0, 140.0, 280.0] {
            let mut point = Point::new(x, if hidden { -300.0 } else { -50.0 }, z);
            point.pointcolor = if hidden { Color::magenta() } else { Color::green() };
            point.width = 24.0;
            let guid = point.guid().to_string();
            session.add_point(point, None);
            session.set_xform(&guid, place.clone());
        }
    }
    session.pb_dump(out);
    println!("wrote {out}: visible blue NURBS and five green coplanar dots; 4 mm lower magenta controls must be absent");
}

/// Build the established fixtures unless a separate coverage variant is requested.
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "/tmp/hidden_line_probe.pb".into());
    if std::env::var_os("HIDDEN_LINE_PROBE_AUTHORED").is_some() { authored_probe(&out); return; }
    if std::env::var_os("HIDDEN_LINE_PROBE_WARPED").is_some() { warped_probe(&out); return; }
    let mut session = Session::new("hidden_line_probe");
    for tile in 0..4 {
        let place = placement(tile);
        let centre = if tile == 3 { [-250.0, 0.0] } else { [0.0, 0.0] };
        let half = if tile == 3 { [100.0, 120.0] } else { [230.0, 130.0] };
        let mut cover = if tile == 3 { concave_cover() } else { slab([1000.0, 800.0, 40.0], [0.0; 2], 0.0) };
        let mut hidden = slab([half[0] * 2.0, half[1] * 2.0, 20.0], centre, -4.0);
        if tile == 1 {
            add_mesh(&mut session, combine(&cover, &hidden), &place);
        } else {
            cover_style(&mut cover);
            hidden_style(&mut hidden);
            add_mesh(&mut session, cover, &place);
            add_mesh(&mut session, hidden, &place);
        }
        add_line(&mut session, outline(centre, [half[0] - 20.0, half[1] - 20.0], -4.0, Color::magenta()), &place);
        add_line(&mut session, outline(centre, half, 0.0, Color::blue()), &place);
        if tile == 3 { add_line(&mut session, outline([250.0, 200.0], [100.0, 100.0], -4.0, Color::blue()), &place); }
    }
    session.pb_dump(&out);
    println!("wrote {out}: magenta must be zero looking from above; blue must remain continuous");
}
