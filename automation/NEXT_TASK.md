# Autonomous task — advance the C++ solid-boolean scorecard toward 45/45

You are running UNATTENDED (headless `claude -p`). There is no human to ask. Make decisions,
state them in one line, and proceed. Do NOT stop to ask questions or to summarize.

## Mission
Increase the number of OK cells in the C++ boolean scorecard (`session_cpp/main_7.cpp`, the 15
primitive-pair × 3-op matrix vs the OCCT oracle) WITHOUT regressing any currently-green cell.
C++ is the reference language; do not touch Rust/Python this loop.

## Measurement (fast loop — never run the 700-test suite)
- Build + score:  `automation/run_scorecard.sh`  (kills stale main_7.exe, builds `main_7`, runs it,
  prints the table + `SCORECARD: N/45`). Incremental rebuild is ~10s; the run is ~2-4 min.
- Target ONE failing cell with a filter while iterating:
  `session_cpp/build/Release/main_7.exe "box  x cone"` (substring = pair label; skips other cells).
- Phase timing: prefix `SESSION_BOOL_PROFILE=1`. Shared-edge path: prefix `SESSION_BOOL_SHARED_EDGES=1`.
- A cell is OK iff volume rel<1e-6 AND exact face count AND `is_solid()`.

## Priority order (buildspec `.claude/occt/OCCT_BOOLEAN_BUILDSPEC.md` P0–P9; pick the most tractable OPEN one)
1. **P0** — box×sph watertight + volume-exact via `make_shared_section_edges` (currently a no-op;
   default==shared==9.5677, OCCT 9.5457). Fixes box×sph, then sph×sph, sph×cyl-class watertightness.
2. **P7** — exact plane-cone conics (hyperbola/parabola, `.claude/occt/OCCT_STUDY_conics_tangency.md` GAP 1) and
   plane-torus, replacing the sampled+fitted arcs in `intersection.cpp::ssi_plane_cone`. Fixes
   box×cone (wrong faces 16≠10) and box×tor.
3. **P8** — cyl×cyl Steinmetz + coaxial quadric pairs (`.claude/occt/OCCT_STUDY_pairs.md`).
4. **P9** — torus family + cone-cone; also tor×tor is a PERF disaster (6-7 s/op) — profile it.
Each phase in the buildspec is flag-gated and lists exact functions/anchors. Read the relevant
`.claude/occt/OCCT_STUDY_*.md` before implementing. Prefer the smallest change that turns a red cell green.

## Hard rules
- **Never ask the user anything.** If a choice needs judgment, pick the option that best advances the
  priority order above, write the choice + one-line reason to the commit message, and proceed.
- **Never regress a green cell.** Before committing, run `automation/run_scorecard.sh`; the OK count
  must be >= the count at the start of this iteration. If it dropped, revert your change
  (`git -C session_cpp checkout -- src`) and try a different approach.
- **Kill main_7.exe before every rebuild** (it holds a binary lock when a cell hangs).
- **Robustness:** the kernel must never infinite-loop. If a cell hangs, add a bounded guard to the
  offending loop (see the CDT `flip_budget` fix in `remesh_cdt.cpp` for the pattern) — a valid input
  finishes far under budget.
- **Scope:** edit only `session_cpp/` (mainly `intersection.cpp`, `brep.cpp`, `nurbssurface_trimmed.cpp`,
  `remesh_cdt.cpp`) and `automation/`. Do NOT touch Rust, Python, the viewer, or other subsystems.
- **Never `git push`. Never `git commit --no-verify`.** Commit locally only.
- **Blocked?** If you cannot improve the count after a genuine attempt, write `automation/BLOCKED.md`
  (the cell, what you tried, the specific obstacle, the file:line) and STOP this turn.

## Context to read first
- `.claude/occt/OCCT_BOOLEAN_BUILDSPEC.md` + the relevant `.claude/occt/OCCT_STUDY_*.md`.
- Memory: `.claude/projects/.../memory/project_solid_booleans_occt.md` (scorecard baseline, oracle
  grammar, dev loop, known bugs) and `project_ssi_split_pipeline.md` (SSI substrate).
- Current scorecard: `automation/scorecard.log` (or run `automation/run_scorecard.sh`).

## End of turn
Commit your verified improvement (`git -C session_cpp add -A && git -C session_cpp commit -m
"boolean: <cell> <red→green>; <choice+reason>"`). Print the new `SCORECARD: N/45`. Then stop — the
outer loop re-invokes you for the next iteration.
