@echo off
REM Windows equivalent of git_push.sh

setlocal enabledelayedexpansion

if "%~1"=="" (
    echo Error: Commit message required
    echo Usage: git_push.bat "your commit message"
    exit /b 1
)

set "MSG=%~1"

REM Resolve repository root (parent of script directory)
set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"
for %%i in ("%SCRIPT_DIR%\..") do set "REPO_ROOT=%%~fi"
cd /d "%REPO_ROOT%"

for %%d in (session_cpp session_py session_rust session_data session_proto) do (
    if exist "%%d" (
        echo.
        echo Repository: %%d
        pushd "%%d"
        git add -A
        git commit -m "%MSG%" 2>nul || echo Nothing to commit in %%d
        git push -f 2>nul && echo %%d updated
        popd
    ) else (
        echo %%d not found, skipping
    )
)

echo.
echo Repository: main repo (this repository)
git add -A
git commit -m "%MSG%" 2>nul || echo Nothing to commit in main repo
git push 2>nul && echo main repo updated

endlocal
