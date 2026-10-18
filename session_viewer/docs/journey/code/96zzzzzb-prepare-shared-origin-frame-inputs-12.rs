impl Rows{
 pub fn ground_at(&self,anchor:[f64;3])->[f32;2] {[(self.ground[0]-anchor[2])as f32,self.ground[1]as f32]}