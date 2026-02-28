$exe = 'C:\pc\3_code\code_rust\session\session_cpp\build\Release\point_minitest.exe'
$psi = New-Object System.Diagnostics.ProcessStartInfo($exe)
$psi.RedirectStandardOutput = $true
$psi.RedirectStandardError = $true
$psi.UseShellExecute = $false
$psi.CreateNoWindow = $true
$psi.WorkingDirectory = 'C:\pc\3_code\code_rust\session\session_cpp'
$p = [System.Diagnostics.Process]::Start($psi)
$stdout = $p.StandardOutput.ReadToEnd()
$stderr = $p.StandardError.ReadToEnd()
$p.WaitForExit()
Write-Output ("EXIT=" + $p.ExitCode)
Write-Output ("STDOUT_LEN=" + $stdout.Length)
Write-Output ("STDERR_LEN=" + $stderr.Length)
if ($stdout.Length -gt 0) { $stdout -split "`n" | Select-Object -First 80 }
if ($stderr.Length -gt 0) { Write-Output "=STDERR="; $stderr -split "`n" | Select-Object -First 50 }
