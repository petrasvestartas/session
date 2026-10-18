                    let point = outline.get_point(0).filter(|p| outline.coords.chunks_exact(3)
                        .all(|q| [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32()));
                    let piece = if let Some(mut point) = point {
                        point.set_guid(uuid::Uuid::new_v4().to_string());
                        point.name = format!("piece {}", part + 1);
                        point.pointcolor = outline.linecolor.clone();
                        point.width = 12.0;
                        point.is_visible = true;
                        point.is_locked = false;
                        let piece = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point))?)?;
                        scene.place_point(piece, model.clone())?;
                        piece
                    } else {
                        let mut source = outline.clone();
                        source.set_guid(uuid::Uuid::new_v4().to_string());
                        source.name = format!("piece {}", part + 1);
                        source.width = 2.0;
                        source.is_visible = true;
                        source.is_locked = false;
                        source.arrowhead = session_rust::Arrowhead::NONE;
                        let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                        scene.place_following(piece, model.clone())?;
                        piece
                    };