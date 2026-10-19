}

#[derive(Clone)]
pub struct Row {
  pub id: crate::scene::ObjectId,
  pub guid: String,
  pub prepared: Prepared,
  pub model: Xform,
}
impl Row {
  pub fn bounds(&self) -> crate::bounds::Bounds {
    let b = &self.prepared.bounds;
    let corners = (0..8).map(|i| {
      self.model.transform_point(&session_rust::Point::new(
        if i & 1 == 0 { b.min[0] } else { b.max[0] },
        if i & 2 == 0 { b.min[1] } else { b.max[1] },
        if i & 4 == 0 { b.min[2] } else { b.max[2] },
      ))
    });
    let mut bounds = crate::bounds::Bounds { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3] };
    for p in corners {
      bounds.include([p[0], p[1], p[2]]);
    }
    bounds
  }
}

fn checked_node(source: &PointCloud, p: &session_rust::proto::PointCloud, i: usize) -> Result<LodNode, &'static str> {
  let n = p.lod_size.len();
