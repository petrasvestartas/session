        )
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
    ) -> Result<(), &'static str> {
        let mut old = std::mem::take(&mut self.draws);
        for row in scene
            .sheets()
            .iter()
            .filter(|row| scene.visible(row.id) && row.prepared.original.segment_count > 0)
        {
            for page in row.prepared.pages() {
                for rows in chunks(
                    page.rows.end - page.rows.start,
                    device.limits().max_buffer_size,
                )? {
                    let key = page.key();
                    let geometry =
                        self.geometry(device, &row.prepared, page.clone(), rows.clone())?;
                    let base = bases
                        .get(&(row.id, key))
                        .ok_or("Missing sheet original entity addresses")?
                        .checked_add(rows.start)
                        .ok_or("Drawing pick address overflow")?;
                    let bytes = settings(scene, row, base, selected.contains(&row.id));
                    let draw = if let Some(i) = old
                        .iter()
                        .position(|d| d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry))
                    {
                        let mut draw = old.swap_remove(i);
                        if draw.bytes != bytes {
                            queue.write_buffer(&draw.uniform, 0, &bytes);
                            draw.bytes = bytes;
                        }
                        draw
                    } else {
                        let uniform =
                            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("independent sheet placement"),
                                contents: &bytes,
                                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            });
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
                            id: row.id,
                            geometry,
                            uniform,
                            group,
                            bytes,
                        }
                    };
                    self.draws.push(draw);
                }
            }
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        Ok(())
    }
}
pub fn chunks(count: u32, bytes: u64) -> Result<Vec<std::ops::Range<u32>>, &'static str> {
