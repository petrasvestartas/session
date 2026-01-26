# Transformation - Python

```python
def transform(self):
    """Apply xform in-place, reset to identity."""
    nx, ny, nz = self.xform.apply(self._x, self._y, self._z)
    self._x = nx
    self._y = ny
    self._z = nz
    self.xform = Xform()

def transformed(self) -> "ClassName":
    """Return transformed copy, original unchanged."""
    result = self.duplicate()
    result.transform()
    return result
```

## Manual Matrix Application

```python
def transform(self):
    m = self.xform
    nx = m[0]*self._x + m[1]*self._y + m[2]*self._z + m[3]
    ny = m[4]*self._x + m[5]*self._y + m[6]*self._z + m[7]
    nz = m[8]*self._x + m[9]*self._y + m[10]*self._z + m[11]

    self._x = nx
    self._y = ny
    self._z = nz
    self.xform = Xform()
```
