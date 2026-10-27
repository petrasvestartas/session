#[derive(Clone,Default)]
pub struct Parts {
    pub dots:Vec<u32>,
    pub dot_styles:Vec<crate::production_glyph::Style>,
    pub glyph_flags:u32,
    pub dot_colours:Vec<[f32;4]>,
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
