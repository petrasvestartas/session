pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let project_layout=layout("finite original projection",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,Type::Uniform),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)]);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let project=compute(device,&crate::finite_project_source::source(),"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);
  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,include_str!("finite_scan_triangle_tiles.wgsl"),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(include_str!("finite_triangle_tiles.wgsl").into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,raster_layout,scan_layout,project,count,fill,scans}
 }
}
