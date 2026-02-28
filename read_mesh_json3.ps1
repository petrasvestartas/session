$json = Get-Content 'C:\pc\3_code\code_rust\session\session_tests\session_cpp\mesh_test.json' -Raw | ConvertFrom-Json
foreach ($test in $json) {
    $status = if ($test.passed) { "PASS" } else { "FAIL" }
    Write-Output "$status  $($test.test_name)"
    if (-not $test.passed -and $test.failures) {
        foreach ($f in $test.failures) {
            Write-Output "     -> $($f.error)"
        }
    }
}
