# Transformation

## Visual Classes Only

Point, Line, Plane, Polyline, NurbsCurve

## Methods

| Method | Behavior |
|--------|----------|
| `transform()` | Apply xform in-place, reset xform to identity |
| `transformed()` | Return copy with xform applied, original unchanged |

## Pattern

```
1. Object stores pending transform in xform field
2. transform() applies xform to coordinates, resets xform to identity
3. transformed() returns new object with applied transform
```

## Example

```
p = Point(1, 2, 3)
p.xform = Xform.translation(10, 0, 0)

copy = p.transformed()  # copy = (11, 2, 3), p unchanged
p.transform()           # p = (11, 2, 3), p.xform = identity
```

## See Language-Specific

- `cpp.md` - C++ implementation
- `py.md` - Python implementation
- `rust.md` - Rust implementation
