        });
        let targets = crate::frame_targets::Targets::new(&device,[640,480],format,1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
