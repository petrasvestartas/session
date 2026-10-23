fill.display=fill.display.with_normals(vec![normal.map(|v|v as f32);count])?.with_alphas(vec![1./255.;count])?;
  let mut edge=outline.clone();edge.linecolor=Color::black();edge.width=1.;edge.arrowhead=session_rust::Arrowhead::NONE;
  let mut segments=crate::production_curve_walk::SegRows::default();let mut heads=crate::production_curve_walk::VectorRows::default();
  crate::production_curve_walk::walk_polyline(&mut segments,&mut heads,&edge,0);
  let facing=crate::production_mesh_ink::pack_facing(Some(&normal.map(|v|v as f32 as f64)),None);
  for segment in &mut segments.ribbons{segment.color=0x01000000;segment.facing=facing;}
  let rows=crate::production_curve_walk::joined_rows(&segments.ribbons,&segments.ribbon_chains,&[],0,&[]);
  fill.display.parts.ink.segments=rows;fill.display.parts.ink.ids=vec![u32::MAX;segments.ribbons.len()];