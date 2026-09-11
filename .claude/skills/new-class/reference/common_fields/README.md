# Common Fields

## All Classes Have

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| guid | string | auto-generated | Unique identifier (UUID v4) |
| name | string | "my_{classname}" | User-friendly name |

## Visual Classes Add

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| width | double | 1.0 | Line/point width for rendering |
| color | Color | varies | Display color |
| xform | Xform | identity | Pending transformation matrix |

## GUID Behavior

- Generated automatically in constructor
- `duplicate()` creates NEW guid
- `operator==` ignores guid (compares values only)
- Never copied between instances

## See Language-Specific

- `cpp.md` - C++ implementation
- `py.md` - Python implementation
- `rust.md` - Rust implementation
