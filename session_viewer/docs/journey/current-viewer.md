# Build against the current viewer

The [completion contract](completion-contract.md) requires a browser-only cumulative viewer, bounded daily coding, and no final implementation shortcut. Full handwritten coverage is still in progress.

This course now targets Session `2f96b35d225015cfcb9bfe5e8ec66565ef99aa28`. The [upstream audit](latest-commit-audit.md) records the fetched Wood and kernel revisions. The [dated comparison](current-viewer-2026-10-11.json) records changed files without replacing the historical destination. The [command table](current-command-grammar.md) comes from the compiled registry: 86 names, 117 static options and 15 aliases.

## Layers: one lamp, independent children

A parent lamp temporarily hides its descendants. It does not rewrite their own lamps or object visibility. In subject 53, `Flags.hidden` stores explicit object hiding, `Flags.lamps` stores switched-off layer keys, and `Flags.held` caches their affected objects. Rebuild the cache when the tree changes or history restores it. Picking and drawing use effective visibility; each lamp displays its own state. Show clears all three stores.

After [53t](53t-input.md), create two sublayers and place an object in each. Hide the first child. Switch the parent off and back on: the first child must stay hidden and the second must reappear. Switch the parent off, change a child lamp, then restore the parent: the child's new choice must survive. Press S with canvas focus to show everything. The [53u checks](53u-layers.md) exercise independent lamp state and original source retention.

Selecting a visible object or layer in the panel also fits the camera to the resulting selection. Canvas selection does not acquire this panel-only fit. The canonical `TreePick` action starts this behavior in [52e](52e-edits.md). Keep rename and command text in control of their own keys.

The October 11 snapshot also records two preserved local edits from October 10: parent Element lamps preserve hidden child state, and lamps use document/hierarchy paths so equal group names under different parents remain independent. The [October 9 snapshot](current-viewer-2026-10-09.json) remains available as historical source evidence.

## Elements: attributes and interactions

`Element Attributes` replaces the old `Element Features` command. With no option it toggles; On and Off set an explicit value. It starts **Off**. Base planes, axes, sections and an element's other own features belong here.

`Element Interactions` starts **On**, with the same toggle/On/Off forms. It controls features another element puts on the host: contacts, joints, cuts and drills. Turning either category off must leave the other category unchanged and must not edit original coordinates or source metadata.

[92i](92i-adopt-elements-and-consume-sources-once.md) separates the two groups during adoption. [92k](92k-own-independent-element-tool-state.md) owns their independent defaults, [92m](92m-route-current-element-commands-and-feature-visibility.md) connects the commands, and [92o](92o-prove-element-recipes-source-history-and-archives.md) checks original payloads and independent visibility.

In the maintained runtime, each host's `interactions` group lists the **full interaction type**, then its name/source, with the drawn pieces below it. Read `src/app/interactions.rs::hosted` and `src/app/hierarchy.rs::feature_lines`. An element nested under another element remains an editable Element; only its `attributes` group classifies its own attached objects as attributes. Parent preview carries descendants once and cancellation restores them.

The teaching adapter in 92 groups newly created feature pieces into attributes/interactions. The [96i–96q bridge](96i-separate-own-and-hosted-element-features.md) adds imported graph grouping, full interaction type/name/source labels, derived read-only pieces, editable nested Elements, atomic import and source-preserving archives. [96r](96r-preserve-exact-command-completions.md) fixes exact command completion exposed by its browser exercise. At [97](97-finish-with-the-maintained-browser-runtime.md), inspect a current Wood scene in the maintained runtime: find a host, its full-type interaction, its pieces and a nested connector Element. Verify separate toggles, selection, parent drag/cancel, Delete/Undo, and Save/Open. The handwritten bridge has its own specimen and regression checks; it does not establish full parity for instances, feature picking/highlighting, graph editing or copied relationship ownership.

## Current commands and checks

With canvas focus, H hides, S shows, and F fits. Ctrl/Cmd+Z undoes; Ctrl+Y or Ctrl/Cmd+Shift+Z redoes. Text fields retain their editing shortcuts. Delete uses the existing undoable Element deletion path. BoundingBox creates a world-axis wire box around selected originals; Length BoundingBox reports X/Y/Z without creating an edit.

For a created beam, type `Line 0,0,0 1000,0,0`, then `Element Beam 100 200`. Run `Length BoundingBox` and expect `X 1000 · Y 100 · Z 200`. Toggle Element Attributes; the beam remains while its own display features change. Create BoundingBox, Undo, then delete the beam and Undo. A Block instead requires selected closed bottom/top outlines; its bare command does not create a sample.

`Close` completes an active Polyline or Curve with at least three points. It **does not clear the document**. Use the maintained `reload_scene(url)` loader export with a valid empty manifest to exercise document replacement/release. The final browser contract checks zero objects, points, vertices and retained source bytes through that path. Global GPU capacity may remain allocated for an empty viewer.

There is no `View Grid Off` command. Start with `?nogrid=1` when a comparison needs the ground grid disabled. This setting does not remove a displayed Plane's own 10-by-10 grey grid or its coloured axes. The original Plane grid is hidden by solids; its coloured X/Y/Z arrows draw over them.

The wider Layers default is 413.33334 CSS pixels before viewport constraints. Arctic flat/curved outlines still need matched DPR and shallow-angle visual checks. Contacts keep original coordinates, minimal depth offset and black boundary ink. Do not infer any of these pixel results solely from command parsing.

## Evidence and your own project

Changes to the checker and canonical sources invalidate previous result signatures. Existing records and images retain historical value, but only renewed matching records count as current acceptance. [Release evidence](release.md) and [local checks](local-checks.md) state the actual scope. Checkpoint 97 is a maintained-runtime comparison; [remaining differences](rendering-differences.md) still separate it from the handwritten implementation.

Current Wood source changes contact hosting, floor/connector hierarchy and recipes. Old floor/dome files were not regenerated by updating the checkout. Preserve their hashes and gather new provenance when rebuilding them. Local software-renderer timing is not Lenovo Chrome/Firefox timing.

Use the [AI pointer guide](command-line.md#point-an-ai-assistant-at-the-exact-lesson): provide the canonical checkpoint ID, snippets, maintained owner and your absolute project path. `check` compares your files; `verify` reconstructs a reference under `target`. Never substitute one result for the other.
