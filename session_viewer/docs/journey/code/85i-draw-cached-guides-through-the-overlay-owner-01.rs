    pub fn labels(&self,existing:&[crate::text::TextLabel])->Vec<crate::text::TextLabel>{self.label.as_ref().map_or_else(Vec::new,|(point,label)|crate::measure_mark::Mark{points:vec![*point],label:label.clone()}.labels(existing))}
}
pub struct Guide{pub overlay:Overlay,segments:Lane,marks:Lane}
impl Guide{
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{overlay:Default::default(),segments:Lane::guides(device,format),marks:Lane::markers(device,format)}}
    pub fn set(&mut self,device:&wgpu::Device,overlay:&Overlay)->Result<bool,&'static str>{if &self.overlay==overlay{return Ok(false)}self.segments.set_guides(device,&overlay.segments)?;self.marks.set_points(device,overlay.marks.iter().copied())?;self.overlay=overlay.clone();Ok(true)}
    pub fn view(&self,queue:&wgpu::Queue,matrix:&[f32;16],size:[u32;2],density:f32){self.segments.view(queue,matrix,size,density);self.marks.view(queue,matrix,size,density)}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>){self.segments.draw(pass);self.marks.draw(pass)}
    pub fn bytes(&self)->u64{self.segments.usage()[1]+self.marks.usage()[1]}
}
