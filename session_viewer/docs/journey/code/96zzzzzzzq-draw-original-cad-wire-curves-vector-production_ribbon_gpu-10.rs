        &mut self,
        device: &wgpu::Device,
        scene: &crate::scene::Scene,
        sources: &crate::pick_sources::Sources,
        anchor: [f64; 3],
        selected: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        let limit = u64::from(device.limits().max_storage_buffer_binding_size);
        let mut flags = selected.to_vec();
        if let Some(owner) = self.highlighted { if !flags.contains(&owner) { flags.push(owner); } }
        let rows = ribbon::rows(scene, sources, anchor, &flags, limit)?;
        if [&rows.segments, &rows.objects, &rows.translations]
            .iter()
            .any(|data| data.len() as u64 > limit)
