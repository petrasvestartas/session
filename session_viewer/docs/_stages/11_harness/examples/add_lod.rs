//! Write the LOD octree into a .pb: load, `build_lod`, save.
//!
//! The octree is built ONCE, offline, by whoever publishes the cloud - a browser paying ten
//! seconds per 14 M cloud to recompute what the file could have carried is the trade this
//! avoids. `build_lod` also REORDERS the points into octree order, so every node becomes one
//! contiguous byte range in the written file and a reader can fetch a node with one HTTP Range
//! request instead of downloading the whole cloud.
//!
//!   cargo run --example add_lod --target x86_64-unknown-linux-gnu --release -- <file.pb> [leaf]
//!
//! The root grid spacing is always derived from the cloud's own bounding box; each level halves
//! it. Leaf capacity defaults to 8192.

use std::time::Instant;

use session_rust::{Geometry, Session};

/// Root grid spacing from the cloud's own size: the longest bounding-box edge over 128, which
/// is Potree's rule of thumb. It has to scale with the cloud or the same number means a coarse
/// tree on one scan and a pointlessly deep one on the next.
fn auto_spacing(coords: &[f64]) -> f64 {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in coords.chunks_exact(3) {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let edge = (0..3).map(|k| hi[k] - lo[k]).fold(0.0, f64::max);
    (edge / 128.0).max(1.0e-6)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: add_lod <file.pb> [root_spacing] [leaf_capacity]");
    // The root grid is ALWAYS derived from the cloud - see `auto_spacing`. There is no manual
    // override on purpose: a per-cloud magic number is a number nobody can justify later, and
    // the derived one measured BETTER than the hand-tuned values it replaced (13.8 M scan at
    // the fit view: 492x fewer points, against 363x for the tuned tree).
    let leaf_capacity: usize = args.next().map_or(8192, |v| v.parse().expect("leaf_capacity"));

    let t = Instant::now();
    let bytes = std::fs::read(&path).expect("read");
    let mut session = Session::pb_loads(&bytes).expect("parse");
    println!("read           {:>7.0} ms  ({:.1} MB)", t.elapsed().as_secs_f64() * 1e3, bytes.len() as f64 / 1.048576e6);

    let guids: Vec<String> = session.order().to_vec();
    let mut built = 0usize;
    for g in guids {
        let Some(Geometry::PointCloud(rc)) = session.lookup.get(&g) else { continue };
        let mut pc = (**rc).clone();
        let n = pc.point_count();
        let spacing = auto_spacing(pc.coords());
        let t = Instant::now();
        pc.build_lod(spacing, leaf_capacity);
        println!("build_lod      {:>7.0} ms  ({} points -> {} nodes, spacing {:.0})", t.elapsed().as_secs_f64() * 1e3, n, pc.lod_node_count(), spacing);
        session.lookup.insert(g, Geometry::PointCloud(std::rc::Rc::new(pc)));
        built += 1;
    }
    if built == 0 {
        println!("no point clouds in {path} - nothing to do");
        return;
    }

    let t = Instant::now();
    let out = session.pb_dumps();
    std::fs::write(&path, &out).expect("write");
    println!("write          {:>7.0} ms  ({:.1} MB, {:+.1} MB)", t.elapsed().as_secs_f64() * 1e3,
        out.len() as f64 / 1.048576e6, (out.len() as f64 - bytes.len() as f64) / 1.048576e6);
}
