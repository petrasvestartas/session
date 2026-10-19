#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Visibility {pub edges:bool,pub lines:bool,pub points:bool,pub vertices:bool}
impl Default for Visibility {fn default()->Self{Self{edges:true,lines:true,points:true,vertices:true}}}
impl Visibility {
 pub fn mask(self)->u8 {
  let mut mask=CONTROLS;
  if self.edges {mask|=EDGES;if self.vertices{mask|=VERTICES;}}
  if self.lines {mask|=LINES;}if self.points {mask|=POINTS;}mask
 }
}

pub struct Runs {