# Skill: New Datastructure in Python

## Files to Create

1. `session_py/src/session_py/name.py` - Implementation
2. `session_py/src/session_py/name_test.py` - Minitest file

## Minimal Implementation (name.py)

```python
import uuid

class Name:
    def __init__(self):
        self.guid = str(uuid.uuid4())
        self.name = "my_name"

    def __str__(self) -> str:
        return "Name()"

    def __repr__(self) -> str:
        return f"Name(name={self.name})"

    def is_valid(self) -> bool:
        return True

    def duplicate(self) -> "Name":
        copy = Name()
        copy.name = self.name
        return copy
```

## Minimal Test (name_test.py)

```python
from session_py.mini_test import MINI_TEST, MINI_CHECK


@MINI_TEST("Name", "constructor")
def test_name_constructor():
    from session_py.name import Name

    obj = Name()

    cstr = str(obj)
    crepr = repr(obj)

    MINI_CHECK(obj.is_valid() == True)
    MINI_CHECK(obj.name == "my_name")
    MINI_CHECK(obj.guid != "")
    MINI_CHECK(cstr == "Name()")
    MINI_CHECK("name=my_name" in crepr)


if __name__ == "__main__":
    test_name_constructor()
```

## Register in Package

Add to `session_py/src/session_py/__init__.py`:

```python
from session_py.name import Name
```

## Register in minitest.sh

Add `"name"` to `CLASS_NAMES` array in `bash/minitest.sh`

## Test

```bash
source uvsession/Scripts/activate
python -m session_py.name_test
```
