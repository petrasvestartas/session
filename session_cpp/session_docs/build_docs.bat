@echo off
setlocal enabledelayedexpansion

REM Session Library - Combined Documentation Builder
REM Builds documentation for all languages (Rust, C++, Python) and creates unified docs

echo 🚀 Building Session Library Documentation...
echo ==============================================

REM Get the absolute path of the session root directory
set "SCRIPT_DIR=%~dp0"
set "SESSION_ROOT=%SCRIPT_DIR%.."
set "DOCS_DIR=%SESSION_ROOT%\session_docs"
set "OUTPUT_DIR=%DOCS_DIR%\combined_docs"

echo 📁 Session root: %SESSION_ROOT%
echo 📁 Docs directory: %DOCS_DIR%
echo 📁 Output directory: %OUTPUT_DIR%

REM Create output directory
if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

REM Copy the main index.html
echo 📋 Copying main documentation index...
copy "%DOCS_DIR%\index.html" "%OUTPUT_DIR%\" >nul

REM Build and copy Rust documentation
echo.
echo 🦀 Building Rust Documentation...
echo =================================
if exist "%SESSION_ROOT%\session_rust" (
    cd /d "%SESSION_ROOT%\session_rust"
    
    REM Check if Rust is installed
    where cargo >nul 2>nul
    if !errorlevel! == 0 (
        echo ✅ Cargo found
        
        REM Build Rust docs
        cargo doc --no-deps --document-private-items
        
        if !errorlevel! == 0 (
            echo ✅ Rust documentation built successfully!
            if not exist "%OUTPUT_DIR%\rust" mkdir "%OUTPUT_DIR%\rust"
            xcopy "target\doc" "%OUTPUT_DIR%\rust\html\" /E /I /Y >nul
        ) else (
            echo ❌ Rust documentation build failed!
        )
    ) else (
        echo ⚠️  Cargo not found - skipping Rust documentation
    )
) else (
    echo ⚠️  session_rust directory not found - skipping
)

REM Build and copy C++ documentation
echo.
echo ⚡ Building C++ Documentation...
echo ===============================
if exist "%SESSION_ROOT%\session_cpp" (
    cd /d "%SESSION_ROOT%\session_cpp"
    
    REM Check if Doxygen is installed
    where doxygen >nul 2>nul
    if !errorlevel! == 0 (
        echo ✅ Doxygen found
        
        REM Build C++ docs using the doc.sh script or batch equivalent
        if exist "doc.bat" (
            call doc.bat
        ) else if exist "doc.sh" (
            REM Try to run the shell script with Git Bash if available
            where bash >nul 2>nul
            if !errorlevel! == 0 (
                bash doc.sh
            ) else (
                REM Fallback to direct doxygen call
                cd docs
                doxygen Doxyfile
                cd ..
            )
        ) else (
            REM Direct doxygen call
            cd docs
            doxygen Doxyfile
            cd ..
        )
        
        if !errorlevel! == 0 (
            echo ✅ C++ documentation built successfully!
            if not exist "%OUTPUT_DIR%\cpp" mkdir "%OUTPUT_DIR%\cpp"
            xcopy "docs_output\*" "%OUTPUT_DIR%\cpp\" /E /I /Y >nul 2>nul || echo No C++ docs output found
        ) else (
            echo ❌ C++ documentation build failed!
        )
    ) else (
        echo ⚠️  Doxygen not found - skipping C++ documentation
    )
) else (
    echo ⚠️  session_cpp directory not found - skipping
)

REM Build and copy Python documentation
echo.
echo 🐍 Building Python Documentation...
echo ==================================
if exist "%SESSION_ROOT%\session_py" (
    cd /d "%SESSION_ROOT%\session_py"
    
    REM Check if Python and pip are installed
    where python >nul 2>nul
    set "PYTHON_CMD=python"
    if !errorlevel! neq 0 (
        where python3 >nul 2>nul
        if !errorlevel! == 0 (
            set "PYTHON_CMD=python3"
        ) else (
            set "PYTHON_CMD="
        )
    )
    
    if not "!PYTHON_CMD!" == "" (
        echo ✅ Python found
        
        REM Install documentation dependencies
        echo 📦 Installing Python documentation dependencies...
        !PYTHON_CMD! -m pip install sphinx sphinxawesome-theme myst-parser 2>nul || echo Warning: Could not install some dependencies
        
        REM Install the package itself for autodoc
        !PYTHON_CMD! -m pip install -e . 2>nul || echo Warning: Could not install package in development mode
        
        REM Build Python docs
        if exist "doc.bat" (
            call doc.bat
        ) else if exist "doc.sh" (
            REM Try to run the shell script with Git Bash if available
            where bash >nul 2>nul
            if !errorlevel! == 0 (
                bash doc.sh
            ) else (
                REM Fallback for manual Sphinx build
                if exist "docs" (
                    cd docs
                    !PYTHON_CMD! -m sphinx -b html . _build 2>nul
                    cd ..
                )
            )
        ) else (
            REM Manual Sphinx build
            if exist "docs" (
                cd docs
                !PYTHON_CMD! -m sphinx -b html . _build 2>nul
                cd ..
            )
        )
        
        if !errorlevel! == 0 (
            echo ✅ Python documentation built successfully!
            if not exist "%OUTPUT_DIR%\python" mkdir "%OUTPUT_DIR%\python"
            xcopy "docs_output\*" "%OUTPUT_DIR%\python\" /E /I /Y >nul 2>nul || echo No Python docs output found
        ) else (
            echo ❌ Python documentation build failed!
        )
    ) else (
        echo ⚠️  Python not found - skipping Python documentation
    )
) else (
    echo ⚠️  session_py directory not found - skipping
)

REM Final summary
echo.
echo 🎉 Documentation Build Complete!
echo ================================
echo 📁 Combined documentation location: %OUTPUT_DIR%
echo 🌐 Open: %OUTPUT_DIR%\index.html
echo.

REM List what was actually built
echo 📋 Built documentation:
if exist "%OUTPUT_DIR%\rust" (
    echo   ✅ Rust documentation
) else (
    echo   ❌ Rust documentation (failed or skipped^)
)

if exist "%OUTPUT_DIR%\cpp" (
    echo   ✅ C++ documentation
) else (
    echo   ❌ C++ documentation (failed or skipped^)
)

if exist "%OUTPUT_DIR%\python" (
    echo   ✅ Python documentation
) else (
    echo   ❌ Python documentation (failed or skipped^)
)

REM Option to open documentation
if "%1" == "--open" (
    echo.
    echo 🚀 Opening documentation in browser...
    start "" "%OUTPUT_DIR%\index.html"
)

echo.
echo ✨ Done!

cd /d "%SESSION_ROOT%"
pause
