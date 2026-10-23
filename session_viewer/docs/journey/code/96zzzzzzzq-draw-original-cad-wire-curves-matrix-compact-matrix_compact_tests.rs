use crate::{production_matrix as matrix, scene::Scene};
use session_rust::{Matrix, Xform};
use std::rc::Rc;

fn fixture(first: u32) -> Scene {
    let mut scene = Scene::empty();
    scene.next_id = first;
    let mut matrix = Matrix::from_rows(&[vec![1.0, -2.0]]);
    matrix.cell = 0.4;
    let id = scene.insert_matrix(Rc::new(matrix.to_component())).unwrap();
    scene.place_matrix(id, Xform::translation(-0.2, 0.0, -0.4)).unwrap();
    scene
}

#[test]
fn sparse_matrix_ids_use_compact_storage_without_rewriting_the_source() {
    let scene = fixture(16_777_217);
    let row = &scene.matrices()[0];
    let original = row.source.to_proto();
    let packed = matrix::rows(&scene, [0.0; 3], &[row.id], 256).unwrap();
    assert_eq!(packed.objects.len(), 96);
    assert_eq!(packed.translations.len(), 16);
    assert_eq!(packed.owners, [row.id.value()]);
    assert_eq!(packed.starts, [0]);
    assert_eq!(u32::from_ne_bytes(packed.words[8..12].try_into().unwrap()), 0);
    assert_eq!(u32::from_ne_bytes(packed.objects[80..84].try_into().unwrap()), 1);
    assert_eq!(row.source.to_proto(), original);
    assert!(matrix::rows(&scene, [0.0; 3], &[], 95).is_err());
}

#[test]
fn matrix_owner_adapter_changes_only_the_integer_output() {
    use sha2::{Digest, Sha256};
    let provenance: serde_json::Value = serde_json::from_str(matrix::PROVENANCE).unwrap();
    let adapter = &provenance["ownership_adapter"];
    let original = adapter["original"].as_str().unwrap();
    let replacement = adapter["replacement"].as_str().unwrap();
    let declaration = adapter["declaration"].as_str().unwrap();
    assert_eq!(matrix::SOURCE.matches(replacement).count(), 1);
    let pristine = matrix::SOURCE.strip_suffix(declaration).unwrap().replace(replacement, original);
    assert_eq!(format!("{:x}", Sha256::digest(pristine.as_bytes())), provenance["pristine_sha256"].as_str().unwrap());
}

#[test]
fn sparse_matrix_color_selection_and_pick_match_the_dense_scene() {
    use crate::cap_gpu_tests::{pixels_at, renderer, wait};
    let low = fixture(1);
    let high = fixture(16_777_217);
    let mut gpu = renderer(&low);
    let transform = crate::cap_gpu_tests::matrix();
    let owner = high.matrices()[0].id;
    let original = high.matrices()[0].source.to_proto();
    for samples in [1, 4] {
        gpu.set_samples(samples);
        gpu.set_scene(&low, None);
        let plain = pixels_at(&gpu, transform);
        gpu.select_objects(&low, &[low.matrices()[0].id]).unwrap();
        let selected = pixels_at(&gpu, transform);
        assert_ne!(plain, selected);
        gpu.set_scene(&high, None);
        assert_eq!(pixels_at(&gpu, transform), plain);
        gpu.request_pick([-0.28, -0.08], transform).unwrap();
        wait(&gpu);
        let pixel = gpu.take_pick(transform).unwrap().unwrap().unwrap();
        assert_eq!(gpu.resolve_pick(&high, pixel).unwrap().owner, owner);
        gpu.select_objects(&high, &[owner]).unwrap();
        assert_eq!(pixels_at(&gpu, transform), selected);
        gpu.select_objects(&high, &[]).unwrap();
    }
    assert_eq!(high.matrices()[0].source.to_proto(), original);
}
