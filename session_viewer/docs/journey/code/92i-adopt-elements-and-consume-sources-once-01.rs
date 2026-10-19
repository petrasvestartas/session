use crate::scene::{Scene, ObjectId};
use crate::element_input::Made;
use std::rc::Rc;

pub fn adopt(scene: &mut Scene, made: Vec<Made>, consumed: &[ObjectId]) -> Result<Vec<ObjectId>, &'static str> {
    let mut ids = Vec::new();
    for item in made {
        let id = crate::element_original::insert(scene, item.source.clone())?;
        ids.push(id);
        let mut attributes = Vec::new();
        let mut interactions = Vec::new();
        if let Some(plane) = item.plane {
            attributes.push(crate::plane_original::insert(scene, Rc::new(plane))?);
        }
        // Own axes and sections are attributes; contact, joint, cut and drill geometry comes from interactions.
        for feature in &item.source.features {
            let interaction = matches!(feature.feature_type.as_str(), "contact" | "joint" | "cut" | "drill");
            for outline in &feature.outlines {
                let mut source = outline.clone();
                source.set_guid(uuid::Uuid::new_v4().to_string());
                source.name = format!("{} {}", feature.feature_type, feature.name);
                let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                if interaction { interactions.push(piece); } else { attributes.push(piece); }
            }
        }
        if !attributes.is_empty() {
            Rc::make_mut(&mut scene.view).hide(&attributes, true);
            crate::layer_edit::attach_attributes(scene, id, attributes, "attributes")?;
        }
        if !interactions.is_empty() {
            crate::layer_edit::attach_attributes(scene, id, interactions, "interactions")?;
        }
    }
    for id in scene.with_descendants(consumed) { scene.remove(id); }
    Ok(ids)
}

fn group_ids(scene: &Scene, name: &str) -> Vec<ObjectId> {
    scene.forest.documents.iter().flat_map(|doc| {
        doc.nodes.iter().enumerate().filter(move |(_, node)| node.source.name == name && node.readonly)
            .flat_map(|(index, node)| doc.nodes[index + 1..node.end].iter().filter_map(|node| node.object))
    }).collect()
}
pub fn attribute_ids(scene: &Scene) -> Vec<ObjectId> { group_ids(scene, "attributes") }
pub fn interaction_ids(scene: &Scene) -> Vec<ObjectId> { group_ids(scene, "interactions") }
