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

    {
        echo "// Auto-generated test data - Do not edit manually"
        echo "// Generated at: $(date)"
        echo "window.TEST_DATA = {"

        local first=true

        # Process each class and language combination
        for class_name in "${CLASS_NAMES[@]}"; do
            for lang_info in "python:session_py" "cpp:session_cpp" "rust:session_rust"; do
                local lang="${lang_info%%:*}"
                local dir="${lang_info#*:}"
                local json_path="${tests_dir}/${dir}/${class_name}_test.json"

                if [[ -f "$json_path" ]]; then
                    [[ "$first" == "false" ]] && echo ","
                    first=false
                    echo -n "  \"${class_name}_test_${lang}\": "
                    cat "$json_path"
                fi
            done
        done

        # Add artifact files (test_<class>.json from each language)
        for class_name in "${CLASS_NAMES[@]}"; do
            for lang_info in "python:session_py" "cpp:session_cpp" "rust:session_rust"; do
                local lang="${lang_info%%:*}"
                local dir="${lang_info#*:}"
                local artifact_path="${repo_root}/${dir}/test_${class_name}.json"

                if [[ -f "$artifact_path" ]]; then
                    echo ","
                    echo -n "  \"artifact_test_${class_name}_${lang}\": "
                    cat "$artifact_path"
                fi
            done
        done

        # Add proto schemas
        local proto_dir="${repo_root}/session_proto"
        if [[ -d "$proto_dir" ]]; then
            for proto_file in "${proto_dir}"/*.proto; do
                if [[ -f "$proto_file" ]]; then
                    local proto_name=$(basename "$proto_file" .proto)
                    echo ","
                    local content=$(awk 'BEGIN{ORS="\\n"}{gsub(/\\/,"\\\\");gsub(/"/,"\\\"");print}' "$proto_file" | sed 's/\\n$//')
                    echo -n "  \"proto_${proto_name}\": \"${content}\""
                fi
            done
        fi

        echo ""
        echo "};"
    } > "$output"

    # Copy to root for backward compatibility
    cp "$output" "${tests_dir}/testData.js"

    log "testData.js updated"
}

# If run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
    consolidate_test_data "$REPO_ROOT"
fi
