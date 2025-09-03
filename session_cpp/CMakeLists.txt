# Set the minimum version of CMake required
cmake_minimum_required(VERSION 3.22)

# Define the project and the languages used
project(MyProject VERSION 1.0 LANGUAGES CXX)

# Enable compilation database generation for better IntelliSense
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add compiler warning flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic)
    # Enable AVX2 for uuid_v4.h library
    add_compile_options(-mavx2)
    # Uncomment to enable more aggressive warnings
    # add_compile_options(-Werror)
elseif(MSVC)
    add_compile_options(/W4)
    # Enable AVX2 for uuid_v4.h library
    add_compile_options(/arch:AVX2)
    # Uncomment to treat warnings as errors
    # add_compile_options(/WX)
endif()

# Add executable for the project. Include main.cpp and all source files from the src directory
file(GLOB_RECURSE SOURCES "src/*.cpp")

# Create a static library from the sources instead of adding them directly to each executable
add_library(myproject_lib STATIC ${SOURCES})

# Add executable for the main project
add_executable(${PROJECT_NAME} main.cpp)
# Link the static library to the main executable
target_link_libraries(${PROJECT_NAME} myproject_lib)

# Include the src directory and its subdirectories to pick up the .hpp files
target_include_directories(myproject_lib PUBLIC ${PROJECT_SOURCE_DIR}/src)

# Get all subdirectories in the examples directory
file(GLOB EXAMPLE_DIRS "examples/*")

foreach(EXAMPLE_DIR ${EXAMPLE_DIRS})
    # Get all .cpp files in the current example directory
    file(GLOB EXAMPLE_FILES "${EXAMPLE_DIR}/*.cpp")

    foreach(EXAMPLE_FILE ${EXAMPLE_FILES})
        # Get the filename without the directory and extension
        get_filename_component(EXAMPLE_NAME ${EXAMPLE_FILE} NAME_WE)
        # Create an executable for each example
        add_executable(${EXAMPLE_NAME} ${EXAMPLE_FILE})
        # Link the static library to the example executable
        target_link_libraries(${EXAMPLE_NAME} myproject_lib)
        # Set the output directory for the executables, preserving the folder structure
        get_filename_component(EXAMPLE_DIR_NAME ${EXAMPLE_DIR} NAME)
        set_target_properties(${EXAMPLE_NAME} PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/examples/${EXAMPLE_DIR_NAME})
    endforeach()
endforeach()