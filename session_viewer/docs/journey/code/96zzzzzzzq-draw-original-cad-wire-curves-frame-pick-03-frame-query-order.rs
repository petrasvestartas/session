        if let (Some(matrices),Some(tables))=(matrices,tables.as_ref()) {
            let frame=matrices.prepare_pick(device,&ctx.queue,transform,window,density,clipping_buffer,tables,&image.depth);
            let mut pass=overlay(&mut encoder,image,"production Matrix IDs",false);matrices.pick(&mut pass,&frame,mode);
        }
        if let (Some(ordinary),Some(tables))=(ordinary,tables.as_ref()) {
            let frame=ordinary.prepare_pick(device,&ctx.queue,transform,window,density,clipping_buffer,tables,&image.depth);
            let mut pass=overlay(&mut encoder,image,"production ordinary line IDs",false);
            ordinary.pick(&mut pass,&frame,mode);
        }
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
