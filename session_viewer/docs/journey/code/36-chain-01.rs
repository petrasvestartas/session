use std::rc::Rc;
use session_rust::{Arrowhead, Line, Polyline};
use crate::stroke::{PreparedLine};

#[derive(Clone)]
pub enum Source { Line(Rc<Line>), Polyline(Rc<Polyline>) }

#[derive(Clone)]
pub struct PreparedChain { pub source: Source, pub points: Rc<Vec<[f32; 3]>>, pub colour: [f32; 4], pub width: f32, pub heads: u32 }

impl Source {
    pub fn coordinates(&self) -> Vec<[f64; 3]> {
        match self {
            Self::Line(line) => [line.start(), line.end()].map(|p| [p[0], p[1], p[2]]).to_vec(),
            Self::Polyline(line) => line.coords.chunks_exact(3).map(|p| [p[0], p[1], p[2]]).collect(),
        }
    }
}

impl PreparedChain {
    pub fn line(source: Rc<Line>) -> Result<Self, &'static str> {
        let display = PreparedLine::new(Rc::clone(&source))?.display;
        Self::new(Source::Line(source), display.colour, display.width)
    }

    pub fn polyline(source: Rc<Polyline>) -> Result<Self, &'static str> {
        if source.coords.len() % 3 != 0 || !(6..=30_000).contains(&source.coords.len()) {
            return Err("Polyline needs 2–10,000 complete points");
        }
        let _ = source.guid();
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        line.linecolor = source.linecolor.clone(); line.width = source.width;
        let display = PreparedLine::new(Rc::new(line))?.display;
        Self::new(Source::Polyline(source), display.colour, display.width)
    }

    fn new(source: Source, colour: [f32; 4], width: f32) -> Result<Self, &'static str> {
        let heads = match &source { Source::Line(line) => line.arrowhead, Source::Polyline(line) => line.arrowhead };
        let heads = match heads { Arrowhead::NONE => 0, Arrowhead::END => 1, Arrowhead::START => 2, Arrowhead::BOTH => 3 };
        let mut points = Vec::new();
        for p in source.coordinates() {
            let p = p.map(|v| v as f32);
            if p.iter().any(|v| !v.is_finite()) { return Err("Chain exceeds the display range"); }
            if points.last() != Some(&p) { points.push(p); }
        }
        Ok(Self { source, points: Rc::new(points), colour, width, heads })
    }

}
