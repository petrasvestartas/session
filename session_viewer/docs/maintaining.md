# Maintaining this site

Not part of the course. Build and check:

```sh
docs/serve.sh build
docs/serve.sh
```

- Each lesson is a runnable crate in `docs/lessons/<id>/`; `docs/lessons/SERIES.txt` lists every id, the id it builds on and the page that teaches it. Edit the crate and the page directly.
- Lesson code never lives in the Markdown. A step names a file in `docs/lessons/<id>/` and a line range; the page includes those lines with a `--8<--` snippet, so editing the lesson crate edits the page. When an edit moves lines, update the range in the page by hand.
- `docs/serve.sh build` fails on a snippet path that does not exist; that is the only site check. `cargo check` inside a lesson directory proves the lesson compiles.
- `diagrams.py` renders the D2 flowcharts; `check_svg.py` and `check_illustrations.cjs` check the illustrations; `theme.py` renders `theme.css`.
