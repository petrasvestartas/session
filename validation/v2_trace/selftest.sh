#!/usr/bin/env bash
# HARNESS SELF-TEST. Diffs every trace against ITSELF.
#
# A comparison tool that reports a divergence between a file and a copy of that file is broken
# in a way no kernel measurement can reveal, because every real comparison then carries that
# error silently. Both sides are run: an OCCT trace against itself exercises the OCCT-shaped
# parsing on the v2 side of the tool, and vice versa, so an asymmetry in the resolvers shows up.
#
#   selftest.sh [--occt <dir>] [--v2 <dir>]
#
# Prints one line per trace and a final PASS/FAIL with counts attempted vs clean.
set -u
cd "$(dirname "$0")"
OCCT=../occt_trace/traces
V2=traces
while [ $# -gt 0 ]; do
  case "$1" in
    --occt) OCCT="$2"; shift 2;;
    --v2) V2="$2"; shift 2;;
    *) echo "selftest.sh: unknown option $1"; exit 2;;
  esac
done

att=0; clean=0
check() { # check <label> <file>
  att=$((att + 1))
  local out
  out=$(python3 ./v2_trace_diff.py "$2" "$2" --row 2>&1)
  if echo "$out" | grep -q " NONE "; then
    clean=$((clean + 1))
  else
    echo "SELFDIFF NOT CLEAN [$1] $(basename "$2"): $out"
  fi
}

for t in "$OCCT"/*.trace; do check occt "$t"; done
for t in "$V2"/*.trace; do check v2 "$t"; done

echo "SELFTEST attempted=$att clean=$clean"
[ "$att" -eq "$clean" ] && echo "SELFTEST PASS" || echo "SELFTEST FAIL"
