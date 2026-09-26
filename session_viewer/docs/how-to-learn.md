# How to use this course

The black triangle in the top-right corner switches between the viewer and these docs.

## Each step

1. Read the sentences above the code: what we build next and why.
2. Type the code at the place the reference line names.
3. Run `cargo check --lib` where the lesson says. Until then the build may be red, because a file is often written across several steps.
4. Where the page says so, run `trunk serve --port 8780` and compare the browser with the description.

## The line above each code block

Every code block follows one line naming the lesson crate file and what to do with it:

- **type this, new file**: create the file and type the block.
- **type this, append at the end of the file**: add the block after everything already in that file.
- **type the line tagged `register:<name>`** (or "one line in `fn x`"): add that single line where its neighbours are; it is how a new file joins a list an earlier lesson wrote, such as the command registry or the list of modules.
- **copy the file** or **copy, append at the end of the file**: tests, examples, assets and ports of kernel code; take them from the lesson crate instead of typing.
- **read**: code shown to explain something, already copied in the lesson named.

Nothing is ever replaced or deleted. A lesson only adds files, appends to the end of a file, or adds tagged registration lines, so whatever you typed in an earlier lesson stays as it is to the end.

A Rust file enters the build only when a `mod` line names it; the bigger lessons create files first and declare them at the end.

## Lesson crates

Every lesson is a complete crate in `docs/lessons/<id>/` that builds on its own: the state of the viewer at the end of that lesson. Compare your files with it when you are lost; `diff -r docs/lessons/18 docs/lessons/18a` shows exactly what lesson 18a adds. `docs/lessons/37` is the finished viewer.

## Stuck

- Read the error. [Reading failures](debugging.md) covers the ones this course produces.
- Compare your file with the one in `docs/lessons/<id>/`.
- A word you do not know: [Words before code](words.md).
