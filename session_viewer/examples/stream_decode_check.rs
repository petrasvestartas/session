//! Does the streaming reader see exactly what the kernel parser sees?
//!
//! The browser opens a large cloud by BYTE RANGE: it locates `coords` by walking tag/length
//! varints and casts the bytes, never building a protobuf message. That is only safe if it
//! lands on the kernel's own values, and a wrong offset is silent - it renders a plausible
//! cloud in the wrong place or the wrong colour. So assert it, against the whole file.
//!
//!   cargo run --release --target x86_64-unknown-linux-gnu --example stream_decode_check -- <file.pb>

use session_rust::{Geometry, Session};
use session_viewer::app::stream::{positions_from, varint, walk_to_coords};

fn main() {
    let path = std::env::args().nth(1).expect("usage: stream_decode_check <file.pb>");
    let bytes = std::fs::read(&path).expect("read");
    let session = Session::pb_loads(&bytes).expect("parse");
    let g = session.order()[0].clone();
    let Some(Geometry::PointCloud(pc)) = session.lookup.get(&g) else { panic!("not a cloud") };

    // COORDS: field 3, located the way `cloud_fields` locates it.
    let (at, len) = walk_to_coords(&bytes).expect("no coords field");
    let streamed = positions_from(&bytes[at as usize..(at + len) as usize]);
    let kernel = pc.coords();
    assert_eq!(streamed.len(), kernel.len(), "coord count");
    let worst = streamed.iter().zip(kernel).fold(0.0f64, |m, (a, b)| m.max((*a as f64 - *b).abs()));
    // The reader casts f64 to f32 for the GPU, so an f32 ulp is the only allowed difference.
    let tol = kernel.iter().fold(0.0f64, |m, v| m.max(v.abs())) * f32::EPSILON as f64;
    assert!(worst <= tol, "coords differ by more than an f32 cast: {worst:e} > {tol:e}");
    println!("coords: {} values identical to the kernel (worst {worst:.3e}, f32 bound {tol:.3e})", streamed.len());

    // COLOURS: the tag/length pair immediately after the coords run, then packed varints.
    let after = (at + len) as usize;
    let (tag, n) = varint(&bytes, after).expect("tag after coords");
    assert_eq!((tag >> 3, tag & 7), (4, 2), "expected the colours field next");
    let (clen, n2) = varint(&bytes, after + n).expect("colours length");
    let (mut j, mut seen) = (after + n + n2, 0usize);
    let stop = j + clen as usize;
    let kc = pc.colors();
    while j < stop && seen * 4 < kc.len() {
        for k in 0..4 {
            let (v, m) = varint(&bytes, j).expect("colour varint");
            j += m;
            assert_eq!(v as i64, kc[seen * 4 + k] as i64, "colour {seen} channel {k}");
        }
        seen += 1;
    }
    assert_eq!(seen * 4, kc.len(), "colour count");
    println!("colors: {seen} points, every channel identical to the kernel");

    // The browser frames the scene off the PREFIX it has resident, not the whole cloud, so the
    // prefix's box has to be the cloud's box or `fit` aims at the wrong place.
    let box_of = |n: usize| {
        let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in kernel.chunks_exact(3).take(n) {
            for a in 0..3 { lo[a] = lo[a].min(p[a]); hi[a] = hi[a].max(p[a]); }
        }
        (lo, hi)
    };
    let (flo, fhi) = box_of(kernel.len() / 3);
    let (plo, phi) = box_of(2_000_000);
    let cover = (0..3).map(|a| (phi[a] - plo[a]) / (fhi[a] - flo[a])).fold(f64::INFINITY, f64::min);
    assert!(cover > 0.99, "the 2 M prefix box is only {cover:.3} of the cloud's - `fit` would be wrong");
    println!("bounds: the 2 M-point prefix spans {cover:.4} of the full box");
}
