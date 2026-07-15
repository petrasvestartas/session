cmake_minimum_required(VERSION 3.20)
project(brep2step CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Full OCCT build (with DataExchange) -- the same source tree step_probe uses.
set(OCCT_ROOT "C:/brg/compas_occt/external/occt/src" CACHE PATH "OCCT source root")
set(OCCT_LIBD "${OCCT_ROOT}/occt-build/win64/vc14/lib")

add_executable(brep2step brep2step.cpp)
target_include_directories(brep2step SYSTEM PRIVATE "${OCCT_ROOT}/occt-build/my_include")

# Static libs: dependents before dependencies.
set(TKS TKDESTEP TKDE TKXSBase TKShHealing TKBool TKBO TKFillet TKOffset TKPrim TKMesh
        TKHLR TKTopAlgo TKGeomAlgo TKBRep TKGeomBase TKG3d TKG2d TKMath TKernel)
foreach(t ${TKS})
    target_link_libraries(brep2step PRIVATE "${OCCT_LIBD}/${t}.lib")
endforeach()
if(WIN32)
    target_link_libraries(brep2step PRIVATE user32 gdi32 advapi32 ws2_32 shell32 ole32)
endif()
