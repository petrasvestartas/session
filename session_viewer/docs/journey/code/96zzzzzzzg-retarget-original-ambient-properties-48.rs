ple,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite}
 }
}

pub(crate) fn shader_source(samples:u32)->String {
 let source=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);if samples==1 {return source;}
 source.replace("const MSAA: bool = false;","const MSAA: bool = true;")
 .replace("var depth: texture_depth_2d;","var depth: texture_depth_multisampled_2d;")
 .replace("var source_radii:texture_2d<f32>;","var source_radii:texture_multisampled_2d<f32>;")
 .replace("var source_normals:texture_2d<f32>;","var source_normals:texture_multisampled_2d<f32>;")
 .replace("var source_primitives:texture_2d<u32>;","var source_primitives:texture_multisampled_2d<u32>;")
}
