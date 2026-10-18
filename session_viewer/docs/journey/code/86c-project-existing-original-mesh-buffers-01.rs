struct View { transform:mat4x4<f32>, size:vec2<f32>, density:f32 }
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<f32>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
fn physical_clip_corner(index:u32)->vec4<f32>{
 let base=physical_indices[index]*6u;
 let point=vec3<f32>(physical_vertices[base],physical_vertices[base+1u],physical_vertices[base+2u]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}
// Project one triangle id per invocation into `projected`: an arena triangle, or one instance's
// copy of its definition's.
