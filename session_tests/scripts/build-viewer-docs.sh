#!/usr/bin/env bash
# Trunk pre-build hook (session_viewer/Trunk.toml): builds this docs site into
# session_viewer/target/docs/vue, which Trunk copies to dist/docs, so the viewer's corner link
# opens it on `trunk serve` too. Base /docs/ because trunk serves the viewer at the root.
# Quiet no-op while nothing the site reads is newer than the last build; a failed build keeps
# the previous site, so a half-edited course page never stops a viewer build.
set -euo pipefail
site=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
session=$(dirname "$site")
viewer="$session/session_viewer"
out="$viewer/target/docs/vue"
stamp="$out/index.html"

stale() {
    [ -f "$stamp" ] || return 0
    local sources=()
    for p in "$site/src" "$site/plugins" "$site/public" "$site/index.html" "$site/vite.config.ts" \
        "$site/package-lock.json" "$viewer/docs" "$viewer/ARCHITECTURE.md" "$viewer/mkdocs.yml" \
        "$session/session_rust/src" "$session/session_cpp/src" "$session/session_proto"; do
        if [ -e "$p" ]; then
            sources+=("$p")
        fi
    done
    [ -n "$(find "${sources[@]}" \( -name target -o -name node_modules -o -name __pycache__ -o -name .git \) -prune \
        -o -type f -newer "$stamp" -print -quit)" ]
}

placeholder() {
    mkdir -p "$out"
    cat > "$stamp" <<'HTML'
<!doctype html><html lang="en"><meta charset="utf-8"><title>Session documentation</title>
<body style="font:16px system-ui;padding:3rem"><h1>Documentation not built</h1>
<p>The docs build failed or npm is missing. Run <code>npm install</code> and <code>npm run build</code>
in <code>session/session_tests</code> (Node 20) to see why, then rebuild the viewer.</p></body></html>
HTML
    # An old mtime keeps the placeholder stale, so the next viewer build tries again.
    touch -d '2000-01-01' "$stamp"
}

stale || exit 0
mkdir -p "$(dirname "$out")"
# Parallel viewer builds share this output; the second waits and then finds it fresh.
exec 9>"$viewer/target/docs/vue-build.lock"
flock 9
stale || exit 0

if ! command -v npm >/dev/null 2>&1; then
    echo "build-viewer-docs.sh: npm not found; dist/docs is a placeholder" >&2
    [ -f "$stamp" ] || placeholder
    exit 0
fi
cd "$site"
if [ ! -d node_modules ]; then
    npm ci --no-audit --no-fund --loglevel=error >&2 || true
fi
rm -rf "$out.new"
log="$viewer/target/docs/vue-build.log"
if DOCS_BASE=/docs/ npx --no-install vite build --logLevel error --outDir "$out.new" --emptyOutDir >"$log" 2>&1; then
    rm -rf "$out"
    mv "$out.new" "$out"
else
    rm -rf "$out.new"
    head -4 "$log" >&2
    echo "build-viewer-docs.sh: docs build failed (full log $log); dist/docs keeps the previous site" >&2
    [ -f "$stamp" ] || placeholder
fi
