# Learn through our command line

**Every interactive checkpoint uses the viewer’s own Rust/egui command dock.** Type a command in the white field and press Enter. The lessons and browser checks use that route for features; picking and camera gestures still happen on the drawing.

The white canvas fills the browser window from lesson 01. The dock sits inside that canvas; lesson titles and explanations stay in these documentation pages.

Start with these lessons, in order:

| Lesson | What you understand afterward |
| --- | --- |
| [03a · Draw our command line](03a-panel.md) | Fonts, layout and the second GPU pass |
| [03b · Give it memory](03b-state.md) | Who owns text, suggestions and history |
| [03c · Draw completion and history](03c-layout.md) | How the production dock reads that state |
| [03co · Align the command row](03co-align.md) | Shared text alignment and usable dock bounds |
| [03d · Type into the dock](03d-input.md) | How browser keys become text and submitted lines |
| [04 · Change the picture](04-input.md) | How a submitted command changes application state |

Each page gives its typing estimate and complete source changes. The opening dock stages draw and organise the interface; keyboard input becomes usable in 03d. Each step stays within one hour of conservative typing.

## Follow one command

In lesson 04, type `Bac`. Completion suggests `Background`. Press Enter: the background changes while the triangle stays put. Submit `Background` again to return to white. The dock remembers both commands.

![A typed command passes through the real dock to application state, then the GPU draws the scene and interface.](../illustrations/journey-command-dock.svg)

The dock helps you write a request and keeps the answer. The application receives the completed line and changes the relevant value. Renderer then paints the new state.

| Part | Owns |
| --- | --- |
| Browser adapter | Event translation, coordinates and focus |
| Command dock | Edited text, completion and history |
| Command vocabulary | Accepted words and options |
| Application or Editor | Camera and document changes |
| GPU painters | Geometry and interface drawing |

The dock, Noto fonts and styling are the production implementation. Early vocabulary such as `Background`, `Pan Right` and `Example Box` is for teaching. `Example Box` creates a fixed specimen; later lessons build the production Box command and its arguments. Type `Help` to see what the current checkpoint accepts.

## Use the new production shortcuts

Click the drawing or press Escape to leave command text before using a letter shortcut. The current production viewer accepts:

| Key | Command and result |
| --- | --- |
| F | Fit the selection, or the whole scene when nothing is selected. |
| H | Hide the selected objects. |
| S | Show all hidden objects. |
| Delete | Delete the selection as one undoable edit. |
| Ctrl+Z / Cmd+Z | Undo a document edit. |
| Ctrl+Y / Cmd+Y | Redo the edit. |
| Ctrl+Shift+Z / Cmd+Shift+Z | Redo the edit. |

Try it on the production viewer: select one floor piece, press F, press H, then press S. Make an edit, undo it and redo it. The dock echoes the same commands used by typed requests. When the command field owns nonempty text, history keys edit that text instead of the document; F/H/S remain ordinary letters there. Press Enter once to run a completed command. Escape can also clear selection; click an object before using H or fitting selected geometry.

The cumulative implementation starts in [26a · Fit and step history from the keyboard](26a-keys.md). H/S join the implementation in the visibility chapters, once the learner owns hidden-object state. Earlier checkpoints accept only the shortcuts they have implemented.

## Review the working checkpoints

These local links require the course bundle server:

- [Type your first command](http://127.0.0.1:8781/03d-input/dist/).
- [Change the background](http://127.0.0.1:8781/04-input/dist/).
- [Use commands in the projection checkpoint](http://127.0.0.1:8781/25-projection/dist/).

The lesson pages include their own Chrome captures. [Release evidence](release.md) describes the exact checks and remaining limitations. The full viewer course continues according to the [complete lesson checklist](roadmap.md); these opening checkpoints do not yet contain all production features.

## Reproduce the checks

From `session_viewer`, build the displayed sources:

```sh
npm --prefix ../session_tests run course -- verify
npm --prefix ../session_tests run course -- serve 8781
```

In another terminal:

```sh
npm --prefix ../session_tests run course -- capture
```

The capture tool launches visible Chrome and types into the drawn egui field. It checks the build fingerprints first. It does not inject commands directly into Rust or use an HTML input substitute. These maintainer commands work under `target`; they do not write into your handwritten project.

## Point an AI assistant at the exact lesson

Give the assistant file paths, your current checkpoint and a concrete result to check. The lesson’s typed edits are the source of truth; `course.json` lists their order, `code/` holds the snippets, and `destination.md` states the final viewer contract. The production `src/` tree is the destination, so copying a whole production file would skip the lessons between your checkpoint and that destination.

For example, write:

> Work from the Session repository root (`session/` inside `wood_research`). I am at checkpoint `26a-keys`. Read `session_viewer/docs/journey/26a-keys.md` and its entries in `session_viewer/docs/journey/course.json`. My learner project is `session_viewer/workspace/journey`. Find why canvas F does not fit the selection. Preserve my project and completed lessons. Change only the missing policy or routing. Run `npm --prefix session_tests run course -- check 26a-keys` against my files, then build and test my project. Check F and text ownership in its browser. Explain which file owns the key and how I can verify the result myself.

For a production fix, name the command or gesture, the scene file, browser, observed behavior and expected behavior. Point to a function or file only when you know it is relevant; ask the assistant to locate the owner otherwise. Include constraints such as preserving Undo/Redo, keeping geometry in world coordinates and retaining black contact boundaries. Ask for the exact validation result, including anything that could not be checked on your hardware.

Maintainer `verify` and `capture` reconstruct the reference under `target`. Their passing result checks the published lesson; validate your handwritten project separately before claiming its bug is fixed.

Canvas Delete runs the same undoable Delete action as the command. `BoundingBox` adds one world axis aligned wire box around the current selection; Undo removes it. `Length BoundingBox` reports its X, Y and Z extents in scene units without editing the document. Their cumulative implementation belongs with the later selection and measurement lessons.
