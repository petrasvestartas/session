# Maintaining this site

Not part of the course. Build and check:

```sh
docs/serve.sh build
docs/serve.sh
```

- Each lesson is a runnable crate in `docs/lessons/<id>/`; `docs/lessons/SERIES.txt` lists every id, the id it builds on and the page that teaches it.
- `docs/lessons/37` is the master: the viewer `src` plus teaching comments and `// --8<-- [start:NN]` .. `[end:NN]` sections. `src/` equals it with the marker lines dropped (`python3 docs/check_lesson37.py` proves it). Edit the master, never an earlier crate.
- `python3 docs/cut.py` writes every other crate from the master. `docs/lessons/FILES.txt` gives each file its first lesson; `docs/lessons/REGISTER.txt` maps each `register:<tag>` to a lesson. A line belongs to its section's lesson, else its tag's lesson, else its file's first lesson.
- Write-once: a lesson only adds lines. A line a later lesson inserts mid-file carries `// register:<tag>` or sits in a trailing section; `python3 docs/check_write_once.py` must report 0. `python3 docs/cut.py --check` compares every cut with the saved raw cuts, comments stripped.
- Lesson code never lives in the Markdown. A step names a file in `docs/lessons/<id>/` and a line range; the page includes those lines with a `--8<--` snippet, so editing the lesson crate edits the page. When an edit moves lines, update the range in the page by hand.
- `docs/serve.sh build` fails on a snippet path that does not exist; that is the only site check. `cargo check` inside a lesson directory proves the lesson compiles.
- `diagrams.py` renders the D2 flowcharts; `check_svg.py` and `check_illustrations.cjs` check the illustrations; `theme.py` renders `theme.css`.
