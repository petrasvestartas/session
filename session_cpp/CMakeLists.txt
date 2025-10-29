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

# Specify the C++ standard (C++17+ for parallel algorithms)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
message(STATUS "Using C++ standard library parallel execution (std::execution::par)")

# Add compiler warning flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic)
    # High optimization for Release builds (removed -ffast-math to allow infinity/NaN)
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -march=native -flto")
elseif(MSVC)
    add_compile_options(/W4)
    add_compile_options(/utf-8)  # Enable UTF-8 support for fmt library
    add_compile_definitions(_CRT_SECURE_NO_WARNINGS)  # Suppress MSVC security warnings for external libs
    # Uncomment to treat warnings as errors
    # add_compile_options(/WX)
endif()

#############################################################################################
# PROTOBUF EXTERNAL LIBRARY - OPTIONAL (BUILT IN ADVANCE BEFORE MAIN PROJECT)
#############################################################################################
# Build order (happens BEFORE your project):
#   1. Abseil C++ library (required by protobuf v22+)
#   2. Protocol Buffers library and compiler (protoc)
#   3. Generate C++ code from .proto files
#
# Your project will then link against these pre-built libraries
#
# Control with: cmake -DENABLE_PROTOBUF=ON/OFF ..
# Default: ON
#############################################################################################

# Option to enable/disable protobuf integration
option(ENABLE_PROTOBUF "Enable Protocol Buffers support" ON)

if(ENABLE_PROTOBUF)
    message(STATUS "Protocol Buffers: ENABLED")
    add_compile_definitions(ENABLE_PROTOBUF)
    
    # Installation directories for external dependencies
    if(NOT DEFINED ENV{INSTALL})
        set(ENV{INSTALL} "${CMAKE_BINARY_DIR}/external")
    endif()

    if(NOT DEFINED ENV{GENERATED})
        set(ENV{GENERATED} "${CMAKE_BINARY_DIR}/generated")
    endif()

    file(MAKE_DIRECTORY $ENV{GENERATED})

    include(ExternalProject)
else()
    message(STATUS "Protocol Buffers: DISABLED")
endif()

if(ENABLE_PROTOBUF)
    # Step 1: Build Abseil (required for protobuf v22+)
    # Using newer version 20240722.0 for better ARM64/Apple Silicon support
    ExternalProject_Add(
    abseil_external
    GIT_REPOSITORY https://github.com/abseil/abseil-cpp.git
    GIT_TAG 20240722.0
    UPDATE_COMMAND ""  # Don't check for updates after initial clone
    CMAKE_ARGS
        -DCMAKE_INSTALL_PREFIX=$ENV{INSTALL}/abseil
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DABSL_ENABLE_INSTALL=ON
        -DABSL_PROPAGATE_CXX_STD=ON
        -DCMAKE_CXX_STANDARD=17
        -DBUILD_SHARED_LIBS=OFF
        -DCMAKE_SYSTEM_PROCESSOR=${CMAKE_SYSTEM_PROCESSOR}
        -DCMAKE_OSX_ARCHITECTURES=${CMAKE_OSX_ARCHITECTURES}
        -DCMAKE_SYSTEM_NAME=${CMAKE_SYSTEM_NAME}
)

# Create monolithic Abseil archive to avoid configure-time GLOB issues
if(APPLE)
    add_custom_command(
        OUTPUT $ENV{INSTALL}/abseil/lib/libabsl_all.a
        COMMAND bash -c "libtool -static -o $ENV{INSTALL}/abseil/lib/libabsl_all.a $ENV{INSTALL}/abseil/lib/libabsl_*.a"
        DEPENDS abseil_external
        COMMENT "Creating monolithic Abseil archive for macOS"
    )
    add_custom_target(abseil_monolith DEPENDS $ENV{INSTALL}/abseil/lib/libabsl_all.a)
elseif(WIN32)
    # Windows: Use lib.exe to combine static libraries
    # Generate a response file with all library paths at build time
    add_custom_command(
        OUTPUT $ENV{INSTALL}/abseil/lib/absl_all.lib
        COMMAND ${CMAKE_COMMAND} -E chdir $ENV{INSTALL}/abseil/lib ${CMAKE_COMMAND} -E echo "Combining Abseil libraries..."
        COMMAND powershell -Command "Get-ChildItem -Path '$ENV{INSTALL}/abseil/lib' -Filter 'absl_*.lib' | ForEach-Object { $_.FullName } | Out-File -FilePath '$ENV{INSTALL}/abseil/lib/abseil_libs.txt' -Encoding ASCII"
        COMMAND lib.exe /OUT:$ENV{INSTALL}/abseil/lib/absl_all.lib @$ENV{INSTALL}/abseil/lib/abseil_libs.txt
        DEPENDS abseil_external
        COMMENT "Creating monolithic Abseil archive for Windows"
    )
    add_custom_target(abseil_monolith DEPENDS $ENV{INSTALL}/abseil/lib/absl_all.lib)
else()
    # Linux: Use ar to create thin archive (similar to macOS libtool approach)
    add_custom_command(
        OUTPUT $ENV{INSTALL}/abseil/lib/libabsl_all.a
        COMMAND bash -c "ar -rcT $ENV{INSTALL}/abseil/lib/libabsl_all.a $ENV{INSTALL}/abseil/lib/libabsl_*.a"
        DEPENDS abseil_external
        COMMENT "Creating monolithic Abseil archive for Linux"
    )
    add_custom_target(abseil_monolith DEPENDS $ENV{INSTALL}/abseil/lib/libabsl_all.a)
endif()

# Step 2: Build protobuf with static linking (depends on Abseil)
ExternalProject_Add(
    protobuf_external
    DEPENDS abseil_external
    GIT_REPOSITORY https://github.com/protocolbuffers/protobuf.git
    GIT_TAG v29.0
    UPDATE_COMMAND ""  # Don't check for updates after initial clone
    CMAKE_ARGS
        -Dprotobuf_BUILD_TESTS=OFF
        -Dprotobuf_BUILD_EXAMPLES=OFF
        -DCMAKE_INSTALL_PREFIX=$ENV{INSTALL}/protobuf
        -Dprotobuf_BUILD_CONFORMANCE=OFF
        -Dprotobuf_BUILD_LIBPROTOC=ON
        -Dprotobuf_BUILD_SHARED_LIBS=OFF
        -Dprotobuf_MSVC_STATIC_RUNTIME=OFF
        -Dprotobuf_WITH_ZLIB=OFF
        -Dprotobuf_ABSL_PROVIDER=package
        -DCMAKE_PREFIX_PATH=$ENV{INSTALL}/abseil
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_SYSTEM_PROCESSOR=${CMAKE_SYSTEM_PROCESSOR}  # Pass architecture
        -DCMAKE_OSX_ARCHITECTURES=${CMAKE_OSX_ARCHITECTURES}  # macOS architecture
    BUILD_BYPRODUCTS
        $ENV{INSTALL}/protobuf/lib/libprotobuf.a
        $ENV{INSTALL}/protobuf/bin/protoc
)

# Step 3: Proto file management and code generation
set(GLOBAL_PROTO_FILES "" CACHE INTERNAL "")

function(append_proto_file filename)
    set(GLOBAL_PROTO_FILES ${GLOBAL_PROTO_FILES} ${filename} CACHE INTERNAL "")
endfunction()

function(CREATE_CPP_PROTO)
    list(REMOVE_DUPLICATES GLOBAL_PROTO_FILES)
    file(MAKE_DIRECTORY $ENV{GENERATED})
    
    set(GENERATED_SOURCES "")
    set(GENERATED_HEADERS "")
    
    foreach(PROTO_FILE ${GLOBAL_PROTO_FILES})
        get_filename_component(PROTO_NAME ${PROTO_FILE} NAME_WE)
        get_filename_component(PROTO_DIR ${PROTO_FILE} DIRECTORY)
        
        set(GENERATED_CC "$ENV{GENERATED}/${PROTO_NAME}.pb.cc")
        set(GENERATED_H "$ENV{GENERATED}/${PROTO_NAME}.pb.h")
        
        list(APPEND GENERATED_SOURCES ${GENERATED_CC})
        list(APPEND GENERATED_HEADERS ${GENERATED_H})
        
        add_custom_command(
            OUTPUT ${GENERATED_CC} ${GENERATED_H}
            COMMAND $ENV{INSTALL}/protobuf/bin/protoc
            ARGS --cpp_out=$ENV{GENERATED} --proto_path=${PROTO_DIR} ${PROTO_FILE}
            DEPENDS protobuf_external ${PROTO_FILE}
            COMMENT "Generating C++ code from ${PROTO_FILE}"
            VERBATIM
        )
    endforeach()
    
    add_library(proto-objects OBJECT ${GENERATED_SOURCES})
    target_include_directories(proto-objects PUBLIC 
        $ENV{GENERATED}
        $ENV{INSTALL}/protobuf/include
        $ENV{INSTALL}/abseil/include
    )
    
    
    add_dependencies(proto-objects abseil_external protobuf_external)
       
    add_custom_target(PROTO_FILES_READY
      DEPENDS ${GENERATED_SOURCES} ${GENERATED_HEADERS}
      COMMENT "Proto files generation completed"
    )  
 
    set_source_files_properties(${GENERATED_SOURCES} ${GENERATED_HEADERS} PROPERTIES GENERATED TRUE)
endfunction()

# Helper function to link protobuf and all required Abseil libraries
function(LINK_PROTOBUF_LIBRARIES target_name)
    # Use monolithic Abseil archive (created at build time, not configure time)
    if(APPLE)
        target_link_libraries(${target_name} PRIVATE
            $<TARGET_OBJECTS:proto-objects>
            $ENV{INSTALL}/protobuf/lib/libprotobuf.a
            $ENV{INSTALL}/protobuf/lib/libutf8_range.a
            $ENV{INSTALL}/protobuf/lib/libutf8_validity.a
            -Wl,-force_load,$ENV{INSTALL}/abseil/lib/libabsl_all.a
            pthread
        )
    elseif(WIN32)
        # Windows: lib.exe already includes all symbols, no special flags needed
        target_link_libraries(${target_name} PRIVATE
            $<TARGET_OBJECTS:proto-objects>
            $ENV{INSTALL}/protobuf/lib/libprotobuf.lib
            $ENV{INSTALL}/protobuf/lib/utf8_range.lib
            $ENV{INSTALL}/protobuf/lib/utf8_validity.lib
            $ENV{INSTALL}/abseil/lib/absl_all.lib
        )
    else()
        # Linux: use --whole-archive with monolithic library
        target_link_libraries(${target_name} PRIVATE
            $<TARGET_OBJECTS:proto-objects>
            $ENV{INSTALL}/protobuf/lib/libprotobuf.a
            $ENV{INSTALL}/protobuf/lib/libutf8_range.a
            $ENV{INSTALL}/protobuf/lib/libutf8_validity.a
            -Wl,--whole-archive
            $ENV{INSTALL}/abseil/lib/libabsl_all.a
            -Wl,--no-whole-archive
            pthread
            dl
        )
    endif()
    
    # Ensure the monolithic archive is built before linking
    add_dependencies(${target_name} abseil_monolith protobuf_external)
endfunction()

    # Register proto files from session_proto directory
    file(GLOB PROTO_FILES "${CMAKE_CURRENT_SOURCE_DIR}/../session_proto/*.proto")
    foreach(PROTO_FILE ${PROTO_FILES})
        append_proto_file(${PROTO_FILE})
    endforeach()

    # Generate proto files
    CREATE_CPP_PROTO()
endif(ENABLE_PROTOBUF)

#############################################################################################
# YOUR ORIGINAL PROJECT - Automatically collect all .cpp files in src directory
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

# Conditionally add protobuf support
if(ENABLE_PROTOBUF)
    target_include_directories(${PROJECT_NAME} PRIVATE 
        $ENV{GENERATED}                          # Protobuf generated headers
        $ENV{INSTALL}/protobuf/include           # Protobuf library headers
        $ENV{INSTALL}/abseil/include             # Abseil library headers
    )
    LINK_PROTOBUF_LIBRARIES(${PROJECT_NAME})
    add_dependencies(${PROJECT_NAME} PROTO_FILES_READY)
endif()

#############################################################################################
# Create Catch2 library - optimized static library
#############################################################################################

# Create static library only if it doesn't exist or source changed
add_library(catch2_lib STATIC src/catch/catch_amalgamated.cpp)
target_include_directories(catch2_lib PUBLIC ${PROJECT_SOURCE_DIR}/src/catch/include)

# Collect test files
file(GLOB TEST_SOURCES "src/*_test.cpp")

# Exclude protobuf test if protobuf is disabled
if(NOT ENABLE_PROTOBUF)
    list(FILTER TEST_SOURCES EXCLUDE REGEX ".*session_proto_test\\.cpp$")
endif()

# Create test executable
add_executable(tests ${TEST_SOURCES} ${PROJECT_SOURCES})
target_include_directories(tests PRIVATE 
    ${PROJECT_SOURCE_DIR}/src                    # Project source headers
    ${PROJECT_SOURCE_DIR}/src/fmt/include        # fmt library headers
    ${PROJECT_SOURCE_DIR}/src/json/include       # JSON library headers
    ${PROJECT_SOURCE_DIR}/src/guid/include       # GUID library headers
    ${PROJECT_SOURCE_DIR}/src/catch/include      # Catch2 library headers
)
target_link_libraries(tests PRIVATE catch2_lib)

# Conditionally add protobuf support to tests
if(ENABLE_PROTOBUF)
    target_include_directories(tests PRIVATE 
        $ENV{GENERATED}                          # Protobuf generated headers
        $ENV{INSTALL}/protobuf/include           # Protobuf library headers
        $ENV{INSTALL}/abseil/include             # Abseil library headers
    )
    LINK_PROTOBUF_LIBRARIES(tests)
    add_dependencies(tests PROTO_FILES_READY)
endif()

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
        
        # Conditionally add protobuf support to examples
        if(ENABLE_PROTOBUF)
            target_include_directories(${EXAMPLE_NAME} PRIVATE 
                $ENV{GENERATED}                          # Protobuf generated headers
                $ENV{INSTALL}/protobuf/include           # Protobuf library headers
                $ENV{INSTALL}/abseil/include             # Abseil library headers
            )
            LINK_PROTOBUF_LIBRARIES(${EXAMPLE_NAME})
            add_dependencies(${EXAMPLE_NAME} PROTO_FILES_READY)
        endif()
        # Set the output directory for the executables, preserving the folder structure
        get_filename_component(EXAMPLE_DIR_NAME ${EXAMPLE_DIR} NAME)
        set_target_properties(${EXAMPLE_NAME} PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/examples/${EXAMPLE_DIR_NAME})
    endforeach()
endforeach()

#############################################################################################
# PROTOBUF INTEGRATION - OPTIONAL
#############################################################################################
# Control: cmake -DENABLE_PROTOBUF=ON/OFF ..
# Default: ON
#
# When enabled, protobuf is integrated into all targets (main, tests, examples)
# - Generated files: ${CMAKE_BINARY_DIR}/generated/session.pb.h and session.pb.cc
# - Include them in your code: #include "session.pb.h"
# - Namespace: session_proto::Point, session_proto::Color, etc.
#
# Build order (only happens once, cached in build/external/):
#   1. Abseil library (external dependency) - builds once, cached
#   2. Protobuf library + protoc compiler (external dependency) - builds once, cached
#   3. Generate C++ from .proto files (regenerated if .proto changes)
#   4. Build your project (links generated code)
#
# To disable: cmake -DENABLE_PROTOBUF=OFF ..
# To rebuild external deps: rm -rf build/external/
#############################################################################################