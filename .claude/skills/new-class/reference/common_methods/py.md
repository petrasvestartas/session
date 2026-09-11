# Common Methods - Python

## Implementation

```python
class ClassName:
    def __str__(self) -> str:
        return f"ClassName(x={self._x}, y={self._y})"

    def __repr__(self) -> str:
        return (
            f"ClassName(\n"
            f"  name={self.name},\n"
            f"  x={self._x},\n"
            f"  y={self._y}\n"
            f")"
        )

    def is_valid(self) -> bool:
        import math
        return not math.isnan(self._x) and not math.isnan(self._y)

    def duplicate(self) -> "ClassName":
        copy = ClassName.__new__(ClassName)
        copy.guid = str(uuid.uuid4())  # NEW guid
        copy.name = self.name
        copy._x = self._x
        copy._y = self._y
        return copy
```

## Alternative duplicate() Pattern

```python
def duplicate(self) -> "ClassName":
    """Create copy with new GUID."""
    copy = ClassName(self._x, self._y)
    copy.name = self.name
    # guid already new from constructor
    return copy
```
