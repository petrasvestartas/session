# Testing - Python

## Test File Structure

```python
from session_py.mini_test import MINI_TEST, MINI_CHECK


@MINI_TEST("ClassName", "constructor")
def test_classname_constructor():
    from session_py import ClassName

    obj = ClassName(1.0, 2.0, 3.0)

    cstr = str(obj)
    crepr = repr(obj)

    copy = obj.duplicate()

    MINI_CHECK(obj.is_valid() == True)
    MINI_CHECK(obj[0] == 1.0)
    MINI_CHECK(obj.name == "my_classname")
    MINI_CHECK(obj.guid != "")
    MINI_CHECK(cstr == "ClassName(1, 2, 3)")
    MINI_CHECK("name=my_classname" in crepr)
    MINI_CHECK(copy.guid != obj.guid)
    MINI_CHECK(copy == obj)


@MINI_TEST("ClassName", "json_roundtrip")
def test_classname_json_roundtrip():
    from session_py import ClassName

    obj = ClassName(1.0, 2.0, 3.0)
    obj.name = "test_json"

    path = "test_classname.json"
    obj.json_dump(path)
    loaded = ClassName.json_load(path)

    MINI_CHECK(loaded.name == obj.name)
    MINI_CHECK(loaded[0] == obj[0])
    MINI_CHECK(loaded == obj)


@MINI_TEST("ClassName", "protobuf_roundtrip")
def test_classname_protobuf_roundtrip():
    from session_py import ClassName

    obj = ClassName(1.0, 2.0, 3.0)
    obj.name = "test_proto"

    path = "test_classname.bin"
    obj.protobuf_dump(path)
    loaded = ClassName.protobuf_load(path)

    MINI_CHECK(loaded.name == obj.name)
    MINI_CHECK(loaded[0] == obj[0])
    MINI_CHECK(loaded == obj)


if __name__ == "__main__":
    test_classname_constructor()
    test_classname_json_roundtrip()
    test_classname_protobuf_roundtrip()
```

## Package Registration

Add to `session_py/src/session_py/__init__.py`:
```python
from session_py.classname import ClassName
```

## minitest.sh Entry

Add `"classname"` to `CLASS_NAMES` array in `bash/minitest.sh`
