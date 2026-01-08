@echo off
REM Minitest - Run tests for Python, C++, and Rust implementations
REM Usage:
REM   minitest.bat              - Run all languages
REM   minitest.bat --py         - Python only
REM   minitest.bat --cpp        - C++ only
REM   minitest.bat --rust       - Rust only
REM   minitest.bat --fast       - Skip dependency installs
REM   minitest.bat --no-web     - Skip Vue server
REM   minitest.bat --kill       - Stop dev server only

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
for %%i in ("%SCRIPT_DIR%..") do set "REPO_ROOT=%%~fi"
if not exist "%REPO_ROOT%\session_py" (
    for %%i in ("%REPO_ROOT%\..") do set "REPO_ROOT=%%~fi"
)

set "TESTS_DIR=%REPO_ROOT%\session_tests"

REM Default: run all languages
set "RUN_PYTHON=true"
set "RUN_CPP=true"
set "RUN_RUST=true"
set "FAST_MODE=false"
set "START_WEB=true"
set "KILL_SERVER=false"

REM Parse arguments
:parse_args
if "%~1"=="" goto :done_args
if /i "%~1"=="--py" (
    set "RUN_CPP=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--python" (
    set "RUN_CPP=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--cpp" (
    set "RUN_PYTHON=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--c++" (
    set "RUN_PYTHON=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--rust" (
    set "RUN_PYTHON=false"
    set "RUN_CPP=false"
)
if /i "%~1"=="--rs" (
    set "RUN_PYTHON=false"
    set "RUN_CPP=false"
)
if /i "%~1"=="--fast" set "FAST_MODE=true"
if /i "%~1"=="-f" set "FAST_MODE=true"
if /i "%~1"=="--no-web" set "START_WEB=false"
if /i "%~1"=="--kill" set "KILL_SERVER=true"
if /i "%~1"=="-k" set "KILL_SERVER=true"
shift
goto :parse_args
:done_args

REM Handle --kill immediately (no other actions)
if "%KILL_SERVER%"=="true" (
    call "%SCRIPT_DIR%lib\server.bat" stop
    goto :end
)

REM Build flags for sub-scripts
set "FAST_ARG="
if "%FAST_MODE%"=="true" set "FAST_ARG=--fast"

REM Regenerate Python protos (skip in fast mode)
if "%FAST_MODE%"=="false" (
    if "%RUN_PYTHON%"=="true" (
        call :regenerate_python_protos
    )
)

REM Run language tests (each script handles its own JSON, no cleanup needed)
if "%RUN_PYTHON%"=="true" (
    echo [mini] === Python Tests ===
    call "%SCRIPT_DIR%test_py.bat" %FAST_ARG% --no-viewer
    if errorlevel 1 (
        echo [mini] Python tests failed
        exit /b 1
    )
)

if "%RUN_CPP%"=="true" (
    echo [mini] === C++ Tests ===
    call "%SCRIPT_DIR%test_cpp.bat" %FAST_ARG% --no-viewer
    if errorlevel 1 (
        echo [mini] C++ tests failed
        exit /b 1
    )
)

if "%RUN_RUST%"=="true" (
    echo [mini] === Rust Tests ===
    call "%SCRIPT_DIR%test_rust.bat" --no-viewer
    if errorlevel 1 (
        echo [mini] Rust tests failed
        exit /b 1
    )
)

REM Consolidate ALL JSON (reads existing files from ALL languages)
echo [mini] === Consolidating Test Data ===
call "%SCRIPT_DIR%lib\consolidate.bat"

REM Check for CI environment
if defined CI goto :ci_exit
if defined GITHUB_ACTIONS goto :ci_exit

REM Handle web server
if "%START_WEB%"=="true" (
    call "%SCRIPT_DIR%lib\server.bat" start %FAST_ARG%
)

echo [mini] Done
echo [mini] Re-run to update (browser auto-refreshes via Vite HMR)
goto :end

:ci_exit
echo [mini] CI environment detected - skipping dev server
echo [mini] Done
goto :end

:regenerate_python_protos
set "PROTO_DIR=%REPO_ROOT%\session_proto"
set "PY_PROTO_OUT=%REPO_ROOT%\session_py\src\session_py\proto"

REM Find protoc
set "PROTOC="
where protoc >nul 2>&1
if not errorlevel 1 (
    set "PROTOC=protoc"
    goto :found_protoc
)
if exist "%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe" (
    set "PROTOC=%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe"
    goto :found_protoc
)
echo [mini] Warning: protoc not found, skipping Python protobuf regeneration
goto :eof

:found_protoc
if not exist "%PROTO_DIR%" (
    echo [mini] Warning: %PROTO_DIR% not found, skipping Python protobuf regeneration
    goto :eof
)

if not exist "%PY_PROTO_OUT%" mkdir "%PY_PROTO_OUT%"

echo [mini] Regenerating Python protobuf bindings...
for %%f in ("%PROTO_DIR%\*.proto") do (
    "%PROTOC%" --python_out="%PY_PROTO_OUT%" -I"%PROTO_DIR%" "%%f" 2>nul
)

echo [mini] Fixing Python protobuf imports...
for %%f in ("%PY_PROTO_OUT%\*_pb2.py") do (
    if exist "%%f" (
        powershell -Command "(Get-Content '%%f') -replace '^import ([a-z_]*_pb2) as', 'from . import $1 as' | Set-Content '%%f'" 2>nul
    )
)
goto :eof

:end
endlocal
