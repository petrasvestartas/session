use session_rust::proto;
use prost::Message;

pub fn message()->proto::Session {
    let mesh=crate::prepared::PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.1,0.3,0.8]).unwrap().geometry;
    proto::Session {name:"Shared original definitions".into(),guid:"definition-document".into(),objects:Some(proto::Objects {instances:vec![proto::InstanceRef {guid:"first".into(),name:"First".into(),definition_guid:mesh.guid().into(),xform:Some(session_rust::Xform::translation(1000.,0.,0.).to_proto()),..Default::default()},proto::InstanceRef {guid:"second".into(),name:"Second".into(),definition_guid:mesh.guid().into(),flags:session_rust::InstanceRef::FLAG_LOCKED,..Default::default()}],..Default::default()}),definitions:Some(proto::Objects {meshes:vec![mesh.to_proto()],..Default::default()}),tree:Some(proto::Tree {root:Some(proto::TreeNode {name:"root".into(),guid:"root-node".into(),children:vec![proto::TreeNode {name:"first".into(),guid:"first-node".into(),..Default::default()},proto::TreeNode {name:"second".into(),guid:"second-node".into(),..Default::default()}],..Default::default()}),..Default::default()}),xforms:vec![proto::XformEntry {guid:"root".into(),xform:Some(session_rust::Xform::translation(1_000_000_000.125,0.,0.).to_proto())},proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-1.,0.,0.).to_proto())},proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(1.,0.,0.).to_proto())}],..Default::default()}
}
pub fn bytes()->Vec<u8> {message().encode_to_vec()}

// A small shared-definition example keeps placements near the camera while the precision fixture exercises distant original coordinates.
pub fn display_bytes()->Vec<u8> {
    let mut source=message();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;
    source.xforms=vec![proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];source.encode_to_vec()
}
