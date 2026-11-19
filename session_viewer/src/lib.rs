/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    ($name:literal) => {
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

mod engine;

mod camera;
pub mod command_dock;

pub mod app;

#[cfg(target_arch = "wasm32")]
pub mod text_quality;

mod state;

pub use state::State;

pub use app::runtime::{CloudChunk, Msg, SheetChunk};
#[cfg(target_arch = "wasm32")]
pub use app::runtime::{App, run_web};
#[cfg(target_arch = "wasm32")]
pub(crate) use app::runtime::desired_canvas_size;

#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;

#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]
#[path = "../examples/course/entry.rs"]
mod course;
