#!/usr/bin/env bash
# Builds session_cpp/main_21.cpp (the random-rotation boolean battery -> STEP) WITHOUT editing
# session_cpp/CMakeLists.txt — same approach as build_main_20.sh: reuse main_17's compile flags
# and link line, substituting only the object file and output name.
#
#   build_main_21.sh [<build-dir>]        default: session_cpp/build_v2diff
set -eu
cd "$(dirname "$0")/../../session_cpp"
BUILD="${1:-build_v2diff}"

if [ ! -f "$BUILD/CMakeCache.txt" ]; then
  echo "configuring $BUILD"
  cmake -S . -B "$BUILD" -DCMAKE_BUILD_TYPE=Release
fi

# builds session_core + session_v2 (picks up src/v2/*.cpp via CONFIGURE_DEPENDS glob) + proto
cmake --build "$BUILD" --target main_17 -j "$(nproc)"

CC=$(python3 - "$BUILD" <<'PY'
import json, sys
b = sys.argv[1]
for e in json.load(open(b + "/compile_commands.json")):
    if e["file"].endswith("main_17.cpp"):
        print(e["command"]); break
PY
)
# same flags, our source, our object; drop main_17's precompiled header (it is per-target)
CC=${CC//main_17.cpp.o/main_21.cpp.o}
CC=${CC//main_17.cpp/main_21.cpp}
CC=$(echo "$CC" | sed -E 's#-Winvalid-pch ##; s#-include [^ ]*cmake_pch.hxx ##')
mkdir -p "$BUILD/CMakeFiles/main_21.dir"
CC=${CC//CMakeFiles\/main_17.dir/CMakeFiles/main_21.dir}
( cd "$BUILD" && eval "$CC" )

LN=$(cat "$BUILD/CMakeFiles/main_17.dir/link.txt")
LN=${LN//CMakeFiles\/main_17.dir\/main_17.cpp.o/CMakeFiles/main_21.dir/main_21.cpp.o}
LN=${LN//-o main_17/-o main_21}
LN=$(echo "$LN" | sed -E 's#-Wl,--dependency-file=[^ ]* ##')
( cd "$BUILD" && eval "$LN" )

ls -l "$BUILD/main_21"
sha1sum "$BUILD/main_21"
