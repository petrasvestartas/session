# Common Methods

## Required Methods (All Classes)

| Method | Returns | Description |
|--------|---------|-------------|
| str() | string | Short representation |
| repr() | string | Detailed multiline representation |
| is_valid() | bool | Validation check |
| duplicate() | Self | Copy with new GUID |

## Method Order in Implementation

1. Constructors / factory methods
2. Accessors / getters
3. In-place mutators (*_self methods)
4. Copy-return operators
5. Utility methods
6. Serialization (to_proto, from_proto, json_dump, json_load)
7. String representation (str, repr)

## str() vs repr()

```
str():  "ClassName(brief_info)"
repr(): "ClassName(\n  field1=value,\n  field2=value,\n  ...\n)"
```

## See Language-Specific

- `cpp.md` - C++ implementation
- `py.md` - Python implementation
- `rust.md` - Rust implementation
