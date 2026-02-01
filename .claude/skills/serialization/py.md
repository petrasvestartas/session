# Serialization - Python

## JSON

```python
import json

class ClassName:
    # Core: dict serialization (alphabetical field order)
    def __jsondump__(self) -> dict:
        return {
            "guid": self.guid,
            "name": self.name,
            "type": "ClassName",
            "x": self._x,
            "y": self._y,
            "z": self._z,
        }

    # Core: dict deserialization
    @classmethod
    def __jsonload__(cls, data: dict) -> "ClassName":
        obj = cls()
        obj.guid = data.get("guid", obj.guid)
        obj.name = data.get("name", obj.name)
        obj._x = data.get("x", 0.0)
        obj._y = data.get("y", 0.0)
        obj._z = data.get("z", 0.0)
        return obj

    # String wrappers
    def json_dumps(self) -> str:
        return json.dumps(self.__jsondump__())

    @classmethod
    def json_loads(cls, json_string: str) -> "ClassName":
        return cls.__jsonload__(json.loads(json_string))

    # File wrappers
    def json_dump(self, filepath: str):
        with open(filepath, 'w') as f:
            json.dump(self.__jsondump__(), f, indent=2)

    @classmethod
    def json_load(cls, filepath: str) -> "ClassName":
        with open(filepath, 'r') as f:
            return cls.__jsonload__(json.load(f))
```

## Protobuf

```python
class ClassName:
    # Core: bytes serialization
    def pb_dumps(self) -> bytes:
        from .proto import classname_pb2
        msg = classname_pb2.ClassName()
        msg.guid = self.guid
        msg.name = self.name
        msg.x = self._x
        msg.y = self._y
        msg.z = self._z
        return msg.SerializeToString()

    # Core: bytes deserialization
    @classmethod
    def pb_loads(cls, data: bytes) -> "ClassName":
        from .proto import classname_pb2
        msg = classname_pb2.ClassName()
        msg.ParseFromString(data)
        obj = cls()
        obj.guid = msg.guid
        obj.name = msg.name
        obj._x = msg.x
        obj._y = msg.y
        obj._z = msg.z
        return obj

    # File wrappers
    def pb_dump(self, filepath: str):
        with open(filepath, 'wb') as f:
            f.write(self.pb_dumps())

    @classmethod
    def pb_load(cls, filepath: str) -> "ClassName":
        with open(filepath, 'rb') as f:
            return cls.pb_loads(f.read())
```

## Nested Objects

```python
# In __jsondump__():
return {
    "linecolor": self.linecolor.__jsondump__(),
    "xform": self.xform.__jsondump__(),
    # ...
}

# In __jsonload__():
if "linecolor" in data:
    obj.linecolor = Color.__jsonload__(data["linecolor"])
if "xform" in data:
    obj.xform = Xform.__jsonload__(data["xform"])

# In pb_dumps():
proto.linecolor.guid = self.linecolor.guid
proto.linecolor.r = int(self.linecolor.r)
# ...
proto.xform.matrix.extend(self.xform.m.flatten().tolist())

# In pb_loads():
if proto.HasField('linecolor'):
    obj.linecolor = Color(proto.linecolor.r, proto.linecolor.g,
                           proto.linecolor.b, proto.linecolor.a)
if proto.HasField('xform'):
    obj.xform = Xform()
    obj.xform.m = np.array(list(proto.xform.matrix)).reshape(4, 4)
```

## Import Convention

Each import on a separate line (per project style):

```python
from .proto import classname_pb2
from .proto import color_pb2
from .proto import xform_pb2
```
