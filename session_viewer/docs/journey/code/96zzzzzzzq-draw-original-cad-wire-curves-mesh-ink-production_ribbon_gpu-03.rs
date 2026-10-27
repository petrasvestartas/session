        self.dots.set(device,&rows.dots,&rows.pick_owners);self.spheres.set(device,&rows.spheres,&rows.pick_owners);
        *self.inside.borrow_mut()=vec![false;rows.inside.len()];
        if self.ids!=rows.ids {self.picks=buffer(device,&rows.ids.iter().flatten().flat_map(|id|id.to_ne_bytes()).collect::<Vec<_>>(),8,wgpu::BufferUsages::STORAGE);}
        let edge=self.edge_highlight.and_then(|target|rows.object_ids.iter().position(|id|*id==target.owner).and_then(|index|crate::production_mesh_ink::source_edge(scene,target).map(|edge|[index as u32,edge,u32::MAX,u32::MAX]))).unwrap_or([u32::MAX;4]);
        self.selection=buffer(device,&edge.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>(),16,wgpu::BufferUsages::UNIFORM);
        self.ids = rows.ids.clone();
        self.rows = rows;
