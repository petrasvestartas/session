
impl Scene {
    pub fn replace_original(&mut self,id:ObjectId,mut geometry:session_rust::Geometry)->Result<(),&'static str>{
        use session_rust::Geometry;
        if self.pick_id(id.value())!=Some(id){return Err("Edit a visible unlocked original")}
        let original=crate::source_edit::original(self,id).ok_or("Reload an editable original source")?;
        let model=self.placement(id).ok_or("Source placement is missing")?;geometry.set_guid(original.guid());
        match geometry{
            Geometry::Point(source)=>{let prepared=crate::marker::PreparedPoint::new(source)?;let p=model.transform_point(&prepared.source);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited point exceeds display range")}self.points.iter_mut().find(|r|r.id==id).ok_or("Expected a point source")?.prepared=prepared;},
            Geometry::Line(source)if self.lines.iter().any(|r|r.id==id)=>{let prepared=crate::stroke::PreparedLine::new(source)?;for p in [prepared.source.start(),prepared.source.end()]{let p=model.transform_point(&p);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited line exceeds display range")}}self.lines.iter_mut().find(|r|r.id==id).unwrap().prepared=prepared;},
            Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)=>{
                let prepared=match geometry{Geometry::Line(source)=>crate::chain::PreparedChain::line(source)?,Geometry::Polyline(source)=>crate::chain::PreparedChain::polyline(source)?,Geometry::NurbsCurve(source)=>crate::chain::PreparedChain::curve(source)?,_=>unreachable!()};
                for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()){let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited path exceeds display range")}}self.paths.iter_mut().find(|r|r.id==id).ok_or("Expected a path source")?.prepared=prepared;
            },
            Geometry::Mesh(_)|Geometry::BRep(_)|Geometry::NurbsSurface(_)=>{
                let(cad,surface,prepared)=match geometry{Geometry::Mesh(source)=>(None,None,crate::prepared::PreparedMesh::new((*source).clone())?),Geometry::BRep(source)=>{let(cad,mesh)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,mesh)},Geometry::NurbsSurface(source)=>{let s=Rc::new(crate::surface::Prepared::new(source)?);let mesh=s.mesh()?;(None,Some(s),mesh)},_=>unreachable!()};
                for v in prepared.display.vertices(){let p=model.transform_point(&session_rust::Point::new(v[0]as f64,v[1]as f64,v[2]as f64));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited object exceeds display range")}}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected an object source")?;row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};
            },
            _=>return Err("This original source cannot be replaced"),
        }Ok(())
    }
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
