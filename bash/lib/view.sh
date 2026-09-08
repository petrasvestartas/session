#!/usr/bin/env bash
# Shared Cloudflare R2 settings and helpers for view_put.sh and view_live.sh.
#
# Stable manifests live under scenes/, compatibility payloads under pb/view_*, and immutable
# payloads under pb/revisions/. The viewer also supports local regression/tutorial fixtures.

R2_BUCKET="${R2_BUCKET:-session-viewer-data}"
R2_ACCOUNT="${R2_ACCOUNT:-0520459c6817bd96c1e25fcb49461c4e}"
R2_ENDPOINT="${R2_ENDPOINT:-https://${R2_ACCOUNT}.r2.cloudflarestorage.com}"
R2_PUBLIC="${R2_PUBLIC:-https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev}"
R2_PROFILE="r2"

# The relay an open viewer listens on. Posting here turns "within one poll" into "within a
# fraction of a second"; the message body is ignored by the page, only the fact of it matters.
R2_NOTIFY="https://ntfy.sh/wood-live-84eaac4a04729911"

# One value of the `[r2]` profile in ~/.aws/credentials.
r2_credential() {
    awk -v want="$1" -v prof="[${R2_PROFILE}]" '
        $0 == prof { in_prof = 1; next }
        /^\[/ { in_prof = 0 }
        in_prof && $1 == want { print $3; exit }
    ' "$HOME/.aws/credentials" 2>/dev/null
}

# Fail early and say what is missing, rather than letting a tool print a stack of XML.
r2_require_credentials() {
    if [ -z "$(r2_credential aws_access_key_id)" ] || [ -z "$(r2_credential aws_secret_access_key)" ]; then
        cat >&2 <<CREDS
ERROR: no [${R2_PROFILE}] profile with keys in ~/.aws/credentials.

Create an R2 API token with **Object Read & Write** at
  https://dash.cloudflare.com/${R2_ACCOUNT}/r2/api-tokens
then add its two values:

  [${R2_PROFILE}]
  region = auto
  aws_access_key_id = <access key id>
  aws_secret_access_key = <secret access key>
CREDS
        return 1
    fi
}

# A curl config file holding the credentials, mode 0600, so the secret never appears on a
# command line (`ps` shows arguments to every user). Made once per shell, removed on exit.
r2_curl_config() {
    if [ -z "${R2_CURL_CONFIG:-}" ]; then
        R2_CURL_CONFIG=$(umask 077 && mktemp) || return 1
        trap 'rm -f "$R2_CURL_CONFIG"' EXIT
        printf 'user = "%s:%s"\n' "$(r2_credential aws_access_key_id)" "$(r2_credential aws_secret_access_key)" > "$R2_CURL_CONFIG"
    fi
}

# PUT one file to one key with curl's built-in SigV4 signing: one HTTPS request and no
# Python start-up (the aws CLI cost ~0.5 s per call). Prints the HTTP status.
r2_put() {
    local src="$1" key="$2" type="application/octet-stream" cache="no-cache"
    r2_curl_config || return 1
    case "$key" in *.toml) type="application/toml" ;; *.yaml|*.yml) type="application/yaml" ;; *.json) type="application/json" ;; esac
    case "$key" in pb/revisions/*) cache="public, max-age=31536000, immutable" ;; esac
    curl --connect-timeout 10 --max-time 180 --retry 2 --retry-delay 1 --retry-max-time 200 -sS -o /dev/null -w "%{http_code}" -X PUT -T "$src" \
        --aws-sigv4 "aws:amz:auto:s3" -K "$R2_CURL_CONFIG" \
        -H "Content-Type: $type" -H "Cache-Control: $cache" \
        "${R2_ENDPOINT}/${R2_BUCKET}/${key}"
}

# The HTTP status the public URL answers a HEAD with: 200 = there, 404 = not there, anything
# else = the store is not answering properly right now.
r2_head_status() {
    curl --connect-timeout 10 --max-time 180 --retry 2 --retry-delay 1 --retry-max-time 200 -sS -o /dev/null -w "%{http_code}" -I "${R2_PUBLIC}/${1}"
}

# Upload one file to one key, then CHECK it arrived: the public URL must answer 200 with the
# same byte count. An upload that reports success and serves nothing is the failure worth
# catching, because the page keeps drawing the previous scene and looks fine.
r2_upload() {
    local src="$1" key="$2"
    local size code served
    size=$(stat -c%s "$src" 2>/dev/null || stat -f%z "$src")

    echo "  ${src}  ->  s3://${R2_BUCKET}/${key}  (${size} bytes)"
    code=$(r2_put "$src" "$key") || return 1
    if [ "$code" != "200" ]; then
        echo "ERROR: PUT ${key} answered HTTP ${code}" >&2
        return 1
    fi

    served=$(curl --connect-timeout 10 --max-time 30 --retry 2 --retry-delay 1 --retry-max-time 60 -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
    if [ "$served" != "$size" ]; then
        echo "ERROR: uploaded ${size} bytes but ${R2_PUBLIC}/${key} serves '${served:-nothing}'" >&2
        return 1
    fi
    echo "  verified: ${R2_PUBLIC}/${key}"
}

# A content-addressed key is immutable: reuse verified bytes instead of uploading them again.
r2_revision() {
    local src="$1" key="$2" code size
    case "$key" in pb/revisions/*.pb) ;; *) echo 'ERROR: expected a content-addressed revision key' >&2; return 1 ;; esac
    size=$(stat -c%s "$src" 2>/dev/null || stat -f%z "$src")
    code=$(r2_head_status "$key") || return 1
    case "$code" in
        200) r2_verify "$key" "$size" ;;
        404) r2_upload "$src" "$key" ;;
        *) echo "ERROR: revision lookup ${key} answered HTTP ${code}" >&2; return 1 ;;
    esac
}

# `r2_upload`, but the verify runs in the background: the caller `wait`s on `$!` and gets the
# verify's exit status, so two uploads and their checks overlap instead of queueing.
r2_upload_start() {
    local src="$1" key="$2"
    local size code
    size=$(stat -c%s "$src" 2>/dev/null || stat -f%z "$src")

    echo "  ${src}  ->  s3://${R2_BUCKET}/${key}  (${size} bytes)"
    code=$(r2_put "$src" "$key") || return 1
    if [ "$code" != "200" ]; then
        echo "ERROR: PUT ${key} answered HTTP ${code}" >&2
        return 1
    fi
    r2_verify "$key" "$size" &
}

# The public URL must serve `size` bytes for `key`.
r2_verify() {
    local key="$1" size="$2" served
    served=$(curl --connect-timeout 10 --max-time 30 --retry 2 --retry-delay 1 --retry-max-time 60 -sSI "${R2_PUBLIC}/${key}" | tr -d '\r' | awk 'tolower($1)=="content-length:" {print $2}')
    if [ "$served" != "$size" ]; then
        echo "ERROR: uploaded ${size} bytes but ${R2_PUBLIC}/${key} serves '${served:-nothing}'" >&2
        return 1
    fi
    echo "  verified: ${R2_PUBLIC}/${key}"
}

# Tell any open viewer to look now instead of waiting for its next poll. Best effort: the page
# converges on its own, so a relay that is down is not a failed publish.
r2_notify() {
    if [ "${R2_NOTIFY_ENABLED:-1}" = 0 ]; then return 0; fi
    if curl -fsS -m 5 -d "${1:-published}" "$R2_NOTIFY" >/dev/null 2>&1; then
        echo "  notified ${R2_NOTIFY}"
    else
        echo "  (relay unreachable; open pages pick this up on their next poll)"
    fi
}

# Copy a verified immutable object to the stable alias without uploading the payload twice.
# R2 supports S3 CopyObject. Read the XML body too: HTTP 200 may contain an embedded error.
r2_alias() {
    local source="$1" key="$2" size="$3" response code
    r2_curl_config || return 1
    response=$(mktemp) || return 1
    code=$(curl --connect-timeout 10 --max-time 180 --retry 2 --retry-delay 1 --retry-max-time 200 \
        -sS -o "$response" -w '%{http_code}' -X PUT --data '' \
        --aws-sigv4 'aws:amz:auto:s3' -K "$R2_CURL_CONFIG" \
        -H "x-amz-copy-source: /${R2_BUCKET}/${source}" \
        -H 'x-amz-metadata-directive: REPLACE' -H 'Content-Type: application/octet-stream' -H 'Cache-Control: no-cache' \
        "${R2_ENDPOINT}/${R2_BUCKET}/${key}") || { rm -f "$response"; return 1; }
    if [ "$code" != 200 ] || ! python3 -c 'import sys, xml.etree.ElementTree as E; root=E.parse(sys.argv[1]).getroot(); sys.exit(root.tag.split("}")[-1] != "CopyObjectResult")' "$response"; then
        rm -f "$response"
        echo "ERROR: copying ${source} to ${key} failed (HTTP ${code})" >&2
        return 1
    fi
    rm -f "$response"
    r2_verify "$key" "$size"
}

# Monotonic elapsed timing is independent of wall-clock adjustments and date formatting.
r2_now_ms() { python3 -c 'import time; print(time.monotonic_ns() // 1000000)'; }
