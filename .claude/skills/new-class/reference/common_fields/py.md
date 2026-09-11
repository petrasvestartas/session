# Common Fields - Python

## Implementation

```python
import uuid

class ClassName:
    def __init__(self):
        self.guid = str(uuid.uuid4())
        self.name = "my_classname"

        # Visual classes only:
        self.width = 1.0
        self.color = Color.red()
        self.xform = Xform()
```

## GUID Generation

```python
import uuid

# In __init__:
self.guid = str(uuid.uuid4())

# In duplicate() - generate NEW guid:
def duplicate(self) -> "ClassName":
    copy = ClassName.__new__(ClassName)
    copy.guid = str(uuid.uuid4())  # NEW guid
    copy.name = self.name
    copy.width = self.width
    copy.color = self.color.duplicate()
    copy.xform = self.xform.duplicate()
    return copy
```

## Private Coordinates Pattern

```python
class Point:
    def __init__(self, x=0.0, y=0.0, z=0.0):
        self._x = float(x)
        self._y = float(y)
        self._z = float(z)

    @property
    def x(self) -> float:
        return self._x

    @x.setter
    def x(self, value: float):
        self._x = float(value)
```
