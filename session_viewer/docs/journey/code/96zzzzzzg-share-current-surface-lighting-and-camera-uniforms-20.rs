mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;