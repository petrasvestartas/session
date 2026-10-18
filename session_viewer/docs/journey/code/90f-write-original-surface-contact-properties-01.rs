struct View{transform:mat4x4<f32>,display:vec4<f32>}
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> object:Object;
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:vec3<f32>)->Vertex{let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(ambient_radius.x,encode_normal(normal)*0.5+0.5);
}
