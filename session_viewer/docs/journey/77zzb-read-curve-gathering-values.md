# 15.1.36 · Read curve gathering values

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Read finite numbers, absolute points and distance vectors through the existing coordinate reader. Reject curve picks supplied as words.

## Type

Continue from [Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). [Save or recover your work](recovery.md).

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

<details>
<summary>Locate the existing block</summary>

```rust
    }, // low < value <= high; Enter takes the default
    Distance(&'static str), // along the recipe's axis: a click, a signed number or a typed vector
}
```

</details>

After that block, type:

```rust
impl Step {
    /// What the step asks for.
    pub fn prompt(&self) -> &'static str {
        match self {
            Step::Curves { prompt, .. } | Step::Number { prompt, .. } => prompt,
            Step::Point(prompt) | Step::Distance(prompt) => prompt,
        }
    }

    /// True for a step answered by picking curves.
    pub fn curves(&self) -> bool {
        matches!(self, Step::Curves { .. })
    }

    /// A typed word as this step's answer.
    pub fn read(&self, word: &str) -> Result<Answer, String> {
        match self {
            Step::Number {
                prompt, low, high, ..
            } => match typed_number(word) {
                Some(value) if value > *low && value <= *high => Ok(Answer::Number(value)),
                Some(_) => Err(format!(
                    "The {prompt} must be above {low} and at most {high}"
                )),
                None => Err(format!("`{word}` is not a number")),
            },
            Step::Point(_) => match coords::absolute(word) {
                Some([x, y, z]) => Ok(Answer::Point(Point::new(x, y, z))),
                None => Err(format!("`{word}` is not a point x,y,z")),
            },
            Step::Distance(_) => match (typed_number(word), coords::parse_world(word)) {
                (Some(value), _) => Ok(Answer::Number(value)),
                (None, Some((coords::Typed::Absolute { x, y, z }, world))) => {
                    let [x, y, z] = coords::offset([x, y, z.unwrap_or(0.0)], world);
                    Ok(Answer::Vector(Vector::new(x, y, z)))
                }
                _ => Err(format!("`{word}` is not a distance or a vector x,y,z")),
            },
            Step::Curves { .. } => Err("Click the curves".into()),
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzb-read-curve-gathering-values-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.36
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

![Native renderer output for read curve gathering values.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curve picks → ordered answers → Session kernel recipe.



Why does a distance accept either a number or a vector?

A number follows the recipe axis; a vector states the displacement directly.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try a finite angle, an out-of-range angle and a distance vector. Predict the accepted answer variants.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.36
npm --prefix ../session_tests run course -- save 15.1.36
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
