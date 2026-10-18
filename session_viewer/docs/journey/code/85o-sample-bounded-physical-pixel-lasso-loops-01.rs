use crate::{scene::{ObjectId,Scene},camera::Camera};
const SPACING:f64=2.;
pub const MAX_POINTS:usize=4096;
#[derive(Default)]
pub struct Lasso{pub points:Vec<(f64,f64)>,pub spacing:f64,pub pointer:Option<i32>,pub size:[f64;2],pub density:f64}
impl Lasso{
    pub fn follow(&mut self, at: (f64, f64)) -> bool {
        let Some(last) = self.points.last() else {
            return false;
        };

        if (at.0 - last.0).hypot(at.1 - last.1) < self.spacing.max(SPACING) {
            return false;
        }

        // full: thin the loop evenly instead of cutting its end off
        if self.points.len() >= MAX_POINTS {
            let mut index = 0;
            self.points.retain(|_| {
                index += 1;
                index % 2 == 1
            });
            self.spacing = 2.0 * self.spacing.max(SPACING);
        }

        self.points.push(at);
        true
    }
    pub fn found(&self,scene:&Scene,camera:&Camera)->Result<Vec<ObjectId>,&'static str>{
        if self.points.len()<3{return Err("Draw a loop first")}
        let region=crate::selection_region::Region::new(&self.points);let view=camera.view_projection();
        let inside=|p:[f64;3]|crate::rect_select::screen(&view.m,p,self.size.map(|v|v*self.density)).is_some_and(|p|region.contains((p[0],p[1])));
        let mut ids=scene.all_ids().into_iter().filter(|id|scene.pick_id(id.value())==Some(*id)).filter(|id|{
            if let Some(source)=crate::source_edit::original(scene,*id){let model=scene.placement(*id).unwrap();if let Some(yes)=crate::selection_region::enclosed(&source,&|p|{let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));inside([p[0],p[1],p[2]])}){return yes}}
            scene.selected_bounds(*id).is_some_and(|bounds|(0..8).all(|mask|inside(std::array::from_fn(|i|if mask&(1<<i)==0{bounds.min[i]}else{bounds.max[i]}))))
        }).collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());if ids.is_empty(){return Err("nothing visible lies entirely inside the lasso; the selection is unchanged")}Ok(ids)
    }
    pub fn abandoned(&mut self){self.points.clear();self.pointer=None;}
}
