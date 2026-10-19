#[test]
fn actual_sample_coordinates_are_measured_without_a_fixed_pattern() {
    let instance=wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends:wgpu::Backends::VULKAN,flags:Default::default(),
        memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None,
    });
    let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device,queue)=pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e|panic!("sample-coordinate probe: {e}")));
    let image=device.create_texture(&wgpu::TextureDescriptor {
        label:Some("measured sample positions"),size:wgpu::Extent3d {width:8,height:8,depth_or_array_layers:1},
        mip_level_count:1,sample_count:4,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba16Float,
        usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,view_formats:&[],
    });
    let view=image.create_view(&Default::default());
    let module=device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label:Some("sample interpolation versus builtin position"),source:wgpu::ShaderSource::Wgsl(r#"
struct Vertex { @builtin(position) position:vec4f, @location(0) @interpolate(linear,sample) pixel:vec2f }
@vertex fn vertex(@builtin(vertex_index) index:u32)->Vertex {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3))[index];
 return Vertex(vec4f(p,0.5,1), (p*vec2f(0.5,-0.5)+0.5)*8.);
}
@fragment fn fragment(input:Vertex)->@location(0) vec4f {return vec4f(input.position.xy,input.pixel);}
"#.into()),
    });
    let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label:Some("actual sample-coordinate output"),layout:None,
        vertex:wgpu::VertexState {module:&module,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},
        fragment:Some(wgpu::FragmentState {module:&module,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format:wgpu::TextureFormat::Rgba16Float,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),
        primitive:Default::default(),depth_stencil:None,multisample:wgpu::MultisampleState {count:4,..Default::default()},multiview_mask:None,cache:None,
    });
    let storage=device.create_buffer(&wgpu::BufferDescriptor {label:None,size:64,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
    let result=device.create_buffer(&wgpu::BufferDescriptor {label:None,size:64,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
    let module=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("every actual sample coordinate"),source:wgpu::ShaderSource::Wgsl(r#"
@group(0) @binding(0) var pixels:texture_multisampled_2d<f32>;
@group(0) @binding(1) var<storage,read_write> values:array<vec4f>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 values[id.x]=textureLoad(pixels,vec2i(2),i32(id.x));
}
"#.into())});
    let compute=device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&module,entry_point:Some("read_samples"),compilation_options:Default::default(),cache:None});
    let group=device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&compute.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&view)},wgpu::BindGroupEntry {binding:1,resource:storage.as_entire_binding()}]});
    let mut encoder=device.create_command_encoder(&Default::default());
    {
        let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label:Some("actual sample positions"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {
                view:&view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
            })],..Default::default()
        });
        pass.set_pipeline(&pipeline);pass.draw(0..3,0..1);
    }
    {
        let mut pass=encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&compute);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups(1,1,1);
    }
    encoder.copy_buffer_to_buffer(&storage,0,&result,0,64);queue.submit([encoder.finish()]);
    let (tx,rx)=std::sync::mpsc::channel();result.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());
    device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();
    let bytes=result.slice(..).get_mapped_range();let values=bytes.chunks_exact(16).map(|b|std::array::from_fn::<_,4,_>(|i|f32::from_ne_bytes(b[i*4..i*4+4].try_into().unwrap()))).collect::<Vec<_>>();drop(bytes);result.unmap();
    assert_eq!(values.len(),4);assert!(values.iter().flatten().all(|v|v.is_finite()&&*v>=2.&&*v<3.));
    for i in 0..4 {for j in 0..i {assert_ne!(&values[i][2..],&values[j][2..],"Actual sample-qualified coordinates must distinguish the four sample invocations");}}
    let report=serde_json::json!({"status":"measured actual Native sample coordinates; no viewer4x coverage claim","values":values,"builtinMatchesSample":values.iter().all(|p|(p[0]-p[2]).abs()<0.002&&(p[1]-p[3]).abs()<0.002),"builtinIsCentre":values.iter().all(|p|p[0]==2.5&&p[1]==2.5),"adapter":format!("{:?}",adapter.get_info())});
    crate::sample_test_proof::write("native-sample-coordinates.json",serde_json::to_string_pretty(&report).unwrap()).unwrap();
}
