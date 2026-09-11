#!/usr/bin/env python3
"""Render `step-checks.json` into each lesson as one measured sentence.

The reader's question is "after this step, can I build?". `step_checks.py` answers it by
running cargo check at the end of every step; this writes the answer into the lesson between
the `step-status` markers, so the sentence is never a guess and never goes stale.

It says only what was MEASURED. Why a mid-lesson check can fail - a Rust file joins the build
only when a `mod` line names it - is the same sentence on every lesson, and how-to-learn.md
already carries it once. Saying it 24 times cost 60 words a lesson and taught nothing new.
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
    """A step's number, or a part's own name: a lesson whose last unit is "Part C" must not be
    described as reaching "step C", which names nothing the reader can find on the page."""
    m = NUMBER.match(name)
    if not m:
        return name
    raw = m.group(1)
    if raw.isdigit():
        return int(raw)
    return f"Part {raw}" if name.startswith("Part ") else raw


def sentence(entry):
    order = entry["order"]
    results = entry["results"]
    ok = [label(n) for n in order if results[n] == "ok"]
    bad = [label(n) for n in order if results[n] != "ok"]
    if not bad:
        return "**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson."
    # The noun counts only the NUMBERED units: a part carries its own noun in its label.
    numbered = sum(1 for x in ok if not isinstance(x, str))
    passes = (f"passes after step{'s' if numbered != 1 else ''} {ranges(ok)}; "
              if ok else "passes at no point during this lesson; ")
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
        many = len(idxs) > 1
        word, fail, build = ("steps", "fail", "build") if many else ("step", "fails", "builds")
        at = nxt if isinstance(nxt, str) and nxt.startswith("Part ") else f"step {nxt}"
        recovery.append(f"{word} {which} {fail} and {build} again at {at}" if nxt else
                        f"{word} {which} {fail} and stay incomplete until the checkpoint build")
    # Why a step can fail is the same on all 24 lessons, and how-to-learn.md already says it once.
    # Repeating it here cost 60 words a lesson; only the measured part belongs in the lesson.
    return f"**Does it compile yet?** `cargo check` {passes}{'; '.join(recovery)}."


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
