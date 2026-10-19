}

pub fn sampled(source:String,samples:u32)->String {
 if samples==1 {return source;}
 source.replace("@location(0) colour:","@location(0) @interpolate(perspective,sample) colour:")
 .replace("@location(1) world:","@location(1) @interpolate(perspective,sample) world:")
 .replace("@location(2) normal:","@location(2) @interpolate(perspective,sample) normal:")
}
