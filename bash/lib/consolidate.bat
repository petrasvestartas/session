@echo off
REM Consolidate all existing JSON test results into testData.js
REM This script ONLY reads - it never deletes source JSON files

setlocal enabledelayedexpansion

REM Get repo root
set "SCRIPT_DIR=%~dp0"
for %%i in ("%SCRIPT_DIR%..") do set "BASH_DIR=%%~fi"
for %%i in ("%BASH_DIR%\..") do set "REPO_ROOT=%%~fi"

set "TESTS_DIR=%REPO_ROOT%\session_tests"
set "OUTPUT=%TESTS_DIR%\public\testData.js"

if not exist "%TESTS_DIR%\public" mkdir "%TESTS_DIR%\public"

echo [mini] Consolidating JSON into testData.js...

REM Use PowerShell for reliable JSON concatenation
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
    "$testDir='%TESTS_DIR%';" ^
    "$repoRoot='%REPO_ROOT%';" ^
    "$output='%OUTPUT%';" ^
    "$classes=@('color','nurbsknot','line','mesh','nurbscurve','nurbssurface','plane','point','pointcloud','polyline','tolerance','vector','xform');" ^
    "$langs=@(@{n='python';d='session_py'},@{n='cpp';d='session_cpp'},@{n='rust';d='session_rust'});" ^
    "" ^
    "Set-Content $output '// Auto-generated test data - Do not edit manually';" ^
    "Add-Content $output ('// Generated at: ' + (Get-Date));" ^
    "Add-Content $output 'window.TEST_DATA = {';" ^
    "" ^
    "$first=$true;" ^
    "foreach($c in $classes){" ^
    "  foreach($l in $langs){" ^
    "    $f=Join-Path $testDir ($l.d+'\'+$c+'_test.json');" ^
    "    if(Test-Path $f){" ^
    "      if(-not $first){Add-Content $output ','}" ^
    "      $first=$false;" ^
    "      $json=Get-Content $f -Raw;" ^
    "      Add-Content $output -NoNewline ('  \"'+$c+'_test_'+$l.n+'\": '+$json.TrimEnd())" ^
    "    }" ^
    "  }" ^
    "}" ^
    "" ^
    "$protoDir=Join-Path $repoRoot 'session_proto';" ^
    "if(Test-Path $protoDir){" ^
    "  Get-ChildItem $protoDir -Filter '*.proto' | ForEach-Object {" ^
    "    $name=$_.BaseName;" ^
    "    $content=[IO.File]::ReadAllText($_.FullName);" ^
    "    $content=$content.Replace('\','\\').Replace('\"','\\\"');" ^
    "    $content=$content.Replace(\"`r`n\",'\\n').Replace(\"`n\",'\\n').TrimEnd();" ^
    "    Add-Content $output ',';" ^
    "    Add-Content $output -NoNewline ('  \"proto_'+$name+'\": \"'+$content+'\"')" ^
    "  }" ^
    "}" ^
    "" ^
    "Add-Content $output '';" ^
    "Add-Content $output '};'"

REM Copy to root for backward compatibility
copy "%OUTPUT%" "%TESTS_DIR%\testData.js" >nul

echo [mini] testData.js updated

endlocal
