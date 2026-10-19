use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
    let model = scene.placement(id).ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let mut source = outline.clone();
                    source.set_guid(uuid::Uuid::new_v4().to_string());
                    source.name = format!("piece {}", part + 1);
                    source.linecolor = session_rust::Color::black();
                    let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                    scene.place_following(piece, model.clone())?;
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(format!("piece {}", part + 1), piece));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(crate::feature_plan::label(feature), outlines));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() { groups.push(FeatureTree::Group("attributes".into(), own)); }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() { hosted.push(FeatureTree::Group(relation.label, children)); }
        }
        if !hosted.is_empty() { groups.push(FeatureTree::Group("interactions".into(), hosted)); }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
