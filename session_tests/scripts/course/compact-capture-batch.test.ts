import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {fileReference} from './capture-storage.ts';
import {readBatchQueue, runCompactBatch} from './compact-capture-batch.ts';

function fixture(ids = ['a', 'b']) {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'compact-batch-'));
    const destinations = path.join(root, 'docs/screenshots/journey'); fs.mkdirSync(destinations, {recursive: true});
    const source = path.join(root, 'source.rs'), tool = path.join(root, 'tool.ts');
    fs.writeFileSync(source, 'source one'); fs.writeFileSync(tool, 'tool one');
    const png = Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+aZ1cAAAAASUVORK5CYII=', 'base64');
    const queueFile = path.join(root, 'queue.json');
    const entries = ids.map(id => ({lane: 'native', id, destination: path.join(destinations, id + '.png')}));
    for (const entry of entries) fs.writeFileSync(entry.destination, png);
    const retained = path.join(destinations, 'unchanged.png'); fs.writeFileSync(retained, png);
    fs.writeFileSync(queueFile, JSON.stringify({entries, retained: [{destination: retained, sha256: fileReference(retained).sha256, reason: 'Explicit unchanged prefix'}]}));
    const calls: string[] = [], attempts: string[] = [];
    const controls = {failCapture: false, failRetirement: false};
    const runtime: any = {viewer: root, destinations, state: path.join(root, 'state'),
        generation: () => ({files: [source, tool].map(fileReference), directories: [], profile: {}}),
        capture: async (_lane: string, id: string) => {
            calls.push('capture ' + id); attempts.push(process.env.COURSE_CAPTURE_ATTEMPT!);
            if (controls.failCapture) throw Error('Fixture capture interrupted');
            const folder = path.join(root, 'capture-' + attempts.length); fs.mkdirSync(folder);
            const image = path.join(folder, 'image.png'); fs.writeFileSync(image, Buffer.concat([png, Buffer.from(id + fs.readFileSync(source, 'utf8'))]));
            const receipt = path.join(folder, 'capture.json'); fs.writeFileSync(receipt, JSON.stringify({image: fileReference(image)}));
            return {receipt: fileReference(receipt), image: fileReference(image)};
        },
        verify: (file: string) => {
            const receipt = JSON.parse(fs.readFileSync(file, 'utf8'));
            assert.equal(fileReference(receipt.image.path).sha256, receipt.image.sha256);
            return {receipt, status: fs.existsSync(file + '.retired') ? 'PUBLISHED_BINARY_RETIRED' : 'CAPTURED_BINARY_AVAILABLE'};
        },
        retire: (file: string, destination: string) => {
            calls.push('retire ' + path.basename(destination, '.png'));
            if (controls.failRetirement) { controls.failRetirement = false; throw Error('Fixture retirement interrupted'); }
            const receipt = JSON.parse(fs.readFileSync(file, 'utf8'));
            assert.equal(fileReference(destination).sha256, receipt.image.sha256);
            fs.writeFileSync(file + '.retired', 'retired');
            return runtime.verify(file);
        }};
    return {root, source, tool, entries, queueFile, calls, attempts, controls, runtime, cleanup: () => fs.rmSync(root, {recursive: true})};
}

test('limit checkpoints retire each image before the next capture and resume without recapture', async () => {
    const f = fixture();
    try {
        const first = await runCompactBatch(f.queueFile, {limit: 1}, f.runtime);
        assert.equal(first.captured, 1); assert.equal(first.published, 1); assert.equal(first.retired, 1); assert.equal(first.remaining, 1);
        assert.equal(first.retainedUnchanged, 1); assert.deepEqual(f.calls, ['capture a', 'retire a']);
        const second = await runCompactBatch(f.queueFile, {limit: 1}, f.runtime);
        assert.equal(second.generation, first.generation); assert.equal(second.remaining, 0);
        assert.deepEqual(f.calls, ['capture a', 'retire a', 'capture b', 'retire b']);
        await runCompactBatch(f.queueFile, {}, f.runtime); assert.equal(f.calls.length, 4);
        const publications = fs.readdirSync(path.join(first.directory, 'publications')).map(name => JSON.parse(fs.readFileSync(path.join(first.directory, 'publications', name), 'utf8')));
        assert(publications.every(row => row.visualReview === 'PENDING_INDEPENDENT_REVIEW' && !Object.hasOwn(row, 'rootPersonallyViewedOriginalResolution')));
    } finally { f.cleanup(); }
});

test('publication interrupted before retirement resumes the captured proof and stops accumulation', async () => {
    const f = fixture();
    try {
        f.controls.failRetirement = true;
        const interrupted = await runCompactBatch(f.queueFile, {}, f.runtime);
        assert.equal(interrupted.status, 'STOPPED_ON_FAILURE'); assert.equal(interrupted.captured, 1); assert.equal(interrupted.published, 1); assert.equal(interrupted.retired, 0);
        assert(!f.calls.includes('capture b'));
        const resumed = await runCompactBatch(f.queueFile, {}, f.runtime);
        assert.equal(resumed.remaining, 0); assert.equal(f.calls.filter(call => call === 'capture a').length, 1);
        assert.deepEqual(f.calls, ['capture a', 'retire a', 'retire a', 'capture b', 'retire b']);
    } finally { f.cleanup(); }
});

test('source and tool changes create fresh generations while old evidence stays immutable', async () => {
    const f = fixture(['a']);
    try {
        const first = await runCompactBatch(f.queueFile, {}, f.runtime), manifest = fileReference(path.join(first.directory, 'manifest.json'));
        fs.appendFileSync(f.source, ' changed');
        const second = await runCompactBatch(f.queueFile, {}, f.runtime); assert.notEqual(second.generation, first.generation);
        fs.appendFileSync(f.tool, ' changed');
        const third = await runCompactBatch(f.queueFile, {}, f.runtime); assert.notEqual(third.generation, second.generation);
        assert.equal(fileReference(manifest.path).sha256, manifest.sha256);
        assert.equal(f.calls.filter(call => call === 'capture a').length, 3); assert.equal(new Set(f.attempts).size, 3);
    } finally { f.cleanup(); }
});

test('a changed current public hash is never skipped as a qualified completed entry', async () => {
    const f = fixture(['a']);
    try {
        await runCompactBatch(f.queueFile, {}, f.runtime);
        fs.writeFileSync(f.entries[0].destination, 'external replacement');
        const resumed = await runCompactBatch(f.queueFile, {}, f.runtime);
        assert.equal(resumed.status, 'STOPPED_ON_FAILURE'); assert.equal(resumed.verifiedCurrent, 0); assert.equal(resumed.remaining, 1);
        assert.equal(f.calls.length, 2); assert.equal(fs.readFileSync(f.entries[0].destination, 'utf8'), 'external replacement');
    } finally { f.cleanup(); }
});

test('only explicit allowed PNG destinations are writable and retained scope stays disjoint', () => {
    const f = fixture(['a']);
    try {
        const input = JSON.parse(fs.readFileSync(f.queueFile, 'utf8'));
        input.entries[0].destination = path.join(f.root, 'outside.png'); fs.writeFileSync(f.queueFile, JSON.stringify(input));
        assert.throws(() => readBatchQueue(f.queueFile, f.runtime));
        input.entries[0].destination = input.retained[0].destination; fs.writeFileSync(f.queueFile, JSON.stringify(input));
        assert.throws(() => readBatchQueue(f.queueFile, f.runtime)); assert.equal(f.calls.length, 0);
    } finally { f.cleanup(); }
});

test('unsealed failed captures block retries and maintained captures remain explicitly pending', async () => {
    const f = fixture(['a']);
    try {
        f.controls.failCapture = true;
        const failed = await runCompactBatch(f.queueFile, {}, f.runtime); assert.equal(failed.status, 'STOPPED_ON_FAILURE'); assert.equal(failed.captured, 0);
        f.controls.failCapture = false;
        const retry = await runCompactBatch(f.queueFile, {}, f.runtime); assert.equal(retry.status, 'STOPPED_ON_FAILURE'); assert.equal(f.calls.length, 1);
        const input = JSON.parse(fs.readFileSync(f.queueFile, 'utf8'));
        input.entries[0] = {...input.entries[0], lane: 'browser', id: '97-finish-with-the-maintained-browser-runtime', route: 'retained-maintained'};
        fs.writeFileSync(f.queueFile, JSON.stringify(input));
        const pending = await runCompactBatch(f.queueFile, {}, f.runtime); assert.equal(pending.unsupported.length, 1); assert.equal(pending.remaining, 1); assert.equal(f.calls.length, 1);
    } finally { f.cleanup(); }
});

test('readonly release retention and separate-helper images stay explicit in total and remaining counts', async () => {
    const f = fixture(['a']);
    try {
        const release = path.join(f.root, 'docs/journey/release'); fs.mkdirSync(release, {recursive: true});
        const unchanged = path.join(release, 'cloud.png'), special = path.join(release, 'measurement.png');
        fs.copyFileSync(f.entries[0].destination, unchanged); fs.copyFileSync(f.entries[0].destination, special);
        const input = JSON.parse(fs.readFileSync(f.queueFile, 'utf8'));
        input.retained.push({destination: unchanged, sha256: fileReference(unchanged).sha256, reason: 'Unchanged cloud'});
        input.special = [{path: special, sha256: fileReference(special).sha256, kind: 'supporting'}];
        fs.writeFileSync(f.queueFile, JSON.stringify(input));
        const result = await runCompactBatch(f.queueFile, {}, f.runtime);
        assert.equal(result.total, 2); assert.equal(result.retired, 1); assert.equal(result.remaining, 1); assert.equal(result.retainedUnchanged, 2);
        assert.equal(result.unsupported[0].destination, special); assert.equal(fileReference(special).sha256, input.special[0].sha256);
    } finally { f.cleanup(); }
});
