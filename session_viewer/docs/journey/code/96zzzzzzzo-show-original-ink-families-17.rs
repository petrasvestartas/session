#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_visibility_gpu_tests.rs"]
mod visibility_tests;

pub struct Renderer {