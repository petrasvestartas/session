use crate::{camera::{Camera, Projection, View}, cplane::CPlane, editor::{Action, Editor}, scene::Scene};

fn pose(camera: &Camera) -> serde_json::Value {
    let axes = [session_rust::Vector::x_axis(), session_rust::Vector::y_axis(), session_rust::Vector::z_axis()]
        .map(|axis| {let v = camera.orientation.rotate_vector(axis); [v[0],v[1],v[2]]});
    serde_json::json!({"target":camera.target,"distance":camera.distance,"axes":axes,
        "projection":format!("{:?}",camera.projection),"aspect":camera.aspect,"frame":camera.frame})
}

#[test]
fn shared_publication_adoption_keeps_navigation_that_happened_after_loading_started() {
    for live in [false, true] {
        let shared = crate::drawing_document::new();
        let ready = {
            let editor = shared.borrow();
            crate::publication::prepare(&editor.scene, &crate::specimen::bytes(), true, None).unwrap()
        };
        let at_start = pose(&shared.borrow().camera);
        {
            let mut editor = shared.borrow_mut();
            editor.apply(Action::NamedView(View::Front)).unwrap();
            editor.apply(Action::Orbit(0.2, -0.1)).unwrap();
            editor.apply(Action::Pan(0.1, 0.2)).unwrap();
            editor.apply(Action::Zoom(1.25)).unwrap();
            editor.apply(Action::Projection(Projection::Orthographic)).unwrap();
        }
        let latest = pose(&shared.borrow().camera);
        assert_ne!(latest, at_start);
        let action = if live {Action::Live(Box::new(ready))} else {Action::Adopt {scene:Box::new(ready),replace:true}};
        shared.borrow_mut().apply(action).unwrap();
        assert_eq!(pose(&shared.borrow().camera), latest, "Delivery uses the current shared Editor");
        assert!(!shared.borrow().scene.objects().is_empty());
    }
}

#[test]
fn displayed_grid_changes_only_when_an_explicit_plane_command_resolves_a_new_frame() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    assert_eq!(editor.grid_plane(), CPlane::Xy);
    editor.apply(Action::NamedView(View::Front)).unwrap();
    assert_eq!(editor.drawing_plane(), CPlane::Xz);
    assert_eq!(editor.grid_plane(), CPlane::Xy);
    editor.apply(Action::ConstructionPlane(None)).unwrap();
    assert_eq!(editor.grid_plane(), CPlane::Xz);
    assert_eq!(editor.construction_plane, None);
    editor.apply(Action::NamedView(View::Top)).unwrap();
    assert_eq!(editor.drawing_plane(), CPlane::Xy);
    assert_eq!(editor.grid_plane(), CPlane::Xz);
    editor.apply(Action::ConstructionPlane(None)).unwrap();
    assert_eq!(editor.grid_plane(), CPlane::Xy);
    let far = CPlane::Frame {origin:[1e12+0.125,-1e12+0.25,1e12+0.5],x:[1.0,0.0,0.0],y:[0.0,0.0,1.0]};
    editor.apply(Action::ConstructionPlane(Some(far))).unwrap();
    for action in [Action::NamedView(View::Right), Action::ResetView, Action::Orbit(0.2,0.1), Action::Pan(0.1,0.2), Action::Zoom(1.5), Action::Live(Box::new(Scene::empty())), Action::Close] {
        editor.apply(action).unwrap();
        assert_eq!(editor.grid_plane(), far);
    }
    editor.apply(Action::Plane3).unwrap();
    editor.apply(Action::ToolPoint([0.0,0.0,0.0])).unwrap();
    editor.apply(Action::ToolPoint([1.0,0.0,0.0])).unwrap();
    assert!(editor.apply(Action::ToolPoint([2.0,0.0,0.0])).is_err());
    assert_eq!(editor.grid_plane(), far);
    editor.apply(Action::CancelTool).unwrap();
    assert_eq!(editor.grid_plane(), far);
}

#[test]
fn far_grid_rebasing_preserves_its_world_frame_and_sub_float_offsets() {
    let plane = CPlane::Frame {origin:[1e12+0.125,-1e12+0.25,1e12+0.5],x:[1.0,0.0,0.0],y:[0.0,0.0,1.0]};
    let settings = crate::grid::Settings {spacing:0.125,half_cells:2};
    let original = settings.strokes().unwrap();
    for anchor in [[1e12,-1e12,1e12], [1e12+0.0625,-1e12+0.125,1e12+0.25], [1e12,-1e12,1e12]] {
        let rebased = settings.strokes_on_at(plane, anchor).unwrap();
        for (local, actual) in original.iter().zip(&rebased) {
            for (point, actual) in [(local.start,actual.start),(local.end,actual.end)] {
                let expected = plane.world(point.map(f64::from));
                let recovered: [f64;3] = std::array::from_fn(|axis| anchor[axis]+f64::from(actual[axis]));
                assert_eq!(recovered, [expected[0],expected[1],expected[2]]);
            }
        }
    }
}

#[test]
fn repeated_grid_commands_and_camera_changes_reuse_gpu_rows_but_a_rebase_updates_coordinates() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    let settings = crate::grid::Settings {spacing:0.125,half_cells:2};
    editor.apply(Action::Grid(Some(settings))).unwrap();
    let mut renderer = crate::cap_gpu_tests::renderer(&editor.scene);
    renderer.set_grid_on(editor.camera.grid, editor.grid_plane()).unwrap();
    let original = renderer.grid_usage();
    for action in [Action::ConstructionPlane(Some(CPlane::Xy)), Action::NamedView(View::Front), Action::Orbit(0.1,0.2), Action::ResetView, Action::Live(Box::new(Scene::empty()))] {
        editor.apply(action).unwrap();
        renderer.set_grid_on(editor.camera.grid, editor.grid_plane()).unwrap();
        assert_eq!(renderer.grid_usage(), original);
    }
    let far = CPlane::Frame {origin:[1e12+0.125,-1e12+0.25,1e12+0.5],x:[1.0,0.0,0.0],y:[0.0,0.0,1.0]};
    editor.apply(Action::ConstructionPlane(Some(far))).unwrap();
    renderer.set_grid_on(editor.camera.grid, editor.grid_plane()).unwrap();
    let changed = renderer.grid_usage();
    assert!(changed[2] > original[2]);
    editor.apply(Action::ConstructionPlane(Some(far))).unwrap();
    renderer.set_grid_on(editor.camera.grid, editor.grid_plane()).unwrap();
    assert_eq!(renderer.grid_usage(), changed);
    editor.camera.target = [1e12,-1e12,1e12];
    renderer.prepare_origin(&editor.camera, &editor.scene, None, &[], 1000.0).unwrap();
    assert_eq!(editor.grid_plane(), far);
    assert_eq!(renderer.anchor(), editor.camera.target);
    assert!(renderer.grid_usage()[2] > changed[2], "Only relative GPU coordinates change on rebase");
    let rebased = renderer.grid_usage();
    renderer.set_grid_on(editor.camera.grid, editor.grid_plane()).unwrap();
    assert_eq!(renderer.grid_usage(), rebased);
    editor.camera.target = [0.0; 3];
    renderer.prepare_origin(&editor.camera, &editor.scene, None, &[], 2000.0).unwrap();
    assert_eq!(renderer.anchor(), [0.0; 3]);
    assert_eq!(editor.grid_plane(), far);
    assert!(renderer.grid_usage()[2] > rebased[2]);
}
