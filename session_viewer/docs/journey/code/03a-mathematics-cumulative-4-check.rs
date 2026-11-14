// --8<-- [start:implementation]
use super::run::{DPrim, Stage};
use super::tex::Shape;

impl Stage {
    /// Ink box of a piece: min x, min y, max x, max y.
    pub fn bounds(&self, p: &DPrim) -> [f32; 4] {
        match p.shape {
            Shape::Glyph { font, gid, size } => {
                let m = self.fonts.metrics(font, gid);
                [
                    p.x + m.left * size,
                    p.y - m.height * size,
                    p.x + m.right * size,
                    p.y + m.depth * size,
                ]
            }
            Shape::Rect { w, h, .. } | Shape::Image { w, h, .. } => [p.x, p.y, p.x + w, p.y + h],
            Shape::Stroke { dx, dy, .. } => [
                p.x.min(p.x + dx),
                p.y.min(p.y + dy),
                p.x.max(p.x + dx),
                p.y.max(p.y + dy),
            ],
        }
    }

    /// Ink box of the visible pieces, or of `subset` whether visible or not.
    pub(super) fn bbox(&self, prims: &[DPrim], subset: Option<&[usize]>) -> Option<[f32; 4]> {
        let mut b = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        let mut any = false;
        let mut take = |p: &DPrim| {
            if p.color[3] <= 0.001 && subset.is_none() {
                return;
            }
            let q = self.bounds(p);
            if q[2] - q[0] <= 0.0 && q[3] - q[1] <= 0.0 {
                return;
            }
            b = [
                b[0].min(q[0]),
                b[1].min(q[1]),
                b[2].max(q[2]),
                b[3].max(q[3]),
            ];
            any = true;
        };
        match subset {
            Some(s) => s.iter().filter_map(|&i| prims.get(i)).for_each(&mut take),
            None => prims.iter().for_each(&mut take),
        }
        any.then_some(b)
    }

    /// Overlapping panel blocks and labels, for the browser checks.
    pub fn overlaps(&mut self) -> Vec<(String, String)> {
        let now = self.now;
        let mut boxes: Vec<(String, [f32; 4])> = Vec::new();
        for i in 0..self.objs.len() {
            let o = &self.objs[i];
            if !(o.block() || o.label()) {
                continue;
            }
            let prims = self.prims_of(i, now);
            if let Some(b) = self.bbox(&prims, None) {
                boxes.push((o.id.clone(), b));
            }
        }
        let mut out = Vec::new();
        for a in 0..boxes.len() {
            for b in a + 1..boxes.len() {
                let (p, q) = (boxes[a].1, boxes[b].1);
                let ix = p[2].min(q[2]) - p[0].max(q[0]);
                let iy = p[3].min(q[3]) - p[1].max(q[1]);
                if ix > 1.0 && iy > 1.0 {
                    out.push((boxes[a].0.clone(), boxes[b].0.clone()));
                }
            }
        }
        out
    }

    /// Ink boxes of the panel blocks and labels, for the browser checks.
    pub fn item_boxes(&self) -> Vec<(String, [f32; 4])> {
        let now = self.now;
        (0..self.objs.len())
            .filter_map(|i| {
                let o = &self.objs[i];
                if !(o.block() || o.label()) {
                    return None;
                }
                let prims = self.prims_of(i, now);
                self.bbox(&prims, None).map(|b| (o.id.clone(), b))
            })
            .collect()
    }
}
// --8<-- [end:implementation]
