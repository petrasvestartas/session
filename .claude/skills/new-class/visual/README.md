# New Visual Class

Use this for classes WITH visual properties (width, color, xform).

Examples: Point, Line, Plane, Polyline, NurbsCurve

## Additional Fields

| Field | Type | Default |
|-------|------|---------|
| width | double | 1.0 |
| color | Color | Color::red() |
| xform | Xform | identity |

## Additional Methods

| Method | Description |
|--------|-------------|
| transform() | Apply xform in-place, reset to identity |
| transformed() | Return copy with xform applied |

## Files to Create

Same as basic class plus transform methods.

## See Templates

- `cpp.md` - Complete C++ template
- `py.md` - Complete Python template
- `rust.md` - Complete Rust template
