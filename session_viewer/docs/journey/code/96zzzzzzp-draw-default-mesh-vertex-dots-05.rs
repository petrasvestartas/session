    let mut dots=Vec::new();let mut colours=Vec::new();let pc=source.get_pointcolors();
    let coloured=source.color_mode==session_rust::mesh::ColorMode::POINTCOLORS&&pc.len()==keys.len();
    for(slot,edge)in vertices.into_iter().enumerate(){if edge!=u32::MAX{dots.push(edge);if coloured{colours.push(pc[slot].to_f32());}}}
    if colours.first().is_some_and(|c|colours.iter().all(|v|v==c)){colours=vec![colours[0]];}
    (topology.edges.into_iter().map(|(a,b,_)|(a,b)).collect(),pens,dots,colours)