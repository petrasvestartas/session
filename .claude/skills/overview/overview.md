# Session Skills Overview

## When to Use Which Skill

| Task | Skill Folder |
|------|--------------|
| Create new class without visual properties | `new_basic_class/` |
| Create new class with width/color/xform | `new_visual_class/` |
| Add guid, name fields | `common_fields/` |
| Add str(), repr(), duplicate() | `common_methods/` |
| Add [], ==, !=, +, -, *, / | `operators/` |
| Add JSON/Protobuf serialization | `serialization/` |
| Add transform(), transformed() | `transformation/` |
| Write minitests | `testing/` |
| See working example | `reference_point/` |

## Class Categories

### Basic Classes (no visual properties)
- Color, Xform, Tolerance, Knot
- Fields: guid, name
- No transform methods

### Visual Classes (with visual properties)
- Point, Line, Plane, Polyline, NurbsCurve, Mesh
- Fields: guid, name, width, color, xform
- Has transform(), transformed()

### Arithmetic Classes (with operators)
- Point, Vector, Line, Polyline
- Operators: +=, -=, *=, /=, +, -, *, /

## File Naming Convention

Each skill folder contains:
- `{skill_name}.md` - Overview and concepts
- `cpp.md` - C++ implementation
- `py.md` - Python implementation
- `rust.md` - Rust implementation
