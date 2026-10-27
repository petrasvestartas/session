  pub eye: [f32; 3],
  pub ortho_h: f32,
  pub lod_px: f32,
  pub scale:f32,
}
impl Default for View {
  fn default() -> Self {
    Self { anchor:[0.;3],eye: [0., 0., 3.], ortho_h: 0., lod_px: 0.,scale:1. }
  }
}
impl View {
