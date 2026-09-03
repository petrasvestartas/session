//! EXACT live-heap accounting for one .pb, via a counting global allocator.
//!
//! RSS cannot answer "what costs what" - it counts allocator slack and never shrinks. A counting
//! allocator can: load the file, then DROP one part at a time and read the delta.
use session_rust::{Session, Geometry, Line, Point, Polyline, Mesh, Color, NurbsCurve};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::Instant;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static NALLOC: AtomicUsize = AtomicUsize::new(0);

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let n = LIVE.fetch_add(l.size(), Relaxed) + l.size();
        PEAK.fetch_max(n, Relaxed);
        NALLOC.fetch_add(1, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new: usize) -> *mut u8 {
        LIVE.fetch_add(new, Relaxed);
        LIVE.fetch_sub(l.size(), Relaxed);
        PEAK.fetch_max(LIVE.load(Relaxed), Relaxed);
        NALLOC.fetch_add(1, Relaxed);
        unsafe { System.realloc(p, l, new) }
    }
}
#[global_allocator]
static A: Counting = Counting;

fn live() -> f64 { LIVE.load(Relaxed) as f64 / 1.048576e6 }
fn mb(b: usize) -> f64 { b as f64 / 1.048576e6 }

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let file_mb = mb(bytes.len());
    let base = live();
    let t = Instant::now();
    let mut s = Session::pb_loads(&bytes).unwrap();
    let decode = t.elapsed();
    drop(bytes);
    let n0 = NALLOC.load(Relaxed);
    let total = live() - base + file_mb;
    println!("{}", path.rsplit('/').next().unwrap());
    println!("  file {file_mb:.1} MB | pb_loads {decode:?} | live heap {:.1} MB ({:.1}x file) | peak {:.1} MB | {:.2} M allocations",
        total, total / file_mb, mb(PEAK.load(Relaxed)), n0 as f64 / 1e6);
    println!("  sizeof: Line {} Point {} Polyline {} Mesh {} Color {} NurbsCurve {} Geometry {}",
        std::mem::size_of::<Line>(), std::mem::size_of::<Point>(), std::mem::size_of::<Polyline>(),
        std::mem::size_of::<Mesh>(), std::mem::size_of::<Color>(), std::mem::size_of::<NurbsCurve>(),
        std::mem::size_of::<Geometry>());
    let o = &s.objects;
    println!("  counts: {} lines {} plines {} points {} meshes {} nurbs | lookup {} | graph v {} e {}",
        o.lines.len(), o.polylines.len(), o.points.len(), o.meshes.len(), o.nurbscurves.len(),
        s.lookup.len(), s.graph.vertex_count, s.graph.edges.len());
    let (mv, mf): (usize, usize) = o.meshes.iter().fold((0, 0), |(a, b), m| (a + m.vertex.len(), b + m.face.len()));
    println!("  mesh interiors: {mv} verts {mf} faces");

    // Drop one part at a time; each delta is that part's exact live cost.
    let mut prev = live();
    let mut step = |name: &str, now: f64, prev: &mut f64| {
        println!("  {name:<28} {:>7.1} MB", *prev - now);
        *prev = now;
    };
    s.lookup = Default::default();               step("lookup (guid -> Rc)", live(), &mut prev);
    s.graph = Default::default();                step("graph", live(), &mut prev);
    s.tree = Default::default();                 step("tree", live(), &mut prev);
    {
        let lines = std::mem::take(&mut s.objects.lines);
        let mut owned: Vec<Line> = lines.into_iter().filter_map(|l| std::rc::Rc::try_unwrap(l).ok()).collect();
        println!("  unwrapped {} lines", owned.len());
        let mut p2 = live();
        for l in owned.iter_mut() { l.name = String::new(); }
        step("  line.name", live(), &mut p2);
        for l in owned.iter_mut() { l.dash = Vec::new(); }
        step("  line.dash", live(), &mut p2);
        for l in owned.iter_mut() { l.linecolor.name = String::new(); }
        step("  line.linecolor.name", live(), &mut p2);
        for l in owned.iter_mut() { l.linecolor = Color::new(0.0, 0.0, 0.0, 1.0); }
        step("  line.linecolor.guid", live(), &mut p2);
        drop(owned);
        step("  line guid + struct", live(), &mut p2);
    }
    step("lines TOTAL", live(), &mut prev);
    s.objects.nurbscurves = Vec::new();          step("nurbscurves", live(), &mut prev);
    s.objects.polylines = Vec::new();            step("polylines", live(), &mut prev);
    s.objects.points = Vec::new();               step("points", live(), &mut prev);
    {
        println!("  sizeof VertexData {} | HashMap {} | Vec<usize> {}",
            std::mem::size_of::<session_rust::mesh::VertexData>(),
            std::mem::size_of::<std::collections::HashMap<usize, f64>>(),
            std::mem::size_of::<Vec<usize>>());
        let meshes = std::mem::take(&mut s.objects.meshes);
        let mut owned: Vec<Mesh> = meshes.into_iter().filter_map(|m| std::rc::Rc::try_unwrap(m).ok()).collect();
        println!("  unwrapped {} meshes", owned.len());
        let mut p2 = live();
        for m in owned.iter_mut() { m.vertex = Default::default(); }
        step("  mesh.vertex", live(), &mut p2);
        for m in owned.iter_mut() { m.face = Default::default(); }
        step("  mesh.face", live(), &mut p2);
        for m in owned.iter_mut() { m.triangulation = Default::default(); }
        step("  mesh.triangulation", live(), &mut p2);
        for m in owned.iter_mut() { m.facedata = Default::default(); m.edgedata = Default::default(); m.face_holes = Default::default(); }
        step("  mesh.facedata/edge/holes", live(), &mut p2);
        for m in owned.iter_mut() { m.clear_pointcolors(); m.clear_facecolors(); m.clear_linecolors(); }
        step("  mesh colors+widths", live(), &mut p2);
        drop(owned);
        step("  mesh rest", live(), &mut p2);
    }
    step("meshes TOTAL", live(), &mut prev);
    s.objects.pointclouds = Vec::new();          step("pointclouds", live(), &mut prev);
    drop(s);                                     step("the rest", live(), &mut prev);
    println!("  residual {:.1} MB", live() - base);
}
