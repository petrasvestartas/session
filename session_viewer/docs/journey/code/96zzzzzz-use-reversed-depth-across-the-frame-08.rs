struct Frame{inverse:mat4x4<f32>,full:vec2<f32>,size:vec2<f32>,radius:f32,pad:vec3<f32>}
@group(0) @binding(0) var depth:texture_depth_2d;
@group(0) @binding(1) var<uniform> frame:Frame;
@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);}
struct DepthRadius{@location(0) depth:f32,@location(1) radius:u32}
fn world(at:vec2<f32>,z:f32)->vec3<f32>{let uv=at/frame.full;let p=frame.inverse*vec4<f32>(uv.x*2.-1.,1.-uv.y*2.,z,1.);return p.xyz/p.w;}
@fragment fn fs_prepare(@builtin(position) pixel:vec4<f32>)->DepthRadius{
 let at=floor(pixel.xy/frame.size*frame.full)+0.5;let z=textureLoad(depth,vec2<i32>(at),0);if(z<=0.){return DepthRadius(0.,0u);}
 let origin=world(at,1.);let direction=normalize(world(at,0.)-origin);let p=world(at,z);let along=dot(p-origin,direction)/dot(direction,direction);
 let bits=(pack2x16float(vec2<f32>(frame.radius,0.))+2u)>>2u;let coord=vec2<u32>(pixel.xy);let encoded=(bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
 return DepthRadius(along,encoded);
}
