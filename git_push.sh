#!/bin/bash
m="$1"; [ -z "$m" ] && echo "❌ msg?" && exit 1

for d in session_cpp session_py session_rust session_data session_proto; do
  if [ -d "$d" ]; then
    echo -e "\nRepository: $d"
    cd "$d"
    git add -A
    git commit -m "$m" 2>/dev/null || echo "ℹ️ Nothing to commit in $d"
    git push -f 2>/dev/null && echo "✅ $d updated"
    cd ..
  else
    echo "⚠️ $d not found, skipping"
  fi
done

echo -e "\nRepository: main repo (this repository)"
git add -A
git commit -m "$m" 2>/dev/null || echo "ℹ️ Nothing to commit in main repo"
git push 2>/dev/null && echo "✅ main repo updated"
