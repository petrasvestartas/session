const { spawnSync } = require('node:child_process');
const { join } = require('node:path');

const windows = process.platform === 'win32';
const script = join(__dirname, `build-viewer-docs.${windows ? 'ps1' : 'sh'}`);
const result = spawnSync(windows ? 'powershell.exe' : 'bash', windows
  ? ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', script]
  : [script], { stdio: 'inherit', windowsHide: true });
if (result.error) console.error(result.error.message);
process.exit(result.status ?? 1);
