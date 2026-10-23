        });
        let targets = crate::frame_targets::Targets::with_formats(&device,[640,480],output_format,format,1);
        let strokes = crate::production_ribbon_gpu::Lane::new(&device, format);
        let matrices = crate::production_matrix_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
