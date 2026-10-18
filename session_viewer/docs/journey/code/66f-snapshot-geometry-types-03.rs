    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    #[test]
    fn snapshot_retains_original_cad_curves_clouds_flags_and_independent_line_guids() {
        let mut scene=Scene::demo();scene.clear();let cad=Rc::new(crate::cad_display::example());let _=cad.guid();let cad_before=cad.to_proto();let cad_id=scene.insert_cad(cad.clone()).unwrap();
        let surface=Rc::new(crate::surface::example());let _=surface.guid();let surface_before=surface.to_proto();let surface_id=scene.insert_surface(surface.clone()).unwrap();
        let mut original_curve=session_rust::NurbsCurve::create(true,2,&[session_rust::Point::new(-0.6,-0.4,0.),session_rust::Point::new(0.123456789012345,0.6,0.),session_rust::Point::new(0.6,-0.4,0.)]);assert!(original_curve.set_cv_4d(1,0.123456789012345*4.,2.4,0.,4.));let curve=Rc::new(original_curve);let _=curve.guid();let curve_before=curve.to_proto();let curve_id=scene.insert_path(crate::chain::PreparedChain::curve(curve.clone()).unwrap()).unwrap();
        let mut proto=crate::cloud::example().to_proto();proto.point_ids[0]=u32::MAX;let cloud=Rc::new(session_rust::PointCloud::from_proto(proto));let _=cloud.guid();let cloud_before=cloud.to_proto();let cloud_id=scene.insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap()).unwrap();
        let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(0.,0.,0.,1.,0.,0.))).unwrap()).unwrap();let copy=scene.copy_row(line,false).unwrap();
        Rc::make_mut(&mut scene.view).hide(&[cad_id],true);Rc::make_mut(&mut scene.view).lock(&[cloud_id],true);
        let ids=[cad_id,surface_id,curve_id,cloud_id,line,copy].into_iter().collect();let out=objects(&scene,&ids).unwrap();
        assert!(out.meshes.is_empty());assert_eq!(out.breps.len(),1);assert_eq!(out.nurbssurfaces.len(),1);assert_eq!(out.nurbscurves.len(),1);assert_eq!(out.lines.len(),2);assert_ne!(out.lines[0].guid,out.lines[1].guid);
        assert_eq!(out.breps[0].is_visible,Some(false));assert_eq!(out.pointclouds[0].is_locked,Some(true));assert_eq!(out.pointclouds[0].point_ids[0],u32::MAX);assert_eq!(out.pointclouds[0].coords,cloud_before.coords);assert_eq!(out.nurbscurves[0].cvs,curve_before.cvs);assert!(out.nurbscurves[0].is_rational);assert_eq!(out.nurbscurves[0].cvs[7],4.);
        assert_eq!(cad.to_proto(),cad_before);assert_eq!(surface.to_proto(),surface_before);assert_eq!(curve.to_proto(),curve_before);assert_eq!(cloud.to_proto(),cloud_before);
        crate::source_validate::session(&proto::Session {objects:Some(out),..Default::default()}).unwrap();
    }
}
