        let primitive=crate::primitive_image::Image::new(device,[size.width,size.height]);
        Self { texture, colour, depth, primitive, section,readback, window }