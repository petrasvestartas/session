#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/view.sh"
r2_curl_config() { R2_CURL_CONFIG=unused; }
encoded=1
curl() {
    case " $* " in
        *' -sSI '*) printf 'Content-Length: 42\r\n'; [ "$encoded" = 0 ] || printf 'Content-Encoding: gzip\r\n' ;;
        *) [[ " $* " = *' Content-Encoding: gzip '* ]] || return 9; printf 200 ;;
    esac
}
[ "$(r2_put unused test.pb gzip)" = 200 ]
r2_verify test.pb 42 gzip
encoded=0
if r2_verify test.pb 42 gzip; then echo 'accepted missing gzip header' >&2; exit 1; fi
r2_verify test.pb 42
