@echo off

REM Session Library - Top-level Documentation Builder
REM Convenience script to build all documentation from project root

echo 🚀 Session Library - Building All Documentation
echo ===============================================

REM Get the directory where this script is located
set "SCRIPT_DIR=%~dp0"

REM Change to session_docs directory and run the build script
cd /d "%SCRIPT_DIR%session_docs"

echo 📁 Building from: %CD%
echo.

REM Run the documentation build script
if "%1" == "--open" (
    call build_docs.bat --open
) else (
    call build_docs.bat
)
