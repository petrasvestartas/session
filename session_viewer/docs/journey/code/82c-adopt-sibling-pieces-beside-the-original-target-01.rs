    }
}
pub fn adopt(
    scene: &mut Scene,
    target: ObjectId,
    pieces: Vec<Geometry>,
) -> Result<(), &'static str> {
    let mut ids = vec![target];
    for _ in 1..pieces.len() {
        ids.extend(crate::layer_edit::copy_objects(
            scene,
            &[target],
            &Xform::identity(),
        )?);
    }
    for (id, piece) in ids.into_iter().zip(pieces) {
        let label = piece.name().to_owned();
        scene.replace_original(id, piece)?;
        for index in 0..scene.forest.documents.len() {
            if scene.forest.documents[index]
                .nodes
                .iter()
                .any(|n| n.object == Some(id))
            {
                let mut doc = (*scene.forest.documents[index]).clone();
                for node in &mut doc.nodes {
                    if node.object == Some(id) {
                        node.label = label.clone();
                    }
                }
                scene.forest.replace(index, doc);
                break;
            }
        }
    }
    Ok(())
}
