    });
    let pipelines=Variants::new((mask_pipeline(&ctx.device,samples),fullscreen(&ctx.device,&shader,&[&composite_layout],"fs_main",format,true,samples)),samples);
    Self