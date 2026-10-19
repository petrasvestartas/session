use wgpu::util::DeviceExt;

pub struct Widget {
    vertices:wgpu::Buffer, count:u32, uniform:wgpu::Buffer, group:wgpu::BindGroup,
    pipeline:wgpu::RenderPipeline, composite:wgpu::RenderPipeline,
    texture_layout:wgpu::BindGroupLayout, tile:Option<Tile>, visible:bool,
    pub allocations:u64, pub writes:u64,
}

impl Widget {
    pub fn new(device:&wgpu::Device, format:wgpu::TextureFormat) -> Self {
        let mesh=crate::widget_mesh::vertices();
        let bytes:Vec<u8>=mesh.iter().copied().flat_map(crate::widget_mesh::Vertex::bytes).collect();
        let vertices=device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label:Some("handle mesh"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX });
        let layout=crate::gpu_layout::uniform(device,96);
        let uniform=device.create_buffer(&wgpu::BufferDescriptor { label:Some("handle placement"),size:96,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false });
        let group=crate::gpu_layout::group(device,&layout,&uniform);
        let texture_layout=crate::widget_pipelines::texture_layout(device);
        let pipeline=crate::widget_pipelines::mesh(device,&layout);
        let composite=crate::widget_pipelines::composite(device,&layout,&texture_layout,format);
        Self {vertices,count:mesh.len() as u32,uniform,group,pipeline,composite,texture_layout,tile:None,visible:false,allocations:0,writes:0}
    }

    pub fn clear(&mut self) { self.tile=None; self.visible=false; }
    pub fn usage(&self) -> [u64;4] { [self.vertices.size()+self.uniform.size(),self.tile.as_ref().map_or(0,|t|u64::from(t.size[0])*u64::from(t.size[1])*36),self.allocations,self.writes] }
    pub fn extent(&self) -> Option<[u32;2]> { self.tile.as_ref().map(|t|t.size) }

    // A small independent depth buffer lets handles overlap correctly while remaining visible over the entire scene.
    pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,layout:Option<crate::widget_layout::Layout>) {
        self.visible=false;
        let Some(layout)=layout else { self.clear(); return; };
        if self.tile.as_ref().is_none_or(|tile| tile.size!=layout.extent) { self.tile=Some(Tile::new(device,&self.texture_layout,layout.extent)); self.allocations+=1; }
        let bytes:Vec<u8>=layout.uniform.into_iter().flat_map(f32::to_ne_bytes).collect();
        queue.write_buffer(&self.uniform,0,&bytes); self.writes+=1; self.visible=true;
    }

    pub fn draw(&self,encoder:&mut wgpu::CommandEncoder,view:&wgpu::TextureView) -> u32 {
        let Some(tile)=self.tile.as_ref().filter(|_|self.visible) else {return 0;};
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle tile"),
                color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&tile.color,resolve_target:Some(&tile.resolved),depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],
                depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&tile.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(crate::depth::FAR as f32),store:wgpu::StoreOp::Discard}),stencil_ops:None}),..Default::default()});
            pass.set_pipeline(&self.pipeline); pass.set_bind_group(0,&self.group,&[]); pass.set_vertex_buffer(0,self.vertices.slice(..)); pass.draw(0..self.count,0..1);
        }
        let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});
        pass.set_pipeline(&self.composite); pass.set_bind_group(0,&self.group,&[]); pass.set_bind_group(1,&tile.group,&[]); pass.draw(0..6,0..1); 2
    }
}

impl Drop for Widget { fn drop(&mut self) {self.vertices.destroy();self.uniform.destroy();} }

struct Tile { size:[u32;2],color:wgpu::TextureView,depth:wgpu::TextureView,resolved:wgpu::TextureView,group:wgpu::BindGroup }

impl Tile {
    fn new(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,size:[u32;2]) -> Self {
        let texture=|format,sample_count,usage| device.create_texture(&wgpu::TextureDescriptor {label:Some("handle tile"),size:wgpu::Extent3d {width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count,dimension:wgpu::TextureDimension::D2,format,usage,view_formats:&[]}).create_view(&Default::default());
        let color=texture(wgpu::TextureFormat::Rgba8Unorm,4,wgpu::TextureUsages::RENDER_ATTACHMENT);
        let depth=texture(wgpu::TextureFormat::Depth32Float,4,wgpu::TextureUsages::RENDER_ATTACHMENT);
        let resolved=texture(wgpu::TextureFormat::Rgba8Unorm,1,wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING);
        let sampler=device.create_sampler(&wgpu::SamplerDescriptor {mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});
        let group=device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some("handle tile"),layout,entries:&[
            wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&resolved)},
            wgpu::BindGroupEntry {binding:1,resource:wgpu::BindingResource::Sampler(&sampler)},
        ]}); Self {size,color,depth,resolved,group}
    }
}
