pub fn geometry(scene:&Scene,id:ObjectId)->Result<Geometry,&'static str> {
    let row=scene.objects().iter().find(|row|row.id==id).ok_or("Source object is missing")?;
    if let Some(original)=&row.original{return Ok(original.clone())}
    if let Some(cad)=&row.cad{return Ok(Geometry::BRep(Rc::clone(&cad.source)))}
    if let Some(surface)=&row.surface{return Ok(Geometry::NurbsSurface(Rc::clone(&surface.source)))}
