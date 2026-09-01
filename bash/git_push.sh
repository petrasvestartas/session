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
    large=$(git diff --cached --name-only | while read f; do
        if [ -f "$f" ]; then
            sz=$(stat --format="%s" "$f" 2>/dev/null || stat -f "%z" "$f" 2>/dev/null)
            [ -n "$sz" ] && [ "$sz" -gt 100000000 ] 2>/dev/null && echo "$f"
        fi
    done)
    if [ -n "$large" ]; then
        echo "ERROR: Files over 100MB (GitHub limit):"
        echo "$large"
        return 1
    fi
    return 0
}

# session_viewer is deliberately absent: it is a plain directory in THIS repo, not a submodule
# (check .gitmodules). Including it made every git command in the loop body operate on the parent
# instead - it worked only because session_viewer sorted last, so the parent happened to be
# committed after the real submodules. The "=== main repo ===" block below is what pushes it.
for d in session_cpp session_py session_rust session_data session_proto session_rhino; do
    if [ -d "$d" ]; then
        echo -e "\n=== $d ==="
        cd "$d"
        git checkout "$BRANCH" 2>/dev/null || git checkout -B "$BRANCH"
        git fetch origin "$BRANCH" 2>/dev/null

        # A submodule can hold submodules of its own (session_py/session_data). The loop never
        # visited them, so a dirty nested checkout left `git status` permanently showing
        # "modified: session_py (modified content)" - noise that trains you to ignore the one
        # signal that says whether a push is complete. Report them by name rather than
        # committing blind: regenerated test data may be legitimate output or may be drift, and
        # the script cannot tell which.
        nested=$(git submodule foreach --quiet 'test -n "$(git status --porcelain)" && echo "$name"' 2>/dev/null)
        if [ -n "$nested" ]; then
            echo "NOTE: $d has dirty NESTED submodule(s): $(echo $nested | tr '\n' ' ')"
            echo "      Left alone. Commit inside them first if the change is intended."
        fi

        git add -A

        if ! check_large_files; then
            echo "SKIPPED: $d has large files"
            FAILED="$FAILED $d(large-files)"
            cd "${REPO_ROOT}"
            continue
        fi

        git commit -m "$m" 2>/dev/null

        # Rebase onto remote, auto-resolve pyproject.toml conflicts (CI version bumps)
        if ! git rebase "origin/$BRANCH" 2>/dev/null; then
            if [ -f "pyproject.toml" ] && git diff --name-only --diff-filter=U 2>/dev/null | grep -q "pyproject.toml"; then
                git checkout --theirs pyproject.toml 2>/dev/null
                git add pyproject.toml 2>/dev/null
                GIT_EDITOR=true git rebase --continue 2>/dev/null || git rebase --abort 2>/dev/null
            else
                git rebase --abort 2>/dev/null
            fi
        fi

        if ! git push -u origin "$BRANCH"; then
            echo "FAILED: $d push"
            FAILED="$FAILED $d"
        fi
        cd "${REPO_ROOT}"
    fi
done

echo -e "\n=== main repo ==="
git add -A
git add session_cpp session_py session_rust session_data session_proto session_rhino session_viewer 2>/dev/null
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
