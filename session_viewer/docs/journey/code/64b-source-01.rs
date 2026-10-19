  pub bounds: Rc<crate::bounds::Bounds>,
  pub spacing: f32,
}

impl Prepared {
  pub fn new(source: Rc<PointCloud>) -> Result<Self, &'static str> {
    let n = source.len();
    if n == 0
      || n > u32::MAX as usize
      || source.coords().len() != n * 3
      || source.coords().iter().any(|v| !v.is_finite() || !(*v as f32).is_finite())
    {
      return Err("Invalid cloud coordinates");
    }
    if !source.colors().is_empty()
      && (source.colors().len() != n * 4 || source.colors().iter().any(|c| !(0..=255).contains(c)))
    {
      return Err("Invalid cloud colors");
    }
    if !source.normals().is_empty()
      && (source.normals().len() != n * 3 || source.normals().iter().any(|v| !v.is_finite()))
    {
      return Err("Invalid cloud normals");
    }
    if !source.point_size.is_finite() || source.point_size <= 0. || !(source.point_size as f32).is_finite() {
      return Err("Invalid cloud point size");
    }
    if !source.point_ids().is_empty()
      && (source.point_ids().len() != n
        || source.point_ids().iter().copied().collect::<std::collections::HashSet<_>>().len() != n)
    {
      return Err("Invalid original cloud IDs");
    }
    let nodes = validate_nodes(&source)?;
    let mut bounds = crate::bounds::Bounds { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3] };
    for p in source.coords().chunks_exact(3) {
      bounds.include([p[0], p[1], p[2]]);
    }
    let mut extents = std::array::from_fn::<_, 3, _>(|i| bounds.max[i] - bounds.min[i]);
    extents.sort_by(|a, b| b.total_cmp(a));
    let area = extents[0] * extents[1];
    let spacing = if n > 1 && area > 0. && area.is_finite() { (area / n as f64).sqrt() as f32 } else { 20. };
    let _ = source.guid();
    Ok(Self { source, nodes: Rc::new(nodes), bounds: Rc::new(bounds), spacing })
  }
  pub fn descriptor(&self) -> Cloud {
    Cloud {
      spacing: self.spacing,
      node_first: 0,
      node_count: self.nodes.len() as u32,
      resident: self.source.len() as u32,
    }
  }
}

