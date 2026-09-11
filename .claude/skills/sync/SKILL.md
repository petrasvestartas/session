Audit cross-language parity for class $ARGUMENTS. Read all 6 files and report every divergence.

## Files to read
- `session_py/src/session_py/$ARGUMENTS.py`
- `session_py/src/session_py/$ARGUMENTS_minitest.py`
- `session_rust/src/$ARGUMENTS.rs`
- `session_rust/src/$ARGUMENTS_minitest.rs`
- `session_cpp/src/$ARGUMENTS.h` + `session_cpp/src/$ARGUMENTS.cpp`
- `session_cpp/src/$ARGUMENTS_minitest.cpp`

## Check for divergences
- Methods present in one language but missing in another
- Test names that differ or are missing across languages
- Test count mismatch (should be identical)
- Variable names that differ between languages
- Coordinate access violations: p.x/p.y/p.z instead of p[0]/p[1]/p[2]
- Test loops using iterators/comprehensions instead of explicit for
- JSON fields not alphabetically ordered
- Imports in wrong location (Rust: must be inside MINI_TEST block)
- Line count divergence > 10% (may indicate missing logic)

## Output format
Report as a compact list grouped by category. For each issue: file + line number + description.
If all languages are in sync, say so explicitly.
