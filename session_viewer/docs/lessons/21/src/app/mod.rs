// `pub mod x;` makes src/app/x.rs part of the crate; each lesson adds the one line of the module it teaches.
// `#[cfg(target_arch = "wasm32")]` above a line compiles that module for the browser only.
pub mod clipping; // register:clipping
pub mod cloud_query; // register:cloud_query
pub mod cplane; // register:cplane
#[cfg(any(target_arch = "wasm32", test))] // register:decode
pub mod decode; // register:decode
pub mod deform; // register:deform
pub mod edit; // register:edit
pub mod feedback; // register:feedback
#[cfg(target_arch = "wasm32")] // register:fetch
pub mod fetch; // register:fetch
pub mod fonts; // register:fonts
pub mod gesture; // register:gesture
pub mod gizmo; // register:gizmo
pub mod input; // register:input
#[cfg(any(target_arch = "wasm32", test))] // register:inspection
pub mod inspection; // register:inspection
pub mod keys; // register:keys
pub mod knobs; // register:knobs
pub mod layers; // register:layers
#[cfg(target_arch = "wasm32")] // register:live
pub mod live; // register:live
#[cfg(target_arch = "wasm32")] // register:loader
pub mod loader; // register:loader
pub mod manifest; // register:manifest
pub mod mesh_preview; // register:mesh_preview
#[cfg(any(target_arch = "wasm32", test))] // register:range_gate
pub mod range_gate; // register:range_gate
#[cfg(target_arch = "wasm32")] // register:route
pub mod route; // register:route
pub mod scene; // register:scene
pub mod selection; // register:selection
pub mod sheet_query; // register:sheet_query
pub mod snap; // register:snap
pub mod stream; // register:stream
pub mod surface_preview; // register:surface_preview
pub mod touch; // register:touch
pub mod validate; // register:validate
pub mod walk; // register:walk
