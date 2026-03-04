$content = Get-Content 'C:\pc\3_code\code_rust\session\session_tests\session_cpp\mesh_test.json' -Raw | ConvertFrom-Json
$fails = $content | Where-Object { $_.passed -eq $false }
if ($fails) {
    Write-Host 'FAILURES:'
    $fails | ForEach-Object { Write-Host ('  - ' + $_.test_name) }
} else {
    Write-Host ('All ' + $content.Count + ' mesh tests pass')
}
