Get-ChildItem 'C:\pc\3_code\code_rust\session\session_tests\session_cpp\*.json' | ForEach-Object {
    $content = Get-Content $_.FullName -Raw | ConvertFrom-Json
    $fails = $content | Where-Object { $_.passed -eq $false }
    if ($fails) {
        Write-Host ('FAIL: ' + $_.Name)
        $fails | ForEach-Object { Write-Host ('  - ' + $_.test_name) }
    }
}
Write-Host 'Done checking'
