# Operators - Python

## Index Operator

```python
def __getitem__(self, i: int) -> float:
    if i == 0: return self._x
    elif i == 1: return self._y
    elif i == 2: return self._z
    else: raise IndexError("Index out of bounds")

def __setitem__(self, i: int, value: float):
    if i == 0: self._x = float(value)
    elif i == 1: self._y = float(value)
    elif i == 2: self._z = float(value)
    else: raise IndexError("Index out of bounds")
```

## Equality Operators

```python
def __eq__(self, other: "ClassName") -> bool:
    if not isinstance(other, ClassName):
        return False
    return self._x == other._x and self._y == other._y and self._z == other._z
    # Note: guid is NOT compared

def __ne__(self, other: "ClassName") -> bool:
    return not self.__eq__(other)
```

## Arithmetic Operators

```python
# In-place
def __iadd__(self, v: "Vector") -> "ClassName":
    self._x += v[0]
    self._y += v[1]
    self._z += v[2]
    return self

def __isub__(self, v: "Vector") -> "ClassName":
    self._x -= v[0]
    self._y -= v[1]
    self._z -= v[2]
    return self

def __imul__(self, scalar: float) -> "ClassName":
    self._x *= scalar
    self._y *= scalar
    self._z *= scalar
    return self

def __itruediv__(self, scalar: float) -> "ClassName":
    self._x /= scalar
    self._y /= scalar
    self._z /= scalar
    return self

# Copy operators
def __add__(self, v: "Vector") -> "ClassName":
    result = self.duplicate()
    result += v
    return result

def __sub__(self, v: "Vector") -> "ClassName":
    result = self.duplicate()
    result -= v
    return result

def __mul__(self, scalar: float) -> "ClassName":
    result = self.duplicate()
    result *= scalar
    return result

def __truediv__(self, scalar: float) -> "ClassName":
    result = self.duplicate()
    result /= scalar
    return result
```
