#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct View { transform: [u32; 16], size: [u32; 2], density: u32 }

impl View {
    // Readback belongs to the camera matrix, physical viewport and CSS density that produced it, even when the document has not changed.
    pub fn new(transform: [f32; 16], size: [u32; 2], density: f32) -> Result<Self, &'static str> {
        if transform.iter().any(|n| !n.is_finite()) || size.contains(&0) || !density.is_finite() || density <= 0.0 {
            return Err("Invalid pick view");
        }
        Ok(Self { transform: transform.map(f32::to_bits), size, density: density.to_bits() })
    }
}
