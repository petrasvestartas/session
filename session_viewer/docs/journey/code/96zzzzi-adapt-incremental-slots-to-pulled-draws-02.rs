
    #[test]
    fn local_pulled_slots_keep_physical_ids_and_detect_reordered_members() {
        let mut space=SlotSpace::default();let definitions=[definition(1,&[7,8],30,2),definition(2,&[9],60,1)];space.reset(&definitions,100);
        assert_eq!(space.local_slot(1),[7,100]);assert_eq!(space.local_slot(2),[8,102]);assert_eq!(space.local_slot(3),[9,104]);assert_eq!(space.local_slot(2)[1]+1,space.slot(2)[1]+11);
        assert!(space.touched(&definitions).is_empty());let changed=[definition(1,&[8,7],30,2),definition(2,&[9],60,1)];let touched=space.touched(&changed);assert_eq!(touched,[(1,0),(1,1)]);let change=space.update(&changed,&touched,100);assert_eq!(change.written(),Some(3));assert_eq!(space.local_slot(1),[8,100]);assert_eq!(space.local_slot(2),[7,102]);assert!(space.touched(&changed).is_empty());
        let shrunk=[definition(1,&[8],30,2),definition(2,&[9],60,1)];let touched=space.touched(&shrunk);assert!(touched.is_empty());space.update(&shrunk,&touched,100);assert_eq!(space.texel(2)[3],0);assert_eq!(space.local_slot(3),[9,104]);
    }
    /// Instances number their triangles one after another, past the arena's: