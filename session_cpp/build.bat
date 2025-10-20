@echo off

echo Skipping code formatting...
REM Skipping clang-format to avoid hanging on large external libraries

REM Change directory to the directory containing this script
cd %~dp0

REM Create a directory for the build files
if not exist build mkdir build

REM Configure the project (Release, out-of-source)
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release

REM Build the project with the specified configuration (parallel if supported)
cmake --build build --config Release -- /m

REM Run the main executable
IF EXIST .\build\Release\MyProject.exe (
    .\build\Release\MyProject.exe
) ELSE IF EXIST .\build\MyProject.exe (
    .\build\MyProject.exe
) ELSE (
    echo Executable not found under build\
)

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