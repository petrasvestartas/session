#!/bin/bash
# Install session_py and session_rhino to Rhino Python environment

RHINO_PATHS=(
    "C:/Users/petrasv/.rhinocode/py39-rh8/python.exe"
    "C:/Users/Petras/.rhinocode/py39-rh8/python.exe"
)

RHINO_PYTHON=""
for p in "${RHINO_PATHS[@]}"; do
    if [ -f "$p" ]; then
        RHINO_PYTHON="$p"
        break
    fi
done

if [ -z "$RHINO_PYTHON" ]; then
    echo "Error: Rhino Python not found in any known path:"
    for p in "${RHINO_PATHS[@]}"; do
        echo "  $p"
    done
    exit 1
fi

echo "Found Rhino Python: $RHINO_PYTHON"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "Installing session_py..."
"$RHINO_PYTHON" -m pip install numpy protobuf
"$RHINO_PYTHON" -m pip install -e "$SCRIPT_DIR/../session_py"

echo "Installing session_rhino..."
"$RHINO_PYTHON" -m pip install -e "$SCRIPT_DIR"

cat << 'EOF'

=== Installation Complete ===

USAGE IN RHINO PYTHON:

  import session_rhino.rhino_point
  import session_rhino.rhino_nurbscurve

  session_rhino.rhino_point.add(Point(1, 2, 3))
  session_rhino.rhino_nurbscurve.add(crv)

  # After editing, reload all modules:
  from session_py.reload import reload_package
  reload_package("session_py")
  reload_package("session_rhino")

EOF
