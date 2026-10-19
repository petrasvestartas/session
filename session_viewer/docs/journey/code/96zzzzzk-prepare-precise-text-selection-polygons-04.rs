pub fn polygon(row:&Row,matrix:&[f64;16],size:[f64;2])->Option<Vec<[f64;2]>> {polygon_in(row,&Screen{matrix:*matrix,size,origin:[0.;3]})}
pub fn polygon_in(row:&Row,view:&Screen)->Option<Vec<[f64;2]>> {
 let row=Row{model:crate::render_space::placed(&row.model,view.origin),..row.clone()};
 let local=Screen{matrix:view.matrix,size:view.size,origin:[0.;3]};let matrix=&local.matrix;let size=local.size;