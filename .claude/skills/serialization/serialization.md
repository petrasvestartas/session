# Serialization

## JSON Methods

| Method | Description |
|--------|-------------|
| `jsondump()` | Return dict/json object |
| `jsonload(data)` | Static: create from dict/json |
| `json_dump(filename)` | Save to file |
| `json_load(filename)` | Static: load from file |

## Protobuf Methods

| Method | Description |
|--------|-------------|
| `to_proto()` | Return protobuf bytes |
| `from_proto(data)` | Static: create from bytes |
| `protobuf_dump(filename)` | Save to binary file |
| `protobuf_load(filename)` | Static: load from binary file |

## JSON Field Order

**CRITICAL: Fields must be in ALPHABETICAL ORDER**

```json
{
  "guid": "...",
  "name": "...",
  "type": "ClassName",
  "x": 0.0,
  "y": 0.0,
  "z": 0.0
}
```

## Protobuf Schema Location

Schemas are in `session_proto/*.proto`

## See Language-Specific

- `cpp.md` - C++ implementation
- `py.md` - Python implementation
- `rust.md` - Rust implementation
