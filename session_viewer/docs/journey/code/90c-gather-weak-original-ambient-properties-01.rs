#[derive(Default)]
pub struct Rows{bounds:crate::cap_source_bounds::Bounds,pub specs:Vec<(u32,f32)>,pub ground:[f32;2]}
impl Rows{
 pub fn set(&mut self,scene:&crate::scene::Scene){self.specs.clear();let mut floor=f64::INFINITY;let mut radius=0f32;for object in scene.objects().iter().filter(|r|scene.visible(r.id)&&!r.mesh.indices().is_empty()){if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let r=crate::ambient_radius::ambient_radius(&bounds);self.specs.push((object.id.value(),r));floor=floor.min(bounds.cz-bounds.hz);radius=radius.max(r);}}self.ground=[if floor.is_finite(){floor as f32}else{0.},radius];self.bounds.prune();if self.specs.is_empty(){self.specs.shrink_to_fit();}}
}
