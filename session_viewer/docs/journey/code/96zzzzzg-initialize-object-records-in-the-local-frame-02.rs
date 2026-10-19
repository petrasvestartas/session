        Self::with_geometry_at(device,layout,object,style,geometry,[0.;3])
    }
    pub fn with_geometry_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,style:Style,geometry:Rc<GpuGeometry>,anchor:[f64;3])->Self {
        let bytes=settings(object,style,0,0,0,geometry.index_start,anchor);