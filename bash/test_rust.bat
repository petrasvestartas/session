@echo off
REM Run Rust minitest only - does NOT touch other languages' JSON
REM Usage:
REM   test_rust.bat              - Build and run all Rust tests
REM   test_rust.bat --no-viewer  - Don't update testData.js

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
for %%i in ("%SCRIPT_DIR%..") do set "REPO_ROOT=%%~fi"
if not exist "%REPO_ROOT%\session_rust" (
    for %%i in ("%REPO_ROOT%\..") do set "REPO_ROOT=%%~fi"
)

set "RUST_DIR=%REPO_ROOT%\session_rust"
set "UPDATE_VIEWER=true"

REM Parse arguments
:parse_args
if "%~1"=="" goto :done_args
if /i "%~1"=="--fast" rem ignored for Rust
if /i "%~1"=="-f" rem ignored for Rust
if /i "%~1"=="--no-viewer" set "UPDATE_VIEWER=false"
shift
goto :parse_args
:done_args

if not exist "%RUST_DIR%" (
    echo [rust] ERROR: session_rust not found at %RUST_DIR%
    exit /b 1
)

REM Find protoc
set "PROTOC="
where protoc >nul 2>&1
if not errorlevel 1 (
    set "PROTOC=protoc"
    goto :protoc_found
)
if exist "%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe" (
    set "PROTOC=%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe"
    goto :protoc_found
)
echo [rust] Warning: protoc not found, build may fail
:protoc_found

REM Get CPU cores
for /f "tokens=2 delims==" %%i in ('wmic cpu get NumberOfLogicalProcessors /value 2^>nul ^| find "="') do set "NUM_CORES=%%i"
if not defined NUM_CORES set "NUM_CORES=4"

echo [rust] Building and running minitest...
pushd "%RUST_DIR%"
cargo run --release --features protobuf --bin minitest -j %NUM_CORES%
if errorlevel 1 (
    echo [rust] Minitest failed
    popd
    exit /b 1
)
popd

echo [rust] Tests complete

REM Update viewer if requested
if "%UPDATE_VIEWER%"=="true" (
    call "%SCRIPT_DIR%lib\consolidate.bat"
)

exit /b 0
endlocal
