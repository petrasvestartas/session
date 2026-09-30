# How to use this course

This page describes the complete implementation reference. For the new smaller lessons and their availability, start with [the course route](journey.md).

Keep one project and build it yourself. Read the explanation, make a prediction, type the addition, then explain what happened. Do not measure progress only by lines typed. If you can change an input and predict the effect, you are beginning to understand the code.

## Keep the whole viewer in sight

Start with [the viewer map](map.md). Follow the three stories: open an object, orbit around it, then select and edit it. Each section’s opening explains which part of those stories you are building and points to the key files.

Before a study session, write one sentence about the file’s input and output. Afterwards, explain the connection without the listing and make a prediction for the section’s experiment. If that is difficult, revisit the unclear connection before adding more code. You do not need to memorize every API name to understand the architecture.

Use the [time estimates](map.md#time-estimates) to plan, then adjust them to your own measured pace. Taking longer than an estimate says nothing about your ability to learn this.

## Start with small programs

Complete the [eight Rust foundations](foundations.md) first. They introduce values, records, lists, ownership, errors, state machines, traits and byte layout in native programs you can run immediately. The notebook lives in `target/notebook`; it is separate from the viewer.

## Your one viewer project

From `session_viewer`, run this once:

```sh
npm --prefix ../session_tests run course -- reference-init
```

This creates `workspace/handwritten`. It copies only the dependency lock, binary fixtures, fonts and their accompanying license/readme. It connects the generated documentation site for the folded-corner link. It copies **no viewer source, shader, configuration, example or test**. Keep the project at this path so the relative geometry-kernel dependency resolves correctly.

Do not run `practice.py`: that was the earlier fill-in-one-region exercise system. The main course no longer uses it. Do not copy `docs/lessons` over your work. Those directories are complete reference checkpoints for comparison only.

## Complete one working step

Each tutorial page is one buildable step. Read its idea, then type all its additions in order. A file listing belongs to that step; it is not a separate lesson. Large steps take several sittings. Save your place at a file boundary and finish the step before expecting a build.

Type comments and preserve blank lines. End every text file with a newline. Later steps only add code; you do not replace earlier implementations.

After typing, run the source comparison from `session_viewer`, using the step number:

```sh
npm --prefix ../session_tests run course -- reference-check 00
```

This command is read-only. It reports missing files and differences without completing your work. In a difference, `expected/` is the reference and `your/` is your file. Fix the first mismatch and retry.

Then run the tutorial’s build and native-test commands inside `workspace/handwritten`. Check the stated result before continuing. A matching source file alone does not prove that a program runs.

Keep backups or commit your handwritten project to Git. The surrounding repository ignores `workspace`.

## What should work?

Step 00 builds a WebAssembly entry point that loads in the browser without drawing. Step 01 adds the renderer and a native example that saves its first frame to `lesson.ppm`. The subsequent steps add complete parts of the viewer and test their behavior.

The browser event loop arrives in step 12 and scene startup in step 14. Before then, use the native tests and image examples named on the page. From step 14, run `trunk serve --port 8780` in your handwritten project and open http://localhost:8780/?data=off.

Native GPU tests run serially. Tests explicitly marked ignored are not counted as passing. Follow each tutorial’s experiment as well: compilation cannot tell you whether a drag, selection or command behaves as intended.

## What do you type?

All viewer implementation, shaders, configuration, HTML, examples and tests. External crates, the geometry kernel, fonts, binary models, their licenses and generated build output remain supplied inputs. Vue hosts these pages; it is a separate project.

The final Rust/WGSL implementation matches the maintained viewer. Teaching comments, local paths, the test-thread setting and the small course scene are intentional differences.

## Rust symbols you will meet

| Rust | Read it this way |
| --- | --- |
| `let value = ...;` | Name a value; it cannot be reassigned by default. |
| `let mut value = ...;` | Name a value and allow changes. |
| `fn name(input: Type) -> Output` | Receive one type and return another. |
| `&value` / `&mut value` | Borrow for reading / borrow with permission to change it. |
| `struct` / `impl Type` | A record with fields / methods belonging to a type. |
| `self` / `Self` | This value / the type we are implementing. |
| `Option<T>` | Either `Some(value)` or `None`. |
| `Result<T, E>` | Either `Ok(value)` or `Err(error)`. |
| `?` | Return an error or an absent result to the caller. |
| `Vec<T>` / `[T; 3]` | A growable list / exactly three elements. |
| `0..3` / `0..=3` | 0, 1, 2 / 0, 1, 2, 3. |
| `..base` in a struct literal | Fill the remaining fields from another value. |
| `match` | Choose a branch based on the value’s form. |
| `use` / `mod` | Bring a name into scope / include a source module. |
| `value.method()` | Ask that value to perform a method. |
| `|value| expression` | A small function passed to another operation. |
| `#[...]` | An instruction to the compiler or code-generation tool. |
| `async` / `.await` | An operation may wait; resume when its result is ready. |

A function’s final expression can return its value without `return`. A semicolon discards an expression’s result, so adding one at the end can change what a function returns.

Rust’s borrow checker protects ownership. When it rejects overlapping access, read which borrow must finish before the next starts. The official [borrowing chapter](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) explains this in more detail.

## Rust, WGSL and wgpu

Rust runs the application on the CPU. WGSL runs shaders on the GPU. Wgpu is the Rust library used to create GPU resources and submit work. A Rust struct and a WGSL struct do not automatically share a layout: their offsets, sizes and alignment must agree.

A device creates resources. A queue accepts uploads and recorded commands. An encoder records commands. A render pass draws into attachments. A pipeline connects shaders with the formats and rules they expect. A bind group supplies resources to numbered shader slots. We revisit these names with examples rather than expecting you to memorize them now.

Cargo.lock resolves wgpu 29.0.4. Calls copied from another release can fail even when their names look familiar.

## Read errors as evidence

Read the first error before the cascade that follows it. For an unknown name, check its declaration and module registration. For a borrow error, identify the owner and the two conflicting accesses. For a blank render, check shader validation and draw inputs; successful compilation alone is insufficient.

Diagrams introduce an idea once. Screenshots show actual rendered results where available; they do not replace the selectable code listings or your own checks.

[Reading failures](debugging.md) and [Words before code](words.md) provide further help.

[Find where each file is taught](file-map.md) when you want to return to a module.

Tap or click a diagram to open it at full size when a detailed figure is small on your screen.
