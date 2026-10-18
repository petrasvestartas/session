pub fn matches(row:&Row,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->bool {matches_in(row,&Screen{matrix:*matrix,size,origin:[0.;3]},rectangle)}
pub fn matches_in(row:&Row,view:&Screen,rectangle:Rectangle)->bool {
 let Some(points)=polygon_in(row,view)