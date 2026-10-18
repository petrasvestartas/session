# 17.2.2 · Describe clipping construction choices once

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

Each construction mode declares its name, required point count and prompts in one owner.

## Type

Continue from [Describe original clipping half-spaces](87-copy-current-original-plane-modes.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_geometry.rs`

Describe clipping construction choices once.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClipPlane {
    pub normal: [f64; 3], // unit normal toward the kept side
    pub offset: f64,      // kept where normal · p + offset >= 0
    pub origin: [f64; 3], // a point on the plane, where the hatch starts
    pub hatch: [f64; 3],  // unit direction across the hatch lines, in the plane
}

impl ClipPlane {
    /// Signed distance of world point `p`; below zero is cut away.
    pub fn distance(&self, p: [f64; 3]) -> f64 {
        dot(self.normal, p) + self.offset
    }


}
```

</details>

After that block, type:

```rust
/// How a clipping plane is placed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal, // an origin, then a point on the side to cut away
    Points, // three points; the cut side by the right-hand rule
    Xy,     // the world XY plane through a point, +Z cut away
    Yz,     // the world YZ plane through a point, +X cut away
    Zx,     // the world ZX plane through a point, +Y cut away
}

impl Mode {
    /// Every mode, in the order the options list them.
    pub const ALL: [Mode; 5] = [Mode::Normal, Mode::Points, Mode::Xy, Mode::Yz, Mode::Zx];

    /// The word typed for it.
    pub fn word(self) -> &'static str {
        match self {
            Mode::Normal => "Normal",
            Mode::Points => "3Point",
            Mode::Xy => "XY",
            Mode::Yz => "YZ",
            Mode::Zx => "ZX",
        }
    }

    /// Points the mode takes.
    pub fn points(self) -> usize {
        match self {
            Mode::Normal => 2,
            Mode::Points => 3,
            _ => 1,
        }
    }

    /// The mode a typed word names, any case.
    pub fn parse(word: &str) -> Option<Mode> {
        Mode::ALL
            .into_iter()
            .find(|mode| mode.word().eq_ignore_ascii_case(word))
    }

    /// What each picked point is for.
    pub fn prompts(self) -> &'static [&'static str] {
        match self {
            Mode::Normal => &[
                "First point: a point on the cutting plane (or choose XY, YZ, ZX, 3Point)",
                "Second point: toward the side to remove; origin-to-point sets the perpendicular direction (e.g. @0,0,1 removes above)",
            ],
            Mode::Points => &[
                "First point",
                "Second point, along the plane",
                "Third point: the cut side follows the right-hand rule",
            ],
            Mode::Xy => &["Origin point: the +Z side is cut away"],
            Mode::Yz => &["Origin point: the +X side is cut away"],
            Mode::Zx => &["Origin point: the +Y side is cut away"],
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/87sa-describe-clipping-construction-choices-once-typed-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.2
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

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for describe clipping construction choices once.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Normal uses an origin and a point toward the cut side. Three-point construction uses a right-handed frame. XY, YZ and ZX each need only an origin. Reuse this same mode table for command choices and input prompts; the following constructor consumes it.

original Plane → checked construction and placement → world half-space → later GPU uniform.



Why keep required point counts with the modes?

Parsing, prompting and construction must agree about when input is complete. One table prevents separate command and geometry rules from drifting.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the case-insensitive word 3point to its mode, required count and third prompt.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.2
npm --prefix ../session_tests run course -- save 17.2.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original-plane preparation; no clipping pass is connected at this checkpoint.

[Full validation scope](release.md).

</details>
