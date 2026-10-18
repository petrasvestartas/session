    (pipeline, pick)
}

pub fn colour(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
// original view/model
// original clipping layout
// original expanded source
 let expanded=if samples==4 {expanded.replace("@interpolate(linear)","@interpolate(linear,sample)").replace("@location(8) original_clip:","@location(8) @interpolate(perspective,sample) original_clip:")}else{expanded};
// original source reference
// original shader
// original colour layout
// original render
 pipeline
}
