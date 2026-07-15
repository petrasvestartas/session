# OCCT oracle superbuild — downloads + builds OCCT V8_0_0_rc2 as static libs
# (recipe adapted from petrasvestartas/compas_occt), then builds a small oracle
# executable that links OCCT and runs SSI / surface-split / interpolation so we
# can compare our kernel against OCCT ground truth. Test-time only; never a
# runtime dependency of the kernel.
cmake_minimum_required(VERSION 3.15)
project(occt_oracle LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(ExternalProject)

# ------------------------------------------------------------------------------
# OCCT (static libraries)
# ------------------------------------------------------------------------------
set(CMAKE_DEPS_DIR ${CMAKE_BINARY_DIR}/deps)
set(OCCT_PREFIX ${CMAKE_DEPS_DIR}/occt)
file(MAKE_DIRECTORY ${CMAKE_DEPS_DIR})
set(OCCT_SRC_DIR ${OCCT_PREFIX}/src/occt)
set(OCCT_BUILD_DIR ${OCCT_PREFIX}/src/occt-build)
set(OCCT_INSTALL_DIR ${OCCT_PREFIX}/install)
set(OCCT_INCLUDE_DIR "${OCCT_INSTALL_DIR}/my_include" CACHE PATH "Path to OCCT headers" FORCE)
set(OCCT_LIB_DIR "${OCCT_INSTALL_DIR}/my_static_libs" CACHE PATH "Path to OCCT libraries" FORCE)

message(STATUS "---------------------- OCCT_INCLUDE_DIR: ${OCCT_INCLUDE_DIR}")
message(STATUS "---------------------- OCCT_LIB_DIR: ${OCCT_LIB_DIR}")

if(WIN32)
    set(LIB_PREFIX "")
    set(LIB_EXT ".lib")
else()
    set(LIB_PREFIX "lib")
    set(LIB_EXT ".a")
endif()

# Link order matters for static libs: dependents before dependencies.
set(OCCT_LIB_TARGETS "")
set(OCCT_MODULES TKDESTEP TKXSBase TKBool TKFillet TKOffset TKFeat TKPrim TKBO TKMesh TKHLR TKShHealing TKTopAlgo TKGeomAlgo TKBRep TKGeomBase TKG3d TKG2d TKMath TKernel)
foreach(MODULE ${OCCT_MODULES})
    list(APPEND OCCT_LIB_TARGETS "${OCCT_LIB_DIR}/${LIB_PREFIX}${MODULE}${LIB_EXT}")
endforeach()

add_custom_target(occt_ext)

if(NOT EXISTS "${OCCT_INSTALL_DIR}/my_include" OR NOT EXISTS "${OCCT_INSTALL_DIR}/my_static_libs")
    ExternalProject_Add(
        occt_download
        URL https://github.com/Open-Cascade-SAS/OCCT/archive/refs/tags/V8_0_0_rc2.zip
        PREFIX ${OCCT_PREFIX}
        SOURCE_DIR ${OCCT_SRC_DIR}
        BINARY_DIR ${OCCT_BUILD_DIR}
        CMAKE_ARGS
            -DCMAKE_INSTALL_PREFIX=${OCCT_INSTALL_DIR}
            -DCMAKE_POLICY_VERSION_MINIMUM=3.5
            -DINSTALL_DIR_LIB=my_static_libs
            -DINSTALL_DIR_INCLUDE=my_include
            -DINSTALL_DIR_LAYOUT=Unix
            -DCMAKE_BUILD_TYPE=Release
            -DBUILD_LIBRARY_TYPE=Static
            -DBUILD_MODULE_Draw=OFF
            -DBUILD_MODULE_ApplicationFramework=OFF
            -DBUILD_MODULE_DataExchange=OFF
            -DBUILD_MODULE_Visualization=OFF
            -DBUILD_SAMPLES_QT=OFF
            -DBUILD_USE_PCH=ON
        BUILD_COMMAND ${CMAKE_COMMAND} --build . --config Release
        INSTALL_COMMAND ${CMAKE_COMMAND} --install . --config Release
        BUILD_BYPRODUCTS ${OCCT_LIB_TARGETS}
    )
    add_dependencies(occt_ext occt_download)
endif()

# ------------------------------------------------------------------------------
# Oracle executable
# ------------------------------------------------------------------------------
add_executable(oracle oracle.cpp)
target_include_directories(oracle SYSTEM PRIVATE ${OCCT_INCLUDE_DIR})
add_dependencies(oracle occt_ext)
target_link_libraries(oracle PRIVATE ${OCCT_LIB_TARGETS})
if(WIN32)
    target_link_libraries(oracle PRIVATE user32 gdi32 advapi32 ws2_32 shell32 ole32)
endif()
