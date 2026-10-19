 r.sheets.set_samples(&r.device,count);
 if let Some(text)=r.text.borrow_mut().as_mut(){text.retarget(&r.text_context(),crate::text_gpu_support::Target{format:r.format,samples:count});}
