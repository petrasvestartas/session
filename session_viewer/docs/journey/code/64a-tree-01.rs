  pub bounds: Rc<crate::bounds::Bounds>,
  pub spacing: f32,
}

fn validate_nodes(source: &PointCloud) -> Result<Vec<LodNode>, &'static str> {
  let p = source.to_proto();
  let n = p.lod_size.len();
  if p.lod_min.len() != n * 3
    || p.lod_spacing.len() != n
    || p.lod_level.len() != n
    || p.lod_first.len() != n
    || p.lod_count.len() != n
    || p.lod_children.len() != n * 8
  {
    return Err("Invalid cloud octree columns");
  }
  let mut nodes = Vec::with_capacity(n);
  let mut owners = vec![0u8; n];
  let mut ranges = Vec::new();
  for i in 0..n {
    let node = checked_node(source, &p, i)?;
    if node.count > 0 {
      ranges.push((node.first as usize, (node.first + node.count) as usize));
    }
    for child in node.children.iter().copied().filter(|c| *c >= 0) {
      let child = child as usize;
      owners[child] = owners[child].checked_add(1).ok_or("Cloud octree child repeated")?;
      if owners[child] != 1 {
        return Err("Cloud octree child repeated");
      }
    }
    nodes.push(node);
  }
  if n > 0 && (p.lod_level[0] != 0 || owners[1..].iter().any(|v| *v != 1)) {
    return Err("Cloud octree is not one rooted tree");
  }
  ranges.sort_unstable();
  let mut end = 0;
  for (first, to) in ranges {
    if first != end {
      return Err("Cloud octree point ranges overlap or omit source rows");
    }
    end = to;
  }
  if n > 0 && end != source.len() {
    return Err("Cloud octree point ranges omit source rows");
  }
  Ok(nodes)
}

