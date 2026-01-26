# Serialization - Python

## JSON

```python
import json

def __jsondump__(self) -> dict:
    """Return dict with keys in alphabetical order."""
    return {
        "guid": self.guid,
        "name": self.name,
        "type": "ClassName",
        "x": self._x,
        "y": self._y,
        "z": self._z,
    }

@staticmethod
def __jsonload__(data: dict) -> "ClassName":
    """Create from dict."""
    obj = ClassName.__new__(ClassName)
    obj.guid = data["guid"]
    obj.name = data["name"]
    obj._x = data["x"]
    obj._y = data["y"]
    obj._z = data["z"]
    return obj

def json_dump(self, filename: str):
    """Save to JSON file."""
    with open(filename, "w") as f:
        json.dump(self.__jsondump__(), f, indent=2)

@staticmethod
def json_load(filename: str) -> "ClassName":
    """Load from JSON file."""
    with open(filename, "r") as f:
        return ClassName.__jsonload__(json.load(f))
```

## Protobuf

```python
from session_py.proto import classname_pb2

def to_proto(self) -> bytes:
    """Serialize to protobuf bytes."""
    msg = classname_pb2.ClassName()
    msg.guid = self.guid
    msg.name = self.name
    msg.x = self._x
    msg.y = self._y
    msg.z = self._z
    return msg.SerializeToString()

@staticmethod
def from_proto(data: bytes) -> "ClassName":
    """Create from protobuf bytes."""
    msg = classname_pb2.ClassName()
    msg.ParseFromString(data)
    obj = ClassName.__new__(ClassName)
    obj.guid = msg.guid
    obj.name = msg.name
    obj._x = msg.x
    obj._y = msg.y
    obj._z = msg.z
    return obj

def protobuf_dump(self, filename: str):
    """Save to binary file."""
    with open(filename, "wb") as f:
        f.write(self.to_proto())

@staticmethod
def protobuf_load(filename: str) -> "ClassName":
    """Load from binary file."""
    with open(filename, "rb") as f:
        return ClassName.from_proto(f.read())
```
