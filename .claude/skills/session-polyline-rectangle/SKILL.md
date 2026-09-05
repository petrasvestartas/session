---
name: session-polyline-rectangle
description: Read before writing any helper that builds a rectangle, quad, or 4-corner closed Polyline in session_cpp / session_py / session_rust or a consumer (wood, wood_nano, compas_wood, session_viewer). Triggers on a hand-written `rect(...)`, `make_rect`, `quad(...)`, `make_quad` in an example, test, or demo, on any request to "add a rectangle function" to the kernel, and when tempted to add a C++-only overload Python and Rust cannot mirror.
---

# Polyline rectangles already exist in the kernel

Use it. Same call in all three:

```cpp
Polyline r = Polyline::rectangle(origin, x_axis, y_axis, width, height, true);
```
```python
r = Polyline.rectangle(origin, x_axis, y_axis, width, height, True)
```
```rust
let r = Polyline::rectangle(&origin, &x_axis, &y_axis, width, height, true);
```

`origin` is a corner, not the center. Points come out in this order, counter-clockwise
about `x_axis` cross `y_axis`:

```
origin
origin + x_axis * width
origin + x_axis * width + y_axis * height
origin + y_axis * height
origin                                      // only when close = true
```