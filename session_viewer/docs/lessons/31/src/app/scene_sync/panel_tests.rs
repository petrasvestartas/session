use super::tests::{check, scene};
use crate::app::command::verbs::point;
use crate::app::hierarchy::Hierarchy;

/// The layers panel sees created objects and loses deleted ones.
#[test]
fn hierarchy_follows_edits() {
    let mut scene = scene();
    let mut panel = Hierarchy::default();
    let (doc, guid) = scene.model(&point::SPEC, &[[1.0, 1.0, 1.0]]).unwrap();
    check(&mut scene);
    panel.refresh(&scene);
    let row = scene.row_of(doc, &guid).unwrap();
    assert!(panel.rows.contains(&row));
    assert!(scene.delete_row(row));
    check(&mut scene);
    panel.refresh(&scene);
    assert!(!panel.rows.contains(&row));
    assert!(scene.undo());
    check(&mut scene);
    panel.refresh(&scene);
    assert!(panel.rows.contains(&row), "the same row comes back");
    let walls = panel.index_of(0, "walls").unwrap();
    let targets = panel.targets(walls);
    let expected: Vec<u32> = {
        let session = &scene.docs[0].session;
        let node = session.tree.get_node_by_name("walls").unwrap();
        let mut rows: Vec<u32> = node
            .borrow()
            .descendants()
            .iter()
            .filter_map(|n| scene.row_of(0, &n.borrow().name))
            .collect();
        rows.sort_unstable();
        rows
    };
    assert_eq!(targets, expected);
}
