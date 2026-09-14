//! Sizes and offsets for replacing one object without rewalking unrelated documents.
use super::Upload;
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct Counts {
    pub verts: u32,
    pub faces: u32,
    pub print: u32,
    pub text: u32,
    pub sources: u32,
    pub pipes: u32,
    pub ribbons: u32,
    pub spheres: u32,
    pub dots: u32,
}
impl Counts {
    pub fn of(up: &Upload) -> Self {
        Self {
            verts: up.arena.verts.len() as u32,
            faces: up.arena.idx.len() as u32,
            print: up.arena.idx_print.len() as u32,
            text: up.arena.idx_text.len() as u32,
            sources: up.arena.face_sources.len() as u32,
            pipes: up.seg.pipes.len() as u32,
            ribbons: up.seg.ribbons.len() as u32,
            spheres: up.glyph.spheres.len() as u32,
            dots: up.glyph.dots.len() as u32,
        }
    }
    pub fn plus(self, other: Self) -> Self {
        Self {
            verts: self.verts + other.verts,
            faces: self.faces + other.faces,
            print: self.print + other.print,
            text: self.text + other.text,
            sources: self.sources + other.sources,
            pipes: self.pipes + other.pipes,
            ribbons: self.ribbons + other.ribbons,
            spheres: self.spheres + other.spheres,
            dots: self.dots + other.dots,
        }
    }
    pub fn minus(self, other: Self) -> Self {
        Self {
            verts: self.verts - other.verts,
            faces: self.faces - other.faces,
            print: self.print - other.print,
            text: self.text - other.text,
            sources: self.sources - other.sources,
            pipes: self.pipes - other.pipes,
            ribbons: self.ribbons - other.ribbons,
            spheres: self.spheres - other.spheres,
            dots: self.dots - other.dots,
        }
    }
}
#[derive(Clone, Copy)]
pub(crate) struct Span {
    pub start: Counts,
    pub count: Counts,
}
