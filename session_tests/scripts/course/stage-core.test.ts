import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';

const viewer = fileURLToPath(new URL('../../../session_viewer/', import.meta.url));
const core = 'actions check deck draw eval font geometry layout matrix morph net object paint run tex tween view'.split(' ');
const lines = (source: string) => source.split(/\r?\n/).filter(line => line.trim() && !line.includes('--8<-- ['));

test('the mathematics checkpoint retains the complete production stage core', () => {
    for (const name of core) {
        const source = fs.readFileSync(path.join(viewer, `src/app/stage/${name}.rs`), 'utf8');
        let lesson = fs.readFileSync(path.join(viewer, `docs/lessons/03a-math/src/app/stage/${name}.rs`), 'utf8');
        for (const logo of ['logo-eth.png', 'logo-brg.png']) {
            lesson = lesson.replace(
                `include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/${logo}"))`,
                `include_bytes!("../../../assets/stage/${logo}")`,
            );
        }
        assert.deepEqual(lines(lesson), lines(source), `${name}: revise the owning subject when production changes`);
    }
});

test('standalone mathematics evidence still names the checked-in sources, assets and captures', () => {
    const lesson = path.join(viewer, 'docs/lessons/03a-math');
    const record = JSON.parse(fs.readFileSync(path.join(lesson, 'full-browser-evidence.json'), 'utf8'));
    const digest = (data: string | Buffer) => createHash('sha256').update(data).digest('hex');
    // Git text checkouts may use CRLF; the cloud receipt fingerprints LF source.
    for (const [name, hash] of Object.entries(record.sourceSha256)) {
        assert.equal(digest(fs.readFileSync(path.join(lesson, name), 'utf8').replaceAll('\r\n', '\n')), hash, name);
    }
    for (const [name, hash] of Object.entries(record.inputs)) {
        assert.equal(digest(fs.readFileSync(path.join(viewer, name))), hash, name);
    }
    for (const entry of record.cases) {
        for (const frame of [...entry.frames, entry.previous, entry.dark, entry.restored]) {
            assert.equal(digest(fs.readFileSync(path.join(viewer, 'docs/screenshots/math-full', frame.file))), frame.sha256, frame.file);
        }
    }
});


test('local Windows mathematics receipts retain their tested source and browser captures', () => {
    const lesson = path.join(viewer, 'docs/lessons/03a-math');
    const digest = (data: string | Buffer) => createHash('sha256').update(data).digest('hex');
    const native = JSON.parse(fs.readFileSync(path.join(lesson, 'native-windows-evidence.json'), 'utf8'));
    const browser = JSON.parse(fs.readFileSync(path.join(lesson, 'browser-windows-evidence.json'), 'utf8'));
    for (const hashes of [native.sourceSha256, browser.normalizedSourceSha256]) {
        for (const [name, hash] of Object.entries(hashes)) {
            assert.equal(digest(fs.readFileSync(path.join(lesson, name), 'utf8').replaceAll('\r\n', '\n')), hash, name);
        }
    }
    for (const [name, hash] of Object.entries(native.kernelSources)) {
        assert.equal(digest(fs.readFileSync(path.join(viewer, '..', name), 'utf8').replaceAll('\r\n', '\n')), hash, name);
    }
    for (const record of [native, browser]) for (const [name, hash] of Object.entries(record.inputs)) {
        assert.equal(digest(fs.readFileSync(path.join(viewer, name))), hash, name);
    }
    for (const entry of browser.cases) for (const frame of [...entry.frames, entry.previous, entry.dark, entry.restored]) {
        assert.equal(digest(fs.readFileSync(path.join(viewer, browser.captureDirectory, frame.file))), frame.sha256, frame.file);
    }
});
