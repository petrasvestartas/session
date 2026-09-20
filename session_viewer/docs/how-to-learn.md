# How to use this course

The black triangle in the top-right corner switches between the viewer and these docs.

## Each step

1. Read the one or two lines above the code: what the file is for.
2. Type the code.
3. Run `cargo check --lib` where the lesson says. The "Does it compile yet?" line at the top of each lesson says after which steps it is expected to pass.
4. At the end of the lesson, build with `trunk serve --port 8780` and compare the page with the screenshot.

## Code block labels

- **NEW FILE** — create the file with exactly this content.
- **CURRENT → REPLACE WITH** — find the first block in your file (it occurs once) and replace it with the second.
- **CURRENT → ADD BELOW** / **ADD ABOVE** — find the first block and insert the second after or before it.
- **DELETE** — remove the shown lines or the whole file.
- **TYPE THIS** — code to write by hand. **COPY** — a fixture or page; a download link is next to it.
- **finished file ↓** — the whole file as it stands at the end of the lesson, for comparing with yours.

A Rust file enters the build only when a `mod` line names it; the bigger lessons create files first and declare them at the end.

## Stuck

- Read the error. [Reading failures](debugging.md) covers the ones this course produces.
- Compare your file with its **finished file ↓** link.
- A word you do not know: [Words before code](words.md).
