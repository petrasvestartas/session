//! Parse coordinate input typed in the command line while a draw tool is active:
//!   "x,y" / "x,y,z"          absolute (world)
//!   "@dx,dy" / "@dx,dy,dz"   relative to the last point
//!   "@dist<angle"            polar (distance, degrees on the construction plane)
//!   "dist"                   bare distance along the current direction

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coord {
    Absolute(f32, f32, f32),
    Relative(f32, f32, f32),
    Polar(f32, f32), // (distance, angle_degrees)
    Distance(f32),
}

pub fn parse(s: &str) -> Option<Coord> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(rest) = s.strip_prefix('@') {
        if let Some((d, a)) = rest.split_once('<') {
            let dist: f32 = d.trim().parse().ok()?;
            let ang: f32 = a.trim().parse().ok()?;
            return Some(Coord::Polar(dist, ang));
        }
        let (x, y, z) = parse_tuple(rest)?;
        return Some(Coord::Relative(x, y, z));
    }
    if s.contains(',') {
        let (x, y, z) = parse_tuple(s)?;
        return Some(Coord::Absolute(x, y, z));
    }
    let d: f32 = s.parse().ok()?;
    Some(Coord::Distance(d))
}

fn parse_tuple(s: &str) -> Option<(f32, f32, f32)> {
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    match parts.len() {
        2 => Some((parts[0].parse().ok()?, parts[1].parse().ok()?, 0.0)),
        3 => Some((parts[0].parse().ok()?, parts[1].parse().ok()?, parts[2].parse().ok()?)),
        _ => None,
    }
}
