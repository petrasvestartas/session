use crate::{
    scene::{ObjectId, Scene},
    text::{TextLabel, TextPlacement},
};
use session_rust::{Geometry, Point};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
            Self::Meters => ["m", "m²", "m³"],
        };
        values[power.clamp(1, 3) - 1]
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Mark {
    pub points: Vec<[f64; 3]>,
    pub label: String,
}
impl Mark {
    pub fn labels(&self, existing: &[TextLabel]) -> Vec<TextLabel> {
        if self.points.is_empty() {
            return Vec::new();
        }
        let mut used = existing
            .iter()
            .map(|l| l.id)
            .collect::<std::collections::HashSet<_>>();
        let mut next = u32::MAX;
        let mut label = |text: String, font_size: f32, placement: TextPlacement, color: [u8; 4]| {
            while used.contains(&next) {
                next = next
                    .checked_sub(1)
                    .expect("Bounded labels leave free identities")
            }
            let id = next;
            used.insert(id);
            TextLabel {
                id,
                object: None,
                text,
                font_size,
                line_height: font_size * 1.25,
                color,
                placement,
                clip: None,
            }
        };
        let c = std::array::from_fn(|i| {
            self.points.iter().map(|p| p[i]).sum::<f64>() / self.points.len() as f64
        });
        let mut out = vec![label(
            self.label.clone(),
            14.,
            TextPlacement::Nameplate {
                world: c,
                padding: [6., 4.],
                rounded: true,
            },
            [255; 4],
        )];
        if self.points.len() > 1 {
            for &world in &self.points {
                out.push(label(
                    "■".into(),
                    8.,
                    TextPlacement::Anchor {
                        world,
                        offset: [-4., -5.],
                    },
                    [20, 20, 20, 255],
                ));
            }
        }
        out
    }
}
