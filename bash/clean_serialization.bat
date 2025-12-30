@echo off
set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..

del /Q "%ROOT_DIR%\session_rust\serialization\*" 2>nul
del /Q "%ROOT_DIR%\session_py\serialization\*" 2>nul
del /Q "%ROOT_DIR%\session_cpp\serialization\*" 2>nul
