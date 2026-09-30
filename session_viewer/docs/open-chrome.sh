#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
chrome=${CHROME_BIN:-google-chrome}

# A separate profile prevents an existing Chrome process from ignoring these flags.
exec "$chrome" \
    --user-data-dir="$root/target/chrome-webgpu" \
    --enable-unsafe-webgpu \
    --enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE \
    --disable-vulkan-surface \
    --ozone-platform=x11 \
    "${1:-http://127.0.0.1:8770/}"
