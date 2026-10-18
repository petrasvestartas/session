        self.document.set_labels(labels)
    }
}

impl Lane {
    fn measure_preparation(&mut self, raster_images_before: usize, start: f64) {
        self.stats.raster_images = 0;
        self.stats.raster_image_capacity_bytes = 0;

        for image in self.raster.image_cache.values().flatten() {
            self.stats.raster_images += 1;
            self.stats.raster_image_capacity_bytes += image.data.capacity();
        }

        self.stats.new_raster_images = self
            .stats
            .raster_images
            .saturating_sub(raster_images_before);
        self.stats.preparations += 1;
        self.stats.shape_count = self.document.shape_count;
        self.stats.shaping_ms = self.document.shaping_ms;
        self.stats.distinct_raster_keys = self.raster_keys.len();
        self.stats.active_instance_bytes = self.stats.requested_glyphs * 28;
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = self.planes.texture_bytes();
        self.stats.world_plane_rasterizations = self.planes.rasterizations;
        self.stats.preparation_ms = crate::load_measure::now() - start;
    }

    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        self.overlay = renderer(ctx, &mut self.atlas, target, wgpu::CompareFunction::Always);
        self.anchored = renderer(
            ctx,
            &mut self.atlas,
            target,
            wgpu::CompareFunction::LessEqual,
        );
        self.prepared = None;
    }

    pub fn reset(&mut self) {
        self.document.clear();
        self.prepared = None;
        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        self.planes.reset();
    }

    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.plates.release(ctx);
        self.planes.release(ctx);
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = 0;
        self.rebuild_resources(ctx);
    }
}
