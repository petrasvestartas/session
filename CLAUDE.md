# CLAUDE.md

Multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and Vue test viewer.

## Workflow

Run claude always: claude --dangerously-skip-permissions

## Structure
```
session_cpp/     # C++ (submodule)
session_py/      # Python (submodule)
session_rust/    # Rust (submodule)
session_proto/   # Protobuf schemas (submodule)
session_tests/   # Vue 3 test viewer
```

## Build

```bash
./build.sh                    # All languages
```

**C++:**
```bash
cd session_cpp && mkdir -p build && cd build && cmake .. && make tests -j$(nproc)
```

**Python:**
```bash
uv venv uvsession --python 3.11 && source uvsession/bin/activate
cd session_py && uv pip install -e . && pytest -v
```

**Rust:**
```bash
cd session_rust && cargo build --release && cargo test
```

## Test Viewer

```bash
./minitest.sh                 # Run all tests + launch viewer at localhost:8769
```

## Git

```bash
git clone --recurse-submodules <url>
git submodule update --init --recursive
./git_push.sh "message"
```

## GitHub Actions

- after pushing with ./git_push.sh, check GitHub Actions build status using: gh run list --limit 5
- if build fails, view logs with: gh run view <run-id> --log-failed
- fix the failing code locally, run ./minitest.sh to verify, then push again
- all three languages (C++, Python, Rust) must pass CI before merge

## MINITEST

- datastructures name_test.py, name_test.rs, name_test.cpp must include separate tests for each class api method
- when using math pi constant, use it from tolerance class
- all api functions must be tested across all three languages (C++, Python, Rust)
- test names and test logic must be identical across languages
- each test should verify one specific api method or behavior
- api method order in all implementations: constructors/factory methods, accessors/getters, in-place mutators (*_self methods), copy-return operators (arithmetic returning new objects), utility methods (is_valid, distance_to, etc.), serialization (to_proto, from_proto, json_dump, json_load), string representation (str, repr)
- json serialization requires json_dump and json_load methods on all geometry classes
- protobuf serialization requires to_proto and from_proto methods on all geometry classes
- test files output to session_tests/session_{lang}/ as JSON for the Vue test viewer
- common methods across all geometry classes (Color, Point, Vector, Line, Plane, Polyline, Xform): constructor with default parameters, guid and name metadata fields, duplicate() for rust and python and cpp = operator duplicates the instance, /clone() creates new instance with new GUID, index operator [] for component access, equality operators == and !=, __str__/__repr__/to_string for string representation, __jsondump__/__jsonload__ for JSON dict conversion, json_dump(filepath)/json_load(filepath) for file I/O, to_proto/from_proto for protobuf serialization
- visual geometry classes (Point, Line, Plane, Polyline) have: width, color, xform fields, transform()/transformed() methods
- arithmetic classes (Point, Vector, Line, Polyline) have: in-place operators (+=, -=, *=, /=), copy operators (+, -, *, /)
- duplicate() copies all data (coordinates, name, visual properties) but generates a new GUID for the copy, in C++ the = operator and copy constructor behave the same way
- constructor test groups related functionality: default constructor, constructor overloads, index operator [], equality operators == !=, str() and repr() output, all tested together in single "constructor" test
- Vue test viewer shows serialized JSON output at bottom of each test result, showing exact JSON structure for each geometry class
- protobuf schemas defined in session_proto/*.proto files, defines binary serialization format for all geometry classes
- check if all tests passes in all languages
- check if you implemented minitest for json de/serialization and protobuf de/serialization
- check if all the operators minitests are part of constructor test not separate tests
- run ./minitest.sh

