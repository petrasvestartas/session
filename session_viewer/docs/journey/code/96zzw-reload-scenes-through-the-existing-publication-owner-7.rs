        Ok(Self::from_route(document,fault,route))
    }
    pub fn at(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,path:&str)->Result<Self,JsValue>{
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;
        Ok(Self::from_route(document,fault,crate::publication_route::named(path,url.search_params().get("data").as_deref())))
    }
    fn from_route(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,route:Route)->Self{
        let index=Index::new(route.base.clone());Self(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None}))))))