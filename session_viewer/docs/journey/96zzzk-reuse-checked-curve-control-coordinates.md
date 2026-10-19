# 20.8.5 · Reuse checked curve control coordinates

**Chapter 20 · Shared GPU rendering · Section 20.8 · Background and shared object records** · [Contents](contents.md)

**Typing: 6–12 minutes.** [Estimate](typing-load.md).

Use one checked rational-control decoder for sampling and control markers.

## Type

Continue from [Share instance object records across definitions](96zzzj-share-instance-object-records-across-definitions.md). [Save or recover your work](recovery.md).

### 1. `src/controls.rs`

Move the checked method already written into this module position; the next edit changes its signature. The body is reused, not a supplied new algorithm.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
#[derive(Clone)]
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-01.rs"
```

</details>

### 2. `src/controls.rs`

Give the moved body a borrowed curve and index; remove the former self binding. Keep the source owner and existing public coordinate API.

<details>
<summary>Locate the existing block</summary>

```rust
use std::rc::Rc;
use session_rust::NurbsCurve;
use crate::marker::Marker;

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || self.index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = self.index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
    }
#[derive(Clone)]
pub struct Control { pub source: Rc<NurbsCurve>, pub index: usize }

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index }; control.position()?; Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || self.index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = self.index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
    }

    pub fn marker(&self) -> Result<Marker, &'static str> {
        let marker = Marker { center: self.position()?.map(|v| v as f32), diameter: 6.0, colour: [0.0, 0.0, 0.0, 1.0] };
        if marker.values().iter().any(|v| !v.is_finite()) { return Err("Control marker exceeds the display range"); }
        Ok(marker)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;
use session_rust::NurbsCurve;
use crate::marker::Marker;

pub(crate) fn position(c:&NurbsCurve,index:usize)->Result<[f64;3],&'static str> {
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
}

#[derive(Clone)]
pub struct Control { pub source: Rc<NurbsCurve>, pub index: usize }

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index }; control.position()?; Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        position(&self.source,self.index)
    }

    pub fn marker(&self) -> Result<Marker, &'static str> {
        let marker = Marker { center: self.position()?.map(|v| v as f32), diameter: 6.0, colour: [0.0, 0.0, 0.0, 1.0] };
        if marker.values().iter().any(|v| !v.is_finite()) { return Err("Control marker exceeds the display range"); }
        Ok(marker)
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-borrowed-signature.rs"
```

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-02.rs"
```

</details>

### 3. `src/curve.rs`

Remove the second rational-coordinate decoder. Reuse checked coordinates without an Rc clone, retaining the sampler’s existing extent error.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    let control = |index: usize| {
        let offset = index * source.m_cv_stride;
        let weight = if source.m_is_rat { source.m_cv[offset + source.m_dim] } else { 1.0 };
        std::array::from_fn::<_, 3, _>(|axis| if axis < source.m_dim { source.m_cv[offset + axis] / weight } else { 0.0 })
    };
    for index in 1..source.m_cv_count {
        let a = control(index - 1); let b = control(index);
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
```

</details>

Replace it with:

```rust
pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    for index in 1..source.m_cv_count {
        let a = crate::controls::position(source,index-1).map_err(|_|"Curve control extent exceeds the display range")?; let b = crate::controls::position(source,index).map_err(|_|"Curve control extent exceeds the display range")?;
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
```

<details>
<summary>Complete updated block reference</summary>

```rust
use session_rust::NurbsCurve;

pub fn validate(source: &NurbsCurve) -> Result<(f64, f64), &'static str> {
    if !(2..=3).contains(&source.m_dim) || !(2..=10_000).contains(&source.m_cv_count)
        || !(2..=32).contains(&source.m_order) || source.m_order > source.m_cv_count {
        return Err("Curve dimension, count or order is invalid");
    }
    let size = source.m_dim + usize::from(source.m_is_rat);
    if source.m_cv_stride < size { return Err("Curve control stride is too short"); }
    let end = (source.m_cv_count - 1).checked_mul(source.m_cv_stride)
        .and_then(|offset| offset.checked_add(size)).ok_or("Curve control range overflows")?;
    if source.m_cv.len() < end { return Err("Curve control values are incomplete"); }
    for index in 0..source.m_cv_count {
        let start = index * source.m_cv_stride;
        let values = &source.m_cv[start..start + size];
        if values.iter().any(|v| !v.is_finite()) { return Err("Curve controls must be finite"); }
        if source.m_is_rat && values[source.m_dim] <= 0.0 { return Err("Curve display requires positive rational weights"); }
    }
    let count = source.m_cv_count + source.m_order - 2;
    if source.m_nurbsknot.len() != count || source.m_nurbsknot.iter().any(|v| !v.is_finite())
        || source.m_nurbsknot.windows(2).any(|pair| pair[0] > pair[1]) {
        return Err("Curve knot vector is incomplete or unordered");
    }
    let domain = source.domain();
    if domain.0 >= domain.1 || !(domain.1 - domain.0).is_finite() { return Err("Curve domain must have positive length"); }
    Ok(domain)
}

pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    for index in 1..source.m_cv_count {
        let a = crate::controls::position(source,index-1).map_err(|_|"Curve control extent exceeds the display range")?; let b = crate::controls::position(source,index).map_err(|_|"Curve control extent exceeds the display range")?;
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
        if !length.is_finite() { return Err("Curve control extent exceeds the display range"); }
        let unit = direction.map(|v| v / length);
        if let Some(before) = previous {
            let dot = (0..3).map(|axis| before[axis] * unit[axis]).sum::<f64>().clamp(-1.0, 1.0);
            turning += dot.acos().to_degrees();
        }
        previous = Some(unit);
    }
    let count = ((turning / 3.0).ceil() as usize).clamp(1, 512);
    let mut parameters: Vec<_> = (0..=count).map(|i| start + (end - start) * i as f64 / count as f64).collect();
    parameters.extend(source.m_nurbsknot.iter().copied().filter(|&t| t > start && t < end));
    parameters.sort_by(f64::total_cmp); parameters.dedup();
    let mut points = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        let point = source.point_at(parameter); let point: [f64; 3] = std::array::from_fn(|axis| point[axis]);
        if point.iter().any(|&v| !(v as f32).is_finite()) { return Err("Sampled curve exceeds the display range"); }
        points.push(point);
    }
    Ok(points)
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzk-reuse-checked-curve-control-coordinates-04.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.8.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original curve samples and control markers retain their checked positions through one coordinate decoder.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for reuse checked curve control coordinates.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The original curve sampler and control owner both decode the same rational coordinates. Extract the existing checked calculation into a borrowed helper. The control owner keeps its Rc; the sampler calls the helper without allocating another owner. Keep curve-domain, positive-weight and sampling guards in their existing owner.

original curve and index → shared checked control coordinate → sampling or marker display.



Why keep curve validation separate?

A valid control coordinate does not establish a valid curve domain, knot vector or display sampling policy. Each caller retains those different obligations.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Draw a rational arc and enable its controls. Move one original control, Undo and confirm the restored arc. Inspect the retained supplied malformed-control and invalid-curve fixtures; no duplicate coordinate decoder remains.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.8.5
npm --prefix ../session_tests run course -- save 20.8.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This consolidates the earlier sampler/control coordinate calculation. The general kernel control collector keeps its own broader geometry policy; original control identity and edit ownership remain unchanged.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.8.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
