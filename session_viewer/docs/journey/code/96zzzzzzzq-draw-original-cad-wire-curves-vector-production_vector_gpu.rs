use std::cell::RefCell;
use crate::production_ribbon_gpu::{buffer,group,storage};
pub const SOURCE:&str=include_str!("production_vector.wgsl");
pub(crate) struct Heads {
    layouts:[wgpu::BindGroupLayout;4],shader:RefCell<Option<wgpu::ShaderModule>>,
    color:RefCell<Option<crate::sample_pipelines::Variants<wgpu::RenderPipeline>>>,pick:RefCell<Option<wgpu::RenderPipeline>>,
    group:wgpu::BindGroup,bytes:Vec<u8>,owners:Vec<u32>,count:u32,format:wgpu::TextureFormat,
}
impl Heads {
    pub fn new(device:&wgpu::Device,shared:&[wgpu::BindGroupLayout;4],format:wgpu::TextureFormat)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("production vector rows and owner ABI"),entries:&[storage(0,wgpu::ShaderStages::VERTEX_FRAGMENT),storage(1,wgpu::ShaderStages::FRAGMENT)]});
        let layouts=[shared[0].clone(),shared[1].clone(),shared[2].clone(),layout];
        let rows=buffer(device,&[],48,wgpu::BufferUsages::STORAGE);let owners=buffer(device,&[],4,wgpu::BufferUsages::STORAGE);
        let group=group(device,&layouts[3],vec![(0,rows.as_entire_binding()),(1,owners.as_entire_binding())]);
        Self{layouts,shader:Default::default(),color:Default::default(),pick:Default::default(),group,bytes:Vec::new(),owners:Vec::new(),count:0,format}
    }
    pub fn set(&mut self,device:&wgpu::Device,bytes:&[u8],owners:&[u32]) {
        if self.bytes==bytes&&self.owners==owners{return;}
        let rows=buffer(device,bytes,48,wgpu::BufferUsages::STORAGE);let owner=buffer(device,&owners.iter().flat_map(|id|id.to_ne_bytes()).collect::<Vec<_>>(),4,wgpu::BufferUsages::STORAGE);
        self.group=group(device,&self.layouts[3],vec![(0,rows.as_entire_binding()),(1,owner.as_entire_binding())]);self.bytes=bytes.to_vec();self.owners=owners.to_vec();self.count=(bytes.len()/48)as u32;
    }
    fn shader(&self,device:&wgpu::Device)->wgpu::ShaderModule {
        self.shader.borrow_mut().get_or_insert_with(||device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("actual production vector"),source:wgpu::ShaderSource::Wgsl(SOURCE.into())})).clone()
    }
    pub fn prepare(&self,device:&wgpu::Device,samples:u32,picking:bool) {
        if self.count==0{return;}let shader=self.shader(device);
        if picking {self.pick.borrow_mut().get_or_insert_with(||pipeline(device,&shader,&self.layouts,wgpu::TextureFormat::Rg32Uint,1,true));}
        else {let mut color=self.color.borrow_mut();if let Some(color)=color.as_mut(){color.select(samples,||pipeline(device,&shader,&self.layouts,self.format,samples,false));}else{*color=Some(crate::sample_pipelines::Variants::new(pipeline(device,&shader,&self.layouts,self.format,samples,false),samples));}}
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,shared:&[wgpu::BindGroup;4],picking:bool) {
        if self.count==0{return;}
        let color=self.color.borrow();let pick=self.pick.borrow();let pipeline=if picking{pick.as_ref()}else{color.as_ref().map(|p|&p.active)};
        if let Some(pipeline)=pipeline {pass.set_pipeline(pipeline);for(i,group)in shared[..3].iter().enumerate(){pass.set_bind_group(i as u32,group,&[]);}pass.set_bind_group(3,&self.group,&[]);pass.draw(0..18,0..self.count);}
    }
}
fn pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layouts: &[wgpu::BindGroupLayout; 4],
    format: wgpu::TextureFormat,
    samples: u32,
    picking: bool,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("production vector"),
        bind_group_layouts: &layouts.iter().map(Some).collect::<Vec<_>>(),
        immediate_size: 0,
    });
    let constants = [("SCENE_MSAA", f64::from(samples > 1))];
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("production vector"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(if picking { "fs_id" } else { "fs_main" }),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: (!picking).then_some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::Always),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}

