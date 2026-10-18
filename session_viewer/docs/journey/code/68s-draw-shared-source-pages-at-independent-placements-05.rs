  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
  let px = row.point_px;
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
