---
name: session-format
description: Read before writing or changing any example, demo, or main_* program in session_cpp / session_py / session_rust or a consumer (wood, wood_research, wood_nano, compas_wood, session_viewer). Triggers on "write an example", "add a demo", a new main_*.cpp / examples/*.rs / examples/*.py, and on any urge to give one a file-header description, a progress print, an input check, or an argument parser.
---


## Avoid Excessive Commenting

- Imports, then code. No header block saying what the example does, no author, no usage, no parameter list. The only comments above code are section banners (see `session-comments`).
- The overal code must be as minimal as possible.

## Print

- Avoid printing statements in all cases, unless user asks.
- The print must generally fit all the code inside one line.

```cpp
fmt::print(stderr, "not found: {}\ngenerate it with:\n  cd data/... && .venv/bin/python step_to_pb.py\n", pb.string());
```
- No progress lines, no counts, no "done", no timing, no logging of what was written. A program that worked says nothing and returns 0.

## Calls stay on one line

For long functions, open parenthesis and write single line declarations, like Polyline::rectangle below and the functions must be closed on the newline as shown with ");".

```cpp
elements.emplace_back(
    Polyline::rectangle(Point(-500, 0, 0), x, y, width, height),
    Polyline::rectangle(Point(-500, 0, -thickness), x, y, width, height)
);
```

A single-statement loop takes no braces and no temporary:

```cpp
for (const std::vector<Polyline>& loops : session.select_by_type<Polyline>())
    elements.emplace_back(loops);
```

## Variable Names

- Use variable names one word, short words.
- Never use single letter variables, unless you use them in loops.
- Declare variables when they used more than one time, otherwise declare them directly in a function.
- Variables that will not change must be const.

```cpp
    const double lift = 174.0;
    const double width = 1000.0;
    const double height = 500.0;
    const double thickness = 15.0;
    const session_cpp::Vector xaxis(1.0, 0.0, 0.0);
    const session_cpp::Vector yaxis(0.0, 1.0, 0.0);

    const session_cpp::Vector fold(0.0, height, -lift);
    const double fold_length = fold.magnitude();
    session_cpp::Polyline::rectangle(session_cpp::Point(-500, 0, 0), xaxis, yaxis, width, height);
```

## Code with Options

- Dont create verbose, cimplicated enums, when choosing an option use a simple bool or integer.
- These global variables must be in capital letters.

```cpp
const bool PLATES_OR_BLOCKS = true;      // then: if constexpr (plates_or_blocks)
```
```python
PLATES_OR_BLOCKS = True
```
```rust
const PLATES_OR_BLOCKS: bool = true;
```

No argparse, no clap, no `std::env::args`, no `--help`. Editing one line and rebuilding is the
interface.
