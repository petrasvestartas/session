#!/usr/bin/env bash
# Nightly T0 corpus + metamorphic invariants (session C tooling).
#
#   bash/corpus_nightly.sh                 # T0 ledger + diff vs baseline + invariants
#   bash/corpus_nightly.sh --set-baseline  # accept this run as the new baseline
#   FLAGS="SESSION_AUTO=1 SESSION_M3=1" bash/corpus_nightly.sh
#
# Regressions and invariant violations are written as tickets in session_cpp/corpus/tickets/.
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PY="$ROOT/uvsession/bin/python"
[ -x "$PY" ] || PY=python3
CORPUS="$ROOT/session_cpp/corpus"
JOBS="${JOBS:-8}"
FLAGS="${FLAGS:-}"
EXTRA="${1:-}"

PROBE="$ROOT/validation/step_probe/build/step_probe"
if [ ! -x "$PROBE" ]; then
  echo "step_probe missing -- build it:"
  echo "  cmake -S $ROOT/validation/step_probe -B $ROOT/validation/step_probe/build \\"
  echo "        -DCMAKE_BUILD_TYPE=Release -DCMAKE_POLICY_VERSION_MINIMUM=3.5 && \\"
  echo "  cmake --build $ROOT/validation/step_probe/build -j"
  exit 1
fi

[ -f "$CORPUS/t0_manifest.json" ] || "$PY" "$CORPUS/runner.py" truth
"$PY" "$CORPUS/runner.py" run --jobs "$JOBS" --flags "$FLAGS" $EXTRA
"$PY" "$CORPUS/invariants.py" partition
"$PY" "$CORPUS/invariants.py" idempotence
echo "tickets:"
ls -1t "$CORPUS/tickets" 2>/dev/null | head -20
