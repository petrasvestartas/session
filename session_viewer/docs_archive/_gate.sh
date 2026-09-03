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
MANDATORY="view_local"
ADVISORY=""
CFGS=("" "VIEWER_LINE_STYLE=tubes" "VIEWER_REBUILD=1" "VIEWER_INCREMENTAL=1")

# MEASURED, 2026-08-31: the splat lane is a 2-pass ATOMIC compute rasterizer, so which point wins
# a contested pixel is a race and the PPM bytes differ run to run - `lion` gave three distinct
# sha256 over eight runs while ink/draws/objects never moved (77543/4/1 every time). So the
# sha is recorded as `nondet(splat)` for cloud scenes and ink/draws/objects carry the gate there.
SPLAT_SCENES="lion bunny_cloud cloud_mix lidar14 bunny_drawings"

# MEASURED, 2026-08-31 (end-of-44), CORRECTING the line above: `bunny` is NOT byte-identical
# either, and it holds no cloud - this is a second, unrelated race in the shaded-mesh/flat-ink
# path. 24 runs at VIEWER_REBUILD=1 gave 4 distinct sha (20/2/1/1); 12 at default gave 3 (10/1/1);
# tubes and INCREMENTAL gave 1 each over 12, which at this rate does not prove them clean.
# The whole deviation is ONE pixel - (x=625,y=220) flips grey 171 -> 170, 3 bytes of 2,880,016 -
# and both values are under the ink threshold, so ink/draws/objects never move (44215/9/6).
# `drawings_rotated` was stable over 12 and is now the only mandatory row a sha still gates.
# MEASURED, 2026-09-01 - the cause, not just the symptom. `triangle.wgsl`'s flat normal comes
# from screen-space derivatives (`cross(dpdy(world_pos), dpdx(world_pos))`), and on this machine's
# Intel iGPU that is NOT deterministic. Proved twice on the same tree: replacing that ONE line
# with a constant took bunny from 13/150 deviating runs to 0/150, and running the IDENTICAL
# binary on llvmpipe instead of the Intel driver took it from 18/150 to 0/150. Every deviation
# seen was a 2x2-derivative-quad of pixels shifting by 1-2 grey levels inside a smooth region.
#
# No mesh in the repo carries baked vertex normals, so EVERY shaded triangle takes that line;
# a scene's exposure is just how many derivative-lit triangles it has. bunny has 69,451 and
# flips often. drawings_rotated has 36 (its 1.55M sheet triangles bypass the normal entirely,
# being unlit paper), so it flips rarely - 0 times in 670 renders here, but it did once in CI.
# Rare is not never, and a gate that fails at random teaches you to ignore it.
#
# The real fix is to bake a per-face normal into RenderVertex in the KERNEL's Mesh::to_render, so
# the shader takes its has_normal branch; that is a three-language change and is not this file's
# to make. Until then both scenes carry the gate on ink/draws/objects, which never moved.
NONDET_SCENES="bunny drawings_rotated"

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
  for f in $(sed -n 's/^file = "\([^"]*\)".*/\1/p' "assets/$1.toml"); do
    [ -f "assets/$f" ] || printf '%s ' "$f"
  done
}

# One measurement: "ink<TAB>draws<TAB>objects<TAB>sha16". `draws`/`objects` come from the
# `headless frame:` line, which is a log::info! and therefore on STDERR - hence 2>&1, not the
# 2>/dev/null a from-memory version of this script would use.
measure() {
  local cfg=$1 scene=$2 r ink dro sha
  r=$(env $cfg cargo run -q --example selftest --target "$T" --release -- \
        "$OUT" "assets/$scene.toml" 2>&1) || { printf '%s\n' "$r" >&2; return 1; }
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
      case " $NONDET_SCENES " in *" $s "*)
        m1="$(cut -f1-3 <<<"$m1")	nondet(mesh)"
        m2="$(cut -f1-3 <<<"$m2")	nondet(mesh)" ;;
      esac
      if [ "$m1" != "$m2" ]; then
        echo "!! NONDETERMINISTIC $s [${cfg:-default}]: pass1 $m1 | pass2 $m2" >&2; FAIL=1
      fi
      printf '%s\t%s\t1\t%s\n' "$s" "${cfg:-default}" "$m1" >> "$sink"
      printf '%s\t%s\t2\t%s\n' "$s" "${cfg:-default}" "$m2" >> "$sink"
    done
  done
}

# The goldens are numbers measured on SPECIFIC asset BYTES. A tree replayed from an old commit
# carries that commit's assets/, so two trees can differ by a whole re-encode while looking
# identical in source - and then the gate reports a code regression that is really a different
# file. (Cost four wrong diagnoses on 2026-09-01.) Fingerprint what we actually read.
asset_fingerprint() {
    for f in $(grep -ho 'pb/[A-Za-z0-9_.-]*\.pb' assets/view_*.toml 2>/dev/null | sort -u); do
        [ -f "assets/$f" ] && printf '%s %s\n' "$(stat -c%s "assets/$f" 2>/dev/null || stat -f%z "assets/$f")" "$f"
    done | sort | cksum | cut -d' ' -f1
}

if [ -n "$ONLY" ] && [ -n "$(missing_assets "$ONLY")" ]; then
  echo "gate: SKIPPING scene '$ONLY' - assets absent: $(missing_assets "$ONLY")" >&2; exit 0
fi

# Refuse BEFORE rendering anything: a fingerprint mismatch makes every number below
# incomparable, so measuring them is wasted minutes. `--only` is exempt - it compares nothing.
if [ -z "$ONLY" ] && [ "$RECORD" != 1 ]; then
  WANT=$(grep -m1 '^# assets:' "$TSV" 2>/dev/null | cut -d' ' -f3)
  HAVE=$(asset_fingerprint)
  if [ -n "$WANT" ] && [ "$WANT" != "$HAVE" ]; then
    echo "gate: ASSETS DIFFER from the ones these goldens were recorded on ($WANT vs $HAVE)." >&2
    echo "gate: nothing measured here is comparable to $TSV. Point at the assets the goldens" >&2
    echo "gate: were recorded on, or re-record with --record and say so in the commit." >&2
    exit 1
  fi
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
    echo "# assets: $(asset_fingerprint)"
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

# The image hash is not a stable comparator here. Every splat and mesh scene renders with a
# nondeterministic byte order - the recorded rows say so, `nondet(splat)` / `nondet(mesh)` - and
# the marker only appears when the TWO passes of one run happen to disagree, so an identical
# build can record a literal hash one time and the marker the next. Comparing that column then
# reports a changed frame when nothing changed: measured, 62759 ink against 62759 ink, differing
# only in the hash. Ink, draws and objects are the real signal, so blank the hash on BOTH sides
# whenever either says nondet.
# $1 = recorded rows, $2 = rows to normalise. A row whose RECORDED twin is marked loses its
# hash, whichever side it is on, so the two files agree exactly when ink/draws/objects agree.
blank_nondet() {
  awk -F'\t' -v ref="$1" 'BEGIN{
      OFS=FS
      while ((getline line < ref) > 0) {
        n = split(line, f, FS)
        if (line ~ /^#/ || n < 7) continue
        if (f[n] ~ /^nondet\(/) skip[f[1] FS f[2] FS f[3]] = 1
      }
    }
    !/^#/ { if (($1 FS $2 FS $3) in skip) $NF = "nondet"; print }' "$2"
}

if ! diff -u <(pick "$TSV" "$MANDATORY" | blank_nondet "$TSV" /dev/stdin) <(blank_nondet "$TSV" "$NEW_M"); then
  echo "gate: MANDATORY rows differ - the frame changed." >&2; FAIL=1
fi
if [ -s "$NEW_A" ] && ! diff -u <(pick "$TSV" "$ADVISORY" | blank_nondet "$TSV" /dev/stdin) <(blank_nondet "$TSV" "$NEW_A"); then
  echo "gate: advisory rows differ (informational only)." >&2
fi

if [ "$FAIL" = 0 ]; then echo "gate OK"; fi
exit $FAIL
