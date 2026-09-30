
/// The extreme points of every vertex, segment end and marker one walk wrote, with `more`; None
/// when the walk wrote rows without points here, or its points do not span `bounds`.
pub fn hull_of(up: &Upload, more: &[[f32; 3]], bounds: &AABB) -> Option<Hull> {
    let seg = &up.seg;
    let lanes = super::patch::Counts::of(up)
        .lanes
        .iter()
        .any(|rows| *rows > 0);
    let unread = lanes || !up.cloud.pos.is_empty() || !seg.sheet_rows.is_empty();

    if unread || !bounds.is_valid() {
        return None;
    }

    // each source on its own first: a mesh's vertices are never gathered
    let sources = [
        Points::of(&up.arena.verts, &[0]),
        Points::of(&seg.pipes, &[0, 4]),
        Points::of(&seg.ribbons, &[0, 4]),
        Points::of(&up.glyph.spheres, &[0]),
        Points::of(&up.glyph.dots, &[0]),
    ];
    let mut points = more.to_vec();

    for source in sources {
        points.extend(extreme_points(source)?);
    }

    let hull = extreme_points(Points::of(&points, &[0]))?;
    spans(&hull, bounds).then(|| Hull::from(hull))
}
