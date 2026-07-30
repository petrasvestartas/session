#!/usr/bin/env bash
# Builds session_cpp/main_20.cpp (the v2 tracer) WITHOUT editing session_cpp/CMakeLists.txt.
#
# Why by hand: CMakeLists.txt enumerates its executables in one `foreach(MAIN_ID 3 6 7 ... 17)`
# and is owned by other concurrent sessions, so this session must not touch it. Everything else
# is normal CMake: src/v2/v2_dump.cpp is picked up automatically by the CONFIGURE_DEPENDS glob
# on src/v2/*.cpp, so it is compiled into the session_v2 object library by the ordinary build.
# This script then reuses main_17's generated compile flags and link line verbatim, substituting
# only the object file and the output name.
#
#   build_main_20.sh [<build-dir>]        default: session_cpp/build_v2diff
set -eu
cd "$(dirname "$0")/../../session_cpp"
BUILD="${1:-build_v2diff}"

if [ ! -f "$BUILD/CMakeCache.txt" ]; then
  echo "configuring $BUILD"
  cmake -S . -B "$BUILD" -DCMAKE_BUILD_TYPE=Release
fi

# builds session_core + session_v2 (which now contains v2_dump.cpp) + proto + libprotobuf
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
CC=${CC//main_17.cpp.o/main_20.cpp.o}
CC=${CC//main_17.cpp/main_20.cpp}
CC=$(echo "$CC" | sed -E 's#-Winvalid-pch ##; s#-include [^ ]*cmake_pch.hxx ##')
mkdir -p "$BUILD/CMakeFiles/main_20.dir"
CC=${CC//CMakeFiles\/main_17.dir/CMakeFiles/main_20.dir}
( cd "$BUILD" && eval "$CC" )

LN=$(cat "$BUILD/CMakeFiles/main_17.dir/link.txt")
LN=${LN//CMakeFiles\/main_17.dir\/main_17.cpp.o/CMakeFiles/main_20.dir/main_20.cpp.o}
LN=${LN//-o main_17/-o main_20}
LN=$(echo "$LN" | sed -E 's#-Wl,--dependency-file=[^ ]* ##')
( cd "$BUILD" && eval "$LN" )

ls -l "$BUILD/main_20"
sha1sum "$BUILD/main_20"
