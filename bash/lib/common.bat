@echo off
REM Shared functions for minitest system (Windows)
REM This file is meant to be called from other batch files

REM Single source of truth for class names
set "CLASS_NAMES=color nurbsknot line mesh nurbscurve nurbssurface plane point pointcloud polyline tolerance vector xform"

REM Get script directory (call with %~dp0)
REM Usage: call :get_repo_root SCRIPT_DIR RESULT_VAR
:get_repo_root
set "%~2=%~1.."
for %%i in ("%~1..") do set "%~2=%%~fi"
goto :eof

REM Check if port is in use
REM Usage: call :port_in_use PORT RESULT_VAR (sets to 1 if in use, 0 otherwise)
:port_in_use
set "%~2=0"
netstat -ano 2>nul | findstr ":%~1" | findstr "LISTENING" >nul 2>&1
if not errorlevel 1 set "%~2=1"
goto :eof

REM Kill process on port
REM Usage: call :kill_port PORT
:kill_port
for /f "tokens=5" %%a in ('netstat -ano 2^>nul ^| findstr ":%~1" ^| findstr "LISTENING"') do (
    taskkill /F /PID %%a >nul 2>&1
)
goto :eof

REM Log with prefix
REM Usage: call :log MESSAGE
:log
echo [mini] %*
goto :eof

REM Log with language prefix
REM Usage: call :log_lang LANG MESSAGE
:log_lang
echo [%~1] %~2 %~3 %~4 %~5 %~6 %~7 %~8 %~9
goto :eof

REM Check if uv is available
REM Usage: call :has_uv RESULT_VAR (sets to 1 if available, 0 otherwise)
:has_uv
set "%~1=0"
where uv >nul 2>&1
if not errorlevel 1 set "%~1=1"
goto :eof

REM Get number of CPU cores
REM Usage: call :get_jobs RESULT_VAR
:get_jobs
for /f "tokens=2 delims==" %%i in ('wmic cpu get NumberOfLogicalProcessors /value 2^>nul ^| find "="') do set "%~1=%%i"
if not defined %~1 set "%~1=4"
goto :eof
