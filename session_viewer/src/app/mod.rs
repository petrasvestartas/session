pub mod clipping; // register:clipping
pub mod cloud_query; // register:cloud_query
pub mod command; // register:command
pub mod coords; // register:coords
pub mod cplane; // register:cplane
pub mod deform; // register:deform
pub mod edit; // register:edit
pub mod feedback; // register:feedback
pub mod fonts; // register:fonts
pub mod gesture; // register:gesture
pub mod gizmo; // register:gizmo
pub mod hierarchy; // register:hierarchy
pub mod input; // register:input
pub mod keys; // register:keys
pub mod knobs; // register:knobs
pub mod layers; // register:layers
pub mod manifest; // register:manifest
pub mod mesh_preview; // register:mesh_preview
pub mod modeling; // register:modeling
pub mod scene; // register:scene
pub mod selection; // register:selection
pub mod session_io; // register:session_io
pub mod sheet_query; // register:sheet_query
pub mod snap; // register:snap
pub mod stream; // register:stream
pub mod touch; // register:touch
pub mod validate; // register:validate
pub mod walk; // register:walk

#[cfg(target_arch = "wasm32")]
pub mod agent; // register:agent
#[cfg(any(target_arch = "wasm32", test))]
pub mod decode; // register:decode
#[cfg(target_arch = "wasm32")]
pub mod fetch; // register:fetch
#[cfg(target_arch = "wasm32")]
pub mod live; // register:live
#[cfg(target_arch = "wasm32")]
pub mod loader; // register:loader
#[cfg(target_arch = "wasm32")]
pub mod route; // register:route

#[cfg(any(target_arch = "wasm32", test))]
pub mod inspection; // register:inspection
#[cfg(any(target_arch = "wasm32", test))]
pub mod range_gate; // register:range_gate

#[cfg(any(target_arch = "wasm32", test))]
pub mod ui; // register:ui

pub mod surface_preview; // register:surface_preview

pub mod splitting; // register:splitting
