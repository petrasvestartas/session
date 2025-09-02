@echo off
REM Change directory to the directory containing this script
cd %~dp0

REM Create a directory for the build files
if not exist build mkdir build
cd build

REM Configure the project
cmake ..

REM Build the project with the specified configuration
cmake --build . --config Release

REM Run the main executable
.\Release\MyProject.exe

echo Current working directory: %cd%

REM Set base directory for examples
set "BASE_DIR=.\examples"

REM Check if the base directory exists
if exist "%BASE_DIR%" (
    REM Find and run all executables in examples directory and subdirectories
    for /r "%BASE_DIR%" %%X in (*.exe) do (
        echo [34mRunning example: %%X[0m
        "%%X"
    )
) else (
    echo Base directory does not exist: %BASE_DIR%
)