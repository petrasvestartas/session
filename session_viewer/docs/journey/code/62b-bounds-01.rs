  crate::text::validate_label(&label).map_err(|_|"Text placement is invalid")?;Ok(label)
 }
 pub fn bounds(&self)->Option<crate::bounds::Bounds> {
  let label=self.label(false,true).ok()?;let (world,right,up,height)=match label.placement {
   TextPlacement::WorldPlane {world,right,up,world_height}=>(world,right,up,world_height),
   TextPlacement::WorldBillboard {world,world_height}=>(world,[1.,0.,0.],[0.,1.,0.],world_height),
   TextPlacement::Anchor {world,..}|TextPlacement::Nameplate {world,..}=>return Some(crate::bounds::Bounds::point(world)),TextPlacement::Screen {..}=>return None,
  };
  let unit=height/label.font_size as f64;let [width,line_height]=self.prepared.extent.map(|v|v*unit);let vertical=height*2./9.;let horizontal=line_height*0.5+vertical;let mut bounds=crate::bounds::Bounds::point(world);
  for x in [-horizontal,width+horizontal] {for y in [-vertical,line_height+vertical] {bounds.include(std::array::from_fn(|i|world[i]+right[i]*x-up[i]*y));}}Some(bounds)
 }
}
pub fn example(placement:TextPlacement)->Source {let mut label=crate::text_example::label();label.text="Original CAD · Ø25 · Ω".into();label.color=[255;4];label.placement=placement;Source {guid:uuid::Uuid::new_v4().to_string(),name:"Original CAD label".into(),visible:true,locked:false,label}}
