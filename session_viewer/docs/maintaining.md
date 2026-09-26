# Maintaining the course

Not part of the course. This page says how the lesson crates are made and which checks keep them honest.

## One chain, cut from one master

The course is one chain of 43 crates, `00` to `37`, listed in order in `docs/lessons/SERIES.txt` (`<id> <parent> <page>`). Letter ids are lessons inserted where their subject belongs: 04a-04d, 18a (instancing), 18b (clipping), 23a-23d (tools, shapes, surfacing, annotate and measure). The numbers 27, 28, 29 and 34 are retired; their code moved to the lessons that own its subject (the deform helpers to 21, saving and opening to 16, the ui split to 22).

`docs/lessons/37` is the master: the viewer `src` byte for byte, plus teaching comments and section markers. Edit the master, never an earlier crate. Every other crate is cut from it by `python3 docs/cut.py`:

- A line belongs to lesson NN when it sits inside a section named `NN-<slug>`, marked `// --8<-- [start:NN-slug]` and `// --8<-- [end:NN-slug]` (`#` in TOML, YAML, shell and Python, `<!-- -->` in HTML). A section whose name has no lesson prefix is a page step inside the lesson around it.
- A single line carrying `register:<tag>` belongs to the lesson `docs/lessons/REGISTER.txt` gives that file and tag; an attribute line such as `#[cfg(...)]` directly above it follows it. Registration lines are how a later lesson adds a module, a command or a pass to a list an earlier lesson wrote.
- Every other line belongs to the file's first lesson in `docs/lessons/FILES.txt`. Tests, examples, assets and HTML are whole files.
- Crate N holds every file whose first lesson is N or earlier, with the lines of lessons up to N in master order, runs of blank lines collapsed, and `lessons/37/` and `Checkpoint 37` renamed to N.

So crate N+1 is crate N plus new files, lines appended at the end of a file, and registration lines. A lesson never replaces what an earlier lesson wrote, by construction.

## Adding to the course

A feature goes to the lesson that owns its subject, as one new file plus registration lines (see "Adding a feature" in `ARCHITECTURE.md`). Change `src`, copy the change into `lessons/37`, give the new lines a section or a `register:` tag, list new files in `FILES.txt` and new tags in `REGISTER.txt`, then run `cut.py`. Only a subject no lesson owns earns a new lesson, with a letter id at its place in `SERIES.txt`.

## Checks

Run them in this order after any change to the master or a page:

```sh
python3 docs/cut.py                 # write every crate from the master
python3 docs/cut.py --check         # compare with the saved raw cuts, comments stripped
python3 docs/check_write_once.py    # must print 0
python3 docs/check_lesson37.py      # lessons/37 equals src with comments stripped
docs/serve.sh build                 # a missing include file or section fails the build by name
python3 docs/diagrams.py --check    # the committed D2 diagrams are current
```

`cut.py --check` reads its baseline from `~/.cache/viewer-push/recut/raw/<id>/`, so it runs only on the machine that holds it; it proves that annotating the master never changed what a lesson contains.

Each crate must pass `cargo check --lib` inside its directory, and `cargo check --tests --examples` natively where it has tests or examples. The crates share one package name, so check them one at a time.

## Pages

Lesson code never lives in the Markdown. A step is a fenced block whose only line is `--8<--` followed by the quoted path and section, for example `lessons/18a/src/engine/gpu/instanced.rs:instanced-gpu`; the build replaces it with the section's lines, markers removed. Only the two kernel listings still use line numbers, because the kernel is never annotated. Illustrations come from `docs/illustrations/draw.py`; it rewrites every SVG, so re-pin the text afterwards with `python3 docs/check_svg.py --write docs/illustrations/*.svg`. `docs/diagrams.py` renders the D2 flowcharts and `theme.py` renders `theme.css`.
