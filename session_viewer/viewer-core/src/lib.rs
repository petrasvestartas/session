//! UI- and GPU-independent core of Session Viewer.
//!
//! Zero dependencies on `wgpu`, `winit`, `egui`, `web-sys`, `wasm-bindgen`, or `glyphon`.

pub mod camera;
pub mod clipping;
pub mod cloud_query;
pub mod command;
pub mod coords;
pub mod cplane;
pub mod decode;
pub mod deform;
pub mod edit;
pub mod gizmo;
pub mod hierarchy;
pub mod interactions;
pub mod knobs;
pub mod layers;
pub mod live_revision;
pub mod manifest;
pub mod mesh_preview;
pub mod modeling;
pub mod op;
pub mod range_gate;
pub mod rows;
pub mod scene;
pub mod selection;
pub mod session_io;
pub mod sheet_query;
pub mod sink;
pub mod snap;
pub mod splitting;
pub mod stream;
pub mod surface_preview;
pub mod validate;
pub mod walk;

pub use crate as app;
pub use sink as core;
pub use sink::GpuSink;
