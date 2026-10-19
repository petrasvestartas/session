        Self::upload_at(device,layout,object,selected,[0.;3])
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,pickable:!object.metadata.is_locked,colour:None},geometry,anchor)