# 15.1.40 · Advance and cancel curve picks

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter. Accept defaults and typed answers through one progress state.

## Type

Continue from [Track ordered curve picks](77zze-track-ordered-curve-picks.md). [Save or recover your work](recovery.md).

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

<details>
<summary>Locate the existing block</summary>

```rust
    /// Every row picked so far, in step and pick order.
    pub fn picked(&self) -> Vec<u32> {
        self.rows.iter().flatten().copied().collect()
    }
```

</details>

After that block, type:

```rust
    /// Pick `row` for the current Curves step; picking it again takes it back.
    pub fn pick(&mut self, steps: &[Step], row: u32) -> Result<(), String> {
        let Some(Step::Curves { max, .. }) = steps.get(self.step) else {
            return Err("Not picking curves now".into());
        };
        let rows = &mut self.rows[self.step];

        if let Some(at) = rows.iter().position(|r| *r == row) {
            rows.remove(at);
            return Ok(());
        }

        if self.rows.iter().flatten().any(|r| *r == row) {
            return Err("That curve is picked already".into());
        }

        self.rows[self.step].push(row);

        if self.rows[self.step].len() >= *max {
            self.step += 1;
            self.skip(steps);
        }

        Ok(())
    }

    /// Enter: a Curves step with enough picks or a Number step's default moves on.
    pub fn next(&mut self, steps: &[Step], answers: &mut Vec<Answer>) -> Result<(), String> {
        match steps.get(self.step) {
            Some(Step::Curves { min, .. }) if self.rows[self.step].len() < *min => {
                Err(format!("Select at least {}", count(*min, "curve")))
            }
            Some(Step::Curves { .. }) => {
                self.step += 1;
                self.skip(steps);
                Ok(())
            }
            Some(Step::Number { default, .. }) => {
                self.answer(steps, answers, Answer::Number(*default));
                Ok(())
            }
            Some(step) => Err(format!("Give the {}", step.prompt())),
            None => Ok(()),
        }
    }

    /// Answer the current step.
    pub fn answer(&mut self, steps: &[Step], answers: &mut Vec<Answer>, answer: Answer) {
        answers.truncate(self.answered);
        answers.push(answer);
        self.answered += 1;
        self.step += 1;
        self.skip(steps);
    }

    /// True once every step is answered.
    pub fn done(&self, steps: &[Step]) -> bool {
        self.step >= steps.len()
    }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzf-advance-and-cancel-curve-picks-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.40
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

![Native renderer output for advance and cancel curve picks.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curve picks → ordered answers → Session kernel recipe.



Why truncate answers before adding a new one?

The current progress position defines the accepted prefix; a replacement must discard later stale answers.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace picking the same row twice, picking it in another group and pressing Enter below the minimum.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.40
npm --prefix ../session_tests run course -- save 15.1.40
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
