  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
  let px = if row.point_px > 0. { row.point_px } else { source.point_size as f32 };
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
