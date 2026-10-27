    for (row,columns) in candidates {
      let (records,group,address,address_group)=if let Some(i)=old.iter().position(|draw|draw.row.id==row.id&&draw.row.rows==row.rows&&Rc::ptr_eq(&draw.columns,&columns)) {
        let draw=old.swap_remove(i);(draw.records,draw.group,draw.address,draw.address_group)
      } else {
        let records=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("bounded cloud draw records"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let group=resource_group(ctx,&self.columns_layout,"cloud source and records",[(0,records.as_entire_binding()),(1,columns.positions.as_entire_binding()),(2,columns.colors.as_entire_binding()),(3,columns.normals.as_entire_binding())]);
        let address=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("cloud page pick address"),size:16,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let address_group=resource_group(ctx,&self.address_layout,"cloud page ownership",[(0,address.as_entire_binding())]);
        (records,group,address,address_group)
      };
      ctx.queue.write_buffer(&address,0,&[row.address,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
      let id=row.id;draws.push(Draw {row,columns,records,group,address,address_group,visible:scene.visible(id),pickable:scene.pick_id(id.value()).is_some(),selected:selected.contains(&id),color:scene.face_rgba(id),total:0});
    }
