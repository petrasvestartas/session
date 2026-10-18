    pub fn recordings(&self)->usize {self.records.values().map(Vec::len).sum()}
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Vector;
    fn source()->BRep {crate::cad_display::example()}
    fn assert_display(record:&PreparedMesh,independent:&PreparedMesh) {
        assert_eq!(record.display.indices(),independent.display.indices());assert_eq!(record.display.parts.triangles,independent.display.parts.triangles);assert_eq!(record.display.parts.edges.len(),independent.display.parts.edges.len());assert_eq!(record.display.vertices().len(),independent.display.vertices().len());
        for (a,b) in record.display.vertices().iter().zip(independent.display.vertices()) {for i in 0..3 {assert!((a[i]-b[i]).abs()<1e-3,"Original display position: {a:?} {b:?}");}}
        for (a,b) in record.display.normals().iter().zip(independent.display.normals()) {for i in 0..3 {assert!((a[i]-b[i]).abs()<1e-5,"Original facing normal: {a:?} {b:?}");}}
    }
    #[test]
    fn rigid_originals_replay_their_display_with_distinct_sources_and_exact_topology() {
        let original=Rc::new(source());let _=original.guid();let before=original.to_proto();let mut cache=Cache::default();let (first,_)=cache.prepare(original.clone()).unwrap();
        for turn in [Xform::rotation_z(90.,true),Xform::rotation(&Vector::new(0.3,-0.5,0.8),37.,true)] {
            let motion=&Xform::translation(25.,-12.,9.)*&turn;let mut copy=original.transformed(&motion);copy.set_guid(uuid::Uuid::new_v4().to_string());let copy=Rc::new(copy);let before_copy=copy.to_proto();let (replayed,mesh)=cache.prepare(copy.clone()).unwrap();let (_,independent)=crate::cad_display::prepare(copy.clone()).unwrap();assert_display(&mesh,&independent);assert!(Rc::ptr_eq(&replayed.source,&copy));assert_ne!(first.source.guid(),replayed.source.guid());assert_eq!(replayed.edge_ids,first.edge_ids);assert_eq!(replayed.samples.len(),first.samples.len());assert_eq!(copy.to_proto(),before_copy);
            for (a,b) in first.edges.iter().zip(&replayed.edges) {assert_eq!(a.edge,b.edge);for (a,b) in a.points.iter().zip(&b.points) {let p=motion.transform_point(&session_rust::Point::new(a[0],a[1],a[2]));for i in 0..3 {assert!((p[i]-b[i]).abs()<1e-10);}}}
        }
        assert_eq!(cache.walks,1);assert_eq!(cache.replays,2);assert_eq!(original.to_proto(),before);
    }
    #[test]
    fn mirrored_scaled_and_changed_originals_walk_independently_and_invalid_sources_are_refused() {
        let first=Rc::new(source());let mut cache=Cache::default();cache.prepare(first.clone()).unwrap();let mut scaled=first.transformed(&Xform::scale_xyz(2.,2.,2.));scaled.set_guid("scaled".into());let (_,a)=cache.prepare(Rc::new(scaled.clone())).unwrap();let (_,b)=crate::cad_display::prepare(Rc::new(scaled)).unwrap();assert_display(&a,&b);
        let mirror=first.transformed(&Xform::scale_xyz(-1.,1.,1.));cache.prepare(Rc::new(mirror)).unwrap();let changed=Rc::new(BRep::create_block_with_hole(4.,3.,0.6,0.8));cache.prepare(changed).unwrap();assert_eq!(cache.walks,4);assert_eq!(cache.replays,0);let mut invalid=(*first).clone();invalid.m_faces[0].surface_index=usize::MAX as i32;assert!(cache.prepare(Rc::new(invalid)).is_err());
    }
    #[test]
    fn scoped_recordings_release_original_owners_when_document_preparation_ends() {
        let source=Rc::new(source());let weak=Rc::downgrade(&source);let mut cache=Cache::default();let output=cache.prepare(source.clone()).unwrap();assert_eq!(cache.recordings(),1);drop(source);drop(output);assert!(weak.upgrade().is_some());drop(cache);assert!(weak.upgrade().is_none());
    }
}
