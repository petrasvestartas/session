use session_rust::{PointCloud, Xform};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct LodNode {
  pub center: [f32; 3],
  pub size: f32,
  pub spacing: f32,
  pub first: u32,
  pub count: u32,
  pub children: [i32; 8],
}
pub struct Cloud {
  pub spacing: f32,
  pub node_first: u32,
  pub node_count: u32,
  pub resident: u32,
}
#[derive(Clone)]
pub struct Prepared {
  pub source: Rc<PointCloud>,
  pub nodes: Rc<Vec<LodNode>>,
  pub bounds: Rc<crate::bounds::Bounds>,
  pub spacing: f32,
}

fn checked_node(source: &PointCloud, p: &session_rust::proto::PointCloud, i: usize) -> Result<LodNode, &'static str> {
  let n = p.lod_size.len();
  let first = p.lod_first[i];
  let count = p.lod_count[i];
  let size = p.lod_size[i];
  let spacing = p.lod_spacing[i];
  if first < 0
    || count < 0
    || first as usize + count as usize > source.len()
    || !size.is_finite()
    || size <= 0.
    || !(size as f32).is_finite()
    || !spacing.is_finite()
    || spacing <= 0.
    || !(spacing as f32).is_finite()
    || p.lod_level[i] < 0
  {
    return Err("Invalid cloud octree range or size");
  }
  let corner = &p.lod_min[i * 3..i * 3 + 3];
  let tolerance = size * 1e-9 + 1e-10;
  for point in source.coords()[first as usize * 3..(first as usize + count as usize) * 3].chunks_exact(3) {
    if (0..3).any(|axis| point[axis] < corner[axis] - tolerance || point[axis] > corner[axis] + size + tolerance) {
      return Err("Cloud octree cube omits its source points");
    }
  }
  let mut children = [-1; 8];
  children.copy_from_slice(&p.lod_children[i * 8..i * 8 + 8]);
  for child in children.iter().copied().filter(|c| *c >= 0) {
    let child = child as usize;
    if child >= n || child == 0 || p.lod_level[child] != p.lod_level[i] + 1 {
      return Err("Invalid cloud octree child");
    }
    let child_size = p.lod_size[child];
    if !child_size.is_finite()
      || child_size <= 0.
      || child_size > size + tolerance
      || (0..3).any(|axis| {
        p.lod_min[child * 3 + axis] < corner[axis] - tolerance
          || p.lod_min[child * 3 + axis] + child_size > corner[axis] + size + tolerance
      })
    {
      return Err("Cloud octree child lies outside its parent");
    }
  }
  if children.iter().any(|c| *c < -1) {
    return Err("Invalid cloud octree child sentinel");
  }
  let center = std::array::from_fn(|axis| p.lod_min[i * 3 + axis] + size * 0.5);
  if center.iter().any(|v| !v.is_finite() || !(*v as f32).is_finite()) {
    return Err("Invalid cloud octree center");
  }
  Ok(LodNode {
    center: center.map(|v| v as f32),
    size: size as f32,
    spacing: spacing as f32,
    first: first as u32,
    count: count as u32,
    children,
  })
}
