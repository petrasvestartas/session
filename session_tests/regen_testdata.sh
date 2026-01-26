#!/bin/bash
output="public/testData.js"
classes=(color line mesh nurbscurve nurbssurface plane point pointcloud polyline tolerance vector xform)
declare -A langs=([python]=session_py [cpp]=session_cpp [rust]=session_rust)

echo "// Auto-generated test data" > "$output"
echo "window.TEST_DATA = {" >> "$output"

first=true
for c in "${classes[@]}"; do
    for lang in python cpp rust; do
        dir="${langs[$lang]}"
        f="$dir/${c}_test.json"
        if [ -f "$f" ]; then
            if [ "$first" = false ]; then
                echo "," >> "$output"
            fi
            first=false
            printf '  "%s_test_%s": ' "$c" "$lang" >> "$output"
            cat "$f" >> "$output"
        fi
    done
done

echo "" >> "$output"
echo "};" >> "$output"
