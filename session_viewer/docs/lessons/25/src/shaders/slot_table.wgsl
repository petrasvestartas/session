// Row an arena triangle draws with: its own vertex's.
const SLOT_OWN_ROW: u32 = 0xffffffffu;

// No triangle behind an id.
const NO_TRIANGLE: u32 = 0xffffffffu;

// Texel `i` of the slot table, 1024 texels a row; texel 0 = (slots, arena triangles).
fn slot_texel(i: u32) -> vec4<u32> {
    return textureLoad(slot_table, vec2<u32>(i & 1023u, i >> 10u), 0);
}

// Triangle id `id` (one-based) as (arena triangle, row): an arena triangle keeps SLOT_OWN_ROW, an
// instance's names its definition's triangle and the instance row; x is NO_TRIANGLE for none.
// `arena` bounds the ids when no instance is drawn.
fn placed_triangle(id: u32, arena: u32) -> vec2<u32> {
    let head = slot_texel(0u);

    if (head.x == 0u || id <= head.y) {
        let own = id != 0u && id <= select(arena, head.y, head.x != 0u);
        return vec2<u32>(select(NO_TRIANGLE, id - 1u, own), SLOT_OWN_ROW);
    }

    // the last slot whose ids start at or before this one
    let index = id - 1u;
    var lo = 1u;
    var hi = head.x;

    while (lo < hi) {
        let mid = (lo + hi + 1u) / 2u;

        if (slot_texel(mid).y <= index) {
            lo = mid;
        } else {
            hi = mid - 1u;
        }
    }

    let slot = slot_texel(lo);

    if (index - slot.y >= slot.w) {
        return vec2<u32>(NO_TRIANGLE, SLOT_OWN_ROW);
    }

    return vec2<u32>(slot.z + index - slot.y, slot.x);
}
