    }),
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
}

#[cfg(test)]
impl Outline {
  pub(crate) fn alpha_texture(&self) -> Option<&wgpu::Texture> {
    self.masks.as_ref().map(|m| &m.alpha.texture)
  }
}
