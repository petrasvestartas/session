    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use {prost::Message,session_rust::proto};
    #[test]
    fn ranged_originals_refuse_unapplied_placements_extended_sources_and_wrong_tree_owners() {
        assert!(metadata(&[],"cloud").is_ok());
        let tail=proto::Session {xforms:vec![proto::XformEntry {guid:"cloud".into(),xform:Some(session_rust::Xform::translation(1_000_000_000.125,0.,0.).to_proto())}],..Default::default()};assert!(metadata(&tail.encode_to_vec(),"cloud").is_err());
        let tail=proto::Session {definitions:Some(Default::default()),..Default::default()};assert!(metadata(&tail.encode_to_vec(),"cloud").is_err());
        let tail=proto::Session {tree:Some(proto::Tree {root:Some(proto::TreeNode {name:"root".into(),children:vec![proto::TreeNode {name:"missing".into(),..Default::default()}],..Default::default()}),..Default::default()}),..Default::default()};assert!(metadata(&tail.encode_to_vec(),"cloud").is_err());assert!(metadata(b"broken","cloud").is_err());
    }
}
