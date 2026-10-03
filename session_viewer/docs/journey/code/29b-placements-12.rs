use crate::{document, specimen};
use prost::Message;
use session_rust::{proto, Xform};

#[test]
fn malformed_placements_are_rejected_before_construction() {
    let base = proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    let guid = base.objects.as_ref().unwrap().meshes[0].guid.clone();
    let entry = proto::XformEntry { guid, xform: Some(Xform::identity().to_proto()) };
    let mut nonfinite = Xform::identity().m.to_vec(); nonfinite[0] = f64::NAN;
    let mut projective = Xform::identity().m.to_vec(); projective[3] = 0.5;
    for matrix in [Vec::new(), vec![0.0; 15], nonfinite, projective] {
        let mut message = base.clone();
        let mut invalid = entry.clone(); invalid.xform.as_mut().unwrap().matrix = matrix;
        message.xforms = vec![invalid];
        assert!(document::load(&message.encode_to_vec()).is_err());
    }
    let mut missing = entry.clone(); missing.xform = None;
    let mut orphan = entry.clone(); orphan.guid = "not-a-mesh".into();
    for entries in [vec![missing], vec![orphan], vec![entry.clone(), entry.clone()]] {
        let mut message = base.clone(); message.xforms = entries;
        assert!(document::load(&message.encode_to_vec()).is_err());
    }
}
