        self.rebuild_resources(ctx);
    }
}

impl Lane {

    pub fn prepare(&mut self, ctx: &GpuCtx, frame: &TextFrame) -> anyhow::Result<()> {
        let fixed = self
            .document
            .runs
            .iter()
            .all(|run| matches!(run.label.placement, TextPlacement::Screen { .. }));
        let key = (
            self.document.revision,
            self.document.font_revision,
            if fixed { frame.canvas() } else { frame.clone() },
        );

        if self.prepared.as_ref() == Some(&key) {
            self.stats.skipped_preparations += 1;
            return Ok(());
        }

        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        let scale = frame.scale()?;
        let start = crate::load_measure::now();
        let fonts_changed = self.atlas_font_revision != self.document.font_revision;

        if fonts_changed || self.raster_keys.len() > 4096 {
            self.rebuild_resources(ctx);
        }

        let raster_images_before = self.raster.image_cache.values().flatten().count();
        self.planes
            .prepare(ctx, &mut self.document, &mut self.raster, frame)?;
        self.atlas.trim();
        self.viewport.update(
            &ctx.queue,
            Resolution {
                width: frame.framebuffer[0],
                height: frame.framebuffer[1],
            },
        );
        let prepared = areas(
            &self.document.runs,
            frame,
            scale,
            &mut self.stats,
            &mut self.raster_keys,
        );
        self.overlay_count = u32::from(prepared.overlay);
        self.anchored_count = u32::from(prepared.anchored);

        self.overlay.prepare(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            prepared.overlays,
            &mut self.raster,
        )?;
        self.anchored.prepare_with_depth(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            prepared.anchors,
            &mut self.raster,
            |id| depth_for(&prepared.depths, id),
        )?;
        self.plates
            .prepare(ctx, &prepared.plates, frame.framebuffer);
        self.measure_preparation(raster_images_before, start);
        self.prepared = Some(key);
        Ok(())
    }
}
