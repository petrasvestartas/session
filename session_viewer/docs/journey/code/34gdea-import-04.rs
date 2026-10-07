use crate::{document, specimen};
use prost::Message;

fn observed(bytes: &[u8]) -> (Result<document::Loaded, &'static str>, Vec<(String, bool)>) {
    let mut tick = 0.0;
    let mut phases = Vec::new();
    let result = document::load_observed(bytes, None, &mut || { tick += 1.0; tick }, &mut |phase| {
        assert_eq!(phase.duration_ms, 1.0);
        assert_eq!(phase.bytes, bytes.len() as u64);
        assert_eq!(phase.elapsed_ms, (phases.len() + 1) as f64 * 2.0);
        phases.push((phase.name.to_owned(), phase.succeeded));
    });
    (result, phases)
}

#[test]
fn successful_preparation_records_actual_stages_and_preserves_source_precision() {
    let bytes = specimen::precise_bytes();
    let (result, phases) = observed(&bytes);
    assert_eq!(phases, ["decode", "validation", "kernel", "display walk"].map(|name| (name.into(), true)));
    let loaded = result.unwrap();
    assert_eq!(loaded.meshes.len(), 3);
    assert_eq!(loaded.document.objects.meshes[0].to_proto().vertices[&0].x, -0.775000000123456);
}

#[test]
fn failed_stages_stop_before_later_work() {
    let (result, phases) = observed(&[0xff]);
    assert!(result.is_err()); assert_eq!(phases, vec![("decode".into(), false)]);
    let mut message = session_rust::proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    message.objects.as_mut().unwrap().meshes[0].color_mode = 1;
    let (result, phases) = observed(&message.encode_to_vec());
    assert!(result.is_err()); assert_eq!(phases, vec![("decode".into(), true), ("validation".into(), false)]);
    message.objects.as_mut().unwrap().meshes[0].color_mode = 0;
    for face in message.objects.as_mut().unwrap().meshes[0].faces.values_mut() { face.vertices.truncate(3); }
    message.objects.as_mut().unwrap().meshes[0].triangulation.clear();
    message.objects.as_mut().unwrap().meshes[0].vertices.get_mut(&0).unwrap().x = 1e100;
    let (result, phases) = observed(&message.encode_to_vec());
    assert!(result.is_err());
    assert_eq!(phases, vec![("decode".into(), true), ("validation".into(), true), ("kernel".into(), true), ("display walk".into(), false)]);
}
