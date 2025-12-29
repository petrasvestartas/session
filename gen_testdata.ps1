$testDir = "c:\rust\session\session_tests"
$output = "$testDir\public\testData.js"

"// Auto-generated test data" | Out-File $output -Encoding utf8
"window.TEST_DATA = {" | Add-Content $output

$classes = @('color','knot','line','mesh','nurbscurve','nurbssurface','plane','point','pointcloud','polyline','tolerance','vector','xform')
$langs = @(
    @{n='python';d='session_py'},
    @{n='cpp';d='session_cpp'},
    @{n='rust';d='session_rust'}
)

$first = $true
foreach ($c in $classes) {
    foreach ($l in $langs) {
        $f = Join-Path $testDir ($l.d + '\' + $c + '_test.json')
        if (Test-Path $f) {
            if (-not $first) { Add-Content $output ',' }
            $first = $false
            $json = Get-Content $f -Raw
            Add-Content $output ('  "' + $c + '_test_' + $l.n + '": ' + $json.TrimEnd())
        }
    }
}

Add-Content $output '};'
Copy-Item $output "$testDir\testData.js"
Write-Host "Generated testData.js with classes: $($classes -join ', ')"
