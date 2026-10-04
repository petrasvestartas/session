use crate::{document, specimen};
use prost::Message;
use session_rust::proto;

#[test]
fn malformed_and_unsupported_records_are_refused_before_construction() {
    assert!(document::load(&[]).is_err());
    assert!(document::load(&vec![0; document::MAX_BYTES + 1]).is_err());
    let original = proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    for case in 0..6 {
        let mut message = original.clone();
        let meshes = &mut message.objects.as_mut().unwrap().meshes;
        match case {
            0 => { meshes[0].vertices.values_mut().next().unwrap().x = f64::NAN; }
            1 => { meshes[0].faces.values_mut().next().unwrap().vertices[0] = u64::MAX; }
            2 => { meshes[1].guid = meshes[0].guid.clone(); }
            3 => { meshes[0].color_mode = 1; }
            4 => { message.xforms.push(proto::XformEntry::default()); }
            _ => { message.objects.as_mut().unwrap().points.push(proto::Point::default()); }
        }
        assert!(document::load(&message.encode_to_vec()).is_err(), "case {case}");
    }
}
