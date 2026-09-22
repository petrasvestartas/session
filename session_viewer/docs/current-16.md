# current-16 · Attribute features in red and a one-row command dock
Attribute features draw in red at twice the default pen, centroids are no longer a feature, and the command dock starts collapsed with its options before the field.

## Step 1 · src/app/walk/mod.rs
Outline, axis and section draw red at 2 px and a one-point section is a 12 px red dot; centroid is gone.

`lessons/current-16/src/app/walk/mod.rs` · edit · type this

Added after the line `use session_rust::AABB;` in `lessons/current-15/src/app/walk/mod.rs`

```rust
--8<-- "lessons/current-16/src/app/walk/mod.rs:step-1a"
```

Replaces the line `const ATTRIBUTE_FEATURES: [&str; 4] = ["outline", "axis",…` in `lessons/current-15/src/app/walk/mod.rs`

```rust
--8<-- "lessons/current-16/src/app/walk/mod.rs:step-1b"
```

Replaces the line `let r = if let (1, Some(p)) = (outline.point_count(), out…` in `lessons/current-15/src/app/walk/mod.rs`

```rust
--8<-- "lessons/current-16/src/app/walk/mod.rs:step-1c"
```

Replaces the 7 lines from `let centroid = Polyline::new(vec![Point::new(5.0, 5.0, 5.…` in `lessons/current-15/src/app/walk/mod.rs`

```rust
--8<-- "lessons/current-16/src/app/walk/mod.rs:step-1e"
```

## Step 2 · src/app/ui.rs
The dock starts as one row, options sit before the field, and `+` opens the history.

`lessons/current-16/src/app/ui.rs` · edit · type this

Replaces the line `command_collapsed: bool,` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3a"
```

Replaces the line `let panel = if model.command_collapsed {` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3b"
```

Replaces the line `.inner_margin(6),` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3c"
```

Replaces the line `ui.max_rect().top() - 6.0,` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3d"
```

Replaces the line `if !model.command_collapsed {` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3e"
```

Added after the line `.sum();` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3f"
```

Delete the 21 lines from `for name in inline_options {` in `lessons/current-15/src/app/ui.rs`.

Replaces the line `.button(if model.command_collapsed { "+" } else { "−" })` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3h"
```

Replaces the line `model.command_collapsed = !model.command_collapsed;` in `lessons/current-15/src/app/ui.rs`

```rust
--8<-- "lessons/current-16/src/app/ui.rs:current-16-step-3i"
```

## Check

Run `trunk serve` in `lessons/current-16/` and open <http://127.0.0.1:8770/>.

Type `Attributes On`: features are red and thicker; the command dock is a single row until you press `+`.

## Next

[Capstone](capstone.md): the viewer as it is today.
