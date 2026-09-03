#!/usr/bin/env bash
# Shared Cloudflare R2 settings and helpers for view_put.sh and view_live.sh.
#
# The bucket is the viewer's ONLY storage location. The `session_viewer_data` git branch it
# replaced was DELETED on 2026-09-03 - geometry does not go in git. What the page does with these files is in session_viewer/src/app/live.rs,
# and the bucket's own view_readme.md documents the layout. Every key in the bucket starts with
# `view_` so it is obvious at a glance what a file is for.

R2_BUCKET="session-viewer-data"
R2_ACCOUNT="0520459c6817bd96c1e25fcb49461c4e"
R2_ENDPOINT="https://${R2_ACCOUNT}.r2.cloudflarestorage.com"
R2_PUBLIC="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev"
R2_PROFILE="r2"

# The relay an open viewer listens on. Posting here is what turns "within one poll" into "within
# about a second"; the message body is ignored by the page, only the fact of it matters.
R2_NOTIFY="https://ntfy.sh/wood-live-84eaac4a04729911"

# `aws` is installed per-user by `uv tool install awscli`, which on this machine lands outside
# PATH. Prefer whatever is on PATH, fall back to that location, complain if neither exists.
r2_aws() {
    local bin=""
    bin=$(command -v aws 2>/dev/null) || bin=""
    if [ -z "$bin" ]; then
        for c in "$HOME/.local/bin/aws" "$HOME"/snap/code/*/.local/bin/aws; do
            [ -x "$c" ] && { bin="$c"; break; }
        done
    fi
    if [ -z "$bin" ]; then
        echo "ERROR: no 'aws' on PATH. Install it with:  uv tool install awscli" >&2
        return 127
    fi
    "$bin" --profile "$R2_PROFILE" --endpoint-url "$R2_ENDPOINT" "$@"
}

# Fail early and say what is missing, rather than letting aws print a stack of XML.
r2_require_credentials() {
    if ! grep -q "^\[${R2_PROFILE}\]" "$HOME/.aws/credentials" 2>/dev/null; then
        cat >&2 <<EOF
ERROR: no [${R2_PROFILE}] profile in ~/.aws/credentials.

Create an R2 API token with **Object Read & Write** at
  https://dash.cloudflare.com/${R2_ACCOUNT}/r2/api-tokens
then add its two values:

  [${R2_PROFILE}]
  region = auto
  aws_access_key_id = <access key id>
  aws_secret_access_key = <secret access key>
EOF
        return 1
    fi
}

# Upload one file to one key, then CHECK it arrived: the public URL must answer 200 with the
# same byte count. An upload that reports success and serves nothing is the failure worth
# catching, because the page will keep drawing the previous scene and look fine.
r2_upload() {
    local src="$1" key="$2"
    local size
    size=$(stat -c%s "$src" 2>/dev/null || stat -f%z "$src")

    echo "  ${src}  ->  s3://${R2_BUCKET}/${key}  (${size} bytes)"
    r2_aws s3 cp "$src" "s3://${R2_BUCKET}/${key}" --no-progress >/dev/null || return 1

    local served
    served=$(curl -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
    if [ "$served" != "$size" ]; then
        echo "ERROR: uploaded ${size} bytes but ${R2_PUBLIC}/${key} serves '${served:-nothing}'" >&2
        return 1
    fi
    echo "  verified: ${R2_PUBLIC}/${key}"
}

# Tell any open viewer to look now instead of waiting for its next poll. Best effort: the page
# converges on its own, so a relay that is down is not a failed publish.
r2_notify() {
    if curl -fsS -m 5 -d "${1:-published}" "$R2_NOTIFY" >/dev/null 2>&1; then
        echo "  notified ${R2_NOTIFY}"
    else
        echo "  (relay unreachable; open pages pick this up on their next poll)"
    fi
}
