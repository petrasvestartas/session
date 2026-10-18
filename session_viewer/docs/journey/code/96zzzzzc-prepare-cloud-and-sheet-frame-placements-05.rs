  pub fn camera(camera:&crate::camera::Camera)->Self {Self::anchored(camera,[0.;3])}
  pub fn anchored(camera:&crate::camera::Camera,anchor:[f64;3])->Self {