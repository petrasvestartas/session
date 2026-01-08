@echo off
REM Manage Vue dev server lifecycle (Windows)

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
for %%i in ("%SCRIPT_DIR%..") do set "BASH_DIR=%%~fi"
for %%i in ("%BASH_DIR%\..") do set "REPO_ROOT=%%~fi"

set "TESTS_DIR=%REPO_ROOT%\session_tests"
set "PORT=8769"
set "URL=http://localhost:%PORT%/session/tests?suite=point_test"

if "%~1"=="start" goto :start_server
if "%~1"=="stop" goto :stop_server
if "%~1"=="restart" goto :restart_server
echo Usage: server.bat {start^|stop^|restart} [--fast]
exit /b 1

:start_server
REM Check if already running
set "SERVER_RUNNING=0"
netstat -ano 2>nul | findstr ":%PORT%" | findstr "LISTENING" >nul 2>&1
if not errorlevel 1 set "SERVER_RUNNING=1"

if "%SERVER_RUNNING%"=="1" (
    echo [mini] Dev server already running on port %PORT%
    echo [mini] Vite will hot-reload changes automatically
    echo [mini] View at: %URL%
    goto :eof
)

REM Ensure npm deps
call :ensure_node_deps "%~2"
if errorlevel 1 exit /b 1

echo [mini] Starting development server...
pushd "%TESTS_DIR%"
start /b npm run dev >nul 2>&1
popd
timeout /t 2 /nobreak >nul

echo [mini] Server started at %URL%
start "" "%URL%"
goto :eof

:stop_server
set "SERVER_RUNNING=0"
netstat -ano 2>nul | findstr ":%PORT%" | findstr "LISTENING" >nul 2>&1
if not errorlevel 1 set "SERVER_RUNNING=1"

if "%SERVER_RUNNING%"=="0" (
    echo [mini] No server running on port %PORT%
    goto :eof
)

echo [mini] Stopping dev server on port %PORT%...
for /f "tokens=5" %%a in ('netstat -ano 2^>nul ^| findstr ":%PORT%" ^| findstr "LISTENING"') do (
    taskkill /F /PID %%a >nul 2>&1
)
echo [mini] Server stopped
goto :eof

:restart_server
call :stop_server
timeout /t 1 /nobreak >nul
call :start_server "%~2"
goto :eof

:ensure_node_deps
set "FAST_MODE=%~1"

where npm >nul 2>&1
if errorlevel 1 (
    echo [mini] npm not found. Please install Node.js first.
    exit /b 1
)

if "%FAST_MODE%"=="--fast" (
    if exist "%TESTS_DIR%\node_modules" (
        echo [mini] Fast mode: Skipping npm install
        exit /b 0
    )
)

echo [mini] Installing npm dependencies...
pushd "%TESTS_DIR%"
call npm install --silent
if errorlevel 1 (
    echo [mini] npm install failed, retrying...
    if exist "node_modules" rmdir /s /q "node_modules"
    if exist "package-lock.json" del "package-lock.json"
    call npm install
    if errorlevel 1 (
        popd
        exit /b 1
    )
)
popd
exit /b 0

endlocal
