#!/bin/bash
# Install session_py to Rhino Python environment

RHINO_PYTHON="C:/Users/petrasv/.rhinocode/py39-rh8/python.exe"

if [ ! -f "$RHINO_PYTHON" ]; then
    echo "Error: Rhino Python not found at $RHINO_PYTHON"
    exit 1
fi

echo "Installing session_py to Rhino Python 3.9..."
cd "$(dirname "$0")/../session_py"

# Install dependencies
"$RHINO_PYTHON" -m pip install numpy protobuf

# Install session_py in editable mode
"$RHINO_PYTHON" -m pip install -e .

cat << 'EOF'

=== Installation Complete ===

USAGE IN RHINO PYTHON:

  # First time import
  from session_py import NurbsCurve, Point

  # After editing session_py, reload all modules:
  from session_py.reload import reload_package
  reload_package("session_py")

  # Reload any other package too:
  reload_package("compas")
  reload_package("my_custom_lib")

  # Nuclear option (clears cache, re-import needed):
  from session_py.reload import clear_package
  clear_package("session_py")
  from session_py import Point  # must re-import

  # Reload single file:
  from session_py.reload import reload_file
  reload_file("C:/path/to/my_module.py")

  # Shortcuts for session_py:
  from session_py.reload import reload_session, clear_session
  reload_session()  # same as reload_package("session_py")

EOF
