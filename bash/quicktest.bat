@echo off
REM Quick single-class test for rapid development
REM Usage:
REM   quicktest.bat point          - Test Point class in all languages
REM   quicktest.bat point --py     - Test Point class in Python only
REM   quicktest.bat point --cpp    - Test Point class in C++ only
REM   quicktest.bat point --rust   - Test Point class in Rust only

setlocal enabledelayedexpansion

set "CLASS_NAME=%~1"
if "%CLASS_NAME%"=="" set "CLASS_NAME=point"

set "LANG_PY=true"
set "LANG_CPP=true"
set "LANG_RUST=true"

:parse_args
shift
if "%~1"=="" goto :done_args
if /i "%~1"=="--py" ( set "LANG_CPP=false" & set "LANG_RUST=false" )
if /i "%~1"=="--python" ( set "LANG_CPP=false" & set "LANG_RUST=false" )
if /i "%~1"=="--cpp" ( set "LANG_PY=false" & set "LANG_RUST=false" )
if /i "%~1"=="--rust" ( set "LANG_PY=false" & set "LANG_CPP=false" )
goto :parse_args
:done_args

set "BATCH_DIR=%~dp0"
for %%i in ("%BATCH_DIR%..") do set "REPO_ROOT=%%~fi"
set "PYTHON=%REPO_ROOT%\uvsession\Scripts\python.exe"

echo === Quick Test: %CLASS_NAME% ===

REM Python (instant)
if "%LANG_PY%"=="true" (
    echo [py] Running %CLASS_NAME%_test...
    "%PYTHON%" -m "session_py.%CLASS_NAME%_test"
)

REM Rust (fast incremental)
if "%LANG_RUST%"=="true" (
    echo [rust] Running %CLASS_NAME% minitest...
    pushd "%REPO_ROOT%\session_rust"
    cargo run --release --features protobuf --bin minitest
    popd
)

REM C++ (slower)
if "%LANG_CPP%"=="true" (
    echo [cpp] Building and running %CLASS_NAME% minitest...
    pushd "%REPO_ROOT%\session_cpp"
    cmake --build build --config Release --parallel 4
    if exist "build\Release\point_minitest.exe" (
        "build\Release\point_minitest.exe"
    )
    popd
)

echo === Done ===
endlocal
