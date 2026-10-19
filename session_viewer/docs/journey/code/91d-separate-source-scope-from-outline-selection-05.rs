      return;
    }
    self.prepare(ctx, size, density,false);
    let key = Key { transform, size, density: density.to_bits(), revision: self.revision,clipping:clip_revision,all:false };
    let m = self.masks.as_ref().unwrap();
    if self.key.as_ref() != Some(&key) {
