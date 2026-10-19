fn near_distance(point:vec4<f32>)->f32{return point.w-point.z;}
fn near_endpoint(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{
 let distance=near_distance(a);
 if(distance>=0.){return a;}
 return mix(a,b,distance/(distance-near_distance(b)));
}
