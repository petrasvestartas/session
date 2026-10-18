fn retarget_basic_fixture(r:&mut Renderer,count:u32) {
 if r.ambient_enabled.get(){if let Some(owner)=r.ambient.borrow_mut().as_mut(){owner.set_samples(&r.device,count);}}
