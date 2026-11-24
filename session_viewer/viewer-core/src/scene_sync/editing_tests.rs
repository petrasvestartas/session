use super::tests::{check, file, find, live, p, scene};
use super::*;
use session_rust::{Mesh, Point};

/// A deleted cloud keeps its points for an undo and counts toward the cap; past it its points die.
#[test]
fn a_deleted_cloud_waits_in_its_tomb() {
    use session_rust::{Color, PointCloud, Vector};

    let mut scene = scene();
    let mut session = Session::new("cloud");
    let points: Vec<Point> = (0..100).map(|i| p(i as f64, 0.0, 0.0)).collect();
    let normals = vec![Vector::new(0.0, 0.0, 1.0); points.len()];
    let colors = vec![Color::red(); points.len()];
    session.add_pointcloud(PointCloud::new(points, normals, colors), None);
    scene.add_file(file("cloud", session, Xform::identity()));
    scene.settle();
    let cloud = find(&scene, |g| matches!(g, Geometry::PointCloud(_))).unwrap();

    assert!(scene.delete_row(cloud));
    scene.sync();
    assert_eq!(scene.staged.bury, [cloud], "hidden, its points kept");
    assert!(scene.staged.clouds.is_empty());
    scene.settle();
    assert_eq!(scene.tomb_bytes(), 100 * CLOUD_POINT_BYTES);

    assert!(scene.undo());
    scene.sync();
    assert_eq!(
        scene.staged.unbury.len(),
        1,
        "drawn again, nothing uploaded"
    );
    scene.settle();
    scene.verify();
    assert_eq!(scene.tomb_bytes(), 0);

    scene.tomb_cap = 100 * CLOUD_POINT_BYTES - 1;
    assert!(scene.delete_row(cloud));
    scene.sync();
    assert!(scene.tombs.is_empty(), "past the cap it is released");
    assert_eq!(scene.staged.clouds, [cloud], "its points die");
    scene.settle();
}

/// A compaction walks the lanes again without the tombs; undo walks the object anew.
#[test]
fn compaction_drops_the_tombs() {
    let mut scene = scene();
    let mesh = find(&scene, |g| matches!(g, Geometry::Mesh(_))).unwrap();
    let id = scene.identity_of(mesh).unwrap();
    assert!(scene.delete_row(mesh));
    check(&mut scene);
    assert_eq!(scene.tombs.len(), 1);
    scene.rewalk_cpu();
    scene.verify();
    assert!(scene.tombs.is_empty() && scene.tombed.is_empty());
    assert!(scene.undo());
    check(&mut scene);
    assert!(live(&scene).iter().any(|(_, held)| *held == id));
}

/// Every object of several documents goes in one step; one undo brings them all back, one redo takes them again.
#[test]
fn delete_rows_is_one_step() {
    let mut scene = scene();
    let before = live(&scene);
    let rows: Vec<u32> = before.iter().map(|(row, _)| *row).collect();
    let ids: HashSet<_> = before.iter().map(|(_, id)| id.clone()).collect();
    let docs: HashSet<usize> = ids.iter().map(|id| id.0).collect();
    let held = |scene: &Scene| {
        live(scene)
            .into_iter()
            .filter(|(_, id)| ids.contains(id))
            .count()
    };
    assert!(docs.len() > 1);
    let steps = scene.undo_steps.len();
    assert!(scene.delete_rows(&rows) > 1);
    check(&mut scene);
    assert_eq!(scene.undo_steps.len(), steps + 1, "one step");
    assert_eq!(held(&scene), 0);

    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(held(&scene), ids.len(), "one undo brings all");
    assert!(scene.tombs.is_empty());
    assert!(scene.redo());
    check(&mut scene);
    assert_eq!(held(&scene), 0, "one redo takes all");
    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(held(&scene), ids.len());
}

/// An object outside the tree is buried by its op's own tomb and shown again by undo.
#[test]
fn a_tree_less_object_is_buried_by_its_op() {
    let mut session = Session::new("loose");
    session.add_mesh(Mesh::create_box(1.0, 1.0, 1.0), None);
    session.node_lookup.clear();
    session.tree = session_rust::Tree::new("loose");
    let mut scene = Scene::new();
    scene.add_file(file("loose", session, Xform::identity()));
    scene.settle();
    assert!(scene.delete_row(0));
    check(&mut scene);
    assert_eq!(scene.tombs.len(), 1);
    assert!(scene.undo());
    scene.sync();
    assert_eq!(scene.staged.unbury.len(), 1);
    assert!(scene.tables_empty(), "nothing uploaded");
    scene.settle();
    scene.verify();
}

/// A thousand boxes deleted together hide in one step and come back in one, nothing uploaded either way.
#[test]
fn a_thousand_boxes_delete_and_undo_in_place() {
    let mut session = Session::new("boxes");

    for i in 0..1000 {
        let node = session
            .add_mesh(Mesh::create_box(1.0, 1.0, 1.0), None)
            .unwrap();
        let guid = node.borrow().name.clone();
        session.set_xform(
            &guid,
            Xform::translation(f64::from(i % 40) * 2.0, f64::from(i / 40) * 2.0, 0.0),
        );
    }

    let mut scene = Scene::new();
    scene.add_file(file("boxes", session, Xform::identity()));
    scene.settle();
    let rows: Vec<u32> = live(&scene).into_iter().map(|(row, _)| row).collect();
    assert_eq!(rows.len(), 1000);
    assert_eq!(scene.delete_rows(&rows), 1000);
    scene.sync();
    assert_eq!(scene.staged.bury.len(), 1000);
    assert!(scene.staged.kills.is_empty() && scene.tables_empty());
    scene.settle();
    scene.verify();
    assert_eq!(scene.object_count(), 0);

    assert!(scene.undo());
    scene.sync();
    assert_eq!(scene.staged.unbury.len(), 1000);
    assert!(
        scene.staged.patches.is_empty() && scene.tables_empty(),
        "nothing uploaded"
    );
    scene.settle();
    scene.verify();
    assert_eq!(scene.object_count(), 1000);
    assert!(!scene.undo(), "one step");
}

/// Moving a parent moves its unselected children too.
#[test]
fn moving_a_parent_moves_its_children() {
    let mut scene = scene();
    let parent = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
    let (doc, guid) = scene.identity_of(parent).unwrap();
    let child = scene.docs[doc]
        .session
        .tree
        .get_node_by_name(&guid)
        .unwrap()
        .borrow()
        .children()[0]
        .borrow()
        .name
        .clone();
    let child = scene.row_of(doc, &child).unwrap();
    scene.transform_rows(&[parent], &Xform::translation(0.0, 7.0, 0.0), "move");
    scene.sync();
    let moved: Vec<u32> = scene.staged.places.iter().map(|(row, _)| *row).collect();
    assert!(
        moved.contains(&parent) && moved.contains(&child),
        "{moved:?}"
    );
    scene.settle();
    scene.verify();
}

/// No per-row preview table: only a dragged row holds preview memory.
#[test]
fn per_row_tables_are_small() {
    let mut scene = scene();
    assert_eq!(scene.preview_cache_bytes(), 0);
    let mesh = find(&scene, |g| matches!(g, Geometry::Mesh(_))).unwrap();
    scene.capture_preview(mesh);
    assert!(scene.mesh_preview(mesh).is_some());
    let held = scene.preview_cache_bytes();
    assert!(
        held > 0 && held < 64 * 1024,
        "{held} bytes for one small mesh"
    );
    scene.drop_preview();
    assert_eq!(scene.preview_cache_bytes(), 0);
    assert!(
        std::mem::size_of::<Footprint>() + std::mem::size_of::<Weak<RefCell<TreeNode>>>() <= 24
    );
}
