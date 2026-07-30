cmake_minimum_required(VERSION 3.20)
project(step_probe CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(WIN32)
    # Full OCCT build (with DataExchange) -- the same source tree the project uses as reference.
    set(OCCT_ROOT "C:/brg/compas_occt/external/occt/src" CACHE PATH "OCCT source root")
    set(OCCT_INC "${OCCT_ROOT}/occt-build/my_include")
    set(OCCT_LIBD "${OCCT_ROOT}/occt-build/win64/vc14/lib")
    set(LIB_PREFIX "")
    set(LIB_EXT ".lib")
else()
    # Linux: the already-built occt_oracle static install (V8_0_0_rc2 + TKDESTEP).
    set(OCCT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../occt_oracle/build/deps/occt/install" CACHE PATH "OCCT install root")
    set(OCCT_INC "${OCCT_ROOT}/my_include")
    set(OCCT_LIBD "${OCCT_ROOT}/my_static_libs")
    set(LIB_PREFIX "lib")
    set(LIB_EXT ".a")
endif()

add_executable(step_probe step_probe.cpp)
target_include_directories(step_probe SYSTEM PRIVATE "${OCCT_INC}")

# Static libs: dependents before dependencies.
set(TKS TKDESTEP TKDE TKXSBase TKShHealing TKBool TKBO TKFillet TKOffset TKPrim TKMesh
        TKHLR TKTopAlgo TKGeomAlgo TKBRep TKGeomBase TKG3d TKG2d TKMath TKernel)
if(NOT WIN32)
    # V8 TKDESTEP pulls the XDE/CAF closure (built via BUILD_ADDITIONAL_TOOLKITS).
    set(TKS TKDESTEP TKDE TKXCAF TKVCAF TKV3d TKService TKCAF TKLCAF TKCDF
            TKXSBase TKShHealing TKBool TKBO TKFillet TKOffset TKPrim TKMesh
            TKHLR TKTopAlgo TKGeomAlgo TKBRep TKGeomBase TKG3d TKG2d TKMath TKernel)
endif()
if(WIN32)
    foreach(t ${TKS})
        target_link_libraries(step_probe PRIVATE "${OCCT_LIBD}/${LIB_PREFIX}${t}${LIB_EXT}")
    endforeach()
    target_link_libraries(step_probe PRIVATE user32 gdi32 advapi32 ws2_32 shell32 ole32)
else()
    # start/end-group: OCCT static archives have circular symbol references.
    set(TKLIBS "")
    foreach(t ${TKS})
        list(APPEND TKLIBS "${OCCT_LIBD}/${LIB_PREFIX}${t}${LIB_EXT}")
    endforeach()
    target_link_libraries(step_probe PRIVATE -Wl,--start-group ${TKLIBS} -Wl,--end-group pthread dl)
endif()
