# Session Viewer — agent brief (start here)

You are taking over work on **Session Viewer**, a browser geometry viewer in Rust + wgpu (WebGPU). This file is the entry point. It tells you what to read, in which order, what rules bind you, and how to report. Everything else in this folder hangs off it.

`Session Viewer.txt` and `audit_agent.txt` are the original briefs. They are **superseded**: the audit they asked for is done, and their requirements are restated, corrected and grounded in [PRODUCT_BRIEF.md](PRODUCT_BRIEF.md) and [TUTORIAL_PROMPT.md](TUTORIAL_PROMPT.md). Do not follow them directly.

## 1. Locations

| What | Path |
|---|---|
| Workspace root (superproject, 5 submodules) | `/home/petras/code/code_cpp/wood_research` |
| Session repo (submodule) | `wood_research/session` |
| The viewer (plain directory inside `session`) | `wood_research/session/session_viewer` |
| Geometry kernel used by the viewer | `wood_research/session/session_rust` |
| Course tooling (TypeScript, Vue docs site) | `wood_research/session/session_tests` |
| This folder | `session_viewer/inspection/` |

## 2. Reading order

| # | File | Why |
|---|---|---|
| 1 | this file | rules, workflow, reporting |
| 2 | [AUDIT_SUMMARY.md](AUDIT_SUMMARY.md) | current state, findings, decisions already made |
| 3 | [PRODUCT_BRIEF.md](PRODUCT_BRIEF.md) | what the viewer must become |
| 4 | [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) | milestones M0–M7 with files, tests, acceptance criteria |
| 5 | [TUTORIAL_PROMPT.md](TUTORIAL_PROMPT.md) | how tutorials are written and verified |
| 6 | `session/.claude/rules/viewer-docs.md`, `session_viewer/docs/maintaining.md` | binding course rules and commands |
| 7 | Reference reports, read the one your task touches | see §3 |

## 3. Document map

| Task touches | Read |
|---|---|
| Builds, versions, tests, CI, disk | [REPOSITORY_INSPECTION.md](REPOSITORY_INSPECTION.md) |
| Any feature's status, source, tests | [FEATURE_INVENTORY.md](FEATURE_INVENTORY.md) |
| Module structure, ownership, refactors | [ARCHITECTURE.md](ARCHITECTURE.md) |
| Structs, buffers, memory, allocations | [DATA_STRUCTURES.md](DATA_STRUCTURES.md) |
| Shaders and pipelines | [SHADER_INVENTORY.md](SHADER_INVENTORY.md) |
| Frame time, lag, measurement | [PERFORMANCE_AUDIT.md](PERFORMANCE_AUDIT.md) |
| Commands, CLI, FastMCP, the `Op` API | [INTERFACE_INVENTORY.md](INTERFACE_INVENTORY.md) |
| Import/export, serialization | [FILE_FORMATS.md](FILE_FORMATS.md) |
| Tutorial files, coverage, sequence | [TUTORIAL_ROADMAP.md](TUTORIAL_ROADMAP.md) |
| The completed 3D-matrix removal | [MATRIX_CLEANUP.md](MATRIX_CLEANUP.md) |

The reports describe the tree at 2026-11-19. Verify a file:line before relying on it; code moves.

## 4. Machine and build rules

This machine has crashed from ignoring these.

- **Job limits:** cargo `-j8`, C++ `-j6`; never `-j$(nproc)`.
- **Build slots:** run every heavy command (build, test sweep, docs build) through
  `bash /home/petras/code/code_cpp/wood_research/wood/tools/buildslot.sh [--run] <your-agent-name> <command...>`. `--run` is for test-only commands. Never create or remove lock directories by hand.
- **Caps:** wrap long jobs as `timeout 10m systemd-run --user --scope -p MemoryMax=6G <cmd>`. Anything still running after ten minutes is wedged.
- **One heavy command at a time.**
- **Timings:** before trusting any timing, check `scaling_max_freq` equals `cpuinfo_max_freq` in `/sys/devices/system/cpu/cpu0/cpufreq/`.
- **Viewer builds:** the real working tree builds cleanly (`session_proto` is at `edc614e1e`). Set `REGEN_PROTO=0` if you want offline builds that skip `session_rust/build.rs`. Note that `session_viewer/build.rs` watches `src/shaders` directory mtime (`cargo:rerun-if-changed=src/shaders`), so run `touch src/shaders` after editing an existing `.wgsl` file in place.
- **Browser here renders black.** This machine's browser WebGPU is a software path. Check rendering natively (`Gpu::new_headless` / the `selftest` example) and leave browser checks to a machine with a working GPU.

## 5. Git rules

- Never add an AI as author, co-author or trailer.
- Never `git add -A`. Other agents may edit the same tree: stage named paths and re-read `git status` before committing.
- **Commit order:** commit inside the `session` repo first, then bump its pointer in `wood_research` with a `superproject: <what>` commit.
- **Pushing:** push children before parents, as fast-forwards. Do not use `.claude/hooks/push_mono.sh` blindly: it runs `git add -A` everywhere and force-pushes with lease.
- **CI:** a push is done only when CI is green. Run `bash /home/petras/code/code_cpp/wood_research/bash/ci_status.sh --wait` and fix any failure before other work.
- Do not touch the dirty submodule pointers `session_data`, `session_proto`, `session_py` or `session_rust` unless your task is about them.
- Never commit per-machine state: `settings.local.json`, `CLAUDE.local.md`, lock files.

## 6. Decisions already made (do not reopen)

| Topic | Decision |
|---|---|
| 3D kernel-matrix lane | removed; kernel `Matrix` class stays; 2D rounded stage matrix stays |
| Default desktop opacity | keep 0.95 |
| Browser tests in CI | no |
| Tutorial diagrams | drawn in the session viewer; no Mermaid |
| Old lessons / archived course | keep |
| Codex work files, dead mask pipelines | deleted (done) |

Open decisions are listed in [AUDIT_SUMMARY.md §7](AUDIT_SUMMARY.md#7-decisions) "Still open". Ask the user before acting on any of them.

## 7. How to work

1. **Pick one milestone** from IMPLEMENTATION_ROADMAP, in order: M1 → M2 → M5/M6 → M7 (M0 and M3 are done; M4 only after M1–M2). Tutorials (M7) follow the code, never lead it.
2. **Check the baseline first.** Before changing code, record the relevant tests and benchmarks with the exact commands from the milestone.
3. **Make the smallest change** that meets the milestone's acceptance criteria.
   - Keep the app building after every commit.
   - No broad refactor without a migration plan written in the roadmap.
   - Never remove a feature without a recorded decision.
4. **Verify** with the milestone's tests and benchmarks, and with TUTORIAL_PROMPT §5 for docs work. Re-measure performance with the same command as the baseline.
5. **Update the reports.** Change the affected report in place, so it states the new state once; do not append a history. Mark a roadmap item done with its date and evidence.
6. **Stop and ask** before any of these:
   - a major rewrite (M4 step 4)
   - removing a feature
   - a change to the kernels' file format (protobuf schema)
   - deleting tracked history (`docs/lessons`, `docs/journey`, `docs/archived`)
   - anything in AUDIT_SUMMARY's "Still open" list

## 8. Reporting

End every work session with a short report:

| Section | Content |
|---|---|
| Changed | files and the resulting behaviour, one line each |
| Verified | the commands that ran and their results; for timings, also the CPU throttle state |
| Not verified | what could not run and why |
| Next | the next roadmap item, or the question that blocks it |

**Never invent results.** Do not fabricate FPS, timings or test results. If something was not measured, say so.
