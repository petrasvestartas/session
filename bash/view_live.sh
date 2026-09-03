#!/usr/bin/env bash
# Publish what the deployed viewer shows: https://petrasvestartas.github.io/session/
#
#   bash/view_live.sh                 publish the view_live pair found next to each other
#   bash/view_live.sh <dir>           look only in <dir>
#
# It takes NO file arguments on purpose. The live scene is one fixed pair - `view_live.toml` and
# `view_live.pb` - so naming them every time is ceremony that can only be got wrong; the script
# finds them instead, and they must be SIDE BY SIDE in one directory. When it cannot find them it
# lists every directory it looked in and which of the two was missing, because "publish failed"
# without that is the message that wastes the next ten minutes.
#
# -> pb/view_live.pb and scenes/view_live.toml, geometry first so the manifest never names bytes
# that are not there yet. There is no version and no history: this replaces, and the old bytes
# are gone. An open page picks it up on its next poll (5 s), sooner if the relay is reachable.
set -u

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "${SCRIPT_DIR}/lib/view.sh"
REPO_ROOT="$( cd "${SCRIPT_DIR}/.." && pwd )"

SLOT_PB="view_live.pb"
SLOT_TOML="view_live.toml"

# Where a pair might sit, nearest first: an explicit directory, then the working directory and
# the places a run usually writes into, then the repo's own assets.
if [ $# -gt 0 ]; then
    [ -d "$1" ] || { echo "ERROR: not a directory: $1" >&2; exit 1; }
    DIRS=("$1")
else
    DIRS=("." "./out" "./pb" "./data/output/pb" "${REPO_ROOT}/session_viewer/assets")
fi

found=""
report=""
for d in "${DIRS[@]}"; do
    [ -d "$d" ] || { report="${report}
  ${d}  - no such directory"; continue; }
    have_pb=0; have_toml=0
    [ -s "${d}/${SLOT_PB}" ]   && have_pb=1
    [ -s "${d}/${SLOT_TOML}" ] && have_toml=1
    if [ "$have_pb" = 1 ] && [ "$have_toml" = 1 ]; then found="$d"; break; fi
    case "${have_pb}${have_toml}" in
        00) report="${report}
  ${d}  - neither ${SLOT_TOML} nor ${SLOT_PB}" ;;
        10) report="${report}
  ${d}  - has ${SLOT_PB}, MISSING ${SLOT_TOML}" ;;
        01) report="${report}
  ${d}  - has ${SLOT_TOML}, MISSING ${SLOT_PB}" ;;
    esac
done

if [ -z "$found" ]; then
    echo "nothing published: no directory holds ${SLOT_TOML} and ${SLOT_PB} side by side." >&2
    echo "looked in:${report}" >&2
    echo "" >&2
    echo "write both next to each other, or name their directory: bash/view_live.sh <dir>" >&2
    exit 1
fi

manifest="${found}/${SLOT_TOML}"
geometry="${found}/${SLOT_PB}"
echo "=== publishing from ${found}"

r2_require_credentials || exit 1

# A manifest that names nothing draws nothing, and the page would warn and keep the previous
# scene - which looks like the publish silently did not happen. Catch it here instead.
files=$(grep -oE '^[[:space:]]*file[[:space:]]*=[[:space:]]*"[^"]+"' "$manifest" | sed 's/.*"\(.*\)"/\1/')
[ -n "$files" ] || { echo "ERROR: ${manifest} lists no 'file = \"...\"' entry" >&2; exit 1; }

# Geometry first: a page polling in the gap must never read a manifest whose files 404.
r2_upload "$geometry" "pb/${SLOT_PB}" || exit 1

# Every OTHER file the manifest names has to be in the bucket already. `pb/view_live.pb` was just
# uploaded, so it is skipped; anything else is the author's to put there first.
missing=""
while IFS= read -r entry; do
    [ -z "$entry" ] && continue
    [ "$entry" = "pb/${SLOT_PB}" ] && { echo "  lists ${entry} (just uploaded)"; continue; }
    case "$entry" in
        https://*) url="$entry" ;;
        *)         url="${R2_PUBLIC}/${entry#./}" ;;
    esac
    code=$(curl -sS -o /dev/null -w "%{http_code}" -I "$url")
    if [ "$code" = "200" ]; then echo "  lists ${entry} (present)"
    else echo "  lists ${entry} -> HTTP ${code}"; missing="${missing} ${entry}"; fi
done <<< "$files"
if [ -n "$missing" ]; then
    echo "ERROR: not published - the manifest names files the bucket does not have:${missing}" >&2
    echo "       upload them first (bash/view_put.sh <file.pb>), or fix the paths." >&2
    exit 1
fi

r2_upload "$manifest" "scenes/${SLOT_TOML}" || exit 1
r2_notify "${SLOT_PB}"
echo "=== live"
