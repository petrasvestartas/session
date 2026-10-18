        total
    }
}

impl Planes {
    fn cached_plane(
        &mut self,
        ctx: &GpuCtx,
        run: &TextRun,
        fonts: &mut FontSystem,
        raster: &mut SwashCache,
        font_revision: u64,
        em_pixels: u32,
    ) -> anyhow::Result<usize> {
        let mut index = None;

        for (at, cached) in self.cached.iter().enumerate() {
            if cached.label.id == run.label.id {
                index = Some(at);
                break;
            }
        }

        let rebuild = match index {
            Some(at) => !same_raster(&self.cached[at], &run.label, font_revision, em_pixels),
            None => true,
        };

        if rebuild {
            let (pixels, size, extent) = rasterize(run, fonts, raster, em_pixels)?;
            let mut allocated = u64::from(size[0]) * u64::from(size[1]);

            for (at, cached) in self.cached.iter().enumerate() {
                if Some(at) != index {
                    allocated += u64::from(cached.size[0]) * u64::from(cached.size[1]);
                }
            }

            anyhow::ensure!(
                allocated <= TEXTURE_BUDGET,
                "world text coverage exceeds 32 MiB budget"
            );
            let cached = upload_plane(
                ctx,
                &self.layout,
                &self.sampler,
                &run.label,
                font_revision,
                em_pixels,
                (pixels, size, extent),
            );

            match index {
                Some(at) => self.cached[at] = cached,
                None => {
                    index = Some(self.cached.len());
                    self.cached.push(cached);
                }
            }

            self.rasterizations += 1;

            if raster.image_cache.len() > 4096 {
                *raster = SwashCache::new();
            }
        }

        Ok(index.expect("a prepared world plane has a cache entry"))
    }
}
