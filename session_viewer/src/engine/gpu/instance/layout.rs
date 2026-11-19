use session_rust::Xform;

// --8<-- [start:object-instance-layout]
/// One object row as the shaders read it, 112 bytes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub model: [f32; 16],    // rotation and scale; translation is stored separately
    pub color: [f32; 4],     // rgb tint, alpha of an unlit fill mesh
    pub flags: u32,          // FLAG_* bits below
    pub ao_radius: f32,      // SSAO contact radius, world units
    pub spacing: f32,        // vertex spacing, world units; 0 = unknown
    pub _pad: u32,           // packed edge color
    pub effect: [f32; 4],    // line reveal, point scale, two unused
}

const _: () = assert!(std::mem::size_of::<Instance>() == 112);
// --8<-- [end:object-instance-layout]

impl Instance {
    /// Selected: drawn tinted.
    pub const FLAG_SELECTED: u32 = 1 << 0;

    /// Hidden: skipped by every draw.
    pub const FLAG_HIDDEN: u32 = 1 << 1;

    /// Camera is inside the object: no back-face culling.
    pub const FLAG_INSIDE: u32 = 1 << 2;

    /// Sheet fill: flat color, no edges.
    pub const FLAG_PRINT: u32 = 1 << 3;

    /// Open mesh: no back-face culling.
    pub const FLAG_OPEN: u32 = 1 << 4;

    /// Part of a drawing sheet.
    pub const FLAG_SHEET: u32 = 1 << 5;

    /// Sampled surface: vertices are samples, not corners.
    pub const FLAG_SMOOTH: u32 = 1 << 6;

    /// Single face: stays shaded in x-ray.
    pub const FLAG_SINGLE: u32 = 1 << 7;

    /// Use the layer color instead of the object's.
    pub const FLAG_COLOR: u32 = 1 << 8;

    /// Use the layer color for edges too.
    pub const FLAG_EDGE_COLOR: u32 = 1 << 9;

    /// The object has faces, not only lines or points.
    pub const FLAG_HAS_FACES: u32 = 1 << 10;

    /// A retired or sink row; CPU only, always with FLAG_HIDDEN.
    pub const FLAG_DEAD: u32 = 1 << 11;

    /// A clipping plane: it cuts the scene and is never cut.
    pub const FLAG_CLIPPING_PLANE: u32 = 1 << 12;

    /// A verified closed solid: a cut through it gets a section cap.
    pub const FLAG_CLOSED: u32 = 1 << 13;

    /// A closed solid whose faces wind inward.
    pub const FLAG_INWARD: u32 = 1 << 14;

    /// A curve with arrowheads: its ribbons look along the curve for a head to stop under.
    pub const FLAG_HEADS: u32 = 1 << 15;

    /// Decorative geometry, such as a reference grid, that must never be picked.
    pub const FLAG_NON_PICKABLE: u32 = 1 << 19;

    /// The one row an empty scene binds: identity, grey, no effects.
    pub fn placeholder() -> Self {
        Self {
            model: Xform::identity().to_f32(),
            color: [0.5, 0.5, 0.5, 1.0],
            flags: 0,
            ao_radius: 0.0,
            spacing: 0.0,
            _pad: 0,
            effect: [1.0, 1.0, 0.0, 0.0],
        }
    }
}
