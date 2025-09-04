@echo off
REM Script to build session_py documentation using Sphinx

echo Building session_py documentation...

REM Change to docs directory
cd docs

REM Clean previous build
echo Cleaning previous build...
if exist _build rmdir /s /q _build
if exist Makefile.bat (
    call make.bat clean
) else (
    sphinx-build -M clean . _build
)

REM Build HTML documentation
echo Building HTML documentation...
if exist Makefile.bat (
    call make.bat html
) else (
    sphinx-build -M html . _build
)

if %errorlevel% equ 0 (
    REM Copy built HTML to a stable output directory used by CI deployment
    if exist ..\docs_output\html rmdir /s /q ..\docs_output\html
    if not exist ..\docs_output mkdir ..\docs_output
    xcopy /e /i _build\html ..\docs_output\html
    echo.
    echo Documentation built successfully!
    echo Open docs_output\html\index.html in your browser to view the documentation.
    echo.
    echo To serve locally, run:
    echo   cd docs_output\html ^&^& python -m http.server 8000
) else (
    echo Build failed!
    exit /b 1
)
