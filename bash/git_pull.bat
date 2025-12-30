@echo off
REM Windows equivalent of git_pull.sh

REM Resolve repository root (parent of script directory)
set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"
for %%i in ("%SCRIPT_DIR%\..") do set "REPO_ROOT=%%~fi"
cd /d "%REPO_ROOT%"

echo Updating main repo...
git pull

echo Updating submodules (init + recursive)...
git submodule update --init --recursive

echo Checking out main branch in all submodules...
git submodule foreach "git checkout main 2>nul || git checkout -b main"

echo Done. Main repo and submodules are up to date on main branch.
