#!/usr/bin/env bash
set -euo pipefail

VERSION=${1:?protobuf version, e.g. 36.2}
ROOT=${SESSION_DEPS_ROOT:-$HOME/.cache/session-deps}
PREFIX=$ROOT/protobuf-$VERSION
SRC=$ROOT/src/protobuf-$VERSION
BUILD=$ROOT/build/protobuf-$VERSION
JOBS=${MINITEST_JOBS:-4}

[[ -f "$PREFIX/.session-deps-ok" ]] && exit 0

# a clone without its marker was interrupted: start it again
clone() {
    [[ -f "$3/.session-deps-clone-ok" ]] && return 0
    rm -rf "$3"
    git clone -q --depth 1 --branch "$2" "$1" "$3"
    touch "$3/.session-deps-clone-ok"
}

clone https://github.com/protocolbuffers/protobuf.git "v$VERSION" "$SRC"
ABSL_VERSION=$(sed -n 's/^set(abseil-cpp-version "\(.*\)")/\1/p' "$SRC/cmake/dependencies.cmake")
ABSL=$ROOT/src/abseil-cpp-$ABSL_VERSION
clone https://github.com/abseil/abseil-cpp.git "$ABSL_VERSION" "$ABSL"

LAUNCHER=()
command -v sccache >/dev/null && LAUNCHER=(-DCMAKE_C_COMPILER_LAUNCHER=sccache -DCMAKE_CXX_COMPILER_LAUNCHER=sccache)
GENERATOR=()
command -v ninja >/dev/null && GENERATOR=(-G Ninja)

# Same switches as session_cpp/CMakeLists.txt, plus protoc/libprotoc and PIC so one prefix serves the kernel, regen and Python bindings.
cmake -S "$SRC" -B "$BUILD" "${GENERATOR[@]}" "${LAUNCHER[@]}" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX="$PREFIX" \
    -DCMAKE_INSTALL_LIBDIR=lib \
    -DCMAKE_CXX_STANDARD=23 \
    -DCMAKE_POSITION_INDEPENDENT_CODE=ON \
    -DABSL_PROPAGATE_CXX_STD=ON \
    -Dprotobuf_BUILD_TESTS=OFF \
    -Dprotobuf_BUILD_EXAMPLES=OFF \
    -Dprotobuf_WITH_ZLIB=OFF \
    -Dprotobuf_BUILD_SHARED_LIBS=OFF \
    -Dprotobuf_BUILD_PROTOC_BINARIES=ON \
    -Dprotobuf_BUILD_LIBPROTOC=ON \
    -Dprotobuf_INSTALL=ON \
    -Dprotobuf_FORCE_FETCH_DEPENDENCIES=ON \
    -DFETCHCONTENT_SOURCE_DIR_ABSL="$ABSL"
cmake --build "$BUILD" --parallel "$JOBS"
cmake --install "$BUILD"
touch "$PREFIX/.session-deps-ok"

# description: build protobuf <version> with abseil once and install it where session_cpp/CMakeLists.txt looks first.
#
# directory: cd ~/code/code_cpp/wood_research/session
# run: timeout 10m systemd-run --user --scope -p MemoryMax=6G bash/deps_protobuf.sh 36.2
# result: ~/.cache/session-deps/protobuf-36.2, found by session_cpp on its own (elsewhere via SESSION_DEPS_ROOT, then pass SESSION_DEPS_PREFIX); rm -rf it to rebuild.
