/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    ($name:literal) => {
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

mod engine;

pub use viewer_core as core;
pub use viewer_core::op;
pub use viewer_core::rows;

pub use viewer_core::camera;
pub mod command_dock;

pub mod app;

#[cfg(target_arch = "wasm32")]
pub mod text_quality;

mod state;

pub use state::State;

#[cfg(target_arch = "wasm32")]
pub(crate) use app::runtime::desired_canvas_size;
#[cfg(target_arch = "wasm32")]
pub use app::runtime::{App, execute_op, run_web};
pub use app::runtime::{CloudChunk, Msg, SheetChunk};

#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;

#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]
#[path = "../examples/course/entry.rs"]
mod course;
