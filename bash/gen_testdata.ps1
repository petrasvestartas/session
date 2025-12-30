# Resolve repository root (parent of script directory)
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$testDir = Join-Path $repoRoot "session_tests"
$output = Join-Path $testDir "public\testData.js"

# Create public directory if needed
$publicDir = Join-Path $testDir "public"
if (-not (Test-Path $publicDir)) {
    New-Item -ItemType Directory -Path $publicDir | Out-Null
}

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
Copy-Item $output (Join-Path $testDir "testData.js")
Write-Host "Generated testData.js with classes: $($classes -join ', ')"
