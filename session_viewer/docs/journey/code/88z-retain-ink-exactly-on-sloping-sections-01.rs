    return ((plane * size.y + at.y) * size.x + at.x) * 4u;
}

// Centerline projection introduces float rounding; retain ink exactly on a section without moving its source.
fn clip_cut_ink_ndc(flags:u32,ndc:vec3<f32>)->bool {
 if((flags&FLAG_CLIPPING_PLANE)!=0u){return false;}
 for(var i=0u;i<clipping.count;i++){
  let plane=clipping.screen[i];let point=vec4<f32>(ndc,1.0);
  let rounding=9.5367431640625e-7*dot(abs(plane),abs(point));
  if(dot(plane,point)<-rounding){return true;}
 }
 return false;
}
