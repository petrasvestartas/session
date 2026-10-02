//! Walk profile: what one document costs to walk, by object kind, before the GPU sees it.

use crate::app::walk::brep::QUALITY;
use crate::app::walk::brep_edges::edge_chains;
use crate::app::walk::brep_orient::face_signs;
use crate::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
use crate::engine::gpu::Upload;
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, Session};
use std::collections::BTreeMap;
use std::time::Instant;

/// Totals of one kind of object.
#[derive(Default)]
struct Kind {
    objects: usize,  // objects walked
    ms: f64,         // walk time
    triangles: u64,  // face triangles
    vertices: u64,   // face vertices
    pipes: u64,      // edge segments
    ribbons: u64,    // curve segments
    spheres: u64,    // vertex markers
    dots: u64,       // point markers
    features: u64,   // element feature outlines
    faces: u64,      // BRep faces
    edges: u64,      // BRep edges
    mesh_ms: f64,    // BRep face meshing
    chains_ms: f64,  // BRep edge chains
    signs_ms: f64,   // BRep face orientation
}

/// The kind an object is reported under.
fn kind_of(geom: &Geometry) -> &'static str {
    match geom {
        Geometry::Mesh(_) => "mesh",
        Geometry::BRep(_) => "brep",
        Geometry::NurbsSurface(_) => "surface",
        Geometry::Line(_) => "line",
        Geometry::Polyline(_) => "polyline",
        Geometry::NurbsCurve(_) => "curve",
        Geometry::Plane(_) => "plane",
        Geometry::OBB(_) => "obb",
        Geometry::Point(_) => "point",
        Geometry::PointCloud(_) => "cloud",
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(_) => "element.mesh",
            ElementGeometry::BRep(_) => "element.brep",
            ElementGeometry::None => "element.none",
        },
    }
}

/// The BRep an object carries, if any.
fn brep_of(geom: &Geometry) -> Option<&BRep> {
    match geom {
        Geometry::BRep(b) => Some(b),
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::BRep(b) => Some(b),
            _ => None,
        },
        _ => None,
    }
}

/// Time the three BRep stages on their own.
fn brep_stages(b: &BRep, kind: &mut Kind) {
    let t = Instant::now();
    let fms = b.face_meshes_q(Some(QUALITY));
    kind.mesh_ms += t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    let chains = edge_chains(b, &fms);
    kind.chains_ms += t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    let _ = face_signs(b, &fms, &chains);
    kind.signs_ms += t.elapsed().as_secs_f64() * 1000.0;
    kind.faces += b.face_count() as u64;
    kind.edges += b.edge_count() as u64;
}

/// Walk every drawable object of `session` into a throwaway upload and print the totals by kind.
pub fn document(session: &Session) {
    let mut kinds: BTreeMap<&'static str, Kind> = BTreeMap::new();
    let mut up = Upload::default();
    let mut slowest: Vec<(f64, String)> = Vec::new();

    for guid in session.order() {
        let Some(geom) = session.lookup.get(&guid) else {
            continue;
        };

        if !is_drawable(geom) {
            continue;
        }

        let kind = kinds.entry(kind_of(geom)).or_default();
        let before = counts(&up);
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row: kind.objects as u32,
            attributes: true,
        };
        let t = Instant::now();
        let row = walk_geometry(&mut Walk::of(&mut up), &cx, geom);
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        let after = counts(&up);
        kind.objects += 1;
        kind.ms += ms;
        kind.triangles += after[0] - before[0];
        kind.vertices += after[1] - before[1];
        kind.pipes += after[2] - before[2];
        kind.ribbons += after[3] - before[3];
        kind.spheres += after[4] - before[4];
        kind.dots += after[5] - before[5];

        if let Geometry::Element(e) = geom {
            kind.features += e.features().iter().map(|f| f.outlines.len() as u64).sum::<u64>();
        }

        if let Some(b) = brep_of(geom) {
            brep_stages(b, kind);

            // VIEWER_PROFILE=circles lists every curved edge's centre and span
            if std::env::var("VIEWER_PROFILE").is_ok_and(|v| v == "circles") {
                print_curved_edges(b);
            }
        }

        let name = match geom {
            Geometry::Element(e) => e.name.clone(),
            Geometry::BRep(b) => b.name.clone(),
            Geometry::Mesh(m) => m.name.clone(),
            _ => guid.clone(),
        };
        slowest.push((ms, format!("{} {name}", kind_of(geom))));

        // VIEWER_PROFILE=names also lists every object with its box center and size
        if std::env::var("VIEWER_PROFILE").is_ok_and(|v| v == "names") {
            let b = &row.bounds;
            println!(
                "  object {name}: {} center ({:.0}, {:.0}, {:.0}) half ({:.0}, {:.0}, {:.0})",
                kind_of(geom),
                b.cx,
                b.cy,
                b.cz,
                b.hx,
                b.hy,
                b.hz
            );
        }
    }

    println!("profile: {} objects in the file, {} instances", session.lookup.len(), session.instance_lookup.len());
    println!(
        "{:<14} {:>5} {:>9} {:>9} {:>9} {:>8} {:>8} {:>8} {:>7} {:>8} {:>6} {:>6} {:>8} {:>8} {:>8}",
        "kind", "n", "walk ms", "tris", "verts", "pipes", "ribbons", "spheres", "dots", "features", "faces", "edges", "mesh ms", "chain ms", "sign ms"
    );

    for (name, k) in &kinds {
        println!(
            "{:<14} {:>5} {:>9.1} {:>9} {:>9} {:>8} {:>8} {:>8} {:>7} {:>8} {:>6} {:>6} {:>8.1} {:>8.1} {:>8.1}",
            name, k.objects, k.ms, k.triangles, k.vertices, k.pipes, k.ribbons, k.spheres, k.dots, k.features, k.faces, k.edges, k.mesh_ms, k.chains_ms, k.signs_ms
        );
    }

    let total = counts(&up);
    println!(
        "profile totals: {} triangles, {} vertices, {} pipes, {} ribbons, {} spheres, {} dots, {:.1} ms walk",
        total[0],
        total[1],
        total[2],
        total[3],
        total[4],
        total[5],
        kinds.values().map(|k| k.ms).sum::<f64>()
    );
    slowest.sort_by(|a, b| b.0.total_cmp(&a.0));

    for (ms, name) in slowest.iter().take(8) {
        println!("  slowest: {ms:.1} ms {name}");
    }
}

/// Row counts of an upload: triangles, vertices, pipes, ribbons, spheres, dots.
fn counts(up: &Upload) -> [u64; 6] {
    [
        (up.arena.idx.len() / 3) as u64,
        up.arena.verts.len() as u64,
        up.seg.pipes.len() as u64,
        up.seg.ribbons.len() as u64,
        up.glyph.spheres.len() as u64,
        up.glyph.dots.len() as u64,
    ]
}

/// Print the centre and extent of every curved edge of `b`: where its circles and arcs are.
fn print_curved_edges(b: &BRep) {
    for (ei, edge) in b.m_edges.iter().enumerate() {
        if edge.degenerated || edge.curve_3d_index < 0 {
            continue;
        }

        let curve = &b.m_curves_3d[edge.curve_3d_index as usize];

        if curve.cv_count() < 3 {
            continue;
        }

        let (t0, t1) = curve.domain();
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];

        for i in 0..=16 {
            let p = curve.point_at(t0 + (t1 - t0) * i as f64 / 16.0);

            for axis in 0..3 {
                lo[axis] = lo[axis].min(p[axis]);
                hi[axis] = hi[axis].max(p[axis]);
            }
        }

        println!(
            "  curved {} edge {ei}: centre ({:.0}, {:.0}, {:.0}) extent ({:.0}, {:.0}, {:.0}) faces {}",
            b.name,
            (lo[0] + hi[0]) * 0.5,
            (lo[1] + hi[1]) * 0.5,
            (lo[2] + hi[2]) * 0.5,
            hi[0] - lo[0],
            hi[1] - lo[1],
            hi[2] - lo[2],
            b.edge_faces(ei).len()
        );
    }
}
