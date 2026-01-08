@echo off
REM Run Python minitest only - does NOT touch other languages' JSON
REM Usage:
REM   test_py.bat              - Run all Python tests
REM   test_py.bat point        - Run only Point tests
REM   test_py.bat --fast       - Skip pip install if already installed
REM   test_py.bat --no-viewer  - Don't update testData.js

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
for %%i in ("%SCRIPT_DIR%..") do set "REPO_ROOT=%%~fi"
if not exist "%REPO_ROOT%\session_py" (
    for %%i in ("%REPO_ROOT%\..") do set "REPO_ROOT=%%~fi"
)

set "VENV_DIR=%REPO_ROOT%\uvsession"
set "PYTHON=%VENV_DIR%\Scripts\python.exe"
set "CLASS_NAMES=color knot line mesh nurbscurve nurbssurface plane point pointcloud polyline tolerance vector xform"

set "FAST_MODE=false"
set "UPDATE_VIEWER=true"
set "CLASS_FILTER="

REM Parse arguments
:parse_args
if "%~1"=="" goto :done_args
if /i "%~1"=="--fast" (
    set "FAST_MODE=true"
    goto :next_arg
)
if /i "%~1"=="-f" (
    set "FAST_MODE=true"
    goto :next_arg
)
if /i "%~1"=="--no-viewer" (
    set "UPDATE_VIEWER=false"
    goto :next_arg
)
REM If not a flag, treat as class filter
if not "%~1:~0,1%"=="-" set "CLASS_FILTER=%~1"
:next_arg
shift
goto :parse_args
:done_args

REM Check uv availability
set "HAS_UV=0"
where uv >nul 2>&1
if not errorlevel 1 set "HAS_UV=1"

REM Ensure Python environment
call :ensure_python_env
if errorlevel 1 (
    echo [py] ERROR: Failed to setup Python environment
    exit /b 1
)

REM Run tests
if not "%CLASS_FILTER%"=="" (
    echo [py] Running %CLASS_FILTER% tests...
    "%PYTHON%" -m "session_py.%CLASS_FILTER%_test"
) else (
    for %%c in (%CLASS_NAMES%) do (
        echo [py] Running %%c tests...
        "%PYTHON%" -m "session_py.%%c_test"
    )
)

echo [py] Tests complete

REM Update viewer if requested
if "%UPDATE_VIEWER%"=="true" (
    call "%SCRIPT_DIR%lib\consolidate.bat"
)

exit /b 0

:ensure_python_env
if not exist "%PYTHON%" (
    echo [py] Creating Python environment...
    if "%HAS_UV%"=="1" (
        pushd "%REPO_ROOT%"
        uv venv uvsession
        popd
    ) else (
        pushd "%REPO_ROOT%"
        python -m venv uvsession
        popd
    )
)

if not exist "%PYTHON%" (
    echo [py] ERROR: Failed to create Python environment
    exit /b 1
)

REM Fast mode: skip install if already working
if "%FAST_MODE%"=="true" (
    "%PYTHON%" -c "import session_py" >nul 2>&1
    if not errorlevel 1 (
        echo [py] Fast mode: session_py already installed
        exit /b 0
    )
)

echo [py] Installing session_py...
if "%HAS_UV%"=="1" (
    uv pip install --python "%PYTHON%" -e "%REPO_ROOT%\session_py" pytest
) else (
    "%PYTHON%" -m pip --version >nul 2>&1
    if errorlevel 1 (
        "%PYTHON%" -m ensurepip --default-pip 2>nul
    )
    "%PYTHON%" -m pip install -e "%REPO_ROOT%\session_py" pytest
)
exit /b 0

endlocal
