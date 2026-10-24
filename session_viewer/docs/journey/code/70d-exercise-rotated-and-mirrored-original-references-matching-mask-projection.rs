struct View {transform:mat4x4<f32>,display:vec4<f32>}
struct Object {model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> object:Object;
@vertex fn vertex(@location(0) position:vec3<f32>)->@builtin(position) vec4<f32> {let world=object.model*vec4<f32>(position,1.0);return view.transform*world;}
@fragment fn fragment()->@location(0) vec4<f32> {return vec4<f32>(1.0);}
