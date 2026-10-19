    }
}

#[cfg(test)]
mod tests {
    use prost::Message;
    #[test]
    fn range_layout_and_original_pages_match_the_kernel_protobuf() {
        let mut cloud=session_rust::PointCloud::from_coords(vec![1_000_000_000.125,2.,3.,1_000_000_000.375,4.,5.],vec![],vec![]);
        cloud.name="Precise ranged source".into();let _=cloud.guid();let mut proto=cloud.to_proto();proto.point_ids=vec![u32::MAX,7];
        let mut objects=session_rust::proto::Objects::default();objects.pointclouds.push(proto.clone());let document=session_rust::proto::Session {objects:Some(objects),..Default::default()};let raw=document.encode_to_vec();
        assert_eq!(crate::stream::cloud_identity(&raw).unwrap(),(proto.guid.clone(),proto.name.clone()));
        let (at,length)=crate::stream::walk_to_coords(&raw).unwrap();assert_eq!(length,48);
        let page=crate::range_page::Page::decode(0..2,2,&raw[at as usize..(at+length) as usize],Some(&proto.point_ids.iter().flat_map(|id|id.to_le_bytes()).collect::<Vec<_>>())).unwrap();
        assert_eq!(page.coordinates,proto.coords);assert_eq!(page.ids,proto.point_ids);
        assert!(crate::stream::walk_to_coords(&raw[..at as usize-1]).is_none());
    }
}
