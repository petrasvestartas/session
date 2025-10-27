#!/bin/bash

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Auto-format Python code
echo -e "${BLUE}Formatting Python code...${NC}"
python3 -m black src examples

# Change directory to the directory containing this script
cd "$(dirname "$0")"

echo -e "${BLUE}Running Python tests...${NC}"

# Check if pytest is available
if command -v pytest &> /dev/null; then
    echo -e "${YELLOW}Using pytest to run tests...${NC}"
    pytest -s -v src/session_py/*_test.py
    test_result=$?
else
    echo -e "${YELLOW}pytest not found, using python -m unittest...${NC}"
    
    # Run each test file individually
    echo -e "${YELLOW}Running color tests...${NC}"
    python -m unittest src.session_py.color_test
    color_result=$?
    
    echo -e "${YELLOW}Running point tests...${NC}"
    python -m unittest src.session_py.point_test
    point_result=$?
    
    echo -e "${YELLOW}Running vector tests...${NC}"
    python -m unittest src.session_py.vector_test
    vector_result=$?
    
    # Check if any test failed
    if [ $color_result -ne 0 ] || [ $point_result -ne 0 ] || [ $vector_result -ne 0 ]; then
        test_result=1
    else
        test_result=0
    fi
fi

# Check test results
if [ $test_result -eq 0 ]; then
    echo ""
    echo -e "${GREEN}All tests passed! ✓${NC}"
else
    echo ""
    echo -e "${RED}Some tests failed! ✗${NC}"
    exit 1
fi
