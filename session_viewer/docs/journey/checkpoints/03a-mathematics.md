# Mathematics: complete checkpoint

This is the complete source checkpoint for [A slide of mathematics](../03a-mathematics.md). Finish first egui painting before applying these files; the reader lesson explains their final design.

## Goal

Paint and navigate a complete mathematical deck inside the cumulative project.

## Apply the complete files

### Cargo.toml

Add the dependencies required by the finished mathematics implementation.

```toml
--8<-- "journey/code/03a-mathematics-cumulative-kernel-feature-0.toml"
```

### src/app/mod.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-1-mod.rs"
```

### src/app/mode.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-2-mode.rs"
```

### src/app/stage/actions.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-3-actions.rs"
```

### src/app/stage/check.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-4-check.rs"
```

### src/app/stage/deck.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-5-deck.rs"
```

### src/app/stage/draw.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-6-draw.rs"
```

### src/app/stage/eval.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-7-eval.rs"
```

### src/app/stage/font.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-8-font.rs"
```

### src/app/stage/geometry.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-9-geometry.rs"
```

### src/app/stage/layout.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-10-layout.rs"
```

### src/app/stage/math_mode.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-11-math_mode.rs"
```

### src/app/stage/matrix.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-12-matrix.rs"
```

### src/app/stage/mod.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-13-mod.rs"
```

### src/app/stage/morph.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-14-morph.rs"
```

### src/app/stage/motion_tests.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-15-motion_tests.rs"
```

### src/app/stage/net.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-16-net.rs"
```

### src/app/stage/object.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-17-object.rs"
```

### src/app/stage/paint.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-18-paint.rs"
```

### src/app/stage/presentation.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-19-presentation.rs"
```

### src/app/stage/presentation_tests.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-20-presentation_tests.rs"
```

### src/app/stage/tests.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-21-production_tests.rs"
```

### src/app/stage/run.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-22-run.rs"
```

### src/app/stage/tex.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-23-tex.rs"
```

### src/app/stage/tween.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-24-tween.rs"
```

### src/app/stage/view.rs

Introduce this finished mathematics module or its supplied acceptance tests.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-25-view.rs"
```

### src/math_tests.rs

Connect the finished presentation to the cumulative checkpoint and its acceptance fixtures.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-26-math_tests.rs"
```

### example.yaml

Connect the finished presentation to the cumulative checkpoint and its acceptance fixtures.

```yaml
--8<-- "journey/code/03a-mathematics-cumulative-27-example.yaml"
```

### examples/course/fonts.rs

Connect the finished presentation to the cumulative checkpoint and its acceptance fixtures.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-28-fonts.rs"
```

### examples/course/mathematics.rs

Connect the finished presentation to the cumulative checkpoint and its acceptance fixtures.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-29-mathematics.rs"
```

### src/lib.rs

Register the mathematics modules once.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-30-lib.rs"
```

### examples/course/registration.rs

Select mathematics while retaining the first painting constructor.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-31-registration.rs"
```

## Run and observe

Four steps display aligned equations, a matching transform, a matrix and its highlighted first row. Left restores the preceding matrix state. T changes the palette without changing the equation.

Use the guarded shared-cache workflow described in the reader lesson. The source-bound mathematics checkpoint passes 27 native CPU/egui tests, optimized wasm compilation, and desktop/upright browser checks. Full-course integration and the final maintained-runtime startup remain unaccepted.

## Predict and try

Predict whether changing the viewport changes the authored coordinates. Rotate the viewport, then restore it and compare the matrix cells and equals column.

## Question and answer

Why does returning to a previous matrix step reproduce its earlier frame?

The stage rebuilds the preceding authored actions and layout instead of reversing accumulated floating-point transforms.
