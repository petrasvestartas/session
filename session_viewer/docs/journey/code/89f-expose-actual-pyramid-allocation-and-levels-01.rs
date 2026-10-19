  self.cached=Some(key);self.error=None;
 }
 pub fn usage(&self)->[u64;4]{[self.frame.size()+self.images.as_ref().map_or(0,|i|i.linear.bytes+i.radii.bytes),self.preparations,self.writes,self.passes]}
 pub fn linear(&self)->Option<&Image>{self.images.as_ref().map(|i|&i.linear)}
 pub fn radii(&self)->Option<&Image>{self.images.as_ref().map(|i|&i.radii)}
}
