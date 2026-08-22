#!/usr/bin/env bash
# Boolean redo campaign -- session resume entry point.
# Run this first in every session (especially after a computer restart):
#   ./bash/boolean_resume.sh          # where am I, what's next
#   ./bash/boolean_resume.sh --full   # also print the full plan
#
# Workflow rules (user-mandated, standing for a month+):
#  1. Read plan+log before touching code; after EVERY executed increment append an
#     entry to the LOG memory file; then keep executing the current phase.
#  2. DOUBLE-RUN: every measurement process runs TWICE and both passes must agree
#     (diff verdict maps / outputs) before results are trusted or logged; any
#     pass-to-pass difference is itself a finding.
#  3. SESSION CLOSE: before ending a session, write a session-close entry in the LOG
#     (what was done, frontier numbers, exact next action) — the next session starts
#     from that point.
#  4. ISOLATION: this campaign is the ONLY active boolean-solid thread. Older boolean
#     memories/plans (charter, BOP2, chairs fix-stacks, BOOL_V3 campaigns) are
#     read-only evidence — never resumed or interleaved as active work.
#  5. Never re-propose anything in the plan's banned/negative-results list.
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MEM="$HOME/.claude/projects/-home-petras-code-code-rust-session/memory"
PLAN="$MEM/project_boolean_redo_plan.md"
LOG="$MEM/project_boolean_redo_log.md"

hr() { printf '%s\n' "----------------------------------------------------------------------"; }

hr; echo "BOOLEAN REDO CAMPAIGN -- resume"
echo "plan      : $PLAN"
echo "log       : $LOG"
echo "why (RC1-7): $MEM/reference_boolean_redo_retrospective.md"
echo "full doc  : https://claude.ai/code/artifact/6c4ff356-b72a-4f01-898a-817f540dd266"
echo "kb anchors: session_cpp/kb/{PORT_ORDER,DECISION_architecture,ARCHITECTURE_v2,V2_STATUS,BOOL_V3_MEMORY,hunt_INDEX,audit_occt_INDEX}.md"

hr; echo "CURRENT STATE (log tail -- newest entry first):"
[ -f "$LOG" ] && awk '/^## /{n++} n==1' "$LOG" || echo "  log missing -- start at Phase 0 of the plan"

hr; echo "REPO STATE (session_cpp):"
git -C "$ROOT/session_cpp" log --oneline -3
git -C "$ROOT/session_cpp" status --short | head -15

hr; echo "COMMANDS:"
cat <<EOF
  build kernel        ./bash/test_cpp.sh                (incremental when build/ exists)
  minitest c++        ./bash/minitest.sh --cpp --no-web
  corpus + invariants ./bash/corpus_nightly.sh          (T0 ledger + diff vs baseline; tickets in session_cpp/corpus/tickets/)
  corpus subset       python3 session_cpp/corpus/runner.py run --cells <substr> --jobs 8
  step_probe build    cmake -S validation/step_probe -B validation/step_probe/build -DCMAKE_BUILD_TYPE=Release -DCMAKE_POLICY_VERSION_MINIMUM=3.5 && cmake --build validation/step_probe/build -j
  OCCT source (truth) /home/petras/code/code_cpp/OCCT
EOF

if [ "${1:-}" = "--full" ]; then hr; echo "FULL PLAN:"; cat "$PLAN"; fi
hr; echo "Next: do the log's 'Next action'. After each increment: append to the log, then continue."
