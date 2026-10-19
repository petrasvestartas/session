      &self.radius,
      0,
      &[radius, if all{0.}else{1.}, block as f32, 0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>(),
    );
    if self.tap_radius != Some(radius) {
