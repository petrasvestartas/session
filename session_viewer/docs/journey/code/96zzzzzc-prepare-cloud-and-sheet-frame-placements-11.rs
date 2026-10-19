  let row = &draw.row;
  let local=crate::render_space::placed(&row.model,view.anchor);
  let model=local.to_f32();