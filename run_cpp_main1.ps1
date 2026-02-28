Set-Location 'C:\pc\3_code\code_rust\session\session_cpp'
cmake --build build --config Release --target main_1 2>&1 | Select-Object -Last 3
if ($LASTEXITCODE -eq 0) {
    & '.\build\Release\main_1.exe'
} else {
    Write-Host "BUILD FAILED"
}
