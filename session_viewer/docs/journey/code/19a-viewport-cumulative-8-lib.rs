pub mod background;
pub mod camera;
pub mod mesh;
pub mod scene;
pub mod picking;
pub mod history;
pub mod editor;
pub mod viewport;
pub mod gesture;
pub mod navigation;
#[cfg(test)]
mod navigation_tests;
#[cfg(test)]
mod gesture_tests;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;


#[cfg(any(test, target_arch = "wasm32"))]
mod command_dock;

#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
mod tool_buttons_tests;
#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;

#[cfg(target_arch = "wasm32")]
mod panel;
