_lod::Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
  crate::cloud_lod::radius_factor(range, px, scale, ortho_h) * if ortho_h > 0. { 1. } else { 1000. }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="cloud_composite_reference.rs"]
pub(crate) mod composite_reference;
