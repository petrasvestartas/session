---
name: session-comments
description: Read before adding a section separator, banner, or divider comment to any file in session_cpp / session_py / session_rust or a consumer (wood, wood_nano, compas_wood, session_viewer). Triggers on writing a run of slashes, dashes, or equals signs to split a file into sections, on "add a header comment", and on grouping methods, tests, or example steps under a heading.
---

# Section banner comments

Use it. Same banner in all three:

```cpp
// ═══════════════════════════════════════════════════════════════════════════
// Text
// ═══════════════════════════════════════════════════════════════════════════
```
```python
# ═══════════════════════════════════════════════════════════════════════════
# Text
# ═══════════════════════════════════════════════════════════════════════════
```
```rust
// ═══════════════════════════════════════════════════════════════════════════
// Text
// ═══════════════════════════════════════════════════════════════════════════
```

## Rules

- Exactly 75 `═` (U+2550). Same length in every language and every file.
- Never a run of `/`, `-`, or `=`.
- Never write comments outside these blocks
- Comments must be title focused, only in rare exceptions you can write bullet points in multiple lines, never long sentences. Coder must understand the code below in few seconds.
- Comment marker, one space, then the bar or the text. No trailing spaces.
- `Text` is a short noun phrase in sentence case: `Serialization`, `Edge cases`. Not a
  sentence, no trailing period.
- Blank line above the banner, none between the banner and the code it heads.
- Banners split a file into sections. They do not document a single item — that is a docstring
  (`///` in C++/Rust, `"""` in Python).
