#[cfg(target_arch = "wasm32")] // register:agent
pub mod agent; // register:agent
pub use viewer_core::clipping; // register:clipping
pub mod cloud_query; // register:cloud_query
pub use viewer_core::command; // register:command
pub use viewer_core::coords; // register:coords
pub use viewer_core::cplane; // register:cplane
#[cfg(any(target_arch = "wasm32", test))] // register:decode
pub mod decode; // register:decode
pub use viewer_core::deform; // register:deform
pub use viewer_core::edit; // register:edit
pub mod feedback; // register:feedback
#[cfg(target_arch = "wasm32")] // register:fetch
pub mod fetch; // register:fetch
pub mod fonts; // register:fonts
pub mod gesture; // register:gesture
pub use viewer_core::gizmo; // register:gizmo
pub use viewer_core::hierarchy; // register:hierarchy
pub mod input; // register:input
#[cfg(any(target_arch = "wasm32", test))] // register:inspection
pub mod inspection; // register:inspection
pub use viewer_core::interactions; // register:interactions
pub mod keys; // register:keys
pub use viewer_core::knobs; // register:knobs
#[cfg(target_arch = "wasm32")]
pub mod launch;
pub use viewer_core::layers; // register:layers
#[cfg(target_arch = "wasm32")] // register:live
pub mod live; // register:live
#[cfg(any(target_arch = "wasm32", test))]
pub use viewer_core::live_revision;
#[cfg(target_arch = "wasm32")] // register:loader
pub mod loader; // register:loader
pub use viewer_core::manifest; // register:manifest
pub use viewer_core::mesh_preview; // register:mesh_preview
pub use viewer_core::modeling; // register:modeling
pub mod mode; // register:mode
pub mod stage;
#[cfg(any(target_arch = "wasm32", test))] // register:range_gate
pub use viewer_core::range_gate; // register:range_gate
#[cfg(target_arch = "wasm32")] // register:route
pub mod route; // register:route
pub(crate) mod runtime;
pub use viewer_core::scene; // register:scene
pub use viewer_core::selection; // register:selection
pub mod session_io; // register:session_io
pub mod sheet_query; // register:sheet_query
pub use viewer_core::snap; // register:snap
pub use viewer_core::splitting; // register:splitting
pub mod stream; // register:stream
pub use viewer_core::surface_preview; // register:surface_preview
pub mod touch; // register:touch
#[cfg(any(target_arch = "wasm32", test))] // register:ui
pub mod ui; // register:ui
pub use viewer_core::validate; // register:validate
pub use viewer_core::walk; // register:walk

#[cfg(target_arch = "wasm32")]
pub mod yielding;
