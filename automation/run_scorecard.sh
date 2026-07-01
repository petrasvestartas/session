#!/usr/bin/env bash
# Boolean scorecard exit-check for the autonomous loop.
# Builds the fast C++ driver (main_7), runs the 15x3 primitive-pair matrix vs the OCCT oracle,
# extracts the "N/45 cells OK" count, and exits 0 iff N >= TARGET (default 45), else 1.
#
# Usage:  automation/run_scorecard.sh [TARGET]
#   TARGET   pass threshold (default 45, or $SCORECARD_TARGET)
# Output:   full scorecard table + "SCORECARD: N/45 (target T)" line; log at automation/scorecard.log
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CPP="$ROOT/session_cpp"
LOG="$ROOT/automation/scorecard.log"
TARGET="${1:-${SCORECARD_TARGET:-45}}"

# Don't let a stale/hung driver hold the binary lock, then (re)build the driver only.
taskkill //F //IM main_7.exe >/dev/null 2>&1 || true
cmake --build "$CPP/build" --config Release --target main_7 --parallel 8 > "$LOG" 2>&1
if [ $? -ne 0 ]; then echo "SCORECARD: BUILD FAILED (see $LOG)"; tail -20 "$LOG"; exit 2; fi

# Run the matrix (bounded so a regression that reintroduces a hang can't wedge the loop).
timeout 600 "$CPP/build/Release/main_7.exe" > "$LOG" 2>&1
RC=$?
cat "$LOG"
if [ $RC -eq 124 ]; then echo "SCORECARD: TIMEOUT (a cell is hanging)"; exit 3; fi

N="$(grep -oE '[0-9]+/45 cells OK' "$LOG" | head -1 | grep -oE '^[0-9]+')"
N="${N:-0}"
echo "SCORECARD: $N/45 (target $TARGET)"
[ "$N" -ge "$TARGET" ]
