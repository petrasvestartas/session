//! EXACT live-heap bytes per OBJECT ROW of the viewer's own bookkeeping, via the counting
//! allocator of probe_mem.rs; the loading lives in `selftest::object_bytes`. The audit's ~1,034 B.
//!
//! cargo run --release --target x86_64-unknown-linux-gnu --example probe_objects -- assets/scenes/<scene>.toml
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static LIVE: AtomicUsize = AtomicUsize::new(0);

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        LIVE.fetch_add(l.size(), Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new: usize) -> *mut u8 {
        LIVE.fetch_add(new, Relaxed);
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.realloc(p, l, new) }
    }
}
#[global_allocator]
static A: Counting = Counting;

/// Live heap, MB.
fn live() -> f64 { LIVE.load(Relaxed) as f64 / 1.048576e6 }

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let files = session_viewer::selftest::SceneFile::from_args(&a);
    print!("{}", session_viewer::selftest::object_bytes(&files, live));
}
