# Serialization Skill

## Overview

Every geometry class needs 6 serialization methods (3 pairs) for both JSON and Protobuf.
The naming follows Python's `json` module convention: `s` suffix = **s**tring/bytes.

## JSON API

| Method | Type | Description |
|--------|------|-------------|
| `jsondump()` | json object | Convert to JSON object (for nesting into other JSON) |
| `jsonload(j)` | json object | Create instance from JSON object |
| `json_dumps()` | string | Convert to JSON string (for network/messaging) |
| `json_loads(s)` | string | Create instance from JSON string |
| `json_dump(path)` | file | Write JSON to file |
| `json_load(path)` | file | Read JSON from file |

### Layering

`json_dumps` / `json_loads` call `jsondump` / `jsonload` internally.
`json_dump` / `json_load` call `jsondump` / `jsonload` internally.
Only `jsondump` / `jsonload` contain the actual serialization logic.

## Protobuf API

| Method | Type | Description |
|--------|------|-------------|
| `pb_dumps()` | bytes/string | Convert to protobuf binary (for network/messaging) |
| `pb_loads(data)` | bytes/string | Create instance from protobuf binary |
| `pb_dump(path)` | file | Write protobuf to file |
| `pb_load(path)` | file | Read protobuf from file |

### Layering

`pb_dump` / `pb_load` call `pb_dumps` / `pb_loads` internally.
Only `pb_dumps` / `pb_loads` contain the actual serialization logic.

## JSON Field Order

**CRITICAL: Fields must be in ALPHABETICAL ORDER** across all languages.

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

- **C++:** Use `nlohmann::ordered_json` and add fields in alphabetical order
- **Python:** Return dict with keys in alphabetical order
- **Rust:** Uses `serde_json` which outputs alphabetically by default

## Protobuf Schema Location

Schemas defined in `session_proto/*.proto` files.

## Implementation Steps

1. Add `.proto` message to `session_proto/` if not exists
2. Implement `jsondump()` / `jsonload()` (core JSON logic)
3. Add `json_dumps()` / `json_loads()` (string wrappers)
4. Add `json_dump()` / `json_load()` (file wrappers)
5. Implement `pb_dumps()` / `pb_loads()` (core protobuf logic)
6. Add `pb_dump()` / `pb_load()` (file wrappers)
7. Add minitest `json_roundtrip` and `protobuf_roundtrip` tests
8. Run `./bash/minitest.sh` to verify

## Minitest Pattern

```
// JSON object
auto j = obj.jsondump();
auto loaded_json = ClassName::jsonload(j);

// JSON string
std::string s = obj.json_dumps();
auto loaded_str = ClassName::json_loads(s);

// JSON file
obj.json_dump(filename_json);
auto loaded_file = ClassName::json_load(filename_json);

// Protobuf string
auto pb = obj.pb_dumps();
auto loaded_pb = ClassName::pb_loads(pb);

// Protobuf file
obj.pb_dump(filename_bin);
auto loaded_pb_file = ClassName::pb_load(filename_bin);

// All should be equal
MINI_CHECK(loaded_json == loaded_str);
MINI_CHECK(loaded_json == loaded_file);
MINI_CHECK(loaded_json == loaded_pb);
MINI_CHECK(loaded_json == loaded_pb_file);
```

## See Language-Specific

- `cpp.md` - C++ implementation details
- `py.md` - Python implementation details
- `rust.md` - Rust implementation details
