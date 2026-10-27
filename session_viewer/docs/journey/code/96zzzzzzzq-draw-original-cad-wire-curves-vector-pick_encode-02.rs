        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);

            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        if let (Some(sheets),Some(tables))=(sheets,tables.as_ref()) {
            let frame=sheets.prepare_pick(device,&ctx.queue,transform,window,density,clipping_buffer,tables,&image.depth);
            let mut pass=overlay(&mut encoder,image,"production sheet source IDs",false);sheets.pick(&mut pass,&frame,mode);
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
