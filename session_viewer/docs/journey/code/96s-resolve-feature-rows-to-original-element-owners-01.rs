use crate::{hierarchy::Document, scene::{ObjectId, Scene}};

fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    let mut parent = doc.nodes[index].parent;
    while let Some(index) = parent {
        let node = &doc.nodes[index];
        if let Some(id) = node.object {
            if scene.objects().iter().any(|row| row.id == id
                && matches!(row.original.as_ref(), Some(session_rust::Geometry::Element(_)))) {
                return scene.pick_id(id.value());
            }
        }
        parent = node.parent;
    }
    None
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
    let (document, guid) = key.split_once('/')?;
    let document = document.parse::<u32>().ok()?;
    let doc = scene.forest.documents.iter().find(|doc| doc.key == document)?;
    let index = doc.nodes.iter().position(|node| node.source.guid == guid)?;
    if !doc.nodes[index].readonly { return None; }
    ancestor(scene, doc, index)
}

pub fn highlighted(scene: &Scene, selected: &[ObjectId]) -> Vec<ObjectId> {
    let mut ids = selected.to_vec();
    for doc in &scene.forest.documents {
        for (index, node) in doc.nodes.iter().enumerate() {
            if !node.derived { continue; }
            let Some(id) = node.object.filter(|id| scene.visible(*id)) else { continue; };
            if ancestor(scene, doc, index).is_some_and(|owner| selected.contains(&owner)) {
                ids.push(id);
            }
        }
    }
    ids.sort_unstable_by_key(|id| id.value());
    ids.dedup();
    ids
}
