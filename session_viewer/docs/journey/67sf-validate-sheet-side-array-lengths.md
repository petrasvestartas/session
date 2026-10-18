# 12.2.7 · Validate sheet side-array lengths

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 14–27 minutes.** [Estimate](typing-load.md).

SheetFields::set records each optional side array once.

## Type

Continue from [Find coordinate arrays without downloading their bodies](67se-find-coordinate-arrays-without-downloading-their-bodies.md). [Save or recover your work](recovery.md).

### 1. `src/stream.rs`

Validate sheet side-array lengths.

<details>
<summary>Locate the existing block</summary>

```rust
/// Start, length of `coords` and end of the sheet message.
pub fn sheet_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 17)?;
    first_array(head, at, end)
}

/// True for a file whose first bytes show neither a cloud nor a sheet: it loads whole.
pub fn plain(head: &[u8]) -> bool {
    cloud_layout(head).is_none() && sheet_layout(head).is_none()
}
```

</details>

After that block, type:

```rust


impl SheetFields {
    /// Bytes per segment: six doubles.
    pub const SEGMENT_BYTES: u64 = 48;

    /// Record one field found after `coords`; false when it is wrong.
    pub fn set(&mut self, f: &Field, body: &[u8]) -> bool {
        let per_segment = u64::from(self.count) * 4;

        match (f.field, f.wire) {
            (4, 2) if self.colors_len == 0 && f.value == per_segment => {
                (self.colors_at, self.colors_len) = (f.body, f.value)
            }
            (5, 2) if self.widths_len == 0 && f.value == per_segment => {
                (self.widths_at, self.widths_len) = (f.body, f.value)
            }
            (15, 2) if self.ids_len == 0 && f.value == per_segment => {
                (self.ids_at, self.ids_len) = (f.body, f.value)
            }
            (6, 0) => return f.value == u64::from(self.count),
            (7, 0) => match u32::try_from(f.value) {
                Ok(entities) => self.entities = entities,
                Err(_) => return false,
            },
            (8, 2) if f.value <= NAME_BYTES => match std::str::from_utf8(body) {
                Ok(meta) => self.meta = meta.to_string(),
                Err(_) => return false,
            },
            (1..=8, _) | (15, _) => return false,
            _ => {}
        }

        true
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sf-validate-sheet-side-array-lengths-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.7
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

The mixed source archive scene remains unchanged.

![Native renderer output for validate sheet side-array lengths.](../screenshots/journey/67-supply-checked-array-layout-readers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.2.1 · Bound scalar headers and byte addresses](67-supply-checked-array-layout-readers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Colours, widths and IDs must match the declared segment count, while metadata names remain bounded UTF-8. Reject duplicate known fields and wrong wire types; skip unknown fields to preserve forward compatibility. Actual sheet GPU drawing is connected at 71, using these original entity IDs.

original file bytes → checked source layout → existing GPU array owners → later range streaming.



Why must a colour field match the segment count?

Its fixed32 values index the same original segments as the coordinate array.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

For two segments, predict the accepted colour byte count. Try a duplicate colour field and an invalid UTF-8 metadata name.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.7
npm --prefix ../session_tests run course -- save 12.2.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Parser preparation preserves the inherited scene; actual browser streaming is exercised after the range queue is connected.

[Full validation scope](release.md).

</details>
