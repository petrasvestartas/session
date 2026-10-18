        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let own_slot=crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(&device,&queue));