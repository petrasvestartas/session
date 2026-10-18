  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2]){
  if ground[1]<=0.0{self.clear(device,queue);return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }

}
