const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const {spawnSync} = require('node:child_process');
const {test} = require('node:test');

test('Windows guard preserves root exit and reaps lingering children', {skip: process.platform !== 'win32'}, () => {
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'viewer-guard-test-'));
    try {
        const parent = path.join(temporary, 'parent.cjs');
        const childId = path.join(temporary, 'child.pid');
        fs.writeFileSync(parent, `const {spawn}=require('node:child_process');
const fs=require('node:fs');
const child=spawn(process.execPath,['-e','setInterval(()=>{},1000)'],{stdio:'ignore'});
fs.writeFileSync(process.argv[2],String(child.pid));
child.unref();
process.exit(7);
`);
        const manifest = path.join(temporary, 'exit.json');
        const result = spawnSync(process.execPath, [path.join(__dirname, 'run-guarded.cjs'),
            '-TimeoutSeconds', '20', '-ManifestPath', manifest, process.execPath, parent, childId], {encoding: 'utf8', timeout: 30000});
        assert.equal(result.error, undefined);
        assert.equal(result.status, 7, result.stderr);
        const record = JSON.parse(fs.readFileSync(manifest));
        assert.equal(record.timedOut, false);
        assert.equal(record.exitCode, 7);
        assert.equal(record.jobCommitLimitBytes, 2147483648);
        assert(record.descendantsAtCleanup >= 1);
        assert.throws(() => process.kill(Number(fs.readFileSync(childId)), 0), {code: 'ESRCH'});

        const timeout = path.join(temporary, 'timeout.json');
        const hung = spawnSync(process.execPath, [path.join(__dirname, 'run-guarded.cjs'),
            '-TimeoutSeconds', '1', '-MemoryLimitMiB', '4096', '-ManifestPath', timeout, process.execPath, '-e', 'setInterval(()=>{},1000)'], {encoding: 'utf8', timeout: 30000});
        assert.equal(hung.error, undefined);
        assert.equal(hung.status, 124, hung.stderr);
        assert.equal(JSON.parse(fs.readFileSync(timeout)).timedOut, true);
        assert.equal(JSON.parse(fs.readFileSync(timeout)).jobCommitLimitBytes, 4294967296);
    } finally {
        assert.equal(path.dirname(temporary), os.tmpdir());
        assert(path.basename(temporary).startsWith('viewer-guard-test-'));
        fs.rmSync(temporary, {recursive: true, force: true});
    }
});
