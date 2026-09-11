# New Visual Class - Python Template

Extends basic class with visual properties.

## Class Additions (src/session_py/name.py)

```python
from session_py.color import Color
from session_py.xform import Xform


class Name:
    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        # ... basic fields ...
        self.width = 1.0
        self.color = Color.red()
        self.xform = Xform()

    def duplicate(self) -> "Name":
        copy = Name(self._x, self._y, self._z)
        copy.name = self.name
        copy.width = self.width
        copy.color = self.color.duplicate()
        copy.xform = self.xform.duplicate()
        return copy

    def transform(self):
        """Apply xform in-place, reset to identity."""
        m = self.xform
        nx = m[0]*self._x + m[1]*self._y + m[2]*self._z + m[3]
        ny = m[4]*self._x + m[5]*self._y + m[6]*self._z + m[7]
        nz = m[8]*self._x + m[9]*self._y + m[10]*self._z + m[11]

        self._x = nx
        self._y = ny
        self._z = nz
        self.xform = Xform()

    def transformed(self) -> "Name":
        """Return transformed copy."""
        result = self.duplicate()
        result.transform()
        return result
```

## JSON Additions

```python
def __jsondump__(self) -> dict:
    return {
        "color": self.color.__jsondump__(),
        "guid": self.guid,
        "name": self.name,
        "type": "Name",
        "width": self.width,
        "x": self._x,
        "xform": self.xform.__jsondump__(),
        "y": self._y,
        "z": self._z,
    }
```

## Test Additions

```python
@MINI_TEST("Name", "transformation")
def test_name_transformation():
    from session_py import Name, Xform

    obj = Name(1.0, 2.0, 3.0)
    obj.xform = Xform.translation(10.0, 0.0, 0.0)

    copy = obj.transformed()
    MINI_CHECK(copy[0] == 11.0)
    MINI_CHECK(obj[0] == 1.0)  # Original unchanged

    obj.transform()
    MINI_CHECK(obj[0] == 11.0)
```
