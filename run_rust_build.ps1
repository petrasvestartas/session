Set-Location 'C:\pc\3_code\code_rust\session\session_rust'
cargo build --release --bin minitest 2>&1 | Out-File 'C:\pc\3_code\code_rust\session\rust_build.txt'
$exitCode = $LASTEXITCODE
Get-Content 'C:\pc\3_code\code_rust\session\rust_build.txt' | Where-Object { $_ -match '^error\[|^error:' } | Select-Object -First 5
if ($exitCode -eq 0) { Write-Host "Build OK" } else { Write-Host "Build FAILED" }
