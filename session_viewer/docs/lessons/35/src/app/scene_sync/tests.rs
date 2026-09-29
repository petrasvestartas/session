use super::*;
use crate::app::scene::{FileDoc, SheetInit, StreamedInit};
use crate::app::stream::{CloudFields, CloudLod, SheetFields};
use crate::app::walk::cloud::StreamRows;
use crate::app::walk::sheet::SheetRows;
use session_rust::element::ElementFeature;
use session_rust::{Element, Line, Mesh, NurbsCurve, Point, Polyline};

/// A document placed at `place`.
pub(super) fn file(name: &str, session: Session, place: Xform) -> FileDoc {
    FileDoc {
        name: name.into(),
        session: Rc::new(session),
        place,
        point_px: 0.0,
        display_only: false,
    }
}

/// A point.
pub(super) fn p(x: f64, y: f64, z: f64) -> Point {
    Point::new(x, y, z)
}

/// A curve through eight points of an arc: trimming it changes its sample count.
pub(super) fn arc() -> NurbsCurve {
    let points: Vec<Point> = (0..8)
        .map(|i| {
            let angle = i as f64 * 0.4;
            p(10.0 * angle.cos(), 10.0 * angle.sin(), 0.0)
        })
        .collect();
    NurbsCurve::create(false, 3, &points)
}

/// Nested placed groups, a child under a placed parent, each curve kind and an element with baked features.
fn site() -> Session {
    let mut session = Session::new("site");
    let walls = session.add_group("walls");
    session.set_xform("walls", Xform::translation(0.0, 0.0, 3.0));
    let inner = TreeNode::new("inner");
    session.add(&inner, Some(&walls));
    session.set_xform("inner", Xform::rotation_z(30.0, true));
    let parent = session.add_point(p(0.0, 0.0, 0.0), Some(&walls));
    let parent_guid = parent.borrow().name.clone();
    session.set_xform(&parent_guid, Xform::translation(1.0, 0.0, 0.0));
    let child = session.add_line(Line::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0), Some(&parent));
    let child_guid = child.borrow().name.clone();
    session.set_xform(&child_guid, Xform::translation(0.0, 2.0, 0.0));
    session.add_polyline(
        Polyline::new(vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(1.0, 1.0, 0.0)]),
        Some(&inner),
    );
    session.add_nurbscurve(arc(), Some(&inner));
    session.add_line(Line::new(-5.0, 0.0, 0.0, 5.0, 0.0, 0.0), None);
    let mut element = Element::new("beam");
    element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
    let axis = Polyline::new(vec![p(0.0, 0.0, 0.0), p(100.0, 0.0, 0.0)]);
    element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
    let beam = session.add_element(element, None);
    let attributes = TreeNode::new("attributes");
    session.add(&attributes, Some(&beam));
    session.add_polyline(
        Polyline::new(vec![p(0.0, 0.0, 0.0), p(0.0, 0.0, 20.0)]),
        Some(&attributes),
    );
    session.add_mesh(Mesh::create_box(2.0, 2.0, 2.0), Some(&inner));
    session.add_group("roof");
    session
}

/// A second document with a layer of its own.
pub(super) fn other() -> Session {
    let mut session = Session::new("other");
    let inbox = session.add_group("inbox");
    session.add_point(p(3.0, 3.0, 3.0), Some(&inbox));
    session
}

/// A streamed sheet and a streamed cloud, read-only shells.
pub(super) fn shells(scene: &mut Scene) {
    scene.stream_sheet(SheetInit {
        name: "plan".into(),
        url: "plan.pb".into(),
        meta_url: None,
        place: Xform::identity(),
        rows: SheetRows {
            positions: vec![
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0,
            ],
            colors: Vec::new(),
            widths: Vec::new(),
            ids: vec![7, 8],
        },
        fields: SheetFields {
            count: 2,
            ..Default::default()
        },
        resident: 2,
    });
    scene.stream_cloud(StreamedInit {
        name: "scan".into(),
        url: "scan.pb".into(),
        place: Xform::translation(0.0, 0.0, 5.0),
        rows: StreamRows {
            positions: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            colors: vec![u32::MAX; 2],
            normals: Vec::new(),
        },
        lod: CloudLod::default(),
        fields: CloudFields {
            end: 0,
            coords_at: 0,
            coords_len: 0,
            colors_at: 0,
            colors_len: 0,
            normals_at: 0,
            normals_len: 0,
            count: 2,
            ids_at: 0,
            ids_len: 0,
            revision: None,
        },
        resident: 2,
        point_px: 3.0,
        col_at: 0,
        ceiling: 2,
    });
}

/// Two placed documents, one shared by two placements, and the streamed shells.
pub(super) fn scene() -> Scene {
    let mut scene = Scene::new();
    scene.add_file(file("site", site(), Xform::translation(100.0, 0.0, 0.0)));
    shells(&mut scene);
    scene.add_file(file("other", other(), Xform::identity()));
    let shared = Rc::new(site());

    for x in [500.0, 900.0] {
        scene.add_file(FileDoc {
            name: format!("shared {x}"),
            session: Rc::clone(&shared),
            place: Xform::translation(x, 0.0, 0.0),
            point_px: 0.0,
            display_only: false,
        });
    }

    scene.settle();
    scene.verify();
    scene
}

/// Sync, flush without a GPU, compare with a fresh scene.
pub(super) fn check(scene: &mut Scene) {
    scene.sync();
    scene.settle();
    scene.verify();
}

/// Live rows of editable documents, by identity.
pub(super) fn live(scene: &Scene) -> Vec<(u32, (usize, Rc<str>))> {
    (0..scene.row_count() as u32)
        .filter_map(|row| {
            let id = scene.identity_of(row)?;
            scene
                .docs
                .get(id.0)
                .is_some_and(|file| !file.display_only)
                .then_some((row, id))
        })
        .collect()
}

/// The first live row whose geometry passes `test`.
pub(super) fn find(scene: &Scene, test: impl Fn(&Geometry) -> bool) -> Option<u32> {
    live(scene)
        .into_iter()
        .map(|(row, _)| row)
        .find(|&row| scene.geometry(row).is_some_and(&test))
}

/// A tiny deterministic random source.
pub(super) struct Dice(pub(super) u64);

impl Dice {
    /// A number below `n`.
    pub(super) fn roll(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 33) as usize) % n.max(1)
    }
}

/// Hide or color one live object as the panel does: its identity and its row.
pub(super) fn mark(scene: &mut Scene, dice: &mut Dice) {
    let rows: Vec<u32> = live(scene).into_iter().map(|(row, _)| row).collect();

    if rows.is_empty() {
        return;
    }

    let row = rows[dice.roll(rows.len())];
    let id = scene.identity_of(row).unwrap();
    let held = scene.ledger.get_mut(&row).unwrap();

    if dice.roll(2) == 0 {
        scene.hidden.insert(id);
        held.flags |= Instance::FLAG_HIDDEN;
    } else {
        scene.colors.insert(id, [200, 40, 40]);
        held.flags |= Instance::FLAG_COLOR;
    }
}
