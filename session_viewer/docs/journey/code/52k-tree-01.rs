use session_rust::{proto,Xform};
use prost::Message;

pub fn nested(attribute:bool)->Vec<u8> {
    let mut message=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();
    let guids=message.objects.as_ref().unwrap().meshes.iter().map(|m|m.guid.clone()).collect::<Vec<_>>();
    let mut child=proto::TreeNode {name:guids[1].clone(),..Default::default()};
    if attribute {child=proto::TreeNode {name:"attributes".into(),children:vec![child],..Default::default()};}
    message.tree=Some(proto::Tree {root:Some(proto::TreeNode {name:"Frame".into(),children:vec![proto::TreeNode {name:"Posts".into(),children:vec![proto::TreeNode {name:guids[0].clone(),children:vec![child],..Default::default()}],..Default::default()},proto::TreeNode {name:guids[2].clone(),..Default::default()}],..Default::default()}),..Default::default()});
    message.xforms=vec![proto::XformEntry {guid:guids[0].clone(),xform:Some(Xform::translation(0.123456789012345,0.,0.).to_proto())},proto::XformEntry {guid:guids[1].clone(),xform:Some(Xform::translation(0.,0.25,0.).to_proto())}];message.encode_to_vec()
}
