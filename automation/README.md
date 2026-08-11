# Autonomous boolean-development loop

Unattended Claude Code loop that iterates on the C++ solid-boolean kernel until the OCCT scorecard
improves, gated by a hard oracle number so it never stalls asking you questions.

## Why Claude never asks questions here (4 layers)

1. **Headless mode `-p`** — there is no interactive terminal, so there is nowhere to ask *into*. If the
   model would normally ask a clarifying question, the turn simply ends; it does not wait for you.
2. **`--dangerously-skip-permissions`** — every tool call (edit, build, run) is auto-approved, so it
   never pauses on a "allow this command?" prompt. (Runs fine from Git Bash on Windows.)
3. **`NEXT_TASK.md` rules** — the prompt explicitly says *"Never ask the user anything. If a choice needs
   judgment, pick the option that best advances the P0→P9 priority order, state it in one line, and
   proceed."* This turns "want P7 or P0 next?" into a decision Claude makes itself.
4. **The script decides "done", not Claude** — `run_scorecard.sh` checks a hard number (`N/45` vs the
   OCCT oracle) after each iteration. Claude is never asked "are you finished?", so it can't reply with a
   "should I proceed?" question — the loop just re-invokes it or stops.

If Claude is ever *blocked* (can't make progress), it writes `automation/BLOCKED.md` and stops **that
iteration** instead of asking — the loop sees the file and exits cleanly.

## Files
| File | Role |
|---|---|
| `run_scorecard.sh` | Exit-check: builds `session_cpp/main_7`, runs the 15×3 pair matrix vs the OCCT oracle, prints `SCORECARD: N/45`. Exit 0 iff `N >= TARGET`. This is the ground truth. |
| `NEXT_TASK.md` | The prompt fed to headless Claude each iteration. Encodes the P0–P9 priority, the "never ask, just proceed" rules, no-regress + no-push guardrails, and the BLOCKED.md escape. |
| `agent_loop.sh` | The outer loop: run headless Claude on `NEXT_TASK.md`, score, repeat until target / budget / max-iters / BLOCKED. Works on a dedicated `auto/booleans` branch. |
| `scorecard.log` | Latest scorecard output (overwritten each run). |
| `iter_N.json` / `iter_N.err` | Per-iteration Claude output (has `total_cost_usd`) and stderr. |
| `BLOCKED.md` | Written by the agent when it cannot make progress; stops the loop. |

## How to run (Git Bash on Windows)

**1. Just the scorecard (fast, no AI — do this first to confirm your toolchain):**
```bash
bash automation/run_scorecard.sh          # prints the table + SCORECARD: 24/45
```

**2. One supervised AI iteration (watch what it does before unleashing the loop):**
```bash
claude -p "$(cat automation/NEXT_TASK.md)" --dangerously-skip-permissions --max-turns 60
bash automation/run_scorecard.sh          # did the count go up? did green stay green?
```

**3. The full unattended loop:**
```bash
MAX_ITERS=3 BUDGET_USD=5 bash automation/agent_loop.sh     # start SMALL — 3 iters, $5 cap
After it finishes, sanity-check:
git -C session_cpp log --oneline auto/booleans     # did it commit real progress?
tail -20 automation/scorecard.log                  # N/45 — did it go up, and stay hang-free?
cat automation/BLOCKED.md 2>/dev/null              # if it stopped early, why
If those look good, re-run with MAX_ITERS=20 BUDGET_USD=40 and let it grind. Go ahead and launch it.

# once you trust it:
MAX_ITERS=20 BUDGET_USD=40 bash automation/agent_loop.sh
```
Tunables (env): `MAX_ITERS`, `SCORECARD_TARGET` (default 45), `BUDGET_USD`, `MAX_TURNS`.

## Safety model
- Runs on the `auto/booleans` branch of the `session_cpp` submodule — **never `main`**. Review with
  `git -C session_cpp log --oneline` and merge only diffs you approve.
- `NEXT_TASK.md` forbids `git push`, `--no-verify`, and touching anything outside `session_cpp/` + `automation/`.
- `--dangerously-skip-permissions` = no prompts (so it never stalls). Because it's a sandboxed repo on a
  branch and can't push, the blast radius is local commits you can `git reset`.
- Cost cap is enforced by the outer loop (summing `total_cost_usd`), independent of any CLI budget flag.
- Every iteration is scored by the OCCT oracle; a change that regresses a green cell is rejected by the task rules.

> Optional stricter permissions instead of `--dangerously-skip-permissions`: an allow/deny list in
> `.claude/settings.json` (e.g. allow `Bash(cmake:*)`, `Edit`, `Read`, `Write`; deny `Bash(git push:*)`).
> Verify exact flag support with `claude --help` first — some flags are version-specific.

## Will this actually develop the boolean over time?

**Yes, on the tractable phases — with honest limits.** The oracle gate (volume rel<1e-6 + exact face
count + `is_solid`, vs real OCCT) is a *strong* correctness check, so committed progress is real, not
hallucinated. Expect the loop to:
- **Climb** on the well-specced phases — P0 (box×sph watertightness) and P7 (exact plane-cone/torus
  conics for box×cone, box×tor) have detailed specs in `.claude/occt/OCCT_BOOLEAN_BUILDSPEC.md` + `.claude/occt/OCCT_STUDY_*.md`
  and clear red→green targets. Each win is committed only if the count goes up and nothing regresses.
- **Plateau** on the research-hard cells — general Steinmetz (cyl×cyl), the torus family, and freeform
  are genuinely hard; the agent will likely hit a wall and write `BLOCKED.md` (the correct outcome, not a
  failure). tor×tor is also a *perf* problem (6–7 s/op) as much as a correctness one.

So: treat it as a **force multiplier that grinds the mechanical/specced work and flags the hard parts**,
not a magic "45/45" button. Start with 2–3 iterations, read the commits and `iter_*.json`, and scale the
budget once you've seen it produce clean, oracle-verified diffs. Merge the `auto/booleans` branch yourself.
