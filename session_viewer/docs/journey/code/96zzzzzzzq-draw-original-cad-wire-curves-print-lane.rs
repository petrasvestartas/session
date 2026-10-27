use std::cell::RefCell;

pub struct Lane { color:RefCell<Option<crate::sample_pipelines::Variants<wgpu::RenderPipeline>>> }
impl Default for Lane {fn default()->Self {Self{color:RefCell::new(None)}}}
impl Lane {
    pub fn prepare(&self,device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32,meshes:&[crate::gpu_mesh::GpuMesh]) {
        if !meshes.iter().any(crate::gpu_mesh::GpuMesh::printed){return;}
        let mut color=self.color.borrow_mut();
        let create=||pipeline(device,format,samples,false);
        if let Some(color)=color.as_mut(){color.select(samples,create);}else{*color=Some(crate::sample_pipelines::Variants::new(create(),samples));}
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],text:bool) {
        let color=self.color.borrow();let Some(color)=color.as_ref()else{return;};
        pass.set_pipeline(&color.active);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);
        for mesh in meshes.iter().filter(|mesh|mesh.printed()&&mesh.geometry.source.print_text()==text){mesh.draw_pulled(pass);}
    }
}

pub(crate) fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32,picking:bool)->wgpu::RenderPipeline {
    let mut source=include_str!("print_fill.wgsl").to_owned();
    if picking {source.push_str(include_str!("pick_crop.wgsl"));source.push_str(r#"
@group(2) @binding(0) var<uniform> crop:Crop;
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->PrintOutput {
    var out=print_vertex(occurrence);out.position=crop.transform*out.position;return out;
}
struct PrintId {@location(0) owner:vec2u,@location(1) primitive:vec2u}
@fragment fn pick_fragment(input:PrintOutput)->PrintId {
    if(print_cut(input,vec2f(f32(crop._a),f32(crop._b)))){discard;}
    return PrintId(vec2u(select(0u,input.owner,crop.mode==0u),0u),vec2u(0u));
}
"#);}
    let source=crate::clipping_gpu::source(&source,2,u32::from(picking));
    let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("printed fill"),source:wgpu::ShaderSource::Wgsl(source.into())});
    let clipping=if picking{crate::pick_pipeline::crop_clipping_layout(device)}else{crate::clipping_gpu::layout(device)};
    let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("printed fill"),bind_group_layouts:&[Some(&crate::surface_view::layout(device)),Some(&crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)),Some(&clipping),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device))],immediate_size:0});
    let targets=if picking{vec![Some(wgpu::TextureFormat::Rg32Uint.into()),Some(crate::primitive_image::target(true))]}else{vec![Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})]};
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("printed fill"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&targets}),primitive:Default::default(),depth_stencil:Some(if picking{crate::depth::ink()}else{crate::depth::grid()}),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="print_fill_tests.rs"]
mod tests;
