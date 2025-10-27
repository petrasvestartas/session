@echo off
setlocal EnableDelayedExpansion

:: Change to the directory containing this script
cd /d "%~dp0"

echo Building and running all tests...

:: Create a directory for the build files
if not exist build mkdir build

:: Configure the project (Release, out-of-source)
echo Configuring project (Release)...
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release >nul 2>&1

if !errorlevel! neq 0 (
    echo Configuration failed!
    exit /b 1
)

:: Build the project
echo Building project...
cmake --build build --config Release -- /m

if !errorlevel! neq 0 (
    echo Build failed!
    exit /b 1
)

:: Check if tests executable exists
set TEST_EXE=
if exist "build\tests.exe" set TEST_EXE=build\tests.exe
if exist "build\Release\tests.exe" set TEST_EXE=build\Release\tests.exe
if not defined TEST_EXE (
    echo Tests executable not found!
    exit /b 1
)

:: List all test cases first
echo Listing all tests...
"%TEST_EXE%" --list-tests

:: Run all tests in verbose mode with prints
echo.
echo Running all tests (verbose)...
echo.
"%TEST_EXE%" -r compact -s -d yes

:: Check test results
if !errorlevel! equ 0 (
    echo.
    echo All tests passed! 
) else (
    echo.
    echo Some tests failed! 
    exit /b 1
)
