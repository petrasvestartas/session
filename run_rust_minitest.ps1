Set-Location 'C:\pc\3_code\code_rust\session\session_rust'
cargo run --release --bin minitest 2>&1 | Out-File 'C:\pc\3_code\code_rust\session\rust_minitest.txt'
$exitCode = $LASTEXITCODE
Write-Host "Exit: $exitCode"
Get-Content 'C:\pc\3_code\code_rust\session\rust_minitest.txt' | Select-Object -Last 10
