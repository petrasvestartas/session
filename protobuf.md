- Implement protobuf from this release bath to session_cpp overly complex cmakelists.txt:
'/Users/petras/brg/code_rust/session/session_cpp/CMakeLists.txt'
- The release based on windows, mac arm64, mac intel, linux is here:
- Use this pattern:

set(CGAL_CMAKE_EXACT_NT_BACKEND BOOST_BACKEND CACHE STRING "Set CGAL backend to Boost")
set(CGAL_DISABLE_GMP ON CACHE BOOL "Disable GMP in CGAL")
set(CMAKE_DISABLE_FIND_PACKAGE_GMP ON CACHE BOOL "Disable CMake find package for GMP")

# Dependencies stored in source tree (persistent across pip builds)
set(DEPS_DIR "${CMAKE_SOURCE_DIR}/.deps")
file(MAKE_DIRECTORY ${DEPS_DIR})

include(FetchContent)
set(FETCHCONTENT_BASE_DIR "${DEPS_DIR}/_fetch")
set(FETCHCONTENT_QUIET OFF)

# Boost (header-only) - only download if not present
if(NOT EXISTS "${DEPS_DIR}/boost/boost")
  message(STATUS "Downloading Boost...")
  FetchContent_Declare(boost URL https://archives.boost.io/release/1.82.0/source/boost_1_82_0.tar.gz SOURCE_DIR ${DEPS_DIR}/boost)
  FetchContent_Populate(boost)
else()
  message(STATUS "Boost already present")
endif()

# CGAL (header-only)
if(NOT EXISTS "${DEPS_DIR}/cgal/include")
  message(STATUS "Downloading CGAL...")
  FetchContent_Declare(cgal URL https://github.com/CGAL/cgal/releases/download/v6.0.1/CGAL-6.0.1-library.zip SOURCE_DIR ${DEPS_DIR}/cgal)
  FetchContent_Populate(cgal)
else()
  message(STATUS "CGAL already present")
endif()

# Eigen (header-only)
if(NOT EXISTS "${DEPS_DIR}/eigen/Eigen")
  message(STATUS "Downloading Eigen...")
  FetchContent_Declare(eigen GIT_REPOSITORY https://gitlab.com/libeigen/eigen.git GIT_TAG 3.4.0 SOURCE_DIR ${DEPS_DIR}/eigen)
  FetchContent_Populate(eigen)
else()
  message(STATUS "Eigen already present")
endif()

# CDT (header-only)
if(NOT EXISTS "${DEPS_DIR}/cdt/CDT")
  message(STATUS "Downloading CDT...")
  FetchContent_Declare(cdt GIT_REPOSITORY https://github.com/artem-ogre/CDT.git GIT_TAG master SOURCE_DIR ${DEPS_DIR}/cdt)
  FetchContent_Populate(cdt)
else()
  message(STATUS "CDT already present")
endif()

# SQLite
if(NOT EXISTS "${DEPS_DIR}/sqlite3/sqlite3.c")
  message(STATUS "Downloading SQLite...")
  FetchContent_Declare(sqlite3 URL https://www.sqlite.org/2024/sqlite-amalgamation-3450000.zip SOURCE_DIR ${DEPS_DIR}/sqlite3)
  FetchContent_Populate(sqlite3)
else()
  message(STATUS "SQLite already present")
endif()

# Clipper2
if(NOT EXISTS "${DEPS_DIR}/clipper2/CPP")
  message(STATUS "Downloading Clipper2...")
  FetchContent_Declare(clipper2 GIT_REPOSITORY https://github.com/AngusJohnson/Clipper2.git GIT_TAG Clipper2_1.4.0 SOURCE_DIR ${DEPS_DIR}/clipper2)
  FetchContent_Populate(clipper2)
endif()
set(CLIPPER2_TESTS OFF CACHE BOOL "" FORCE)
set(CLIPPER2_EXAMPLES OFF CACHE BOOL "" FORCE)
add_subdirectory(${DEPS_DIR}/clipper2/CPP ${CMAKE_BINARY_DIR}/clipper2_build EXCLUDE_FROM_ALL)

# GoogleTest
if(NOT EXISTS "${DEPS_DIR}/googletest/googletest")
  message(STATUS "Downloading GoogleTest...")
  FetchContent_Declare(googletest GIT_REPOSITORY https://github.com/google/googletest.git GIT_TAG v1.14.0 SOURCE_DIR ${DEPS_DIR}/googletest)
  FetchContent_Populate(googletest)
endif()
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
add_subdirectory(${DEPS_DIR}/googletest ${CMAKE_BINARY_DIR}/googletest_build EXCLUDE_FROM_ALL)

# SQLite library
add_library(sqlite3_lib STATIC ${DEPS_DIR}/sqlite3/sqlite3.c)
target_include_directories(sqlite3_lib PUBLIC ${DEPS_DIR}/sqlite3)

# Global includes for PCH
include_directories(${DEPS_DIR}/boost ${DEPS_DIR}/cgal/include)

# Platform options
if(MSVC)
  add_compile_options(/O2)
else()
  add_compile_options(-O3)
endif()
