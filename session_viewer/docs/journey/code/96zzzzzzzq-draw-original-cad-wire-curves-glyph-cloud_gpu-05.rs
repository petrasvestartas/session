      eye: view.eye.map(f32::to_bits),
      ortho: view.ortho_h.to_bits(),
      lod: view.lod_px.to_bits(),
      scale:view.scale.to_bits(),
      revision: self.revision,clipping:self.clip_revision,
    };
    if self.key.as_ref() == Some(&key) {
