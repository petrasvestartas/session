        }
    }
    fn geometry(
        &mut self,
        device: &wgpu::Device,
        source: &Rc<Prepared>,
        page: crate::sheet_stream::DisplayPage,
        rows: std::ops::Range<u32>,
    ) -> Result<Rc<Geometry>, &'static str> {
        let key = (page.key(), rows.start);
        Ok(
            if let Some(owner) = self.cache.get(&key).and_then(Weak::upgrade) {
                owner
            } else {
                let count = rows.end - rows.start;
                if count as u64 * 32 > device.limits().max_buffer_size {
                    return Err("Sheet display exceeds the device buffer limit");
                }
                let mut bytes = Vec::with_capacity(count as usize * 32);
                for (i, p) in page
                    .display
                    .chunks_exact(6)
                    .enumerate()
                    .skip(rows.start as usize)
                    .take(count as usize)
                {
                    for v in &p[..3] {
                        bytes.extend(v.to_le_bytes());
                    }
                    bytes.extend(page.radius(i).to_le_bytes());
                    for v in &p[3..] {
                        bytes.extend(v.to_le_bytes());
                    }
                    bytes.extend(page.colors[i].to_le_bytes());
                }
                let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("shared original sheet segments"),
                    contents: &bytes,
                    usage: wgpu::BufferUsages::VERTEX,
                });
                self.uploads += 1;
                self.uploaded_bytes += bytes.len() as u64;
                let owner = Rc::new(Geometry {
                    _page: page.clone(),
                    source: source.clone(),
                    vertices,
                    count,
                });
                self.cache.insert(key, Rc::downgrade(&owner));
                owner
            },
        )
    }
}
pub fn chunks(count: u32, bytes: u64) -> Result<Vec<std::ops::Range<u32>>, &'static str> {
