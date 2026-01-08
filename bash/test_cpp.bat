@echo off
REM Run C++ minitest only - does NOT touch other languages' JSON
REM Usage:
REM   test_cpp.bat              - Build and run all C++ tests
REM   test_cpp.bat --no-viewer  - Don't update testData.js

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
for %%i in ("%SCRIPT_DIR%..") do set "REPO_ROOT=%%~fi"
if not exist "%REPO_ROOT%\session_cpp" (
    for %%i in ("%REPO_ROOT%\..") do set "REPO_ROOT=%%~fi"
)

set "CPP_DIR=%REPO_ROOT%\session_cpp"
set "UPDATE_VIEWER=true"

REM Parse arguments
:parse_args
if "%~1"=="" goto :done_args
if /i "%~1"=="--fast" rem ignored for C++
if /i "%~1"=="-f" rem ignored for C++
if /i "%~1"=="--no-viewer" set "UPDATE_VIEWER=false"
shift
goto :parse_args
:done_args

if not exist "%CPP_DIR%" (
    echo [cpp] ERROR: session_cpp not found at %CPP_DIR%
    exit /b 1
)

REM Get CPU cores
for /f "tokens=2 delims==" %%i in ('wmic cpu get NumberOfLogicalProcessors /value 2^>nul ^| find "="') do set "NUM_CORES=%%i"
if not defined NUM_CORES set "NUM_CORES=4"

echo [cpp] Building with %NUM_CORES% jobs...

pushd "%CPP_DIR%"

echo [cpp] Configuring CMake...
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
if errorlevel 1 (
    echo [cpp] CMake configuration failed
    popd
    exit /b 1
)

echo [cpp] Compiling...
cmake --build build --config Release --parallel %NUM_CORES% -- /v:m /nologo
if errorlevel 1 (
    echo [cpp] Build failed
    popd
    exit /b 1
)

echo [cpp] Running minitest...
if exist "build\Release\point_minitest.exe" (
    "build\Release\point_minitest.exe"
) else if exist "build\point_minitest.exe" (
    "build\point_minitest.exe"
) else (
    echo [cpp] ERROR: point_minitest.exe not found
    popd
    exit /b 1
)

popd
echo [cpp] Tests complete

REM Update viewer if requested
if "%UPDATE_VIEWER%"=="true" (
    call "%SCRIPT_DIR%lib\consolidate.bat"
)

exit /b 0
endlocal
