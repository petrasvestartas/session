import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {gzipSync, gunzipSync} from 'node:zlib';
import {hash, json} from './model.ts';
import {fileReference, observeExecution, finishExecution} from './capture-storage.ts';
import {sealCapture, verifyCaptureLedger, retirePublishedCapture, ledgerReceiptFile} from './capture-ledger.ts';

function fixture(compressed = true) {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'capture-ledger-')), owned = path.join(root, 'owned');
    fs.mkdirSync(owned);
    const executable = path.join(owned, 'run.exe'), source = path.join(owned, 'source.rs');
    const image = path.join(owned, 'image.png'), capture = path.join(owned, 'result.json');
    fs.writeFileSync(executable, 'actual executable'); fs.writeFileSync(source, 'actual source'); fs.writeFileSync(image, 'image');
    const execution = finishExecution(observeExecution([executable, source]));
    fs.writeFileSync(capture, JSON.stringify({image: fileReference(image), executionIntegrity: execution}));
    const directory = path.join(root, 'proof', 'case');
    const options = {directory, ownedRoots: [owned], capture, evidence: [source], image, execution};
    let receipt = sealCapture(options).path;
    if (!compressed) {
        const plain = path.join(directory, 'capture.json');
        fs.writeFileSync(plain, gunzipSync(fs.readFileSync(receipt)), {flag: 'wx'}); fs.unlinkSync(receipt); receipt = plain;
    }
    const publication = path.join(root, 'published.json'); fs.writeFileSync(publication, '{}');
    return {root, owned, directory, executable, source, image, receipt, publication, options, cleanup: () => fs.rmSync(root, {recursive: true})};
}

function interruptedRetirement(f: ReturnType<typeof fixture>, compressed: boolean) {
    const receipt = json(f.receipt), evidence = fileReference(f.publication);
    fs.writeFileSync(path.join(receipt.blobs, evidence.sha256 + '.gz'), gzipSync(fs.readFileSync(f.publication)), {flag: 'wx'});
    const publication = path.join(f.directory, 'publication.json');
    fs.writeFileSync(publication, JSON.stringify({capture: fileReference(f.receipt), destination: fileReference(f.image), evidence}), {flag: 'wx'});
    const file = path.join(f.directory, compressed ? 'retirement.json.gz' : 'retirement.json');
    const bytes = JSON.stringify({capture: fileReference(f.receipt), publication: fileReference(publication), at: new Date().toISOString(),
        files: receipt.candidates.filter((item: any) => item.retire)}, null, 2) + '\n';
    fs.writeFileSync(file, compressed ? gzipSync(bytes) : bytes, {flag: 'wx'});
    return file;
}

test('new capture and retirement receipts use gzip while references hash their stored bytes', () => {
    const f = fixture();
    try {
        assert.equal(f.receipt, path.join(f.directory, 'capture.json.gz'));
        assert(!fs.existsSync(path.join(f.directory, 'capture.json')));
        const decoded = gunzipSync(fs.readFileSync(f.receipt));
        assert.deepEqual(json(f.receipt), JSON.parse(decoded.toString('utf8')));
        assert.notEqual(fileReference(f.receipt).sha256, hash(decoded));
        retirePublishedCapture(f.receipt, f.image, f.publication);
        const retirement = ledgerReceiptFile(f.directory, 'retirement')!;
        assert.equal(retirement, path.join(f.directory, 'retirement.json.gz'));
        assert(!fs.existsSync(path.join(f.directory, 'retirement.json')));
        assert.deepEqual(json(path.join(f.directory, 'publication.json')).capture, fileReference(f.receipt));
        assert.deepEqual(json(retirement).capture, fileReference(f.receipt));
        assert.deepEqual(json(path.join(f.directory, 'retired.json')).retirement, fileReference(retirement));
        assert.equal(verifyCaptureLedger(f.receipt).status, 'PUBLISHED_BINARY_RETIRED');
    } finally { f.cleanup(); }
});

test('a legacy live capture retires into a new gzip retirement without rewriting its capture proof', () => {
    const f = fixture(false);
    try {
        const before = fileReference(f.receipt);
        assert.equal(verifyCaptureLedger(f.receipt).status, 'CAPTURED_BINARY_AVAILABLE');
        assert.equal(retirePublishedCapture(f.receipt, f.image, f.publication).status, 'PUBLISHED_BINARY_RETIRED');
        assert.equal(ledgerReceiptFile(f.directory, 'retirement'), path.join(f.directory, 'retirement.json.gz'));
        assert.deepEqual(fileReference(f.receipt), before);
        assert(!fs.existsSync(path.join(f.directory, 'capture.json.gz')));
    } finally { f.cleanup(); }
});

for (const compressed of [false, true]) {
    test(`an arbitrary explicit ${compressed ? 'gzip' : 'plain'} capture filename verifies and retires by its exact caller path`, () => {
        const f = fixture(compressed);
        try {
            const renamed = path.join(f.directory, compressed ? 'explicit-proof.json.gz' : 'explicit-proof.json');
            fs.renameSync(f.receipt, renamed); f.receipt = renamed;
            const before = fileReference(f.receipt);
            assert.equal(ledgerReceiptFile(f.directory, 'capture'), undefined);
            assert.equal(verifyCaptureLedger(f.receipt).status, 'CAPTURED_BINARY_AVAILABLE');
            fs.writeFileSync(path.join(f.directory, 'capture.json'), '{}', {flag: 'wx'});
            assert.equal(verifyCaptureLedger(f.receipt).status, 'CAPTURED_BINARY_AVAILABLE');
            assert.equal(retirePublishedCapture(f.receipt, f.image, f.publication).status, 'PUBLISHED_BINARY_RETIRED');
            assert.deepEqual(json(path.join(f.directory, 'publication.json')).capture, before);
            assert.deepEqual(json(ledgerReceiptFile(f.directory, 'retirement')!).capture, before);
            assert.deepEqual(fileReference(f.receipt), before);
            assert.equal(verifyCaptureLedger(f.receipt).status, 'PUBLISHED_BINARY_RETIRED');
        } finally { f.cleanup(); }
    });
}

for (const captureGzip of [false, true]) for (const retirementGzip of [false, true]) {
    test(`capture gzip=${captureGzip}, retirement gzip=${retirementGzip}: verify and resume preserve existing proof bytes`, () => {
        const f = fixture(captureGzip);
        try {
            const retirement = interruptedRetirement(f, retirementGzip);
            const files = [f.receipt, retirement, path.join(f.directory, 'publication.json')];
            const before = files.map(fileReference);
            assert.equal(ledgerReceiptFile(f.directory, 'capture'), f.receipt);
            assert.equal(ledgerReceiptFile(f.directory, 'retirement'), retirement);
            assert.equal(verifyCaptureLedger(f.receipt).status, 'RETIREMENT_INCOMPLETE');
            fs.writeFileSync(f.executable, 'another active job');
            assert.throws(() => retirePublishedCapture(f.receipt, f.image, f.publication));
            assert.equal(fs.readFileSync(f.executable, 'utf8'), 'another active job');
            fs.writeFileSync(f.executable, 'actual executable');
            assert.equal(retirePublishedCapture(f.receipt, f.image, f.publication).status, 'PUBLISHED_BINARY_RETIRED');
            assert(!fs.existsSync(f.executable)); assert(!fs.existsSync(f.source));
            assert.deepEqual(files.map(fileReference), before);
            const completed = fileReference(path.join(f.directory, 'retired.json'));
            assert.equal(retirePublishedCapture(f.receipt, f.image, f.publication).status, 'PUBLISHED_BINARY_RETIRED');
            assert.deepEqual(files.map(fileReference), before); assert.deepEqual(fileReference(completed.path), completed);
        } finally { f.cleanup(); }
    });
}

test('discovery rejects ambiguous capture and retirement siblings and sealing refuses legacy overwrite', () => {
    const f = fixture(false);
    try {
        const before = fileReference(f.receipt);
        assert.throws(() => sealCapture(f.options), /already exists/); assert.deepEqual(fileReference(f.receipt), before);
        fs.writeFileSync(f.receipt + '.gz', gzipSync(fs.readFileSync(f.receipt)), {flag: 'wx'});
        assert.throws(() => ledgerReceiptFile(f.directory, 'capture'), /Ambiguous/);
        assert.throws(() => verifyCaptureLedger(f.receipt), /Ambiguous/); fs.unlinkSync(f.receipt + '.gz');
        const retirement = interruptedRetirement(f, false);
        fs.writeFileSync(retirement + '.gz', gzipSync(fs.readFileSync(retirement)), {flag: 'wx'});
        assert.throws(() => ledgerReceiptFile(f.directory, 'retirement'), /Ambiguous/);
        assert.throws(() => retirePublishedCapture(f.receipt, f.image, f.publication), /Ambiguous/);
        assert(fs.existsSync(f.executable));
    } finally { f.cleanup(); }
});

for (const name of ['capture', 'retirement'] as const) {
    test(`a valid gzip ${name} with identical decoded JSON but different physical bytes fails its recorded hash`, () => {
        const f = fixture();
        try {
            retirePublishedCapture(f.receipt, f.image, f.publication);
            const file = ledgerReceiptFile(f.directory, name)!, before = json(file), original = fileReference(file);
            fs.writeFileSync(file, gzipSync(gunzipSync(fs.readFileSync(file)).toString('utf8') + '\n'));
            assert.deepEqual(json(file), before); assert.notEqual(fileReference(file).sha256, original.sha256);
            assert.throws(() => verifyCaptureLedger(f.receipt));
        } finally { f.cleanup(); }
    });
}

test('JSON gzip decoding is explicit by suffix and corrupted gzip fails closed', () => {
    const f = fixture();
    try {
        const plain = path.join(f.root, 'plain.json'), gz = path.join(f.root, 'value.json.gz');
        assert.equal(ledgerReceiptFile(f.root, 'capture'), undefined);
        fs.writeFileSync(plain, gzipSync('{}')); assert.throws(() => json(plain));
        fs.writeFileSync(gz, '{}'); assert.throws(() => json(gz));
        fs.writeFileSync(f.receipt, 'corrupted gzip'); assert.throws(() => verifyCaptureLedger(f.receipt));
    } finally { f.cleanup(); }
});
