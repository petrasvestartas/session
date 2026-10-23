    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    Ok(key)})?;
    let titles=candidate.matrices().iter().filter(|row|candidate.forest.documents.iter().find(|doc|doc.key==key).is_some_and(|doc|doc.nodes.iter().any(|node|node.object==Some(row.id))))
        .filter_map(|row|crate::production_matrix::title(&row.source,&row.model)).collect::<Vec<_>>();
    for title in titles {candidate.insert_text(crate::scene_label::Prepared::new(Rc::new(title)).map_err(str::to_owned)?).map_err(str::to_owned)?;}
    scene.restore(candidate);Ok(key)
}

