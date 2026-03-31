# CLAUDE.md

Multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and Vue test viewer.

## Style
- Least verbose, no print messages, no excessive comments
- C++ is ground truth — port to Rust and Python with identical APIs, variable names, test logic, line counts

## Structure
```
session_cpp/     # C++ (submodule)
session_py/      # Python (submodule)
session_rust/    # Rust (submodule)
session_proto/   # Protobuf schemas
session_tests/   # Vue 3 test viewer
```

## Build & Test
```bash
./bash/minitest.sh --py --no-web      # Python only (fastest)
./bash/minitest.sh --rust --no-web    # Rust only
./bash/minitest.sh --cpp --no-web     # C++ only
./bash/minitest.sh                    # All + viewer at localhost:8769
./bash/quicktest.sh <class> --py      # Single class test
./bash/git_push.sh "message"          # Push all submodules
```
Dev order: Python → Rust → C++. Use `/build` command for full reference.

## Git
- NEVER add Claude/AI as git contributor, author, or co-author
- Check CI: `gh run list --limit 5`, failures: `gh run view <id> --log-failed`
- CI: macOS-15 (ARM64), manylinux_2_28 (Linux), chmod +x bash scripts in CI

## Minitest Rules (essentials)
- Tests identical across all 3 languages (names, logic, line count)
- One test per API method; constructor test groups: ctor, [], ==, !=, str, repr
- JSON fields alphabetically ordered across all languages
- Every class needs: json_dump/json_load + to_proto/from_proto tests
- Operators go inside constructor test, not separate tests
- Method order: constructors → accessors → mutators (*_self) → operators → utilities → serialization → str/repr
- Use `/test-rules` command for full import patterns and conventions

## Code Style
- Python: one import per line. TOLERANCE/PI from `.tolerance` at top of file. Geometry imports inside test functions. Use flat imports: `from session_py import Line, Plane` not `from session_py.line import Line`. Exception: `from session_py.intersection import line_line`.
- C++: never `#include "tolerance.h"` in production code. Use `std::cout << point` not manual coords.
- Rust: `use crate::tolerance::{TOLERANCE, PI};` at top. Geometry imports inside MINI_TEST blocks.

## Custom Commands
- `/new-class <name>` — full checklist for adding a new geometry class
- `/build` — all build/test/git commands
- `/test-rules` — detailed minitest conventions and import patterns
- `/decompile` — Rhino reverse engineering reference

## Reference Files
- `.claude/skills/` — language-specific templates for common patterns
- `SKILLS_RHINO_GEOMETRY.md` — 711 C exports + ~7100 C++ methods from Rhino
- `SKILLS_RHINO_DECOMPILE.md` — Ghidra decompilation guide
