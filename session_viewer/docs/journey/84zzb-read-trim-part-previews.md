# 16.4.16 · Read trim part previews

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Expose part counts, sampled curve outlines, section edges and meshes through one Parts owner. These helpers prepare later pointer picking.

## Type

Continue from [Describe removable CAD parts](84zza-describe-removable-cad-parts.md). [Save or recover your work](recovery.md).

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

<details>
<summary>Locate the existing block</summary>

```rust
/// One cutter and, for a straight line, the plane it stands for off the object.
pub struct Blade {
    pub cutter: Cutter,       // in the target's frame
    pub fence: Option<Plane>, // a line's vertical plane, in the target's frame
}
```

</details>

After that block, type:

```rust
impl Parts {
    /// How many parts there are.
    pub fn count(&self) -> usize {
        match self {
            Parts::Curve { curve, cuts } if curve.is_closed() => cuts.len(),
            Parts::Curve { cuts, .. } => cuts.len() + 1,
            Parts::Faces { groups, .. } => groups.len(),
            Parts::Mesh { sides, .. } => sides.len(),
        }
    }

    /// Part `index` of a curve as points, for drawing and clicking.
    pub fn outline(&self, index: usize) -> Vec<Point> {
        let Parts::Curve { curve, cuts } = self else {
            return Vec::new();
        };
        let (a, b) = interval(curve, cuts, index);
        piece(curve, a, b).map_or_else(Vec::new, |part| samples(&part, 48))
    }

    /// The points where a curve was cut.
    pub fn cut_points(&self) -> Vec<Point> {
        match self {
            Parts::Curve { curve, cuts } => cuts.iter().map(|t| curve.point_at(*t)).collect(),
            _ => Vec::new(),
        }
    }

    /// The cut edges as polylines.
    pub fn section(&self) -> Vec<Vec<Point>> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { section, .. } => section.clone(),
            Parts::Mesh { section, .. } => section.iter().map(|pair| pair.to_vec()).collect(),
        }
    }

    /// The meshes a click can hit, with the part each belongs to.
    pub fn hit_meshes(&self) -> Vec<(usize, &Mesh)> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { groups, meshes, .. } => groups
                .iter()
                .enumerate()
                .flat_map(|(part, faces)| {
                    faces
                        .iter()
                        .filter_map(move |f| Some((part, meshes.get(*f)?)))
                })
                .collect(),
            Parts::Mesh { sides, .. } => sides.iter().enumerate().collect(),
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84zzb-read-trim-part-previews-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.16
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read trim part previews.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why return face meshes with their part index?

Several faces can belong to one removable part; the click must recover that group.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a two-face group and identify the part returned by either face mesh.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.16
npm --prefix ../session_tests run course -- save 16.4.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
