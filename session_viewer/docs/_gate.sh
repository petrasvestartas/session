#!/usr/bin/env bash
# docs/_gate.sh [--record] [--only <scene>]
#
# The pixel gate for the restructure lessons. Seven of them move code that must come out
# BYTE-IDENTICAL, and the only proof of that is the frame: same ink, same draw count, same
# object count, same PPM bytes.
#
# Mandatory block: the four scenes that resolve entirely to TRACKED .pb assets - lion, bunny,
# bunny_cloud, drawings_rotated - so a fresh clone can run the gate. Advisory block: drawings,
# bunny_drawings, cloud_mix, lidar14, whose .pb files are gitignored (>50 MB); they are skipped
# LOUDLY BY NAME when absent and never fail the gate.
#
# Every scene runs under four configs and, per the house rule, TWICE. The two passes must agree
# to the byte; a disagreement is a finding in itself (nondeterminism) and fails before any
# comparison against the goldens happens.
#
#   ./docs/_gate.sh                 diff against docs/_GOLDENS.tsv, non-zero on any difference
#   ./docs/_gate.sh --record        re-baseline docs/_GOLDENS.tsv (do this at tag end-of-44)
#   ./docs/_gate.sh --only lion     smoke one scene, print its rows, compare nothing

set -euo pipefail
cd "$(dirname "$0")/.."

T=x86_64-unknown-linux-gnu
OUT=${TMPDIR:-/tmp}/gate.ppm
TSV=docs/_GOLDENS.tsv
MANDATORY="lion bunny bunny_cloud drawings_rotated"
ADVISORY="drawings bunny_drawings cloud_mix lidar14"
CFGS=("" "VIEWER_LINE_STYLE=tubes" "VIEWER_REBUILD=1" "VIEWER_INCREMENTAL=1")

# MEASURED, 2026-08-31: the splat lane is a 2-pass ATOMIC compute rasterizer, so which point wins
# a contested pixel is a race and the PPM bytes differ run to run - `lion` gave three distinct
# sha256 over eight runs while ink/draws/objects never moved (77543/4/1 every time). Only these
# scenes are affected; `bunny` is byte-identical across all four configs and both passes. So the
# sha is recorded as `nondet(splat)` for cloud scenes and ink/draws/objects carry the gate there.
SPLAT_SCENES="lion bunny_cloud cloud_mix lidar14 bunny_drawings"

# The harness reads a dozen VIEWER_* knobs; an inherited one silently re-frames the camera and
# every row goes red for a reason that is not in the diff.
unset VIEWER_LINE_STYLE VIEWER_REBUILD VIEWER_INCREMENTAL VIEWER_FRAMES VIEWER_ORBIT \
      VIEWER_ORTHO VIEWER_VIEW VIEWER_ZOOM VIEWER_W VIEWER_H VIEWER_THICKNESS || true

RECORD=0; ONLY=""
while [ $# -gt 0 ]; do
  case "$1" in
    --record) RECORD=1 ;;
    --only)   ONLY=${2:?--only needs a scene name}; shift ;;
    *) echo "usage: $0 [--record] [--only <scene>]" >&2; exit 2 ;;
  esac
  shift
done

# Which .pb files a manifest names, and whether they are all on disk.
missing_assets() {
  local f
  for f in $(sed -n 's/^file = "\([^"]*\)".*/\1/p' "assets/scenes/$1.toml"); do
    [ -f "assets/$f" ] || printf '%s ' "$f"
  done
}

# One measurement: "ink<TAB>draws<TAB>objects<TAB>sha16". `draws`/`objects` come from the
# `headless frame:` line, which is a log::info! and therefore on STDERR - hence 2>&1, not the
# 2>/dev/null a from-memory version of this script would use.
measure() {
  local cfg=$1 scene=$2 r ink dro sha
  r=$(env $cfg cargo run -q --example selftest --target "$T" --release -- \
        "$OUT" "assets/scenes/$scene.toml" 2>&1) || { printf '%s\n' "$r" >&2; return 1; }
  ink=$(sed -n 's/.*non-background pixels: \([0-9]*\).*/\1/p' <<<"$r" | tail -1)
  dro=$(sed -n 's/.*headless frame: \([0-9]*\) draws, \([0-9]*\) objects.*/\1\t\2/p' <<<"$r" | tail -1)
  sha=$(sha256sum "$OUT" | cut -c1-16)
  [ -n "$ink" ] && [ -n "$dro" ] || { printf '%s\n' "$r" >&2; return 1; }
  printf '%s\t%s\t%s\n' "$ink" "$dro" "$sha"
}

FAIL=0
rows() {                                    # rows <sink> <scene>...
  local sink=$1 s cfg m1 m2; shift
  for s in "$@"; do
    for cfg in "${CFGS[@]}"; do
      m1=$(measure "$cfg" "$s") || { echo "!! selftest FAILED: $s [${cfg:-default}]" >&2; FAIL=1; continue; }
      m2=$(measure "$cfg" "$s") || { echo "!! selftest FAILED: $s [${cfg:-default}]" >&2; FAIL=1; continue; }
      case " $SPLAT_SCENES " in *" $s "*)
        m1="$(cut -f1-3 <<<"$m1")	nondet(splat)"
        m2="$(cut -f1-3 <<<"$m2")	nondet(splat)" ;;
      esac
      if [ "$m1" != "$m2" ]; then
        echo "!! NONDETERMINISTIC $s [${cfg:-default}]: pass1 $m1 | pass2 $m2" >&2; FAIL=1
      fi
      printf '%s\t%s\t1\t%s\n' "$s" "${cfg:-default}" "$m1" >> "$sink"
      printf '%s\t%s\t2\t%s\n' "$s" "${cfg:-default}" "$m2" >> "$sink"
    done
  done
}

if [ -n "$ONLY" ] && [ -n "$(missing_assets "$ONLY")" ]; then
  echo "gate: SKIPPING scene '$ONLY' - assets absent: $(missing_assets "$ONLY")" >&2; exit 0
fi

echo "gate: building the harness once, so the timing of the first run is not the compiler's"
cargo build -q --example selftest --target "$T" --release

NEW_M=$(mktemp); NEW_A=$(mktemp); trap 'rm -f "$NEW_M" "$NEW_A"' EXIT

if [ -n "$ONLY" ]; then
  rows "$NEW_M" "$ONLY"
  echo "# scene	config	pass	ink	draws	objects	ppm_sha256_16"
  cat "$NEW_M"
  exit $FAIL
fi

rows "$NEW_M" $MANDATORY

for s in $ADVISORY; do
  miss=$(missing_assets "$s")
  if [ -n "$miss" ]; then
    echo "gate: SKIPPING advisory scene '$s' - gitignored assets absent: $miss" >&2
    continue
  fi
  rows "$NEW_A" "$s"
done

if [ "$RECORD" = 1 ]; then
  {
    echo "# docs/_GOLDENS.tsv - recorded by docs/_gate.sh --record at git tag end-of-44."
    echo "# The baseline is a TAG, not a working tree: an uncommitted edit moves these numbers."
    echo "# columns: scene	config	pass	ink	draws	objects	ppm_sha256_16"
    echo "# MANDATORY - resolves entirely to tracked .pb; a fresh clone must reproduce these."
    cat "$NEW_M"
    echo "# ADVISORY - local-only scenes; gitignored .pb assets. Never fails the gate."
    cat "$NEW_A"
  } > "$TSV"
  echo "gate: recorded $(grep -vc '^#' "$TSV") rows into $TSV"
  exit $FAIL
fi

if [ ! -s "$TSV" ] || ! grep -qv '^#' "$TSV" 2>/dev/null; then
  echo "gate: NOT ENFORCED - $TSV holds no rows yet. Run './docs/_gate.sh --record' at end-of-44." >&2
  echo "# scene	config	pass	ink	draws	objects	ppm_sha256_16"
  cat "$NEW_M" "$NEW_A"
  exit $FAIL
fi

pick() { awk -v set=" $2 " '!/^#/ && index(set, " " $1 " ")' "$1"; }

if ! diff -u <(pick "$TSV" "$MANDATORY") "$NEW_M"; then
  echo "gate: MANDATORY rows differ - the frame changed." >&2; FAIL=1
fi
if [ -s "$NEW_A" ] && ! diff -u <(pick "$TSV" "$ADVISORY") "$NEW_A"; then
  echo "gate: advisory rows differ (informational only)." >&2
fi

if [ "$FAIL" = 0 ]; then echo "gate OK"; fi
exit $FAIL
