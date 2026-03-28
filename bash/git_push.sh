#!/bin/bash
m="$1"; [ -z "$m" ] && echo "Usage: git_push.sh \"commit message\"" && exit 1

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}/.."
cd "${REPO_ROOT}"

BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo -e "\n=== Branch: $BRANCH ==="

echo -e "\n=== Running tests before push ==="
if ! "${SCRIPT_DIR}/minitest.sh" --no-web; then
    echo -e "\nABORTED: tests failed — fix before pushing"
    exit 1
fi

FAILED=""

check_large_files() {
    large=$(git diff --cached --name-only | xargs -I {} sh -c 'test -f "{}" && stat --format="%s {}" "{}" 2>/dev/null || stat -f "%z {}" "{}" 2>/dev/null' | awk '$1 > 100000000 {print $2}')
    if [ -n "$large" ]; then
        echo "ERROR: Files over 100MB (GitHub limit):"
        echo "$large"
        return 1
    fi
    return 0
}

for d in session_cpp session_py session_rust session_data session_proto session_rhino; do
    if [ -d "$d" ]; then
        echo -e "\n=== $d ==="
        cd "$d"
        git checkout "$BRANCH" 2>/dev/null || git checkout -b "$BRANCH"
        git add -A

        if ! check_large_files; then
            echo "SKIPPED: $d has large files"
            FAILED="$FAILED $d(large-files)"
            cd "${REPO_ROOT}"
            continue
        fi

        git commit -m "$m" 2>/dev/null
        git pull --rebase origin "$BRANCH" 2>/dev/null
        if ! git push -u origin "$BRANCH"; then
            echo "FAILED: $d push"
            FAILED="$FAILED $d"
        fi
        cd "${REPO_ROOT}"
    fi
done

echo -e "\n=== main repo ==="
git add -A
git add session_cpp session_py session_rust session_data session_proto session_rhino 2>/dev/null
git commit -m "$m" 2>/dev/null
if ! git push -u origin "$BRANCH"; then
    echo "FAILED: main repo push"
    FAILED="$FAILED main"
fi

echo -e "\n=== Verify ==="
git submodule foreach --quiet 'echo "$name: local=$(git rev-parse --short HEAD) remote=$(git ls-remote origin '"$BRANCH"' 2>/dev/null | cut -c1-7)"'

if [ -n "$FAILED" ]; then
    echo -e "\nFAILED:$FAILED"
    exit 1
else
    echo -e "\nALL OK"
fi
