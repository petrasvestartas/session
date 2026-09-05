#!/usr/bin/env bash
# Put one .pb in the bucket AND give it a scene, so it is viewable in one step.
#
#   bash/view_put.sh <file.pb> [name]
#
#   bash/view_put.sh out/scan.pb          -> pb/view_scan.pb + scenes/view_scan.yaml
#   bash/view_put.sh out/scan.pb lidar_a  -> pb/view_lidar_a.pb + scenes/view_lidar_a.yaml
#
# It prints the `?scene=` to open. A .pb on its own is not viewable - the page loads a MANIFEST
# and draws what that names - so uploading one without writing a scene for it just leaves an
# orphan nobody can see. The scene is a single item at the origin; edit it afterwards
# (`aws s3 cp` it down, change `at`, put it back) when the file needs placing.
#
# An EXISTING scene of that name is never overwritten: it may place several files, and clobbering
# it with a one-item scene would silently drop the rest. Pass a different name instead.
set -u

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
source "${SCRIPT_DIR}/lib/view.sh"

src="${1:-}"
if [ -z "$src" ]; then
    echo "Usage: view_put.sh <file.pb> [name]"
    echo "       uploads pb/view_<name>.pb and writes scenes/view_<name>.yaml"
    exit 1
fi
[ -f "$src" ] || { echo "ERROR: no such file: $src" >&2; exit 1; }
[ -s "$src" ] || { echo "ERROR: $src is empty" >&2; exit 1; }
case "$src" in *.pb) ;; *) echo "ERROR: $src is not a .pb" >&2; exit 1 ;; esac

# view_<name>: from the second argument, else the file's own basename. Already-prefixed names are
# left alone rather than becoming view_view_x.
name="${2:-$(basename "$src" .pb)}"
name="${name#view_}"
stem="view_${name}"
key="pb/${stem}.pb"
scene="scenes/${stem}.yaml"

r2_require_credentials || exit 1

existing=$(curl -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
[ -n "$existing" ] && echo "  replacing ${key} (was ${existing} bytes)"

r2_upload "$src" "$key" || exit 1

# The scene. Written only when there is none, so a hand-placed scene survives a re-upload of the
# geometry it names - which is the normal case: the .pb changes, the placement does not.
scene_status=$(r2_head_status "$scene")
if [ "$scene_status" = "200" ]; then
    echo "  ${scene} exists - kept as it is (delete it first if you want a fresh one)"
elif [ "$scene_status" != "404" ]; then
    echo "ERROR: ${R2_PUBLIC}/${scene} answered HTTP ${scene_status}; not writing a scene blind" >&2
    exit 1
else
    tmp=$(mktemp) && trap 'rm -f "$tmp" "${R2_CURL_CONFIG:-}"' EXIT
    cat > "$tmp" <<EOF
# Written by bash/view_put.sh for ${stem}.pb. One item at the origin - edit \`at\` to place it,
# or add more entries under \`items\`; nothing regenerates this file, so your changes stay.
name: "${name}"

items:
  - file: "${key}"
    name: "${name}"
    at: [0, 0, 0]
EOF
    r2_upload "$tmp" "$scene" || exit 1
fi

echo
echo "  open:  ?scene=${stem}.yaml"
