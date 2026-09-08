#!/usr/bin/env bash
# Publish a coherent live revision, preserving the existing stable scene/geometry aliases.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/view.sh"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

if [ "$#" = 2 ]; then
    manifest="$1"
    geometry="$2"
    case "$manifest" in *.toml) suffix=toml ;; *.yaml|*.yml|*.json) suffix=yaml ;; *) echo "ERROR: expected a TOML, YAML or JSON scene" >&2; exit 1 ;; esac
elif [ "$#" -le 1 ]; then
    suffix=yaml
    if [ "$#" = 1 ]; then
        [ -d "$1" ] || { echo "ERROR: not a directory: $1" >&2; exit 1; }
        directories=("$1")
    else
        directories=("." "./out" "./pb" "./data/output/pb" "${REPO_ROOT}/session_viewer/assets")
    fi
    manifest=""
    for directory in "${directories[@]}"; do
        if [ -s "${directory}/view_live.yaml" ] && [ -s "${directory}/view_live.pb" ]; then
            manifest="${directory}/view_live.yaml"
            geometry="${directory}/view_live.pb"
            break
        fi
    done
    [ -n "$manifest" ] || { printf 'ERROR: no view_live.yaml/view_live.pb pair in: %s\n' "${directories[*]}" >&2; exit 1; }
else
    echo "Usage: view_live.sh [directory] | view_live.sh scene.toml scan.pb" >&2
    exit 1
fi
[ -s "$manifest" ] && [ -s "$geometry" ] || { echo "ERROR: manifest or geometry missing/empty" >&2; exit 1; }
case "$geometry" in *.pb) ;; *) echo "ERROR: geometry must be .pb" >&2; exit 1 ;; esac

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"; rm -f "${R2_CURL_CONFIG:-}"' EXIT
revision="pb/revisions/$(sha256sum "$geometry" | cut -d' ' -f1).pb"
prepared="${scratch}/view_live.${suffix}"
python3 "${SCRIPT_DIR}/lib/view_manifest.py" "$manifest" "$geometry" "$revision" "$prepared"
python3 "${SCRIPT_DIR}/lib/view_manifest.py" "$prepared" > "${scratch}/files"
r2_require_credentials
# Materialize credential ownership before command-substitution/background helpers run.
r2_curl_config
trap 'rm -rf "$scratch"; rm -f "${R2_CURL_CONFIG:-}"' EXIT

# Verify other referenced payloads before any publication mutation.
while IFS= read -r entry; do
    [ "$entry" = "$revision" ] && continue
    case "$entry" in https://*) url="$entry" ;; *) url="${R2_PUBLIC}/${entry#./}" ;; esac
    code=$(curl --connect-timeout 10 --max-time 30 --retry 2 -sS -o /dev/null -w '%{http_code}' -I "$url")
    [ "$code" = 200 ] || { echo "ERROR: referenced payload $entry returned HTTP $code" >&2; exit 1; }
done < "${scratch}/files"

started=$(r2_now_ms)
r2_revision "$geometry" "$revision"
# Existing consumers can still read the stable payload; the new manifest uses immutable bytes.
r2_alias "$revision" "pb/view_live.pb" "$(stat -c%s "$geometry")"
r2_upload "$prepared" "scenes/view_live.${suffix}"
# Default deployed consumers poll YAML. The optional TOML alias has identical semantics.
if [ "$suffix" = toml ]; then
    python3 "${SCRIPT_DIR}/lib/view_manifest.py" "$prepared" "$geometry" "$revision" "${scratch}/view_live.yaml"
    r2_upload "${scratch}/view_live.yaml" "scenes/view_live.yaml"
fi
published=$(r2_now_ms)
printf 'publication verified: %s ms; open ?scene=view_live.%s\n' "$((published-started))" "$suffix"
r2_notify "view_live.pb"

# Workspace root: ./bash/view_live.sh scene.toml scan.pb (or ./bash/view_live.sh [directory]).
# View: http://localhost:8770/?scene=view_live.toml ; credentials stay in ~/.aws/credentials.
