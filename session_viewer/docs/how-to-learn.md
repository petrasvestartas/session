# How to use this course

The black triangle in the top-right corner switches between the viewer and these docs.

## Each step

1. Read the one or two lines above the code: what the file is for.
2. Type the code.
3. Run `cargo check --lib` where the lesson says. Until then the build may be red: a file is often written across several steps. Every finished lesson is a crate in `docs/lessons/<id>/` to diff against when you are lost.
4. At the end of the lesson, build with `trunk serve --port 8780` and compare the page with the screenshot.

## Code block labels

Every code block is preceded by one line naming the lesson crate file, the line range shown and what to do with it:

- **type this, replace the whole file / append at the end of the file / new file** — write the shown lines by hand at that place.
- **replace lines A–B of `lessons/<parent>/…`** — the shown lines take the place of that range in the previous lesson's file.
- **delete lines A–B of `lessons/<parent>/…`** — remove them.
- **copy the file** — a fixture or page; copy it from the lesson crate.

The whole file as it stands at the end of the lesson is in `docs/lessons/<id>/`, for comparing with yours.

A Rust file enters the build only when a `mod` line names it; the bigger lessons create files first and declare them at the end.

## Stuck

- Read the error. [Reading failures](debugging.md) covers the ones this course produces.
- Compare your file with the one in `docs/lessons/<id>/`.
- A word you do not know: [Words before code](words.md).
