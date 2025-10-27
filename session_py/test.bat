@echo off

echo Formatting Python code...
python -m black src examples

setlocal EnableDelayedExpansion

:: Change to the directory containing this script
cd /d "%~dp0"

echo Running Python tests...

:: Check if pytest is available
python -m pytest --version >nul 2>&1
if !errorlevel! equ 0 (
    echo Using pytest to run tests...
    python -m pytest -s -v src/session_py/*_test.py
    set test_result=!errorlevel!
) else (
    echo pytest not found, using python -m unittest...
    
    :: Run each test file individually
    echo Running color tests...
    python -m unittest src.session_py.color_test
    set color_result=!errorlevel!
    
    echo Running point tests...
    python -m unittest src.session_py.point_test
    set point_result=!errorlevel!
    
    echo Running vector tests...
    python -m unittest src.session_py.vector_test
    set vector_result=!errorlevel!
    
    :: Check if any test failed
    if !color_result! neq 0 (
        set test_result=1
    ) else if !point_result! neq 0 (
        set test_result=1
    ) else if !vector_result! neq 0 (
        set test_result=1
    ) else (
        set test_result=0
    )
)

:: Check test results
if !test_result! equ 0 (
    echo.
    echo All tests passed!
) else (
    echo.
    echo Some tests failed!
    exit /b 1
)
