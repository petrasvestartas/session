        if rows.words!=self.rows.words {self.segments=buffer(device,&rows.words,64,wgpu::BufferUsages::STORAGE);self.uploads+=1;self.uploaded_bytes+=rows.words.len() as u64;}
        if rows.objects!=self.rows.objects {self.objects=buffer(device,&rows.objects,96,wgpu::BufferUsages::STORAGE);}
        if rows.translations!=self.rows.translations {self.translations=buffer(device,&rows.translations,16,wgpu::BufferUsages::STORAGE);}
        if rows.owners!=self.rows.owners {self.owners=buffer(device,&rows.owners.iter().flat_map(|id|id.to_ne_bytes()).collect::<Vec<_>>(),4,wgpu::BufferUsages::STORAGE);}
        self.rows=rows;self.anchor=anchor;self.frame.borrow_mut().take();Ok(())
    }

