use crate::{chain::{PreparedChain, Source}, scene::Scene, memory};
use session_rust::{NurbsCurve, Point, Xform};
use std::rc::Rc;

#[test]
fn curve_rows_share_original_sources_and_samples_with_distinct_ids() {
    let curve = NurbsCurve::create(false, 2, &[Point::new(-0.6, -0.4, 0.0), Point::new(0.123456789012345, 0.6, 0.0), Point::new(0.6, -0.4, 0.0)]);
    let source = Rc::new(curve); let weak = Rc::downgrade(&source); let guid = source.guid().to_owned();
    let prepared = PreparedChain::curve(source).unwrap(); let Source::Curve(_, samples) = &prepared.source else { panic!("Original curve source") };
    let sample_weak = Rc::downgrade(samples); let mut scene = Scene::demo(); scene.clear();
    let first = scene.insert_path(prepared.clone()).unwrap(); let second = scene.insert_path(prepared).unwrap(); assert_ne!(first, second);
    assert_eq!(scene.paths()[0].guid, guid); assert_ne!(scene.paths()[1].guid, guid);
    assert_eq!(&memory::curves([&scene])[..3], &[2, 1, 1]);
    let controls = scene.paths()[0].prepared.source.controls(); assert_eq!(controls[1].index, 1);
    assert_eq!(controls[1].position().unwrap()[0], 0.123456789012345);
    scene.place_path(first, Xform::translation(2.0, 3.0, 4.0)).unwrap();
    let bounds = scene.bounds().unwrap(); assert!(bounds.max[1] >= 3.6); assert!(bounds.max[2] >= 4.0);
    assert!(scene.paths()[0].segments().len() > 20); assert_eq!(controls[1].source.m_cv[3], 0.123456789012345);
    drop(controls); scene.clear(); assert!(weak.upgrade().is_none()); assert!(sample_weak.upgrade().is_none()); assert_eq!(memory::curves([&scene]), [0; 4]);
}
