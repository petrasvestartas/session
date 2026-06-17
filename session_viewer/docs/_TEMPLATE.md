# NN · Title of this step

<!--
HOW TO ADD A SECTION (it becomes an item in the left sidebar under the Viewer tab):

1. Copy this file to a new one named  NN-title.md  in this folder
   (session_tests/viewer_sections/). The NN number sets the order, e.g. 02-grid.md, 03-camera.md.
   The first "# heading" becomes the sidebar label.
2. Write your notes as normal Markdown.
3. Paste code in fenced blocks with the language after the backticks. Highlighted by Shiki;
   supported languages: rust, cpp, python, json, bash, toml.

       ```rust
       fn main() { println!("hi"); }
       ```

4. Save → it appears in the sidebar at #/viewer (dev: hot-reloads instantly).

Files starting with "_" (like this one) are ignored, so this template never shows up.
-->

## Goal
One sentence: what the viewer can do after this step that it couldn't before.

## How it works
The idea in your own words — the *why*, not just the *what*.

## Code
```rust
// paste the key snippet from session_viewer/src/...  (file + line so you can jump back)
```

## My notes
> your reading notes — what each call does, what confused you, what you'd forget next week

## Compare to the archive
How `session_viewer_archive/` does the same thing, and what it adds on top.

## Run
```bash
../bash/build_viewer.sh   # rebuild the viewer into public/viewer/, then it shows in the Live item
```

## Verify
#/viewer → **Live** → what you should see on screen.
