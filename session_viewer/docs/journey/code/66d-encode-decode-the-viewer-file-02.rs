        let mut bytes=Vec::with_capacity(size);bytes.extend_from_slice(MAGIC);archive.encode(&mut bytes).map_err(|e|e.to_string())?;Ok(bytes)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn metadata()->Metadata {
        Metadata {created_doc:Some(0),current_layer:Some((0,"layer".into())),documents:vec![Document {name:"first".into(),place:session_rust::Xform::translation(1_000_000_000.125,0.,0.).m,point_px:3.},Document {name:"second".into(),place:session_rust::Xform::identity().m,point_px:0.}],hidden:vec![(0,"00000000-0000-0000-0000-000000000001".into())],locked:vec![(1,"00000000-0000-0000-0000-000000000001".into())],colors:vec![(0,"00000000-0000-0000-0000-000000000001".into(),[2,5,7])],edge_colors:Some(vec![(1,"00000000-0000-0000-0000-000000000001".into(),[0;3])]),texts:vec![],groups:vec![]}
    }
    fn documents()->Vec<Vec<u8>> {
        [1.125,2.25].into_iter().map(|x| {
            let mut cloud=session_rust::PointCloud::from_coords(vec![x,2.5,3.75],vec![],vec![]).to_proto();cloud.guid="00000000-0000-0000-0000-000000000001".into();
            let objects=session_rust::proto::Objects {pointclouds:vec![cloud],..Default::default()};
            session_rust::proto::Session {name:"Source cloud".into(),objects:Some(objects),..Default::default()}.encode_to_vec()
        }).collect()
    }
    #[test]
    fn archive_round_trip_keeps_document_namespaces_flags_and_precise_placements() {
        let bundle=Bundle {documents:documents(),metadata:metadata()};let bytes=bundle.encode().unwrap();assert!(bytes.starts_with(MAGIC));
        let reopened=Bundle::decode(&bytes).unwrap();assert_eq!(reopened.documents,bundle.documents);assert_eq!(reopened.metadata.documents[0].place[12],1_000_000_000.125);assert_eq!(reopened.metadata.hidden,bundle.metadata.hidden);assert_eq!(reopened.metadata.locked,bundle.metadata.locked);assert_eq!(reopened.metadata.edge_colors,bundle.metadata.edge_colors);
        assert_eq!(Bundle::decode(&reopened.encode().unwrap()).unwrap().metadata.documents[0].place,bundle.metadata.documents[0].place);
    }
    #[test]
    fn archive_rejects_bad_headers_counts_owners_and_placements_before_scene_adoption() {
        assert!(Bundle::decode(b"invalid").is_err());assert!(Bundle::decode(MAGIC).is_err());
        let mut m=metadata();m.created_doc=Some(2);assert!(m.validate(2).is_err());m=metadata();m.hidden.push((2,"guid".into()));assert!(m.validate(2).is_err());m=metadata();m.documents[0].place[12]=f64::NAN;assert!(m.validate(2).is_err());m=metadata();assert!(m.validate(1).is_err());
        let wrong=Archive {documents:vec![vec![]],metadata:serde_json::to_vec(&metadata()).unwrap()};let mut bytes=MAGIC.to_vec();wrong.encode(&mut bytes).unwrap();assert!(Bundle::decode(&bytes).is_err());
    }
    #[test]
    fn archive_checks_original_geometry_before_kernel_construction() {
        let curve=session_rust::proto::NurbsCurve {dimension:3,order:2,cv_count:i32::MAX,..Default::default()};
        let source=session_rust::proto::Session {objects:Some(session_rust::proto::Objects {nurbscurves:vec![curve],..Default::default()}),..Default::default()};
        assert!(validate_document(&source.encode_to_vec()).is_err());assert!(validate_document(&[0xff]).is_err());
        for document in documents() {validate_document(&document).unwrap();let source=session_rust::proto::Session::decode(document.as_slice()).unwrap();assert_eq!(source.objects.unwrap().pointclouds[0].guid,"00000000-0000-0000-0000-000000000001");}
    }
    #[test]
    fn old_metadata_keeps_face_colour_fallback_and_default_identity_fields() {
        let source=br#"{"documents":[{"name":"old","place":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],"point_px":0}],"hidden":[],"colors":[[0,"guid",[12,34,56]]],"texts":[]}"#;
        let m:Metadata=serde_json::from_slice(source).unwrap();m.validate(1).unwrap();assert!(m.created_doc.is_none());assert!(m.current_layer.is_none());assert!(m.locked.is_empty());assert!(m.groups.is_empty());assert!(m.edge_colors.is_none());assert_eq!(m.edge_colors.unwrap_or_else(||m.colors.clone()),m.colors);
    }
}
