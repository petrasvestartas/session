 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
  let (format,stride,grid,varying)=self.spec;
  if self.color.select(samples,|| {
   let layout=if samples==1 {self.query.clone()}else{layout(device,samples)};
   let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
   (pipeline(device,format,stride,grid,false,varying,samples,&view,&crop,&layout,attributes(stride)),layout)
  }) {self.bindings.get_mut().take();}
 }
 pub fn clear_bindings(&mut self){self.bindings.get_mut().take();}
 fn group(