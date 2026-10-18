pub const FACE: u32 = 0x2000_0000;
pub const CONTROL: u32 = 0x4000_0000;
pub const EDGE: u32 = 0x8000_0000;
const MASK: u32 = 0xe000_0000;
pub const LIMIT: u32 = 0x1fff_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind { Face, Control, Edge }

pub fn encode(kind: Kind, address: usize) -> Result<u32, &'static str> {
    let address = u32::try_from(address).map_err(|_| "Pick address overflows")?;
    if address > LIMIT { return Err("Too many sub-object addresses"); }
    let tag = match kind { Kind::Face => FACE, Kind::Control => CONTROL, Kind::Edge => EDGE };
    Ok((tag | address) + 1)
}

pub fn decode(pixel: u32) -> Option<(Kind, usize)> {
    let value = pixel.checked_sub(1)?;
    let kind = match value & MASK { FACE => Kind::Face, CONTROL => Kind::Control, EDGE => Kind::Edge, _ => return None };
    Some((kind, (value & !MASK) as usize))
}

pub fn surface(pixel: u32) -> bool { pixel == 0 || matches!(decode(pixel), Some((Kind::Face, _))) }
