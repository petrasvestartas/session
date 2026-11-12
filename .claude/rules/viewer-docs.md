---
paths:
  - "session_viewer/docs/**"
  - "session_viewer/src/**"
---

# Viewer tutorials

## Destination and ownership

- Teach one cumulative program from Rust and wgpu fundamentals to the maintained viewer's production tree and behavior.
- A lesson owns a subject, not a commit or a fix. Introduce the subject in its final form where its dependencies permit it.
- Use the fewest coherent subject checkpoints. Small typing instructions belong inside the subject; do not turn each edit into a lesson.
- Every production source file has a named subject owner, or an explicit approved omission with a reason. Inventory membership, a source link and a matching hash do not establish teaching.
- Keep completed code in place. A later lesson adds modules and their declared registrations; it does not replace an earlier lesson's implementation.
- Fix a subject in its owning lesson and propagate that correction to stored checkpoints. Do not make learners perform the correction as a new step.
- Apply these rules to existing lessons as well as new content. Consolidate repeated implementations, repair sequences and redundant checkpoints into their subject owner. Archive superseded material while preserving links and required fixtures.
- Count executable subject checkpoints when reporting the course size. Combining navigation headings while retaining the same rewrite sequence is not consolidation. Retain a separate checkpoint only when it introduces a distinct coherent subject with a visible result.
- Design a small registry, pass list or command list before teaching an integration that would otherwise reopen completed files. Do not add a parallel demonstration runtime and call it the final viewer.
- Put the 2D mathematics subject immediately after the first egui painting. Put scene geometry, camera projection and scene animation after their GPU and loading prerequisites.
- Preserve the maintained viewer's behavior. Preserve other agents' edits and serialize shared Git work under the workspace coordination rules.

## Lesson format

Use this order; omit an empty section instead of filling it with boilerplate:

1. Goal: one sentence naming the visible result and subject.
2. Prerequisites: the earlier subjects and exact files this lesson owns.
3. Build: short instructions with named includes from the runnable source. Label what to type and what to copy.
4. Run and observe: the exact checkpoint command and its own visible result.
5. Predict and try: one focused experiment with a prediction before execution.
6. Question and answer: one check of the central concept.
7. Validation: actual checks and material limitations, separate from the expected result.

- Describe the program as it should be now. Do not narrate previous drafts, attempted fixes, commits or agent work inside a lesson.
- Explain historical context only when it establishes a present constraint, such as the field-by-field arrow interface required by a device driver.
- Keep build instructions to one plain sentence, normally under 25 words. Explain Rust and rendering concepts once at their first use, preferably in a concise source comment.
- Do not repeat an identifier in a comment. Explain the mechanism or reason the learner cannot infer from the name.
- Avoid a shared generic ?what you see? sentence across checkpoints. A source-only preparation belongs inside its visible subject lesson.
- Runnable Rust, WGSL, TOML and deck listings come from source includes, never separately typed copies in Markdown. Use named section markers; retain them when moving code.
- Include kernel code from the live kernel rather than copying it into a lesson. Keep font and other asset licenses.
- Use plain Markdown supported by the Vue site. No MkDocs-only admonitions, tabs or attribute syntax.
- Use D2 for course flow diagrams; keep source and rendered artifacts together. Do not hand-edit generated SVG positions.

## Course tooling and evidence

- `docs/journey/course.json` defines the current cumulative sequence. The TypeScript tooling may render pages and reconstruct its exact sources; it must not invent teaching prose or conceal implementation changes.
- The numbered pages in `docs/archived/`, `docs/lessons/SERIES.txt` and `docs/typing/` are historical reconstruction inputs. Do not claim that historical lesson 37 equals the current viewer.
- Preserve unused tutorials in the archive with stable links. Do not remove shared checkpoint fixtures while tooling still consumes them.
- Every accepted checkpoint must compile and run. Verify browser behavior in the browser; native tests and source hashes cannot establish it.
- Compare the reconstructed final source with production. Never substitute a production runtime link, refreshed inventory or copied final source dump for cumulative implementation parity.
- Validate source includes and links with the strict Vue build in `session_tests`. Run the course's structural checks, relevant Cargo checks/tests and browser checks through the workspace resource guards.
- Use one shared build cache. Do not retain a separate dependency cache or binary set per checkpoint.
- Record unfinished coverage as unfinished. Do not weaken a check or rewrite historical evidence to obtain a passing result.

## Reports, changelogs and agent instructions

- Describe the resulting state, not a chronological account of reaching it.
- A normal changelog entry is one to three bullets, at most 100 words total: resulting behavior; relevant validation; material remaining limitation when present.
- Name concrete files or subjects when useful. Omit routine commands, repeated progress claims, abandoned approaches and ?before we did X, then fixed Y? narratives.
- A complex compatibility change may include a short rationale. Keep detailed audit measurements and debugging evidence in their dedicated reports, linked when relevant.
- Instructions state the current rule once. Update the rule in place; do not append successive corrections that leave contradictory policies.
- Completion reports state what is actually accepted, what remains and why. A successful site build is not proof of tutorial completeness.

See `session_viewer/docs/maintaining.md` for commands and `session_viewer/docs/journey/coverage-audit-2026-11-12.json` for the initial per-file audit.
