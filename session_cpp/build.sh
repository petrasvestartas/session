#!/bin/bash

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
cd build

# Configure the project with Ninja generator for faster builds
echo -e "${BLUE}Configuring project with Ninja...${NC}"

# Check if Ninja is installed
if command -v ninja &> /dev/null; then
    cmake -G Ninja .. 2>&1 | sed "s/.*warning.*/${YELLOW}&${NC}/g; s/.*error.*/${RED}&${NC}/g"
    CMAKE_STATUS=$?
else
    echo -e "${YELLOW}Ninja build system not found. Using default generator.${NC}"
    cmake .. 2>&1 | sed "s/.*warning.*/${YELLOW}&${NC}/g; s/.*error.*/${RED}&${NC}/g"
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
cmake --build . --config Release -- -j${NUM_CORES} 2>&1 | sed "s/.*warning.*/${YELLOW}&${NC}/g; s/.*error.*/${RED}&${NC}/g"
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit $BUILD_STATUS
fi

echo -e "${GREEN}Build successful!${NC}"

# Run the main executable
echo -e "${BLUE}Running main executable...${NC}"
./MyProject

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