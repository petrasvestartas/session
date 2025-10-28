# Python Dependencies Fix for GitHub Actions

## Problem

GitHub Actions was failing with:
```
ModuleNotFoundError: No module named 'numpy'
```

When trying to import from `session_py`, the BVH module requires numpy and numba, but these weren't being installed in the CI pipeline.

## Root Cause

The GitHub Actions workflow (`.github/workflows/build-python.yml`) was only installing:
- `build` (for building the package)
- `pytest` (for running tests)

But it wasn't installing the actual package dependencies defined in `pyproject.toml`:
- `numpy>=1.20.0`
- `numba>=0.56.0`

## Solution

Updated the workflow to install the package in **editable mode**:

```yaml
- name: Install dependencies
  working-directory: ./session_py
  run: |
    python -m pip install --upgrade pip
    pip install build
    pip install -e .  # ← This installs package + all dependencies
```

## Why This Works

Installing with `pip install -e .` (editable mode):
1. ✅ Reads `pyproject.toml` dependencies section
2. ✅ Automatically installs all required packages (numpy, numba, pytest, pytest-cov)
3. ✅ Makes the package importable without building
4. ✅ Works across all platforms (Ubuntu, Windows, macOS)

## Dependencies Now Installed

From `pyproject.toml`:
```toml
dependencies = [
    "numpy>=1.20.0",    # Required for BVH arena arrays
    "numba>=0.56.0",    # Required for JIT-compiled collision detection
    "pytest>=7.0",      # Test framework
    "pytest-cov>=4.0",  # Coverage reporting
]
```

## Verification

The workflow now:
1. Installs dependencies (including numpy and numba)
2. Runs tests (if tests directory exists)
3. Builds the package
4. Tests import: `from src.session_py import Point, Color`
5. Uploads artifacts

All steps should now pass on all platforms.

## Testing Locally

To verify the same environment as CI:
```bash
cd session_py
python -m pip install --upgrade pip
pip install build
pip install -e .
python -c "from src.session_py import Point, Color; print('Import successful')"
```

Should output: `Import successful ✓`
