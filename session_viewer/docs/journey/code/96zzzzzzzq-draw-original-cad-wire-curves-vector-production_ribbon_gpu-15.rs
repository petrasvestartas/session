        if self.rows.owners.is_empty() {
            return;
        }
        self.prepare_color(device);self.heads.prepare(device,self.samples,false);
        let bytes = matrix
            .into_iter()
            .flat_map(f32::to_ne_bytes)
