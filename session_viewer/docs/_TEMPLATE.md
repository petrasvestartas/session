# NN Title — one-line subtitle

<!--
The house form for a lesson (files starting with "_" never show in the sidebar).
- Prose is minimal: one or two sentences per step, the WHY once, no war stories.
- Every edit is an op the checker can replay: Create / Find+Replace with / Add below it /
  Add above it / Delete / Remove … through … / Replace-all … (N hits). Anchors are whole,
  unique lines quoted from the tree the reader has at that step.
- Verify before shipping (from session_viewer/):
    python3 docs/_replay_check.py <prev-snapshot> /tmp/v docs/NN-*.md   # 0 failed
    python3 docs/_replay_check.py --audit docs/NN-*.md                  # 0 orphaned
    python3 docs/_replay_check.py --render docs/NN-*.md                 # page renders
    diff -r /tmp/v/src <end-snapshot>/src                               # empty
-->

<svg viewBox="0 0 640 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="what this lesson builds" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <text x="320" y="60" fill="#888" text-anchor="middle">one diagram of what this lesson builds</text>
</svg>

## Goal

Two sentences: what the viewer can do (or how the code reads) after this lesson.

## Why

Three sentences at most.

## Files

| file | change | lines after |
|---|---|---|
| `src/…` | created / edited / deleted | 0 |

## Step 1 — `src/path.rs`

One or two sentences, then the ops.

**Create `src/path.rs`**

```rust
```

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown
cargo check --all-targets --target x86_64-unknown-linux-gnu
cargo xtest
./docs/_gate.sh
```

The numbers these print, measured twice.

## Recap

- three bullets

## Next

Lesson [NN+1](NN+1-title.md) — one line.
