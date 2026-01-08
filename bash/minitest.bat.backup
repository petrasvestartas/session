@echo off
REM Windows equivalent of minitest.sh
REM Run mini tests for Python, C++, and Rust implementations.
REM Usage:
REM   minitest.bat          - Full build (all languages)
REM   minitest.bat --fast   - Skip protobuf regen, pip install, npm install
REM   minitest.bat --py     - Python only
REM   minitest.bat --cpp    - C++ only
REM   minitest.bat --rust   - Rust only
REM   minitest.bat --kill   - Stop running dev server

setlocal enabledelayedexpansion

set "FAST_MODE=false"
set "RUN_PYTHON=true"
set "RUN_CPP=true"
set "RUN_RUST=true"
set "KILL_SERVER=false"

REM Check if uv is available globally (use where for reliable detection)
set "HAS_UV=0"
where uv >nul 2>&1
if not errorlevel 1 set "HAS_UV=1"

REM Parse arguments
:parse_args
if "%~1"=="" goto :done_args
if /i "%~1"=="--fast" set "FAST_MODE=true"
if /i "%~1"=="-f" set "FAST_MODE=true"
if /i "%~1"=="--py" (
    set "RUN_CPP=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--python" (
    set "RUN_CPP=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--cpp" (
    set "RUN_PYTHON=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--c++" (
    set "RUN_PYTHON=false"
    set "RUN_RUST=false"
)
if /i "%~1"=="--rust" (
    set "RUN_PYTHON=false"
    set "RUN_CPP=false"
)
if /i "%~1"=="--rs" (
    set "RUN_PYTHON=false"
    set "RUN_CPP=false"
)
if /i "%~1"=="--kill" set "KILL_SERVER=true"
if /i "%~1"=="-k" set "KILL_SERVER=true"
shift
goto :parse_args
:done_args

REM Handle --kill flag to stop running server
if "%KILL_SERVER%"=="true" (
    echo [mini] Stopping dev server on port 8769...
    for /f "tokens=5" %%a in ('netstat -ano ^| findstr ":8769" ^| findstr "LISTENING" 2^>nul') do (
        taskkill /F /PID %%a >nul 2>&1
    )
    echo [mini] Server stopped.
    goto :end
)

REM Resolve repository root - try multiple methods
set "BATCH_PATH=%~f0"
for %%i in ("%BATCH_PATH%") do set "BATCH_DIR=%%~dpi"
for %%i in ("%BATCH_DIR%..") do set "REPO_ROOT=%%~fi"

REM Verify REPO_ROOT by checking for session_py folder
if not exist "%REPO_ROOT%\session_py" (
    REM Try going up one more level
    for %%i in ("%REPO_ROOT%\..") do set "REPO_ROOT=%%~fi"
)
if not exist "%REPO_ROOT%\session_py" (
    REM Fallback: check common locations
    if exist "C:\rust\session\session_py" set "REPO_ROOT=C:\rust\session"
)
set "TESTS_DIR=%REPO_ROOT%\session_tests"
set "VENV_DIR=%REPO_ROOT%\uvsession"
set "PYTHON=%VENV_DIR%\Scripts\python.exe"

REM Class names for tests (sorted alphabetically)
set "CLASS_NAMES=color knot line mesh nurbscurve nurbssurface plane point pointcloud polyline tolerance vector xform"

REM Remove old JSON test result files
echo [mini] Cleaning up old JSON test files...
for %%c in (%CLASS_NAMES%) do (
    if exist "%TESTS_DIR%\session_py\%%c_test.json" del "%TESTS_DIR%\session_py\%%c_test.json"
    if exist "%TESTS_DIR%\session_cpp\%%c_test.json" del "%TESTS_DIR%\session_cpp\%%c_test.json"
    if exist "%TESTS_DIR%\session_rust\%%c_test.json" del "%TESTS_DIR%\session_rust\%%c_test.json"
)

REM Regenerate Python protobuf bindings (skip in fast mode)
if "%FAST_MODE%"=="false" (
    call :regenerate_python_protos
)

REM Run Python tests
if "%RUN_PYTHON%"=="true" (
    call :ensure_python_env
    if errorlevel 1 (
        echo [mini] Skipping Python mini tests due to environment setup failure.
    ) else (
        for %%c in (%CLASS_NAMES%) do (
            echo [mini] Running Python %%c mini tests...
            "%PYTHON%" -m "session_py.%%c_test"
        )
    )
)

REM Run C++ tests
set "CPP_DIR=%REPO_ROOT%\session_cpp"
if not "%RUN_CPP%"=="true" goto :skip_cpp

REM Detect number of CPU cores for parallel build
for /f "tokens=2 delims==" %%i in ('wmic cpu get NumberOfLogicalProcessors /value 2^>nul ^| find "="') do set "NUM_CORES=%%i"
if not defined NUM_CORES set "NUM_CORES=4"
echo [mini] Building C++ project (session_cpp) with %NUM_CORES% parallel jobs...

if not exist "%CPP_DIR%" (
    echo [mini] session_cpp directory not found at %CPP_DIR%
    goto :skip_cpp
)
pushd "%CPP_DIR%"
echo [mini] Configuring C++ with CMake...
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
if errorlevel 1 (
    echo [mini] C++ CMake configuration failed.
    popd
    exit /b 1
)
echo [mini] Building C++ with %NUM_CORES% cores...
cmake --build build --config Release --parallel %NUM_CORES%
if errorlevel 1 (
    echo [mini] C++ build failed.
    popd
    exit /b 1
)
popd
echo [mini] Running C++ mini tests...
if exist "%CPP_DIR%\build\Release\point_minitest.exe" (
    pushd "%CPP_DIR%"
    "build\Release\point_minitest.exe"
    popd
) else if exist "%CPP_DIR%\build\point_minitest.exe" (
    pushd "%CPP_DIR%"
    "build\point_minitest.exe"
    popd
) else (
    echo [mini] C++ executable 'point_minitest.exe' not found.
)
:skip_cpp

REM Run Rust tests
set "RUST_DIR=%REPO_ROOT%\session_rust"
if not "%RUN_RUST%"=="true" goto :skip_rust
echo [mini] Building and running Rust mini tests...
if not exist "%RUST_DIR%" (
    echo [mini] session_rust directory not found at %RUST_DIR%
    goto :skip_rust
)

REM Auto-detect protoc for Rust build if not already set
if defined PROTOC goto :protoc_ready
where protoc >nul 2>&1
if not errorlevel 1 (
    set "PROTOC=protoc"
    goto :protoc_ready
)
if exist "%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe" (
    set "PROTOC=%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe"
    echo [mini] Using protoc from C++ build
    goto :protoc_ready
)
echo [mini] Warning: protoc not found. Rust build may fail.
echo [mini] Build C++ first or install protoc.
:protoc_ready

pushd "%RUST_DIR%"
cargo run --release --features protobuf --bin minitest -j %NUM_CORES%
if errorlevel 1 (
    echo [mini] Rust minitest failed.
    popd
)
popd
:skip_rust

echo [mini] Done. JSON results:
echo   Point     Python: %TESTS_DIR%\session_py\point_test.json
echo             C++   : %TESTS_DIR%\session_cpp\point_test.json
echo             Rust  : %TESTS_DIR%\session_rust\point_test.json

echo [mini] Generating consolidated testData.js...
call :generate_test_data_js

REM Setup and build Vue.js application
echo [mini] Setting up Vue.js application...
call :ensure_node_env
if errorlevel 1 (
    echo [mini] Skipping Vue.js application due to missing Node.js/npm.
    goto :end
)

echo [mini] Building Vue application...
pushd "%TESTS_DIR%"
call npm run build
if not errorlevel 1 goto :vue_build_done
echo [mini] Vue build failed. Attempting reinstall...
if exist "node_modules" rmdir /s /q "node_modules"
if exist "package-lock.json" del "package-lock.json"
call npm install
call npm run build
if errorlevel 1 (
    echo [mini] Failed to build Vue application.
    popd
    goto :end
)
:vue_build_done
popd

REM Check for CI environment
if defined CI goto :ci_exit
if defined GITHUB_ACTIONS goto :ci_exit

REM Local development: start dev server
set "PORT=8769"
set "WEBSITE_URL=http://localhost:%PORT%/session/tests?suite=point_test"

REM Check if dev server is already running - if so, skip restart (Vite hot-reloads)
set "SERVER_RUNNING=0"
netstat -ano 2>nul | findstr ":%PORT%" | findstr "LISTENING" >nul 2>&1
if not errorlevel 1 set "SERVER_RUNNING=1"

if "%SERVER_RUNNING%"=="1" (
    echo [mini] Dev server already running on port %PORT% - skipping restart.
    echo [mini] Vite will hot-reload changes automatically.
    echo [mini] View at: %WEBSITE_URL%
    goto :end
)

echo [mini] Starting development server...
pushd "%TESTS_DIR%"
start /b npm run dev >nul 2>&1
popd
timeout /t 2 /nobreak >nul

echo [mini] Development server started at %WEBSITE_URL%
echo [mini] Re-run minitest.bat to update test results (browser auto-refreshes).
start "" "%WEBSITE_URL%"
goto :end

:ci_exit
echo [mini] CI environment detected - skipping dev server.
echo [mini] Build complete.
goto :end

REM === SUBROUTINES ===

:regenerate_python_protos
set "PROTO_DIR=%REPO_ROOT%\session_proto"
set "PY_PROTO_OUT=%REPO_ROOT%\session_py\src\session_py\proto"

REM Find protoc
set "PROTOC="
where protoc >nul 2>&1
if not errorlevel 1 (
    set "PROTOC=protoc"
    goto :found_protoc
)
if exist "%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe" (
    set "PROTOC=%REPO_ROOT%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe"
    goto :found_protoc
)
echo [mini] Warning: protoc not found, skipping Python protobuf regeneration
exit /b 0

:found_protoc

if not exist "%PROTO_DIR%" (
    echo [mini] Warning: %PROTO_DIR% not found, skipping Python protobuf regeneration
    exit /b 0
)

if not exist "%PY_PROTO_OUT%" mkdir "%PY_PROTO_OUT%"

echo [mini] Regenerating Python protobuf bindings from session_proto/...
for %%f in ("%PROTO_DIR%\*.proto") do (
    "%PROTOC%" --python_out="%PY_PROTO_OUT%" -I"%PROTO_DIR%" "%%f" 2>nul
)

echo [mini] Fixing Python protobuf imports to use relative imports...
for %%f in ("%PY_PROTO_OUT%\*_pb2.py") do (
    if exist "%%f" (
        powershell -Command "(Get-Content '%%f') -replace '^import ([a-z_]*_pb2) as', 'from . import $1 as' | Set-Content '%%f'" 2>nul
    )
)
exit /b 0

:ensure_python_env
REM Check if venv already exists
if exist "%PYTHON%" goto :check_deps

REM Create venv
if "%HAS_UV%"=="1" (
    echo [mini] Creating Python environment with uv at %VENV_DIR%...
    pushd "%REPO_ROOT%"
    uv venv uvsession
    popd
) else (
    where python >nul 2>&1
    if errorlevel 1 (
        echo [mini] Python not found. Please install Python or uv first.
        exit /b 1
    )
    echo [mini] Creating Python environment at %VENV_DIR%...
    pushd "%REPO_ROOT%"
    python -m venv uvsession
    popd
)

if not exist "%PYTHON%" (
    echo [mini] Failed to create Python environment
    exit /b 1
)

:check_deps
REM Check if session_py already installed (fast mode)
if not "%FAST_MODE%"=="true" goto :install_deps
"%PYTHON%" -c "import session_py" >nul 2>&1
if not errorlevel 1 (
    echo [mini] Fast mode: session_py already installed
    exit /b 0
)

:install_deps
REM Install dependencies using uv if available, otherwise pip
if "%HAS_UV%"=="1" (
    echo [mini] Installing session_py and pytest with uv...
    uv pip install --python "%PYTHON%" -e "%REPO_ROOT%\session_py" pytest
) else (
    REM Check if pip exists in venv, if not try to bootstrap it
    "%PYTHON%" -m pip --version >nul 2>&1
    if errorlevel 1 (
        echo [mini] pip not found in venv, trying to bootstrap...
        "%PYTHON%" -m ensurepip --default-pip 2>nul
        if errorlevel 1 (
            echo [mini] Error: Cannot install dependencies. Please install uv: pip install uv
            exit /b 1
        )
    )
    echo [mini] Installing session_py and pytest with pip...
    "%PYTHON%" -m pip install -e "%REPO_ROOT%\session_py" pytest
)
exit /b 0

:ensure_node_env
where npm >nul 2>&1
if errorlevel 1 (
    echo [mini] npm not found. Please install Node.js first.
    exit /b 1
)

where node >nul 2>&1
if errorlevel 1 (
    echo [mini] node not found. Please install Node.js first.
    exit /b 1
)

for /f "tokens=*" %%v in ('node --version') do echo [mini] Found Node.js %%v

if not exist "%TESTS_DIR%\package.json" (
    echo [mini] No package.json found in session_tests
    exit /b 1
)

if "%FAST_MODE%"=="true" (
    if exist "%TESTS_DIR%\node_modules" (
        echo [mini] Fast mode: Skipping npm install (node_modules exists)
        exit /b 0
    )
)

echo [mini] Ensuring npm dependencies are installed...
pushd "%TESTS_DIR%"
call npm install --silent
popd
exit /b 0

:generate_test_data_js
if not exist "%TESTS_DIR%\public" mkdir "%TESTS_DIR%\public"
set "OUTPUT=%TESTS_DIR%\public\testData.js"

echo // Auto-generated test data - Do not edit manually > "%OUTPUT%"
echo // Generated at: %DATE% %TIME% >> "%OUTPUT%"
echo window.TEST_DATA = { >> "%OUTPUT%"

REM Use PowerShell to generate testData.js properly
powershell -NoProfile -ExecutionPolicy Bypass -Command "$testDir='%TESTS_DIR%';$output='%OUTPUT%';$classes=@('color','knot','line','mesh','nurbscurve','nurbssurface','plane','point','pointcloud','polyline','tolerance','vector','xform');$langs=@(@{n='python';d='session_py'},@{n='cpp';d='session_cpp'},@{n='rust';d='session_rust'});$first=$true;foreach($c in $classes){foreach($l in $langs){$f=Join-Path $testDir ($l.d+'\'+$c+'_test.json');if(Test-Path $f){if(-not $first){Add-Content $output ','}$first=$false;$json=Get-Content $f -Raw;Add-Content $output ('  \"'+$c+'_test_'+$l.n+'\": '+$json.TrimEnd())}}}"

REM Add proto schemas via PowerShell (escape for JSON string like shell script)
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$protoDir='%REPO_ROOT%\session_proto';" ^
  "$output='%OUTPUT%';" ^
  "Get-ChildItem $protoDir -Filter '*.proto' | ForEach-Object {" ^
  "  $name=$_.BaseName;" ^
  "  $content=[IO.File]::ReadAllText($_.FullName);" ^
  "  $content=$content.Replace('\','\\').Replace('\"','\\\"');" ^
  "  $content=$content.Replace(\"`r`n\",'\\n').Replace(\"`n\",'\\n');" ^
  "  $content=$content.TrimEnd();" ^
  "  Add-Content $output (',  \"proto_'+$name+'\": \"'+$content+'\"')" ^
  "}"

echo. >> "%OUTPUT%"
echo }; >> "%OUTPUT%"

echo [mini] testData.js generated at %OUTPUT%
copy "%OUTPUT%" "%TESTS_DIR%\testData.js" >nul
exit /b 0

:end
endlocal
