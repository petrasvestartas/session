# New Basic Class - Python Template

## Implementation (src/session_py/name.py)

```python
import uuid
import json
import math


class Name:
    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_name"
        self._x = float(x)
        self._y = float(y)
        self._z = float(z)

    def __getitem__(self, i: int) -> float:
        if i == 0: return self._x
        elif i == 1: return self._y
        elif i == 2: return self._z
        raise IndexError("Index out of bounds")

    def __setitem__(self, i: int, value: float):
        if i == 0: self._x = float(value)
        elif i == 1: self._y = float(value)
        elif i == 2: self._z = float(value)
        else: raise IndexError("Index out of bounds")

    def __eq__(self, other: "Name") -> bool:
        if not isinstance(other, Name):
            return False
        return self._x == other._x and self._y == other._y and self._z == other._z

    def __ne__(self, other: "Name") -> bool:
        return not self.__eq__(other)

    def __str__(self) -> str:
        return f"Name({self._x}, {self._y}, {self._z})"

    def __repr__(self) -> str:
        return f"Name(\n  name={self.name},\n  x={self._x},\n  y={self._y},\n  z={self._z}\n)"

    def is_valid(self) -> bool:
        return not math.isnan(self._x) and not math.isnan(self._y) and not math.isnan(self._z)

    def duplicate(self) -> "Name":
        copy = Name(self._x, self._y, self._z)
        copy.name = self.name
        return copy

    def __jsondump__(self) -> dict:
        return {
            "guid": self.guid,
            "name": self.name,
            "type": "Name",
            "x": self._x,
            "y": self._y,
            "z": self._z,
        }

    @staticmethod
    def __jsonload__(data: dict) -> "Name":
        obj = Name.__new__(Name)
        obj.guid = data["guid"]
        obj.name = data["name"]
        obj._x = data["x"]
        obj._y = data["y"]
        obj._z = data["z"]
        return obj

    def json_dump(self, filename: str):
        with open(filename, "w") as f:
            json.dump(self.__jsondump__(), f, indent=2)

    @staticmethod
    def json_load(filename: str) -> "Name":
        with open(filename, "r") as f:
            return Name.__jsonload__(json.load(f))
```

## Package Registration

Add to `src/session_py/__init__.py`:
```python
from session_py.name import Name
```
