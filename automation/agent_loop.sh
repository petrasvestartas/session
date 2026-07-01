#!/usr/bin/env bash
# Unattended "Ralph loop": repeatedly invoke headless Claude Code on NEXT_TASK.md until the boolean
# scorecard reaches TARGET, a cost budget is hit, MAX_ITERS is exhausted, or the agent writes BLOCKED.md.
#
# The KEY idea: Claude never decides it is "done" — this script does, by running the OCCT scorecard
# after each iteration and checking a hard number. So there is nothing to ask you, and no stalling.
#
# Guardrails used (all flags below are stable): -p (no TUI), --output-format json (parse cost),
# --dangerously-skip-permissions (never blocks on a prompt), --max-turns (runaway cap). Cost budget is
# enforced HERE by summing total_cost_usd, so it does not depend on any (uncertain) --max-budget flag.
#
# Run:  bash automation/agent_loop.sh
# Tunables (env):  MAX_ITERS=20  SCORECARD_TARGET=45  BUDGET_USD=20  MAX_TURNS=60
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

MAX_ITERS="${MAX_ITERS:-20}"
TARGET="${SCORECARD_TARGET:-45}"
BUDGET_USD="${BUDGET_USD:-20}"
MAX_TURNS="${MAX_TURNS:-60}"
BRANCH="${AUTO_BRANCH:-auto/booleans}"

command -v claude >/dev/null 2>&1 || { echo "FATAL: 'claude' CLI not on PATH"; exit 1; }

# Safety: do all autonomous work on a dedicated branch in the session_cpp submodule, never main.
git -C session_cpp rev-parse --verify "$BRANCH" >/dev/null 2>&1 \
  && git -C session_cpp switch "$BRANCH" \
  || git -C session_cpp switch -c "$BRANCH"
echo "Working on branch: $(git -C session_cpp branch --show-current)"

rm -f automation/BLOCKED.md
spent=0
echo "=== baseline scorecard ==="
bash automation/run_scorecard.sh "$TARGET" >/dev/null 2>&1
base=$(grep -oE 'SCORECARD: [0-9]+' automation/scorecard.log | grep -oE '[0-9]+' | tail -1); base=${base:-0}
echo "baseline: $base/45"

for i in $(seq 1 "$MAX_ITERS"); do
  echo "==================== iteration $i / $MAX_ITERS (spent \$$spent, at $base/45) ===================="
  out="automation/iter_${i}.json"

  claude -p "$(cat automation/NEXT_TASK.md)" \
    --output-format json \
    --dangerously-skip-permissions \
    --max-turns "$MAX_TURNS" \
    > "$out" 2> "automation/iter_${i}.err" || echo "  (claude exited non-zero; continuing to score)"

  # Accumulate cost from the JSON result (grep/awk so we don't require jq on Windows git bash).
  cost=$(grep -oE '"total_cost_usd":[[:space:]]*[0-9.]+' "$out" | grep -oE '[0-9.]+' | tail -1); cost=${cost:-0}
  spent=$(awk "BEGIN{printf \"%.4f\", $spent + $cost}")

  # Score. run_scorecard.sh exits 0 iff N >= TARGET.
  if bash automation/run_scorecard.sh "$TARGET"; then
    echo "*** TARGET REACHED: $TARGET/45 after iteration $i (spent \$$spent) ***"; break
  fi
  base=$(grep -oE 'SCORECARD: [0-9]+' automation/scorecard.log | grep -oE '[0-9]+' | tail -1); base=${base:-0}

  if [ -f automation/BLOCKED.md ]; then
    echo "*** AGENT BLOCKED after iteration $i ***"; cat automation/BLOCKED.md; break
  fi
  if awk "BEGIN{exit !($spent >= $BUDGET_USD)}"; then
    echo "*** BUDGET \$$BUDGET_USD reached (spent \$$spent) after iteration $i ***"; break
  fi
done

echo "=== loop finished: $base/45, spent \$$spent ==="
echo "Review the branch:  git -C session_cpp log --oneline"
