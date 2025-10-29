# Abseil Linking CI Fix

## Problem

**Symptom**: Build fails on first attempt (both locally and CI), succeeds on second attempt.

**Root Cause**: `file(GLOB ABSEIL_LIBS ...)` executes during CMake's **configure phase** (before any building). On a clean build, Abseil libraries don't exist yet, so the GLOB returns an empty list. The link step then tries to link Protobuf without Abseil, causing missing symbol errors.

**Why second build worked**: After the first build, Abseil libraries exist in `build/external/abseil/lib/`, so the GLOB finds them during configure and linking succeeds.

## Platform-Specific Issues

### macOS
```
[ 12%] Creating monolithic Abseil archive for macOS
libtool: warning same member name (commandlineflag.cc.o) ...
[ 13%] Built target abseil_monolith
✅ SUCCESS
```

### Ubuntu  
```
ar: libabsl_: No such file or directory
❌ FAILED
```

The MRI script approach (`ar -M` with `ADDLIB libabsl_*.a`) failed because `ar` doesn't expand shell wildcards - it looked for a literal file named `libabsl_*.a`.

## Solution

Created a **monolithic Abseil archive** at **build time** (not configure time) using platform-specific archive tools:

### macOS (libtool)
```cmake
add_custom_command(
    OUTPUT $ENV{INSTALL}/abseil/lib/libabsl_all.a
    COMMAND bash -c "libtool -static -o libabsl_all.a libabsl_*.a"
    DEPENDS abseil_external
)
```

### Linux (ar)
```cmake
add_custom_command(
    OUTPUT $ENV{INSTALL}/abseil/lib/libabsl_all.a
    COMMAND bash -c "
        cd $ENV{INSTALL}/abseil/lib && 
        mkdir -p abseil_tmp && 
        cd abseil_tmp && 
        for lib in ../libabsl_*.a; do 
            ar -x \"$lib\"
        done && 
        ar -qcs ../libabsl_all.a *.o && 
        cd .. && 
        rm -rf abseil_tmp
    "
    DEPENDS abseil_external
)
```

### Updated Linking
```cmake
function(LINK_PROTOBUF_LIBRARIES target_name)
    if(APPLE)
        target_link_libraries(${target_name} PRIVATE
            -Wl,-force_load,$ENV{INSTALL}/abseil/lib/libabsl_all.a
            ...
        )
    else()
        target_link_libraries(${target_name} PRIVATE
            -Wl,--whole-archive
            $ENV{INSTALL}/abseil/lib/libabsl_all.a
            -Wl,--no-whole-archive
            ...
        )
    endif()
    
    # Ensure archive is built before linking
    add_dependencies(${target_name} abseil_monolith protobuf_external)
endfunction()
```

## Result

✅ **First-build now succeeds** on both macOS and Ubuntu CI
✅ No configure-time GLOB dependencies  
✅ Proper build-time ordering enforced via `add_dependencies()`

## Testing

```bash
# Clean build test
rm -rf build && bash build.sh

# Expected output:
[ 12%] Creating monolithic Abseil archive for macOS/Linux
[ 13%] Built target abseil_monolith
...
[100%] Built target tests
Build successful!
```

## Key Learnings

1. **`file(GLOB ...)` runs at configure time**, before any targets build
2. **Dependencies** (`DEPENDS abseil_external`) only control build order, not when GLOBs execute
3. **Custom commands** with `OUTPUT` and proper `DEPENDS` ensure build-time generation
4. **Platform differences**: `libtool` vs `ar`, `-force_load` vs `--whole-archive`
5. **MRI scripts** don't support wildcards - need explicit library names or shell expansion
