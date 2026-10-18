    clipping:&wgpu::BindGroup,clip_revision:u64,
  ) {
    self.encode_masks(ctx,encoder,depth,view_group,meshes,transform,size,density,clipping,clip_revision,false,None);
    if let Some(m)=self.masks.as_ref(){
      self.composite(encoder,colour,&m.composite);
    }
  }
  pub fn encode_masks(
    &mut self,
    ctx: &GpuCtx,
    encoder: &mut wgpu::CommandEncoder,
    depth: &wgpu::TextureView,
    view_group: &wgpu::BindGroup,
    meshes: &[GpuMesh],
    transform: [f32; 16],
    size: [u32; 2],
    density: f32,
    clipping:&wgpu::BindGroup,clip_revision:u64,
    all:bool,sections:Option<&wgpu::TextureView>,
  ) {
    if !meshes.iter().any(|m|all||m.selected()) {
      self.release();
      return;
    }
    self.prepare(ctx, size, density,all);
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius);}
    let key = Key { transform, size, density: density.to_bits(), revision: self.revision,clipping:clip_revision,all };
    let m = self.masks.as_ref().unwrap();
    if self.key.as_ref() != Some(&key) {
