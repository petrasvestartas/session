        Ok(Self { source, points: Rc::new(points), colour, width, heads })
    }

}
use crate::stroke::Stroke;
#[derive(Clone, Copy)]
pub struct Segment { pub stroke: Stroke, pub previous: [f32; 3], pub next: [f32; 3], pub heads: u32 }

impl PreparedChain {
    pub fn segments(&self) -> Vec<Segment> {
        let points = &self.points; let last = points.len().saturating_sub(1);
        let closed = last > 1 && points.first() == points.last();
        points.windows(2).enumerate().map(|(i, ends)| Segment {
            stroke: Stroke { start: ends[0], end: ends[1], colour: self.colour, width: self.width },
            previous: if i > 0 { points[i - 1] } else if closed { points[last - 1] } else { ends[0] },
            next: if i + 2 <= last { points[i + 2] } else if closed { points[1] } else { ends[1] },
            heads: if closed { 0 } else { (if i == 0 { self.heads & 2 } else { 0 }) | (if i + 1 == last { self.heads & 1 } else { 0 }) },
        }).collect()
    }
}

impl Segment {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = self.stroke.bytes();
        bytes.extend(self.previous.into_iter().chain(self.next).flat_map(f32::to_ne_bytes));
        bytes.extend(self.heads.to_ne_bytes()); bytes
    }
}
