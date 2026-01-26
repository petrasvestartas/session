$output = "public/testData.js"
$testDir = "."
$classes = @('color','line','mesh','nurbscurve','nurbssurface','plane','point','pointcloud','polyline','tolerance','vector','xform')
$langs = @(@{n='python';d='session_py'},@{n='cpp';d='session_cpp'},@{n='rust';d='session_rust'})

"// Auto-generated test data" | Out-File $output
"window.TEST_DATA = {" | Out-File $output -Append

$first = $true
foreach ($c in $classes) {
    foreach ($l in $langs) {
        $f = Join-Path $testDir ("$($l.d)/$($c)_test.json")
        if (Test-Path $f) {
            if (-not $first) {
                "," | Out-File $output -Append -NoNewline
            }
            $first = $false
            $json = Get-Content $f -Raw
            "  `"$($c)_test_$($l.n)`": $($json.TrimEnd())" | Out-File $output -Append -NoNewline
        }
    }
}

"`n};" | Out-File $output -Append
