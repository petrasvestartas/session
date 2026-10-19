}

pub fn example() -> session_rust::PointCloud {
  let coords = (0..1681)
    .flat_map(|i| {
      let x = (i % 41) as f64 * 0.05 - 1.;
      let y = (i / 41) as f64 * 0.05 - 1.;
      [x, y, 0.25 * (x * 3.).sin() * (y * 3.).cos()]
    })
    .collect();
  let colors = (0..1681).flat_map(|i| [(80 + (i % 41) * 4) as i32, (100 + (i / 41) * 3) as i32, 220, 255]).collect();
  let mut source = session_rust::PointCloud::from_coords(coords, colors, vec![]);
  source.name = "Original cloud".into();
  source.point_size = 3.;
  source.build_lod(0.2, 64);
  source
}

fn checked_node(source: &PointCloud, p: &session_rust::proto::PointCloud, i: usize) -> Result<LodNode, &'static str> {
  let n = p.lod_size.len();
