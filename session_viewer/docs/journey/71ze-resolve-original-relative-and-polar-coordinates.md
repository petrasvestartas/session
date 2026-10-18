# 14.1.42 · Resolve original relative and polar coordinates

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Recognize the existing world prefix and resolve parsed coordinates from the plane origin, previous point and current direction. Reuse one finite parser for every form. Require a previous point and direction where the original distance form needs them.

## Type

Continue from [Parse original drawing coordinate forms](71zd-parse-original-drawing-coordinate-forms.md). [Save or recover your work](recovery.md).

### 1. `src/drawing_coords.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// Parse coordinate text without the world prefix.
fn parse_plain(text: &str) -> Option<Typed> {
    let text = text.trim();

    if text.is_empty() {
        return None;
    }

    let (body, relative) = match text.strip_prefix('@') {
        Some(rest) => (rest.trim(), true),
        None => (text, false),
    };

    if let Some((d, a)) = body.split_once('<') {
        let distance = number(d)?;
        let degrees = number(a)?;
        return Some(Typed::Polar { distance, degrees });
    }

    let parts: Vec<&str> = body.split(',').map(str::trim).collect();

    match parts.len() {
        1 if relative => None, // `@5` has no direction
        1 => Some(Typed::Distance(number(parts[0])?)),
        2 | 3 => {
            let x = number(parts[0])?;
            let y = number(parts[1])?;
            let z = match parts.get(2) {
                Some(v) => Some(number(v)?),
                None => None,
            };
            Some(if relative {
                Typed::Relative { x, y, z }
            } else {
                Typed::Absolute { x, y, z }
            })
        }
        _ => None,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/71ze-resolve-original-relative-and-polar-coordinates-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.42
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Relative and polar coordinates can resolve against the declared drawing frame.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for resolve original relative and polar coordinates.](../screenshots/journey/71aj-exercise-complete-and-ranged-original-drawings.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.1.37 · Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original frame → parsed coordinates → world drawing point.



Why does a distance need both a previous point and direction?

Its scalar value alone cannot locate a point. The existing context supplies the starting point and the direction to follow.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a polar input from a previous point, then a distance with and without a direction.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.42
npm --prefix ../session_tests run course -- save 14.1.42
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original frame and coordinate helpers are preparatory until the existing input bridge connects them. The earlier drawing stays unchanged.



[Full validation scope](release.md).

</details>
