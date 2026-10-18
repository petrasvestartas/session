nt_contact_tests.rs"]
mod ambient_contact_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="frame_target_tests.rs"]
mod frame_target_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="sample_surface_tests.rs"]
mod sample_tests;
