    }
    self.prepare(ctx, size, density);
    let key = Key { transform, size, density: density.to_bits(), revision: self.revision,clipping:clip_revision };
    let m = self.masks.as_ref().unwrap();
    if self.key.as_ref() != Some(&key) {
