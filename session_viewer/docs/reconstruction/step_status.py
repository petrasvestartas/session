#!/usr/bin/env python3
"""Render `step-checks.json` into each lesson as one measured sentence.

The reader's question is "after this step, can I build?". `step_checks.py` answers it by
running cargo check at the end of every step; this writes the answer into the lesson between
the `step-status` markers, so the sentence is never a guess and never goes stale.
"""
import argparse
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
DOCS = HERE.parent
START = "<!-- step-status: start -->"
END = "<!-- step-status: end -->"
NUMBER = re.compile(r"^(?:Step|Part) (\S+?)(?: ·|:| )")


def ranges(numbers):
    """[1,2,3,5,6] -> '1-3, 5 and 6'; a run of two stays two items."""
    if not numbers:
        return ""
    runs, run = [], [numbers[0]]
    for n in numbers[1:]:
        if isinstance(n, int) and isinstance(run[-1], int) and n == run[-1] + 1:
            run.append(n)
        else:
            runs.append(run); run = [n]
    runs.append(run)
    items = []
    for r in runs:
        if len(r) > 2:
            items.append(f"{r[0]}\u2013{r[-1]}")
        else:
            items.extend(str(x) for x in r)
    if len(items) == 1:
        return items[0]
    return ", ".join(items[:-1]) + " and " + items[-1]


def label(name):
    m = NUMBER.match(name)
    if not m:
        return name
    raw = m.group(1)
    return int(raw) if raw.isdigit() else raw


def sentence(entry):
    order = entry["order"]
    results = entry["results"]
    ok = [label(n) for n in order if results[n] == "ok"]
    bad = [label(n) for n in order if results[n] != "ok"]
    if not bad:
        return ("**Does it compile yet?** Yes, after every step of this lesson — `cargo check` was run "
                "at the end of each one to make sure. A step that writes a file Rust has not been told "
                "about yet compiles without checking any of it, so keep going to the checkpoint: that "
                "build is the real test.")
    passes = (f"passes after step{'s' if len(ok) != 1 else ''} {ranges(ok)}, and "
              if ok else "passes at no point during this lesson, and ")
    # For each failing step, the first later step that builds again: "when would it work".
    labels = [label(n) for n in order]
    groups, run = [], []
    for i, name in enumerate(order):
        if results[name] != "ok":
            run.append(i)
            continue
        if run:
            groups.append((run, labels[i])); run = []
    if run:
        groups.append((run, None))
    recovery = []
    for idxs, nxt in groups:
        which = ranges([labels[i] for i in idxs])
        word = "steps" if len(idxs) > 1 else "step"
        recovery.append(f"{word} {which} build again at step {nxt}" if nxt else
                        f"{word} {which} stay incomplete to the end, where the checkpoint build takes over")
    return (f"**Does it compile yet?** `cargo check` {passes}"
            f"fails after {ranges(bad)}: a file is written across several steps, and a check can only "
            f"pass once its last piece is in. Concretely, {'; '.join(recovery)}. This is measured at the "
            "end of every step rather than guessed. And where a check passes while your new files are not "
            "yet named by a `mod` line, it is telling you only that you have not broken the previous "
            "checkpoint — the checkpoint build at the end of the lesson is the real test.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if a lesson is out of date")
    args = parser.parse_args()
    data = json.loads((HERE / "step-checks.json").read_text())
    stale = []
    for step_id, entry in sorted(data.items()):
        lesson = next(p for p in DOCS.glob(f"{step_id}-*.md"))
        text = lesson.read_text()
        block = f"{START}\n\n{sentence(entry)}\n\n{END}"
        if START in text:
            new = re.sub(re.escape(START) + r".*?" + re.escape(END), block, text, flags=re.S)
        else:
            anchor = "\n## Step 1" if "\n## Step 1" in text else "\n## Part A"
            new = text.replace(anchor, "\n" + block + "\n" + anchor, 1)
        if new != text:
            if args.check:
                stale.append(lesson.name)
            else:
                lesson.write_text(new)
                print("updated", lesson.name)
    if stale:
        raise SystemExit("step status out of date: " + ", ".join(stale))
    print("step status current" if args.check else "done")


if __name__ == "__main__":
    main()
