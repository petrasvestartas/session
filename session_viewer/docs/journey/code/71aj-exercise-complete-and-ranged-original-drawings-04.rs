use crate::{
    editor::{Action, Editor},
    scene::Scene,
    sheet_source::Prepared,
};
use prost::Message;
use session_rust::{Xform, proto};
use std::rc::Rc;
#[test]
fn sheet_import_copy_move_save_and_reopen_keep_original_columns_namespaces_and_flags() {
    let mut original = crate::sheet_example::source();
    original.coords[0] = 0.123456789012345;
    let before = original.clone();
    let source = proto::Session {
        objects: Some(proto::Objects {
            sheets: vec![original],
            points: vec![{
                let point = session_rust::Point::new(0., 0., 0.);
                let _ = point.guid();
                point.to_proto()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &source.encode_to_vec(),
        "mixed",
        &Xform::translation(1_000_000_000.125, 0., 0.),
        0.,
    )
    .unwrap();
    assert_eq!(scene.sheets().len(), 1);
    assert_eq!(scene.points().len(), 1);
    let id = scene.sheets()[0].id;
    let copy = scene.copy_row(id, false).unwrap();
    scene
        .place_sheet(copy, Xform::translation(0., 2., 0.))
        .unwrap();
    assert!(Rc::ptr_eq(
        &scene.sheets()[0].prepared,
        &scene.sheets()[1].prepared
    ));
    assert_ne!(scene.row_guid(id), scene.row_guid(copy));
    Rc::make_mut(&mut scene.view).hide(&[id], true);
    Rc::make_mut(&mut scene.view).lock(&[copy], true);
    let saved = crate::scene_archive::save(&scene).unwrap();
    let bundle = crate::archive::Bundle::decode(&saved).unwrap();
    let sheets = bundle
        .documents
        .iter()
        .flat_map(|bytes| {
            proto::Session::decode(bytes.as_slice())
                .unwrap()
                .objects
                .unwrap()
                .sheets
        })
        .collect::<Vec<_>>();
    assert_eq!(sheets.len(), 2);
    assert!(sheets.iter().all(|sheet| sheet.coords == before.coords
        && sheet.colors == before.colors
        && sheet.widths == before.widths
        && sheet.source_ids == before.source_ids
        && sheet.meta == before.meta));
    let mut reopened = Scene::empty();
    crate::scene_document::open(&mut reopened, &saved).unwrap();
    assert_eq!(reopened.sheets().len(), 2);
    assert_eq!(reopened.points().len(), 1);
    assert_eq!(reopened.view.hidden.len(), 1);
    assert_eq!(reopened.view.locked.len(), 1);
    assert!(
        reopened
            .sheets()
            .iter()
            .any(|row| row.model.m[12] == 1_000_000_000.125)
    );
    assert!(reopened.sheets().iter().any(|row| row.model.m[13] == 2.));
    assert_eq!(*scene.sheets()[0].prepared.original, before);
}
#[test]
fn rejected_sheet_coordinates_do_not_replace_the_scene_and_close_releases_history_owners() {
    let mut editor = Editor::default();
    let ids = editor.scene.all_ids();
    let mut invalid = crate::sheet_example::source();
    invalid.coords[0] = f64::NAN;
    let bad = proto::Session {
        objects: Some(proto::Objects {
            sheets: vec![invalid],
            ..Default::default()
        }),
        ..Default::default()
    }
    .encode_to_vec();
    assert!(crate::publication::prepare(&editor.scene, &bad, true, None).is_err());
    assert_eq!(editor.scene.all_ids(), ids);
    let prepared =
        crate::publication::prepare(&editor.scene, &crate::sheet_example::bytes(), true, None)
            .unwrap();
    editor
        .apply(Action::Adopt {
            scene: Box::new(prepared),
            replace: true,
        })
        .unwrap();
    let weak = Rc::downgrade(&editor.scene.sheets()[0].prepared.original);
    let page = Rc::downgrade(&editor.scene.sheets()[0].prepared.page);
    let id = editor.scene.sheets()[0].id;
    editor.select_objects([id], false);
    editor.apply(Action::Translate([1., 0., 0.])).unwrap();
    assert_eq!(editor.scene.sheets()[0].model.m[12], 1.);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.sheets()[0].model.m[12], 0.);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.sheets()[0].model.m[12], 1.);
    editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert!(page.upgrade().is_none());
}
#[test]
fn repeated_published_sheet_placements_share_original_display_and_resolve_metadata_relative_to_source()
 {
    let mut source = Scene::empty();
    let id = source
        .insert_sheet(Rc::new(
            Prepared::new(Rc::new(crate::sheet_example::source())).unwrap(),
        ))
        .unwrap();
    source.sheet_url("https://example.test/drawings/plan.pb");
    let manifest =
        crate::manifest::Manifest::parse(b"items: [{file: plan.pb}, {file: plan.pb, at: [2,0,0]}]")
            .unwrap();
    let result = crate::scene_manifest::prepare(
        &Scene::empty(),
        &manifest,
        &["plan.pb".into(), "plan.pb".into()],
        &[("plan.pb".into(), source)].into_iter().collect(),
    )
    .unwrap();
    assert_eq!(result.sheets().len(), 2);
    assert!(Rc::ptr_eq(
        &result.sheets()[0].prepared,
        &result.sheets()[1].prepared
    ));
    assert_eq!(result.sheets()[1].model.m[12], 2.);
    assert_eq!(
        result.sheets()[0].url.as_deref(),
        Some("https://example.test/drawings/plan.pb")
    );
    assert_ne!(result.sheets()[0].id, result.sheets()[1].id);
    let _ = id;
    let raw = crate::sheet_example::metadata();
    let count = crate::sheet_entity::table_count(&raw[..8]).unwrap();
    let at = crate::sheet_entity::record_at(1) as usize;
    let (offset, length) =
        crate::sheet_entity::blob_range(count, 1, &raw[at..at + 16], raw.len() as u64).unwrap();
    let entity =
        crate::sheet_entity::entity_from(&raw[offset as usize..][..length as usize]).unwrap();
    assert_eq!(entity.name, "Plan boundary");
    assert_eq!(entity.guid, "source-wall");
}
#[test]
fn streamed_sheet_partial_sources_refuse_editable_save_and_preserve_original_entity_columns() {
    let budget = crate::page_cache::Budget::new(4096);
    let source = Rc::new(crate::sheet_range::Source {
        guid: crate::sheet_example::source().guid,
        name: "ranged plan".into(),
        url: "https://example.test/plan.pb".into(),
        fields: crate::stream::SheetFields {
            count: 10,
            entities: u32::MAX,
            meta: "plan.shm".into(),
            revision: Some("v1".into()),
            ..Default::default()
        },
    });
    let stream = crate::sheet_stream::Stream::new(source, budget.clone());
    let raw = [-0.5f64, 0., 0.3, 0.5, 0., 0.3]
        .into_iter()
        .flat_map(f64::to_le_bytes)
        .collect::<Vec<_>>();
    let ids = (u32::MAX - 1).to_le_bytes();
    let page =
        crate::sheet_page::Page::decode(7..8, 10, u32::MAX, &raw, None, None, Some(&ids)).unwrap();
    drop(stream.adopt(page, budget.reserve(192).unwrap()).unwrap());
    let mut scene = Scene::empty();
    let prepared = Rc::new(Prepared::streamed(stream.clone()));
    let id = scene.insert_sheet(prepared).unwrap();
    assert!(scene.sheets()[0].has_entity(u32::MAX - 1));
    assert!(!scene.sheets()[0].has_entity(7));
    let picks = crate::pick_sources::Sources::new(&scene).unwrap();
    assert_eq!(
        picks.edges.last().unwrap().part,
        crate::pick_sources::Part::SheetEntity(u32::MAX - 1)
    );
    assert!(
        crate::scene_archive::save(&scene)
            .unwrap_err()
            .contains("complete drawing")
    );
    let copy = scene.copy_row(id, false).unwrap();
    scene
        .place_sheet(copy, Xform::translation(1., 0., 0.))
        .unwrap();
    assert_eq!(scene.sheets()[1].bounds().unwrap().min[0], 0.5);
    stream.pages.borrow_mut().clear();
    assert_eq!(budget.used(), 0);
    assert!(!scene.sheets()[0].has_entity(u32::MAX - 1));
}
#[test]
fn streamed_sheet_revision_inventory_reuses_checked_sources_and_repeated_manifest_placements() {
    let budget = crate::page_cache::Budget::new(4096);
    let source = Rc::new(crate::sheet_range::Source {
        guid: crate::sheet_example::source().guid,
        name: "original drawing".into(),
        url: "plan.pb".into(),
        fields: crate::stream::SheetFields {
            count: 5,
            entities: 3,
            revision: Some("v1".into()),
            ..Default::default()
        },
    });
    let stream = crate::sheet_stream::Stream::new(source, budget);
    stream.loaded.set(2);
    let prepared = Rc::new(Prepared::streamed(stream));
    let mut index = crate::publication_index::Index::new("".into());
    index
        .manifest(
            "scene",
            None,
            b"items: [{file: plan.pb},{file: plan.pb,at: [2,0,0]}]",
        )
        .unwrap();
    assert!(index.stream_sheet("plan.pb", prepared.clone()).unwrap());
    assert_eq!(index.streamed_segments(), 2);
    let scene = index.replacement(&Scene::empty()).unwrap().unwrap();
    assert_eq!(scene.sheets().len(), 2);
    assert!(Rc::ptr_eq(
        &scene.sheets()[0].prepared,
        &scene.sheets()[1].prepared
    ));
    assert_eq!(scene.sheets()[1].model.m[12], 2.);
    assert!(!index.stream_sheet("plan.pb", prepared).unwrap());
    assert!(index.replacement(&scene).unwrap().is_none());
}
#[test]
fn appending_a_published_drawing_preserves_the_metadata_base_of_existing_originals() {
    let original = crate::sheet_example::bytes();
    let first = crate::publication::prepare(
        &Scene::empty(),
        &original,
        true,
        Some(Rc::new(crate::reload_url::ReloadUrl::new(
            "https://example.test/first/plan.pb".into(),
        ))),
    )
    .unwrap();
    let second = crate::publication::prepare(
        &first,
        &original,
        false,
        Some(Rc::new(crate::reload_url::ReloadUrl::new(
            "https://example.test/second/plan.pb".into(),
        ))),
    )
    .unwrap();
    assert_eq!(
        second.sheets()[0].url.as_deref(),
        Some("https://example.test/first/plan.pb")
    );
    assert_eq!(
        second.sheets()[1].url.as_deref(),
        Some("https://example.test/second/plan.pb")
    );
    assert_ne!(second.sheets()[0].id, second.sheets()[1].id);
}
