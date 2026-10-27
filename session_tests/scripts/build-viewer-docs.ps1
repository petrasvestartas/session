$ErrorActionPreference = 'Stop'
$sessionRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$viewer = Join-Path $sessionRoot 'session_viewer'
$docs = Join-Path $viewer 'target\docs'
$site = Split-Path -Parent $PSScriptRoot

function Test-Stale {
    $stamp = Join-Path $docs 'vue\index.html'
    if (![IO.File]::Exists($stamp)) { return $true }
    $built = [IO.File]::GetLastWriteTimeUtc($stamp)
    $sources = @(
        "$site\src", "$site\plugins", "$site\public", "$site\index.html", "$site\vite.config.ts",
        "$site\package-lock.json", "$site\scripts\build-viewer-docs.sh", "$PSCommandPath",
        "$viewer\docs", "$viewer\ARCHITECTURE.md", "$viewer\mkdocs.yml",
        "$sessionRoot\session_rust\src", "$sessionRoot\session_cpp\src", "$sessionRoot\session_proto"
    )
    $pending = New-Object 'Collections.Generic.Stack[IO.DirectoryInfo]'
    foreach ($source in $sources) {
        if ([IO.File]::Exists($source)) {
            if ([IO.File]::GetLastWriteTimeUtc($source) -gt $built) { return $true }
        } elseif ([IO.Directory]::Exists($source)) {
            $pending.Push((New-Object IO.DirectoryInfo $source))
        }
    }
    while ($pending.Count) {
        foreach ($entry in $pending.Pop().EnumerateFileSystemInfos()) {
            if ($entry.Name -cin @('target', 'node_modules', '__pycache__', '.git')) { continue }
            if ($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) { continue }
            if ($entry.Attributes -band [IO.FileAttributes]::Directory) { $pending.Push($entry) }
            elseif ($entry.LastWriteTimeUtc -gt $built) { return $true }
        }
    }
    return $false
}

# Git's find stat() calls are costly on NTFS; check the same inputs with native directory metadata.
if (!(Test-Stale)) { exit 0 }
$gitDirectory = Split-Path -Parent (Get-Command git.exe -CommandType Application -ErrorAction Stop | Select-Object -First 1).Source
$bash = @(
    (Join-Path $gitDirectory 'bash.exe'),
    (Join-Path $gitDirectory '..\..\bin\bash.exe'),
    (Join-Path $gitDirectory '..\bin\bash.exe')
) | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
if (!$bash) { throw "Git Bash not found beside git.exe in $gitDirectory" }
$bash = (Resolve-Path -LiteralPath $bash).Path
$script = (Join-Path $PSScriptRoot 'build-viewer-docs.sh').Replace('\', '/')
[IO.Directory]::CreateDirectory($docs) | Out-Null
$lockPath = Join-Path $docs 'vue-build.lock'
$lock = $null
try {
    while ($null -eq $lock) {
        try {
            # Keep sharing exclusive, but let Bash retain the lock if this wrapper exits first.
            $lock = [IO.File]::Open($lockPath, [IO.FileMode]::OpenOrCreate, [IO.FileAccess]::ReadWrite, [IO.FileShare]::Inheritable)
        } catch [IO.IOException] {
            $nativeCode = $_.Exception.GetBaseException().HResult -band 0xFFFF
            if ($nativeCode -ne 32 -and $nativeCode -ne 33) { throw }
            Start-Sleep -Milliseconds 100
        }
    }
    $buildExit = 0
    if (Test-Stale) {
        & $bash $script --locked
        $buildExit = $LASTEXITCODE
    }
} finally {
    if ($null -ne $lock) { $lock.Dispose() }
}
exit $buildExit
