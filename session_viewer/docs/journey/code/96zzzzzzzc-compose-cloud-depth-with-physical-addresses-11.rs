u,@builtin(frag_depth) depth:f32}
fn log_depth(d:f32)->f32{return -log2(max(d,1.0e-7));}
@group(2) @binding(0) var<uniform> cloud:vec4f;
@fragmen