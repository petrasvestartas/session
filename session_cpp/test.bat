@echo off
setlocal EnableDelayedExpansion

:: Change to the directory containing this script
cd /d "%~dp0"

echo Building and running all tests...

:: Create a directory for the build files
if not exist build mkdir build
cd build

:: Configure the project
echo Configuring project...
cmake .. >nul 2>&1

if !errorlevel! neq 0 (
    echo Configuration failed!
    exit /b 1
)

:: Build the project
echo Building project...
cmake --build . --config Release

if !errorlevel! neq 0 (
    echo Build failed!
    exit /b 1
)

:: Check if tests executable exists
if not exist "Release\tests.exe" (
    echo Tests executable not found!
    exit /b 1
)

:: Run all tests
echo Running all tests...
echo.
Release\tests.exe --reporter compact

:: Check test results
if !errorlevel! equ 0 (
    echo.
    echo All tests passed! 
) else (
    echo.
    echo Some tests failed! 
    exit /b 1
)
