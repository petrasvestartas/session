cmake_minimum_required(VERSION 3.20)
project(occt_trace CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Linux: the already-built occt_oracle static install (V8_0_0_rc2 + TKDESTEP).
set(OCCT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../occt_oracle/build/deps/occt/install" CACHE PATH "OCCT install root")
set(OCCT_INC "${OCCT_ROOT}/my_include")
set(OCCT_LIBD "${OCCT_ROOT}/my_static_libs")

add_executable(occt_trace occt_trace.cpp)
target_include_directories(occt_trace SYSTEM PRIVATE "${OCCT_INC}")

set(TKS TKDESTEP TKDE TKXCAF TKVCAF TKV3d TKService TKCAF TKLCAF TKCDF
        TKXSBase TKShHealing TKBool TKBO TKFillet TKOffset TKPrim TKMesh
        TKHLR TKTopAlgo TKGeomAlgo TKBRep TKGeomBase TKG3d TKG2d TKMath TKernel)

set(TKLIBS "")
foreach(t ${TKS})
    list(APPEND TKLIBS "${OCCT_LIBD}/lib${t}.a")
endforeach()
# start/end-group: OCCT static archives have circular symbol references.
target_link_libraries(occt_trace PRIVATE -Wl,--start-group ${TKLIBS} -Wl,--end-group pthread dl)
