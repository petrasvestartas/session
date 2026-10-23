let at = min(vec2<u32>(px), size - 1u);
return ((plane * size.y + at.y) * size.x + at.x) * 4u;
}

@group(3) @binding(1) var<storage, read> course_owners: array<u32>;
