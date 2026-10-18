        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{0.},0.];
        let bytes=crate::surface_view::bytes(*transform,display);