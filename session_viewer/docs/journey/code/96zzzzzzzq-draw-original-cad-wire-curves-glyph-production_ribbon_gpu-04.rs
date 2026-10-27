                wgpu::BufferUsages::STORAGE,
            );
        }
        if rows.objects != self.rows.objects || self.inside.borrow().iter().any(|v|*v) {
            self.objects = buffer(device, &rows.objects, 96, wgpu::BufferUsages::STORAGE);
        }
        if rows.translations != self.rows.translations {
