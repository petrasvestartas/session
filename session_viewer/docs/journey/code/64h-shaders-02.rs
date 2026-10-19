fn sign_not_zero(v: f64) -> f64 {
  if v < 0.0 { -1.0 } else { 1.0 }
}

fn quant_snorm8(v: f64) -> u32 {
  (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

pub fn oct16(n: &[f64; 3]) -> Option<u32> {
  let l = n[0].abs() + n[1].abs() + n[2].abs();

  if l.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
    return None;
  }

  let (mut x, mut y) = (n[0] / l, n[1] / l);

  if n[2] < 0.0 {
    let (ax, ay) = (x.abs(), y.abs());
    (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
  }

  Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}
