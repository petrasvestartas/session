import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {gzipSync, gunzipSync} from 'node:zlib';
import {hash, json, docs} from './model.ts';
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

test('missing or replaced unaliased publication stays a false current-publication status', () => {
    const f = fixture();
    try {
        const destination = path.join(f.root, 'published.png');
        fs.copyFileSync(f.image, destination);
        retirePublishedCapture(f.receipt, destination, f.publication);
        fs.writeFileSync(destination, 'changed');
        assert.equal(verifyCaptureLedger(f.receipt).currentPublication, false);
        fs.unlinkSync(destination);
        assert.equal(verifyCaptureLedger(f.receipt).currentPublication, false);
    } finally { f.cleanup(); }
});

test('aliased PNG reads preserve immutable ledger references and repeat retirement', t => {
    const f = fixture(), root = path.join(docs, 'screenshots/journey');
    const original = path.join(root, '__ledger-removed.png'), canonical = path.join(root, '__ledger-retained.png');
    const manifest = path.join(root, 'image-aliases.json'), sha256 = fileReference(f.image).sha256;
    let aliasHash = sha256, removed = false;
    const read = fs.readFileSync.bind(fs), exists = fs.existsSync.bind(fs), realpath = fs.realpathSync.bind(fs);
    const stat = fs.statSync.bind(fs), lstat = fs.lstatSync.bind(fs);
    t.mock.method(fs, 'existsSync', (file: any) => file === original ? !removed : file === canonical || file === manifest ? true : exists(file));
    t.mock.method(fs, 'readFileSync', (file: any, ...args: any[]) => file === manifest
        ? JSON.stringify({version: 1, aliases: {'__ledger-removed.png': {canonical: '__ledger-retained.png', sha256: aliasHash}}})
        : (read as any)(file === canonical || file === original ? f.image : file, ...args));
    t.mock.method(fs, 'realpathSync', (file: any, ...args: any[]) => file === canonical || file === root ? file : (realpath as any)(file, ...args));
    t.mock.method(fs, 'statSync', (file: any, ...args: any[]) => (stat as any)(file === canonical || file === original ? f.image : file, ...args));
    t.mock.method(fs, 'lstatSync', (file: any, ...args: any[]) => (lstat as any)(file === canonical ? f.image : file, ...args));
    try {
        const receipt = sealCapture({...f.options, directory: path.join(f.root, 'proof', 'alias'), image: original}).path;
        retirePublishedCapture(receipt, original, f.publication);
        removed = true;
        const proofFiles = [receipt, ...['publication.json', 'retirement.json.gz', 'retired.json'].map(name => path.join(path.dirname(receipt), name))];
        const before = proofFiles.map(fileReference);
        assert.equal(json(receipt).image.path, original);
        assert.equal(json(proofFiles[1]).destination.path, original);
        assert.equal(verifyCaptureLedger(receipt).currentPublication, true);
        assert.equal(retirePublishedCapture(receipt, original, f.publication).status, 'PUBLISHED_BINARY_RETIRED');
        assert.deepEqual(proofFiles.map(fileReference), before);
        aliasHash = hash('different');
        assert.throws(() => verifyCaptureLedger(receipt), /hash mismatch/);
        aliasHash = sha256; fs.writeFileSync(f.image, 'different');
        assert.throws(() => verifyCaptureLedger(receipt), /hash mismatch/);
        fs.writeFileSync(f.image, 'image');
        assert.deepEqual(proofFiles.map(fileReference), before);
        assert.equal(verifyCaptureLedger(receipt).currentPublication, true);
    } finally { t.mock.restoreAll(); f.cleanup(); }
});

function sharedBrowserFixture() {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'ledger-shared-course-')), owned = path.join(root, 'owned');
    const workspace = path.join(owned, 'workspace'), dist = path.join(workspace, 'target/course-checks/fixture/dist');
    fs.mkdirSync(dist, {recursive: true});
    const put = (file: string, value: unknown) => {fs.mkdirSync(path.dirname(file), {recursive: true}); fs.writeFileSync(file, typeof value === 'string' ? value : JSON.stringify(value)); return fileReference(file);};
    const content = '{"title":"Original full course","steps":[]}', course = put(path.join(root, 'shared', hash(content), 'course.json'), content);
    const pointer = put(path.join(workspace, 'docs/journey/course-manifest-input.json'), course);
    const before = observeExecution([pointer.path, course.path]);
    const wasm = put(path.join(owned, 'raw.wasm'), 'compiled wasm'), bundleFile = put(path.join(dist, 'viewer.wasm'), 'compiled wasm');
    const image = put(path.join(owned, 'image.png'), 'actual fixture image');
    const execution = finishExecution(observeExecution([bundleFile.path]));
    const commands = ['wasm', 'bindgen'].map(label => {
        const args = label === 'wasm' ? ['cargo', 'build'] : ['bindgen.exe', 'compiler/viewer.wasm'];
        const resources = {exitCode: 0, timedOut: false};
        const resource = put(path.join(owned, label + '.resources.json'), resources), log = put(path.join(owned, label + '.log'), label);
        const command = put(path.join(owned, label + '.command.json'), {executable: args[0], arguments: args.slice(1)});
        return {label, args, resources, resourcesFile: resource.path, resourcesSha256: resource.sha256, log: log.path, logSha256: log.sha256, commandFile: command.path};
    });
    const bundle = {'viewer.wasm': bundleFile.sha256}, source = {files: 'source-bound fixture'};
    const build = put(path.join(owned, 'built.json'), {source, bundle, group: {workbench: path.join(root, 'workbench')},
        compilerArtifact: {manifest_path: path.join(root, 'workbench/Cargo.toml'), filenames: ['compiler/viewer.wasm']},
        wasm: {...wasm, compilerReportedPath: 'compiler/viewer.wasm'}, commands, bindgen: {executable: 'bindgen.exe', sha256: hash('bindgen')}});
    const preparedFile = path.join(owned, 'prepared.json');
    const prepared = {workspace, source, bundle, imageBuild: {path: build.path, sha256: build.sha256}, sharedCourseManifest: course,
        pins: {[pointer.path]: pointer.sha256, [course.path]: course.sha256}};
    put(preparedFile, prepared);
    const capture = put(path.join(owned, 'capture-result.json'), {id: 'fixture', image, bundle, source, executionIntegrity: execution,
        served: {'fixture/dist/viewer.wasm': bundleFile.sha256}, resources: {exitCode: 0}});
    const sharedCourseIntegrity = {...finishExecution(before), prepared: fileReference(preparedFile)};
    const options = {directory: path.join(root, 'proof/case'), ownedRoots: [owned], transientRoots: [workspace], capture: capture.path,
        build: build.path, evidence: [preparedFile, pointer.path, course.path], image: image.path, execution, sharedCourseIntegrity};
    return {root, owned, workspace, course, pointer, prepared, preparedFile, capture, options,
        updatePrepared() {put(preparedFile, prepared); options.sharedCourseIntegrity.prepared = fileReference(preparedFile);},
        cleanup() {fs.rmSync(root, {recursive: true});}};
}

test('shared course observations seal separately while raw browser execution stays identical, and verify after retirement', () => {
    const f = sharedBrowserFixture();
    try {
        const original = fs.readFileSync(f.capture.path), receipt = sealCapture(f.options);
        assert.deepEqual(fs.readFileSync(f.capture.path), original);
        const verified = verifyCaptureLedger(receipt.path);
        assert.deepEqual(verified.receipt.execution, JSON.parse(original.toString()).executionIntegrity);
        assert.equal(verified.receipt.sharedCourseIntegrity.after.files.length, 2);
        const publication = path.join(f.root, 'published.json'); fs.writeFileSync(publication, '{}');
        retirePublishedCapture(receipt.path, f.options.image, publication);
        fs.unlinkSync(f.course.path);
        assert.equal(verifyCaptureLedger(receipt.path).status, 'PUBLISHED_BINARY_RETIRED');
    } finally {f.cleanup();}
});

for (const mode of ['missing-input', 'wrong-prepared-pin', 'wrong-build', 'raw-execution-changed', 'observation-too-late'] as const)
    test('shared course ledger rejects ' + mode, () => {
        const f = sharedBrowserFixture();
        try {
            const shared = f.options.sharedCourseIntegrity;
            if (mode === 'missing-input') {shared.before.files.pop(); shared.after.files.pop();}
            if (mode === 'wrong-prepared-pin') {f.prepared.pins[f.course.path] = hash('wrong course'); f.updatePrepared();}
            if (mode === 'wrong-build') {f.prepared.imageBuild.sha256 = hash('other build'); f.updatePrepared();}
            if (mode === 'raw-execution-changed') f.options.execution.before.at = '2000-01-01T00:00:00.000Z';
            if (mode === 'observation-too-late') shared.before.at = '2999-01-01T00:00:00.000Z';
            assert.throws(() => sealCapture(f.options));
        } finally {f.cleanup();}
    });

test('live shared manifest corruption fails ledger verification', () => {
    const f = sharedBrowserFixture();
    try {
        const receipt = sealCapture(f.options); fs.writeFileSync(f.course.path, 'replaced input');
        assert.throws(() => verifyCaptureLedger(receipt.path), /Live artifact was replaced/);
    } finally {f.cleanup();}
});
