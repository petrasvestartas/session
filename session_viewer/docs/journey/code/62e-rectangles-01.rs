use crate::{scene_label::Row,text::TextPlacement,rect_select::{screen,Rectangle}};

pub fn polygon(row:&Row,matrix:&[f64;16],size:[f64;2])->Option<Vec<[f64;2]>> {
 let label=row.label(false,true).ok()?;let [width,height]=row.prepared.extent;
 if let TextPlacement::WorldPlane {world,right,up,world_height}=label.placement {
  let unit=world_height/label.font_size as f64;let vertical=label.font_size as f64*2./9.;let horizontal=height*0.5+vertical;
  return [[-horizontal,-vertical],[-horizontal,height+vertical],[width+horizontal,height+vertical],[width+horizontal,-vertical]].into_iter().map(|[x,y]|screen(matrix,std::array::from_fn(|i|world[i]+(right[i]*x-up[i]*y)*unit),size)).collect();
 }
 let (world,offset,scale,center,padding)=match label.placement {
  TextPlacement::Anchor {world,offset}=>(world,offset,1.,false,[height*0.5+label.font_size as f64*2./9.,label.font_size as f64*2./9.]),
  TextPlacement::Nameplate {world,padding,rounded}=>(world,[0.;2],1.,true,[if rounded {(padding[0] as f64).max(height*0.5+padding[1] as f64)} else {padding[0] as f64},padding[1] as f64]),
  TextPlacement::WorldBillboard {world,world_height}=>{
   let w=(0..3).map(|i|matrix[i*4+3]*world[i]).sum::<f64>()+matrix[15];let projection=[matrix[1],matrix[5],matrix[9]].iter().map(|n|n*n).sum::<f64>().sqrt();let scale=world_height*projection*size[1]/(2.*w*label.font_size as f64);if !scale.is_finite()||scale<=0. {return None;}(world,[0.;2],scale,false,[height*0.5+label.font_size as f64*2./9.,label.font_size as f64*2./9.])
  },_=>return None,
 };
 let [mut left,mut top]=screen(matrix,world,size)?;left+=offset[0] as f64;top+=offset[1] as f64;if center {left-=width*scale*0.5;top-=height*scale*0.5;}
 let bounds=[left-padding[0]*scale,top-padding[1]*scale,left+(width+padding[0])*scale,top+(height+padding[1])*scale];Some(vec![[bounds[0],bounds[1]],[bounds[0],bounds[3]],[bounds[2],bounds[3]],[bounds[2],bounds[1]]])
}
fn inside(p:[f64;2],points:&[[f64;2]])->bool {let mut sign=0.;for (a,b) in points.iter().zip(points.iter().cycle().skip(1)).take(points.len()) {let cross=(b[0]-a[0])*(p[1]-a[1])-(b[1]-a[1])*(p[0]-a[0]);if cross==0. {continue;}if sign==0. {sign=cross.signum();} else if sign!=cross.signum() {return false;}}sign!=0.}
pub fn matches(row:&Row,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->bool {
 let Some(points)=polygon(row,matrix,size) else {return false;};if !rectangle.crossing {return points.iter().all(|p|rectangle.contains(*p));}
 points.iter().any(|p|rectangle.contains(*p))||points.iter().zip(points.iter().cycle().skip(1)).take(points.len()).any(|(a,b)|rectangle.crosses(*a,*b))||inside([rectangle.bounds[0],rectangle.bounds[1]],&points)
}
