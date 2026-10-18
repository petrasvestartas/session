                    if feature.feature_type=="contact" && let Some(mut fill)=contact_fill(outline)? {
                        fill.model=model.clone();let id=scene.insert(fill)?;
                        outlines.push(FeatureTree::Piece(format!("piece {}",part+1),id));continue;
                    }
                    let point = outline.get_point(0)