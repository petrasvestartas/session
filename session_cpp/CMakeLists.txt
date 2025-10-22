# Set the minimum version of CMake required
cmake_minimum_required(VERSION 3.22)

# Define the project and the languages used
project(MyProject VERSION 1.0 LANGUAGES CXX)

# Set default build type to Release if not specified
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Enable compilation database generation for better IntelliSense
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Specify the C++ standard
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add compiler warning flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic -Werror)
    # High optimization for Release builds
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -march=native -ffast-math -flto")
elseif(MSVC)
    add_compile_options(/W4)
    add_compile_options(/utf-8)  # Enable UTF-8 support for fmt library
    add_compile_definitions(_CRT_SECURE_NO_WARNINGS)  # Suppress MSVC security warnings for external libs
    # Uncomment to treat warnings as errors
    # add_compile_options(/WX)
endif()

#############################################################################################
# Automatically collect all .cpp files in src directory
#############################################################################################

# Collect source files except test files
file(GLOB PROJECT_SOURCES "src/*.cpp")
list(FILTER PROJECT_SOURCES EXCLUDE REGEX ".*_test\\.cpp$")

# Add executable for the main project
add_executable(${PROJECT_NAME} main.cpp ${PROJECT_SOURCES})

# Include directories for headers
target_include_directories(${PROJECT_NAME} PRIVATE 
    ${PROJECT_SOURCE_DIR}/src                    # Project source headers
    ${PROJECT_SOURCE_DIR}/src/fmt/include        # fmt library headers
    ${PROJECT_SOURCE_DIR}/src/json/include       # JSON library headers
    ${PROJECT_SOURCE_DIR}/src/guid/include       # GUID library headers
)
#############################################################################################
# Create Catch2 library - optimized static library
#############################################################################################

# Create static library only if it doesn't exist or source changed
add_library(catch2_lib STATIC src/catch/catch_amalgamated.cpp)
target_include_directories(catch2_lib PUBLIC ${PROJECT_SOURCE_DIR}/src/catch/include)

# Collect test files
file(GLOB TEST_SOURCES "src/*_test.cpp")

# Create test executable
add_executable(tests ${TEST_SOURCES} ${PROJECT_SOURCES})
target_include_directories(tests PRIVATE 
    ${PROJECT_SOURCE_DIR}/src                    # Project source headers
    ${PROJECT_SOURCE_DIR}/src/fmt/include        # fmt library headers
    ${PROJECT_SOURCE_DIR}/src/json/include       # JSON library headers
    ${PROJECT_SOURCE_DIR}/src/guid/include       # GUID library headers
    ${PROJECT_SOURCE_DIR}/src/catch/include      # Catch2 library headers
)
target_link_libraries(tests catch2_lib) # link catch2 to tests, not main executable

#############################################################################################
# Get all subdirectories in the examples directory
#############################################################################################
file(GLOB EXAMPLE_DIRS "examples/*")

foreach(EXAMPLE_DIR ${EXAMPLE_DIRS})
    # Get all .cpp files in the current example directory
    file(GLOB EXAMPLE_FILES "${EXAMPLE_DIR}/*.cpp")

    foreach(EXAMPLE_FILE ${EXAMPLE_FILES})
        # Get the filename without the directory and extension
        get_filename_component(EXAMPLE_NAME ${EXAMPLE_FILE} NAME_WE)
        # Create an executable for each example
        add_executable(${EXAMPLE_NAME} ${EXAMPLE_FILE} ${PROJECT_SOURCES})
        # Include directories for headers
        target_include_directories(${EXAMPLE_NAME} PRIVATE 
            ${PROJECT_SOURCE_DIR}/src                    # Project source headers
            ${PROJECT_SOURCE_DIR}/src/fmt/include        # fmt library headers (for fmt/core.h, fmt/format.h, etc.)
            ${PROJECT_SOURCE_DIR}/src/json/include       # JSON library headers (for json.h)
            ${PROJECT_SOURCE_DIR}/src/guid/include       # GUID library headers (for guid.h)
            ${PROJECT_SOURCE_DIR}/src/catch/include      # Catch2 library headers (for catch.h)
        )
        # Set the output directory for the executables, preserving the folder structure
        get_filename_component(EXAMPLE_DIR_NAME ${EXAMPLE_DIR} NAME)
        set_target_properties(${EXAMPLE_NAME} PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/examples/${EXAMPLE_DIR_NAME})
    endforeach()
endforeach()