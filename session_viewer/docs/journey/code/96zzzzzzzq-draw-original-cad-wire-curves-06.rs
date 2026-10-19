impl PreparedChain {pub fn segments(&self)->Vec<Segment>{segments(&self.points,self.colour,self.width,self.heads)}}
pub fn segments(points:&[[f32;3]],colour:[f32;4],width:f32,heads:u32)->Vec<Segment>{
        