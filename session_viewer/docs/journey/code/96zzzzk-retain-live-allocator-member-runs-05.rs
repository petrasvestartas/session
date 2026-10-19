 #[test]
 fn plans_leave_bounded_headroom_before_reserved_caps() {
  let small=[Members {key:0,faces:0..3,rows:vec![7,8]}];assert!(bounded(&small,1));assert!(bounded(&[],0));assert!(!bounded(&[],crate::primitive_image::CAP_START));let large=[Members {key:0,faces:0..u32::MAX,rows:vec![7,8]}];assert!(!bounded(&large,1));
 }
 #[test]
 fn exact_ranges_merge_members_and_reject_unrelated_storage()