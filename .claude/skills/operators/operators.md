# Operators

## Index Operator []

All classes support index access:
- `obj[0]` = x or first element
- `obj[1]` = y or second element
- `obj[2]` = z or third element

## Equality Operators

| Operator | Description |
|----------|-------------|
| == | Compare values only (ignore GUID) |
| != | Inverse of == |

## Arithmetic Operators (Point, Vector, Line, Polyline)

### In-Place (modify self)

| Operator | Description |
|----------|-------------|
| += | Add vector |
| -= | Subtract vector |
| *= | Multiply by scalar |
| /= | Divide by scalar |

### Copy (return new object)

| Operator | Description |
|----------|-------------|
| + | Add, return new |
| - | Subtract, return new |
| * | Multiply, return new |
| / | Divide, return new |

## Testing Note

ALL operators are tested in the `constructor` test, NOT in separate tests.

## See Language-Specific

- `cpp.md` - C++ implementation
- `py.md` - Python implementation
- `rust.md` - Rust implementation
