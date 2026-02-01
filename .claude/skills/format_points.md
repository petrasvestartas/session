# Format Points Skill

When the user pastes coordinate text (lines of `x, y, z` values), format them as Rhino `Point3d` list:

## Output Format

```python
import Rhino.Geometry as rg

pts = [
    rg.Point3d(x, y, z),
    ...
]
```

## Rules

- Each line of input `x, y, z` becomes `rg.Point3d(x, y, z),`
- Preserve original numeric formatting
- Wrap in a Python list named `pts`
- Import `Rhino.Geometry as rg`
