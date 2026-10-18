fn contact_layer(clip:vec4<f32>,world:vec3<f32>,normal:vec3<f32>,m:mat4x4<f32>,size:vec2<f32>,eye:vec4<f32>)->vec4<f32> {
  if(clip.w<=0.||dot(normal,normal)==0.) {return clip;}
  let n=normalize(normal);
  let u=normalize(cross(n,select(vec3<f32>(1.,0.,0.),vec3<f32>(0.,1.,0.),abs(n.x)>0.9)));
  let v=cross(n,u);let reach=select(length(eye.xyz-world),eye.w,eye.w>0.)*0.01;
  let a=m*vec4<f32>(world+u*reach,1.);let b=m*vec4<f32>(world+v*reach,1.);
  var slope=0.;
  if(a.w>0.&&b.w>0.) {
    let p=vec3<f32>(clip.xy/clip.w*size*0.5,clip.z/clip.w);
    let du=vec3<f32>(a.xy/a.w*size*0.5,a.z/a.w)-p;
    let dv=vec3<f32>(b.xy/b.w*size*0.5,b.z/b.w)-p;
    let det=du.x*dv.y-du.y*dv.x;
    if(abs(det)>=1e-12) {slope=max(abs((du.z*dv.y-dv.z*du.y)/det),abs((du.x*dv.z-dv.x*du.z)/det));}
  }
  let z=abs(clip.z/clip.w);let pull=min(4.7683716e-7*z+0.125*slope,1e-4*z);
  return vec4<f32>(clip.xy,clip.z+pull*clip.w,clip.w);
}
