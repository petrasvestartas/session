pub fn insert(scene:&mut crate::scene::Scene,plane:Rc<Plane>)->Result<crate::scene::ObjectId,&'static str>{let id=scene.insert(prepared(&plane)?)?;scene.objects_mut().iter_mut().find(|r|r.id==id).unwrap().original=Some(Geometry::Plane(plane));Ok(id)}


#[cfg(test)]
mod guide_pen_tests {
    use super::*;
    use crate::pick_sources::Sources;

    #[test]
    fn clipping_guide_keeps_source_pen_identity_and_override_precedence() {
        let mut plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[1.,0.,0.]],1.).unwrap();
        plane.width=7.;plane.linecolor=session_rust::Color::new(0.2,0.4,0.8,1.);
        let plane=Rc::new(plane);plane.guid();let original=plane.to_proto();
        let mut scene=crate::scene::Scene::empty();let id=insert(&mut scene,plane.clone()).unwrap();
        let initial=Sources::new(&scene).unwrap();assert_eq!(initial.mesh_ink.len(),7);
        assert!(initial.mesh_ink.iter().all(|stroke|stroke.width==7.&&stroke.colour==[0.2,0.4,0.8,1.]),"The complete clipping rectangle and arrow keep the original custom CSS pen");
        assert!(initial.mesh_ids.iter().all(|hit|hit[0]==id.value()));
        assert!(initial.mesh_metadata.chunks_exact(2).all(|row|row[0]==4096),"Guide ink retains its clipping-plane classification");
        let original_pens=scene.objects()[0].mesh.parts.pens.clone();
        Rc::make_mut(&mut scene.objects_mut()[0].mesh).parts.pens=vec![(3.,u32::from_le_bytes([17,34,51,255]))];
        assert!(Sources::new(&scene).unwrap().mesh_ink.iter().all(|stroke|stroke.width==3.&&stroke.colour==[17./255.,34./255.,51./255.,1.]),"Explicit prepared pens retain their colour and width");
        Rc::make_mut(&mut scene.view).edges.insert(id,[201,101,51]);
        assert!(Sources::new(&scene).unwrap().mesh_ink.iter().all(|stroke|stroke.width==3.&&stroke.colour==[201./255.,101./255.,51./255.,1.]),"The existing display colour override keeps precedence");
        Rc::make_mut(&mut scene.view).edges.remove(&id);Rc::make_mut(&mut scene.objects_mut()[0].mesh).parts.pens=original_pens;
        let restored=Sources::new(&scene).unwrap();assert_eq!(restored.mesh_ids,initial.mesh_ids);assert_eq!(restored.mesh_metadata,initial.mesh_metadata);
        assert!(restored.mesh_ink.iter().zip(&initial.mesh_ink).all(|(a,b)|a.start==b.start&&a.end==b.end&&a.width==b.width&&a.colour==b.colour));
        assert!(matches!(&scene.objects()[0].original,Some(Geometry::Plane(actual)) if Rc::ptr_eq(actual,&plane)));
        assert_eq!(plane.to_proto(),original,"Every original Plane field and owner identity must survive presentation preparation");
        let ordinary=prepared(&Plane::xy_plane()).unwrap();assert!(ordinary.display.parts.edges.is_empty()&&ordinary.display.parts.pens.is_empty(),"Ordinary procedural Plane packing is unchanged");
    }
}
