use crate::{object_selection::Objects, scene::Scene};

#[test]
fn repeated_and_blank_additive_picks_keep_unique_original_order() {
    let scene = Scene::demo(); let a = scene.objects()[0].id; let b = scene.objects()[1].id;
    let mut selected = Objects::default(); selected.set(&scene, [b, a, b], false);
    assert_eq!(selected.rows(), &[b, a]); assert_eq!(selected.primary(), Some(a));
    selected.set(&scene, [], true); assert_eq!(selected.rows(), &[b, a]);
    selected.set(&scene, [], false); assert!(selected.rows().is_empty());
}
