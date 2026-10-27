param(
    [int]$TimeoutSeconds = 600,
    [ValidateRange(256, 65536)] [int]$MemoryLimitMiB = 2048,
    [string]$ManifestPath = (Join-Path (Get-Location) 'guarded-job.json'),
    [string]$WorkingDirectory = (Get-Location).Path,
    [string]$CommandFile,
    [Parameter(Position = 0)] [string]$Executable,
    [Parameter(Position = 1, ValueFromRemainingArguments = $true)] [string[]]$ArgumentList
)

$ErrorActionPreference = 'Stop'
if ($CommandFile) {
    $commandSpec = [IO.File]::ReadAllText((Resolve-Path -LiteralPath $CommandFile).Path) | ConvertFrom-Json
    $Executable = [string]$commandSpec.executable
    $ArgumentList = @($commandSpec.arguments | ForEach-Object { [string]$_ })
}
if (-not $Executable) { throw 'Choose an executable or a command JSON file.' }
if ($TimeoutSeconds -lt 1 -or $TimeoutSeconds -gt 2147483) { throw 'TimeoutSeconds must be between 1 and 2147483.' }
$program = (Get-Command $Executable -CommandType Application -ErrorAction Stop | Select-Object -First 1).Source
$directory = (Resolve-Path -LiteralPath $WorkingDirectory).Path
$manifest = [IO.Path]::GetFullPath($ManifestPath)

Add-Type -TypeDefinition @'
using System;
using System.ComponentModel;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;

public static class GuardedJob {
    [StructLayout(LayoutKind.Sequential)] struct IO_COUNTERS {
        public ulong ReadOperationCount, WriteOperationCount, OtherOperationCount;
        public ulong ReadTransferCount, WriteTransferCount, OtherTransferCount;
    }
    [StructLayout(LayoutKind.Sequential)] struct BASIC_LIMIT {
        public long PerProcessUserTimeLimit, PerJobUserTimeLimit;
        public uint LimitFlags;
        public UIntPtr MinimumWorkingSetSize, MaximumWorkingSetSize;
        public uint ActiveProcessLimit;
        public UIntPtr Affinity;
        public uint PriorityClass, SchedulingClass;
    }
    [StructLayout(LayoutKind.Sequential)] struct EXTENDED_LIMIT {
        public BASIC_LIMIT BasicLimitInformation;
        public IO_COUNTERS IoInfo;
        public UIntPtr ProcessMemoryLimit, JobMemoryLimit, PeakProcessMemoryUsed, PeakJobMemoryUsed;
    }
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)] struct STARTUPINFO {
        public uint cb;
        public string lpReserved, lpDesktop, lpTitle;
        public uint dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags;
        public ushort wShowWindow, cbReserved2;
        public IntPtr lpReserved2, hStdInput, hStdOutput, hStdError;
    }
    [StructLayout(LayoutKind.Sequential)] struct PROCESS_INFORMATION {
        public IntPtr hProcess, hThread;
        public uint dwProcessId, dwThreadId;
    }
    [StructLayout(LayoutKind.Sequential)] struct ACCOUNTING {
        public long TotalUserTime, TotalKernelTime, ThisPeriodTotalUserTime, ThisPeriodTotalKernelTime;
        public uint TotalPageFaultCount, TotalProcesses, ActiveProcesses, TotalTerminatedProcesses;
    }
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)] static extern IntPtr CreateJobObject(IntPtr attributes, string name);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool SetInformationJobObject(IntPtr job, int infoClass, ref EXTENDED_LIMIT info, uint size);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool QueryInformationJobObject(IntPtr job, int infoClass, ref EXTENDED_LIMIT info, uint size, IntPtr length);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool QueryInformationJobObject(IntPtr job, int infoClass, ref ACCOUNTING info, uint size, IntPtr length);
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)] static extern bool CreateProcess(string application, StringBuilder command, IntPtr processAttributes, IntPtr threadAttributes, bool inherit, uint flags, IntPtr environment, string directory, ref STARTUPINFO startup, out PROCESS_INFORMATION process);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
    [DllImport("kernel32.dll", SetLastError = true)] static extern uint ResumeThread(IntPtr thread);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool TerminateJobObject(IntPtr job, uint exitCode);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool TerminateProcess(IntPtr process, uint exitCode);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool GetExitCodeProcess(IntPtr process, out uint exitCode);
    [DllImport("kernel32.dll", SetLastError = true)] static extern uint WaitForSingleObject(IntPtr handle, uint milliseconds);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool CloseHandle(IntPtr handle);
    [DllImport("kernel32.dll")] static extern IntPtr GetStdHandle(int which);

    public sealed class Result {
        public int exitCode;
        public bool timedOut;
        public double elapsedSeconds;
        public ulong peakJobCommitBytes;
        public ulong affinityMask;
        public int processors;
        public uint processId;
        public uint descendantsAtCleanup;
    }
    static void Check(bool ok) { if (!ok) throw new Win32Exception(Marshal.GetLastWin32Error()); }
    static string Quote(string value) {
        if (value == null) value = "";
        StringBuilder s = new StringBuilder("\"");
        int slashes = 0;
        foreach (char c in value) {
            if (c == '\\') { slashes++; continue; }
            if (c == '"') { s.Append('\\', slashes * 2 + 1); s.Append(c); }
            else { s.Append('\\', slashes); s.Append(c); }
            slashes = 0;
        }
        s.Append('\\', slashes * 2); s.Append('"');
        return s.ToString();
    }
    public static Result Run(string executable, string[] args, string directory, int seconds, ulong memoryBytes) {
        using (Mutex gate = new Mutex(false, "Local\\SessionViewerCourseHeavyJob")) {
            bool owned = false;
            try {
                try { owned = gate.WaitOne(0); } catch (AbandonedMutexException) { owned = true; }
                if (!owned) throw new InvalidOperationException("Another guarded heavy job is running.");
                return RunOwned(executable, args, directory, seconds, memoryBytes);
            } finally { if (owned) gate.ReleaseMutex(); }
        }
    }
    static Result RunOwned(string executable, string[] args, string directory, int seconds, ulong memoryBytes) {
        IntPtr job = IntPtr.Zero;
        PROCESS_INFORMATION process = new PROCESS_INFORMATION();
        bool assigned = false;
        Stopwatch watch = Stopwatch.StartNew();
        Result result = new Result();
        try {
            ulong allowed = unchecked((ulong)Process.GetCurrentProcess().ProcessorAffinity.ToInt64());
            ulong chosen = 0;
            for (int bit = 0; bit < IntPtr.Size * 8 && result.processors < 2; bit++) {
                ulong flag = 1UL << bit;
                if ((allowed & flag) != 0) { chosen |= flag; result.processors++; }
            }
            if (chosen == 0) throw new InvalidOperationException("No allowed processors.");
            result.affinityMask = chosen;
            job = CreateJobObject(IntPtr.Zero, null);
            Check(job != IntPtr.Zero);
            EXTENDED_LIMIT limits = new EXTENDED_LIMIT();
            limits.BasicLimitInformation.LimitFlags = 0x10 | 0x200 | 0x2000;
            limits.BasicLimitInformation.Affinity = new UIntPtr(chosen);
            limits.JobMemoryLimit = new UIntPtr(memoryBytes);
            Check(SetInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(limits)));
            STARTUPINFO startup = new STARTUPINFO();
            startup.cb = (uint)Marshal.SizeOf(startup);
            startup.dwFlags = 0x100;
            startup.hStdInput = GetStdHandle(-10);
            startup.hStdOutput = GetStdHandle(-11);
            startup.hStdError = GetStdHandle(-12);
            StringBuilder command = new StringBuilder(Quote(executable));
            if (args != null) foreach (string arg in args) command.Append(" ").Append(Quote(arg));
            Check(CreateProcess(executable, command, IntPtr.Zero, IntPtr.Zero, true, 0x4 | 0x08000000, IntPtr.Zero, directory, ref startup, out process));
            result.processId = process.dwProcessId;
            Check(AssignProcessToJobObject(job, process.hProcess));
            assigned = true;
            Check(ResumeThread(process.hThread) != uint.MaxValue);
            CloseHandle(process.hThread); process.hThread = IntPtr.Zero;
            while (true) {
                uint state = WaitForSingleObject(process.hProcess, 50);
                Check(state == 0 || state == 258);
                if (state == 0) break;
                if (watch.Elapsed.TotalSeconds >= seconds) {
                    result.timedOut = true;
                    Check(TerminateJobObject(job, 124));
                    break;
                }
            }
            uint completed = WaitForSingleObject(process.hProcess, 5000);
            if (completed == 258) throw new InvalidOperationException("Root process did not terminate after cancellation.");
            Check(completed == 0);
            uint code;
            Check(GetExitCodeProcess(process.hProcess, out code));
            result.exitCode = result.timedOut ? 124 : unchecked((int)code);
            ACCOUNTING accounting = new ACCOUNTING();
            Check(QueryInformationJobObject(job, 1, ref accounting, (uint)Marshal.SizeOf(accounting), IntPtr.Zero));
            result.descendantsAtCleanup = accounting.ActiveProcesses;
            Check(TerminateJobObject(job, 125));
            Stopwatch cleanup = Stopwatch.StartNew();
            do {
                Check(QueryInformationJobObject(job, 1, ref accounting, (uint)Marshal.SizeOf(accounting), IntPtr.Zero));
                if (accounting.ActiveProcesses == 0) break;
                if (cleanup.Elapsed.TotalSeconds >= 5) throw new InvalidOperationException("Guarded descendants did not terminate.");
                Thread.Sleep(10);
            } while (true);
            Check(QueryInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(limits), IntPtr.Zero));
            result.peakJobCommitBytes = limits.PeakJobMemoryUsed.ToUInt64();
            result.elapsedSeconds = watch.Elapsed.TotalSeconds;
            return result;
        } finally {
            if (job != IntPtr.Zero) {
                TerminateJobObject(job, 125);
                CloseHandle(job);
            }
            if (!assigned && process.hProcess != IntPtr.Zero) TerminateProcess(process.hProcess, 125);
            if (process.hThread != IntPtr.Zero) CloseHandle(process.hThread);
            if (process.hProcess != IntPtr.Zero) CloseHandle(process.hProcess);
        }
    }
}
'@

$record = [ordered]@{
    platform = 'windows-job-object'
    startedUtc = [DateTime]::UtcNow.ToString('o')
    executable = $program
    arguments = @($ArgumentList)
    workingDirectory = $directory
    timeoutSeconds = $TimeoutSeconds
    jobCommitLimitBytes = [uint64]$MemoryLimitMiB * 1048576
    swapCapSupported = $false
    swapCapNote = 'Windows Job Objects have no separate per-job swap quota; zero swap is not enforced.'
    output = 'Inherited stdout and stderr; redirect the runner to capture a log.'
}
$exitCode = 125
try {
    $result = [GuardedJob]::Run($program, $ArgumentList, $directory, $TimeoutSeconds, $record.jobCommitLimitBytes)
    $record.exitCode = $result.exitCode
    $record.timedOut = $result.timedOut
    $record.elapsedSeconds = [Math]::Round($result.elapsedSeconds, 3)
    $record.peakJobCommitBytes = $result.peakJobCommitBytes
    $record.affinityMask = ('0x{0:X}' -f $result.affinityMask)
    $record.processors = $result.processors
    $record.processId = $result.processId
    $record.descendantsAtCleanup = $result.descendantsAtCleanup
    $exitCode = $result.exitCode
} catch {
    $record.error = $_.Exception.GetBaseException().Message
    $record.exitCode = $exitCode
    [Console]::Error.WriteLine($record.error)
} finally {
    $parent = Split-Path -Parent $manifest
    [IO.Directory]::CreateDirectory($parent) | Out-Null
    [IO.File]::WriteAllText($manifest, ($record | ConvertTo-Json -Depth 4) + [Environment]::NewLine, (New-Object Text.UTF8Encoding $false))
}
exit $exitCode
