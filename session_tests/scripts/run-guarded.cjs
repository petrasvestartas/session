const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const {spawnSync} = require('node:child_process');

const args = process.argv.slice(2);
const options = [];
const names = new Map(['TimeoutSeconds', 'MemoryLimitMiB', 'ManifestPath', 'WorkingDirectory'].map(name => ['-' + name.toLowerCase(), name]));
while (names.has(args[0]?.toLowerCase())) {
    const name = names.get(args.shift().toLowerCase());
    const value = args.shift();
    if (value === undefined) throw Error(`Missing ${name}`);
    options.push('-' + name, value);
}
if (!args.length) throw Error('Usage: run-guarded.cmd [guard options] EXECUTABLE [arguments]');

const temporaryRoot = fs.realpathSync(os.tmpdir());
const temporary = fs.mkdtempSync(path.join(temporaryRoot, 'viewer-course-guard-'));
try {
    const command = path.join(temporary, 'command.json');
    fs.writeFileSync(command, JSON.stringify({executable: args[0], arguments: args.slice(1)}));
    const result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File',
        path.join(__dirname, 'run-guarded.ps1'), ...options, '-CommandFile', command],
        {stdio: 'inherit', windowsHide: true});
    if (result.error) throw result.error;
    process.exitCode = result.status ?? 125;
} finally {
    if (path.dirname(temporary) !== temporaryRoot || !path.basename(temporary).startsWith('viewer-course-guard-')) {
        throw Error('Refusing cleanup outside the guard temporary directory');
    }
    fs.rmSync(temporary, {recursive: true, force: true});
}
