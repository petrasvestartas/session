 if let Some(text)=r.text.borrow_mut().as_mut(){text.retarget(&r.text_context(),crate::text_gpu_support::Target{format:r.format,samples:count});}
 let ctx=r.text_context();
 for owner in [&r.global_outline,&r.outline] {if let Some(owner)=owner.borrow_mut().as_mut(){owner.set_samples(&ctx,count);}}
 if let Some(owner)=r.outline_composite.borrow_mut().as_mut(){owner.set_samples(&r.device,count);}
