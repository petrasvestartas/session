#!/bin/bash

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Change directory to the directory containing this script
cd "$(dirname "$0")"

# Auto-format Rust code
echo -e "${BLUE}Formatting Rust code...${NC}"
cargo fmt --all

# Run clippy
echo -e "${BLUE}Running clippy (release)...${NC}"
cargo clippy --all-targets --all-features --release -- -D warnings
if [ $? -ne 0 ]; then
    echo -e "${RED}Clippy failed! Fix warnings above.${NC}"
    exit 1
fi

# Run tests
echo -e "${BLUE}Running Rust tests (release)...${NC}"
cargo test --release

if [ -f "src/main.rs" ]; then
    echo -e "${BLUE}Running demo binary (release)...${NC}"
    cargo run --release
fi
