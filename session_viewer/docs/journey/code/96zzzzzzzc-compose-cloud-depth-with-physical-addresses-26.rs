        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(