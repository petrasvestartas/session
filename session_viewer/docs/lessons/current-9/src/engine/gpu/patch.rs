use super::Upload;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// Row counts per lane, for placing one object's rows.
pub(crate) struct Counts {
    pub verts: u32, // mesh vertices
    pub faces: u32, // solid face indices
    pub print: u32, // sheet fill indices
    pub text: u32, // sheet lettering indices
    pub sources: u32, // source faces
    pub pipes: u32, // line segments drawn as pipes
    pub ribbons: u32, // line segments drawn flat
    pub spheres: u32, // vertex markers
    pub dots: u32, // flat dots
}

impl Counts {
    /// Counts of one upload.
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

    /// Add two counts.
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

    /// Subtract two counts.
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
/// Where one object's rows sit: first row and row count per lane.
pub(crate) struct Span {
    pub start: Counts, // first row per lane
    pub count: Counts, // rows per lane
}
