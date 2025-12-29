@echo off
REM Windows equivalent of generate_python_proto.sh
REM Generate Python protobuf files from all .proto files in session_proto/

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"
set "PROTO_DIR=%SCRIPT_DIR%\session_proto"
set "PYTHON_PROTO_DIR=%SCRIPT_DIR%\session_py\src\session_py\proto"

REM Find protoc - check system PATH first, then C++ build location
set "PROTOC="
where protoc >nul 2>&1
if %errorlevel%==0 (
    set "PROTOC=protoc"
) else if exist "%SCRIPT_DIR%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe" (
    set "PROTOC=%SCRIPT_DIR%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\Release\protoc.exe"
) else if exist "%SCRIPT_DIR%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\protoc.exe" (
    set "PROTOC=%SCRIPT_DIR%\session_cpp\build\protobuf_external-prefix\src\protobuf_external-build\protoc.exe"
) else if exist "%USERPROFILE%\.local\install\protobuf\bin\protoc.exe" (
    set "PROTOC=%USERPROFILE%\.local\install\protobuf\bin\protoc.exe"
)

if "%PROTOC%"=="" (
    echo Error: protoc not found. Build the C++ project first or install protobuf.
    exit /b 1
)

echo Using protoc: %PROTOC%
echo Proto source: %PROTO_DIR%
echo Python output: %PYTHON_PROTO_DIR%

REM Create output directory if needed
if not exist "%PYTHON_PROTO_DIR%" mkdir "%PYTHON_PROTO_DIR%"

REM Generate Python files for all .proto files
for %%f in ("%PROTO_DIR%\*.proto") do (
    echo Generating %%~nf_pb2.py...
    "%PROTOC%" --python_out="%PYTHON_PROTO_DIR%" -I"%PROTO_DIR%" "%%f"
    if errorlevel 1 (
        echo Warning: Failed to generate %%~nf_pb2.py
    )
)

REM Fix imports: convert absolute imports to relative imports for package use
echo Fixing imports for package use...
for %%f in ("%PYTHON_PROTO_DIR%\*_pb2.py") do (
    if exist "%%f" (
        powershell -Command "(Get-Content '%%f') -replace '^import ([a-z_]*_pb2) as', 'from . import $1 as' | Set-Content '%%f'"
    )
)

echo Done! Generated Python protobuf files in %PYTHON_PROTO_DIR%

endlocal
