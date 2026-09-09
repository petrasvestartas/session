#!/usr/bin/env bash
# Trunk pre-build hook: keep target/docs/site current so the viewer can copy it into dist/docs.
# Rebuilds only when a documentation source is newer than the built site; quiet otherwise.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
site="$root/target/docs/site"
stamp="$site/index.html"
newer=$(find "$root/docs" "$root/ARCHITECTURE.md" "$root/mkdocs.yml" -newer "$stamp" -type f -not -path '*/__pycache__/*' 2>/dev/null | head -1 || true)
if [ -f "$stamp" ] && [ -z "$newer" ]; then
    exit 0
fi
if command -v uvx >/dev/null 2>&1 && [ -x "$root/docs/serve.sh" ] && [ -f "$root/mkdocs.yml" ]; then
    "$root/docs/serve.sh" build --quiet
else
    mkdir -p "$site"
    cat > "$stamp" <<'HTML'
<!doctype html><html lang="en"><meta charset="utf-8"><title>Session Viewer course</title>
<body style="font:16px system-ui;padding:3rem"><h1>Documentation not built</h1>
<p>The course sources are not in this checkout, or <a href="https://docs.astral.sh/uv/">uv</a> is missing.
Run <code>docs/serve.sh build</code> in the maintained viewer checkout, then rebuild.</p></body></html>
HTML
    echo "docs/build_site.sh: course sources or uvx not found; wrote a placeholder page" >&2
fi
