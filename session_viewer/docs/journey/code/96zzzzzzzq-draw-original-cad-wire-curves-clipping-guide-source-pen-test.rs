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

    #[test]
    fn clipping_guide_uses_production_ribbons_once_with_original_pens_and_owner() {
        let mut plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[1.,0.,0.]],1.).unwrap();
        plane.width=7.;plane.linecolor=session_rust::Color::new(0.2,0.4,0.8,1.);let plane=Rc::new(plane);plane.guid();let original=plane.to_proto();
        let mut scene=crate::scene::Scene::empty();let id=insert(&mut scene,plane.clone()).unwrap();
        let sources=Sources::new(&scene).unwrap();assert_eq!(sources.mesh_ink.len(),7);assert!(sources.mesh_legacy.iter().all(|v|*v==0),"Queries retain all seven source strokes; the legacy lane draws none");
        let rows=crate::production_joined::rows(&scene,&sources,[0.;3],&[id],4096).unwrap();
        assert_eq!(rows.owners,vec![id.value();7]);assert_eq!(rows.ids,vec![[id.value(),crate::sub_id::OBJECT_INK];7]);assert_eq!(rows.edges,vec![u32::MAX;7]);assert_eq!(rows.mesh_from,7);assert_eq!(rows.runs[3],[0..42]);assert!(rows.runs[..3].iter().all(Vec::is_empty));
        assert_eq!(u32::from_ne_bytes(rows.objects[80..84].try_into().unwrap()),4097);
        let expected=crate::production_ribbon::pack_rgba([0.2,0.4,0.8,1.]);
        for (segment,ends) in rows.segments.chunks_exact(52).zip(&scene.objects()[0].mesh.parts.endpoints) {
            assert_eq!(f32::from_ne_bytes(segment[12..16].try_into().unwrap()),-3.5);assert_eq!(u32::from_ne_bytes(segment[32..36].try_into().unwrap()),expected);
            for offset in[36,40,44]{assert_eq!(u32::from_ne_bytes(segment[offset..offset+4].try_into().unwrap()),u32::MAX);}
            for (offset,point) in[(0,ends[0]),(16,ends[1])]{for axis in 0..3{assert_eq!(f32::from_ne_bytes(segment[offset+axis*4..offset+axis*4+4].try_into().unwrap()),point[axis]as f32);}}
        }
        let pens=scene.objects()[0].mesh.parts.pens.clone();Rc::make_mut(&mut scene.objects_mut()[0].mesh).parts.pens=vec![(3.,u32::from_le_bytes([17,34,51,255]))];
        Rc::make_mut(&mut scene.view).edges.insert(id,[201,101,51]);let overridden=crate::production_joined::rows(&scene,&Sources::new(&scene).unwrap(),[0.;3],&[id],4096).unwrap();
        assert!(overridden.segments.chunks_exact(52).all(|s|f32::from_ne_bytes(s[12..16].try_into().unwrap())==-1.5&&u32::from_ne_bytes(s[32..36].try_into().unwrap())==u32::from_le_bytes([201,101,51,255])));
        Rc::make_mut(&mut scene.view).edges.remove(&id);Rc::make_mut(&mut scene.objects_mut()[0].mesh).parts.pens=pens;
        let restored=crate::production_joined::rows(&scene,&Sources::new(&scene).unwrap(),[0.;3],&[id],4096).unwrap();assert_eq!(restored.segments,rows.segments);assert_eq!(restored.objects,rows.objects);assert_eq!(restored.ids,rows.ids);assert_eq!(plane.to_proto(),original);
        let mut ordinary=crate::scene::Scene::empty();insert(&mut ordinary,Rc::new(Plane::xy_plane())).unwrap();let ordinary=crate::production_joined::rows(&ordinary,&Sources::new(&ordinary).unwrap(),[0.;3],&[],4096).unwrap();assert!(ordinary.segments.is_empty());assert!(!ordinary.planes.is_empty(),"Ordinary Plane retains its procedural lane");
    }
}
