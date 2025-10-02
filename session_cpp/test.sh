#!/bin/bash

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Change directory to the directory containing this script
cd "$(dirname "$0")"

echo -e "${BLUE}Building and running all tests...${NC}"

# Create a directory for the build files
mkdir -p build
cd build

# Configure the project with Ninja generator for faster builds
echo -e "${YELLOW}Configuring project with Ninja...${NC}"

# Check if Ninja is installed
if command -v ninja &> /dev/null; then
    cmake -G Ninja .. > /dev/null 2>&1
    CMAKE_STATUS=$?
else
    echo -e "${YELLOW}Ninja build system not found. Using default generator.${NC}"
    cmake .. > /dev/null 2>&1
    CMAKE_STATUS=$?
fi

if [ $CMAKE_STATUS -ne 0 ]; then
    echo -e "${RED}Configuration failed!${NC}"
    exit $CMAKE_STATUS
fi

# Build the project with parallel jobs
echo -e "${YELLOW}Building project...${NC}"

# Get number of CPU cores for parallel builds
NUM_CORES=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2)

# Build with parallel jobs
cmake --build . --config Release -- -j${NUM_CORES} > /dev/null 2>&1
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit $BUILD_STATUS
fi

# Check if tests executable exists
if [ ! -f "./tests" ]; then
    echo -e "${RED}Tests executable not found!${NC}"
    exit 1
fi

# Run all tests
echo -e "${GREEN}Running all tests...${NC}"
echo ""
./tests

# Check test results
TEST_EXIT_CODE=$?
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo ""
    echo -e "${GREEN}All tests passed! ✓${NC}"
else
    echo ""
    echo -e "${RED}Some tests failed! ✗${NC}"
    exit 1
fi
