use super::*;
pub(crate) fn pixels(lane:&Lane,ctx:&GpuCtx,samples:u32,reference:bool,edl:f32)->Vec<u8> {
    ctx.queue.write_buffer(&lane.settings,12,&edl.to_ne_bytes());
    let size=[320,240];let format=wgpu::TextureFormat::Rgba8Unorm;
    let target=crate::frame_targets::Targets::new(&ctx.device,size,format,samples);
    let output=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("independent cloud composite"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});
    let view=output.create_view(&Default::default());
    let reference_pipeline=reference.then(||{
        let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("exact maintained cloud resolve witness"),source:wgpu::ShaderSource::Wgsl(PRIMARY.into())});
        let uniform=crate::gpu_layout::uniform(&ctx.device,48);
        let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&uniform),Some(&lane.image_layout)],immediate_size:0});
        ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("exact maintained cloud composite"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg16Uint,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{depth_compare:Some(crate::depth::CLOSER_EQUAL),..crate::depth::surface()}),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
    });
    let mut encoder=ctx.device.create_command_encoder(&Default::default());{
        let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("actual cached point images under two independent composites"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target.color(&view),resolve_target:None,depth_slice:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&target.primitive.view,resolve_target:None,depth_slice:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&target.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Clear(0.),store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
        if let Some(pipeline)=&reference_pipeline {pass.set_pipeline(pipeline);pass.set_bind_group(0,&lane.settings_group,&[]);pass.set_bind_group(1,&lane.targets.as_ref().unwrap().group,&[]);pass.draw(0..3,0..1);}else{lane.draw(&mut pass);}
    }
    target.resolve(&mut encoder,&view);
    let read=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:320*240*4,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
    encoder.copy_texture_to_buffer(output.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(1280),rows_per_image:Some(240)}},output.size());ctx.queue.submit([encoder.finish()]);
    let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());ctx.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(10))}).unwrap();rx.recv().unwrap().unwrap();let data=read.slice(..).get_mapped_range().to_vec();read.unmap();data
}

const PRIMARY:&str=r##"// Point cloud settings, 48 bytes; matches CloudUniform in Rust.
struct CloudUniform {
    size: f32, // point size scale; applied on the CPU
    vp_w: f32, // target width, px
    vp_h: f32, // target height, px
    edl: f32, // eye-dome lighting strength; 0 = off
    _pad0: f32, // padding
    _pad1: f32, // padding
    origin: vec2<f32>, // top-left of this target in the canvas, px
    frame: vec2<f32>, // canvas size, px
};

@group(0) @binding(0) var<uniform> cloud: CloudUniform; // cloud settings
@group(1) @binding(0) var sdepth: texture_depth_2d; // nearest point depth per pixel
@group(1) @binding(1) var scolor: texture_2d<f32>; // its color

// One vertex of the fullscreen triangle.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
};

@vertex
// Place the three corners of a screen-covering triangle.
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    let x = f32(i32(vid & 1u) * 4 - 1);
    let y = f32(i32(vid >> 1u) * 4 - 1);
    o.pos = vec4<f32>(x, y, 0.0, 1.0);
    return o;
}

// Output: color, triangle id, and the depth written to the scene.
struct FsOut {
    @location(0) color: vec4<f32>, // rgba
    @location(1) primitive: vec2<u32>, // triangle id; none for points
    @builtin(frag_depth) depth: f32, // point depth into the scene
};

// Depth on a log scale that grows with distance.
fn log_depth(d: f32) -> f32 {
    return -log2(max(d, 1.0e-7));
}

// Copy the point texture into the scene, darkening edges when EDL is on.
fn shade(in: VsOut) -> FsOut {
    let pix = vec2<i32>(in.pos.xy);
    let d = textureLoad(sdepth, pix, 0);

    // no point here
    if (d == 0.0) {
        discard;
    }

    var o: FsOut;
    var rgb = textureLoad(scolor, pix, 0).rgb;

    // eye-dome lighting: darken where neighbours are nearer
    if (cloud.edl > 0.0) {
        let w = i32(cloud.vp_w);
        let h = i32(cloud.vp_h);
        let me = log_depth(d);
        var sum = 0.0;
        var taps = array<vec2<i32>, 4>(vec2<i32>(-1, 0), vec2<i32>(1, 0), vec2<i32>(0, -1), vec2<i32>(0, 1));

        for (var k = 0; k < 4; k++) {
            let q = pix + taps[k];

            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) {
                continue;
            }

            let nd = textureLoad(sdepth, q, 0);

            if (nd == 0.0) {
                continue;
            }

            sum += max(0.0, me - log_depth(nd));
        }

        // darken by the depth steps, never below a quarter
        let shade = max(exp(-sum * 75.0 * cloud.edl), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
    o.depth = d;
    o.primitive = vec2<u32>(0u);
    return o;
}

@fragment
// Resolve one pixel.
fn fs_main(in: VsOut) -> FsOut {
    return shade(in);
}
"##;
