        if let (Some(ordinary),Some(tables))=(ordinary,tables.as_ref()) {
            let frame=ordinary.prepare_pick(device,&ctx.queue,transform,window,density,clipping_buffer,tables,&image.depth);
            let mut pass=overlay(&mut encoder,image,"production ordinary line IDs",false);
            ordinary.pick(&mut pass,&frame,mode);
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);