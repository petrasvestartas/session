#!/bin/bash

# Skipping code formatting to avoid hanging on large external libraries
echo -e "\033[0;34mSkipping code formatting...\033[0m"

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Change directory to the directory containing this script
cd "$(dirname "$0")"

# Create a directory for the build files
mkdir -p build

# Configure the project in Release mode (out-of-source)
echo -e "${BLUE}Configuring project (Release)...${NC}"

# Check if Ninja is installed
if command -v ninja &> /dev/null; then
    cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS_RELEASE="-O3 -march=native -DNDEBUG" 2>&1 | sed "s/.*warning.*/${YELLOW}&${NC}/g; s/.*error.*/${RED}&${NC}/g"
    CMAKE_STATUS=$?
else
    echo -e "${YELLOW}Ninja build system not found. Using default generator.${NC}"
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS_RELEASE="-O3 -march=native -DNDEBUG" 2>&1 | sed "s/.*warning.*/${YELLOW}&${NC}/g; s/.*error.*/${RED}&${NC}/g"
    CMAKE_STATUS=$?
fi

if [ $CMAKE_STATUS -ne 0 ]; then
    echo -e "${RED}Configuration failed!${NC}"
    exit $CMAKE_STATUS
fi

# Build the project with colored output and parallel jobs
echo -e "${BLUE}Building project...${NC}"

# Get number of CPU cores for parallel builds
NUM_CORES=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2)

# Build with parallel jobs
cmake --build build --parallel ${NUM_CORES} 2>&1 | sed "s/.*warning.*/${YELLOW}&${NC}/g; s/.*error.*/${RED}&${NC}/g"
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit $BUILD_STATUS
fi

echo -e "${GREEN}Build successful!${NC}"

# Run the main executable
echo -e "${BLUE}Running main executable...${NC}"
if [ -x "./build/MyProject" ]; then
    ./build/MyProject
elif [ -x "./build/Release/MyProject" ]; then
    ./build/Release/MyProject
else
    echo -e "${RED}Executable not found in build/.${NC}"
fi

# Run all executables in the examples directory and its subdirectories
echo "Current working directory: $(pwd)"

# Use an absolute path for BASE_DIR if the script's execution location might vary
BASE_DIR="./examples"

# # Check if the base directory exists
# if [ -d "$BASE_DIR" ]; then
#     # Find all directories and files in the base directory and its subdirectories
#     find "$BASE_DIR" -type d -o -type f -executable | sort | while read -r item; do
#         if [ -d "$item" ]; then
#             # If the item is a directory, find all executables within it
#             find "$item" -type f -executable | sort | while read -r example; do
#                 # Print in blue color
#                 echo -e "\033[34mRunning example: $example\033[0m"
#                 # Run the example
#                 "$example"
#             done
#         elif [ -x "$item" ]; then
#             # If the item is an executable file, run it
#             echo -e "\033[34mRunning example: $item\033[0m"
#             "$item"
#         fi
#     done
# else
#     echo "Base directory does not exist: $BASE_DIR"
# fi