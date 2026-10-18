#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub copy: [u32; 4],
    pub draw: [u32; 4],
    pub cursor: [u32; 2],
    pub radius: u32,
    pub full: [u32; 2],
}

impl Window {
    pub fn new(screen: [f32; 2], full: [u32; 2], density: f32) -> Option<Self> {
        if full.contains(&0) || !density.is_finite() || density <= 0.0
            || screen.iter().any(|v| !v.is_finite() || v.abs() > 1.0) { return None; }
        let at = [(f64::from(screen[0]) + 1.0) * f64::from(full[0]) * 0.5,
            (1.0 - f64::from(screen[1])) * f64::from(full[1]) * 0.5];
        let at: [u32; 2] = std::array::from_fn(|i| (at[i].floor() as u32).min(full[i] - 1));
        let radius = (6.0 * f64::from(density)).ceil().min(128.0) as u32;
        let [w, h] = full.map(|n| n.min(2 * radius + 1));
        let x = at[0].saturating_sub(radius).min(full[0] - w);
        let y = at[1].saturating_sub(radius).min(full[1] - h);
        let left = x.saturating_sub(3); let top = y.saturating_sub(3);
        let right = (u64::from(x) + u64::from(w) + 3).min(u64::from(full[0])) as u32;
        let bottom = (u64::from(y) + u64::from(h) + 3).min(u64::from(full[1])) as u32;
        Some(Self { copy: [x, y, w, h], draw: [left, top, right - left, bottom - top],
            cursor: [at[0] - x, at[1] - y], radius, full })
    }

    // Cropping changes clip x/y only; homogeneous depth and the perspective divide retain the main view's meaning.
    pub fn crop(self) -> [f32; 16] {
        let [x, y, w, h] = self.draw.map(f64::from);
        let [fw, fh] = self.full.map(f64::from);
        [fw / w, 0.0, 0.0, 0.0, 0.0, fh / h, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            (fw - 2.0 * x - w) / w, (2.0 * y + h - fh) / h, 0.0, 1.0].map(|n| n as f32)
    }

    pub fn row_bytes(self) -> u32 { (self.copy[2] * 8).div_ceil(256) * 256 }

    pub fn bytes(self) -> u64 { u64::from(self.row_bytes()) * u64::from(self.copy[3]) }

    pub fn closest(self, bytes: &[u8]) -> Result<Option<[u32; 2]>, &'static str> {
        if (bytes.len() as u64) < self.bytes() { return Err("Incomplete pick image"); }
        let mut best = None;
        for y in 0..self.copy[3] { for x in 0..self.copy[2] {
            let offset = (y * self.row_bytes() + x * 8) as usize;
            let row = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
            let sub = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap());
            let dx = i64::from(x) - i64::from(self.cursor[0]);
            let dy = i64::from(y) - i64::from(self.cursor[1]);
            let distance = (dx * dx + dy * dy) as u64;
            if row == 0 || distance > u64::from(self.radius).pow(2) { continue; }
            let key = (sub == 0, distance, row, sub);
            if best.is_none_or(|old| key < old) { best = Some(key); }
        } }
        Ok(best.map(|(_, _, row, sub)| [row, sub]))
    }
}
