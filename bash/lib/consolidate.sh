#!/usr/bin/env bash
# Consolidate all existing JSON test results into testData.js
# This script ONLY reads - it never deletes source JSON files
# Enables incremental updates: run one language, keep others' data

# Only set SCRIPT_DIR if not already set (when sourced from parent)
if [[ -z "${SCRIPT_DIR:-}" ]] || [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    source "${SCRIPT_DIR}/common.sh"
fi

consolidate_test_data() {
    local repo_root="$1"
    local tests_dir="${repo_root}/session_tests"
    local output="${tests_dir}/public/testData.js"

    mkdir -p "${tests_dir}/public"

    log "Consolidating JSON into testData.js..."

    # Use Python for consolidation — avoids Windows MSYS fork overhead
    # (bash loops with cat/awk per file take ~60s on Windows, Python takes <1s)
    local PYTHON
    PYTHON=$(get_python_path "$repo_root")
    local repo_native
    repo_native=$(cd "$repo_root" && pwd -W 2>/dev/null || pwd)
    "$PYTHON" -c "
import os, glob, json

repo = r'''${repo_native}'''
tests = os.path.join(repo, 'session_tests')
classes = '''${CLASS_NAMES[*]}'''.split()
langs = [('cpp','session_cpp'), ('python','session_py'), ('rust','session_rust')]

parts = []

# Test results
for cls in classes:
    for lang, d in langs:
        name = cls + '_test.json'
        p = os.path.join(tests, d, name)
        if os.path.isfile(p):
            with open(p, 'r') as f:
                parts.append('  \"' + cls + '_test_' + lang + '\": ' + f.read().rstrip())

# Artifacts
for cls in classes:
    for lang, d in langs:
        p = os.path.join(repo, d, 'test_' + cls + '.json')
        if not os.path.isfile(p):
            p = os.path.join(repo, d, 'serialization', 'test_' + cls + '.json')
        if os.path.isfile(p):
            with open(p, 'r') as f:
                parts.append('  \"artifact_test_' + cls + '_' + lang + '\": ' + f.read().rstrip())

# Proto schemas
proto_dir = os.path.join(repo, 'session_proto')
if os.path.isdir(proto_dir):
    for pf in sorted(glob.glob(os.path.join(proto_dir, '*.proto'))):
        name = os.path.splitext(os.path.basename(pf))[0]
        with open(pf, 'r') as f:
            content = f.read().replace('\\\\', '\\\\\\\\').replace('\"', '\\\\\"').replace('\\n', '\\\\n')
        parts.append('  \"proto_' + name + '\": \"' + content + '\"')

from datetime import datetime
out = '// Auto-generated test data - Do not edit manually\n'
out += '// Generated at: ' + datetime.now().strftime('%c') + '\n'
out += 'window.TEST_DATA = {\n'
out += ',\n'.join(parts)
out += '\n};\n'

op = os.path.join(tests, 'public', 'testData.js')
with open(op, 'w') as f:
    f.write(out)
import shutil
shutil.copy2(op, os.path.join(tests, 'testData.js'))
"

    log "testData.js updated"
}

# If run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
    consolidate_test_data "$REPO_ROOT"
fi
