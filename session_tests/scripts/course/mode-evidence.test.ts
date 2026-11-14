import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import {docs, viewer, read, hash, filesIn} from './model.ts';

test('mode evidence binds the tested source and retains failed history acceptance', () => {
    const native = JSON.parse(read(path.join(docs, 'journey/release/mode-native-windows-20261114.json')));
    const browser = JSON.parse(read(path.join(docs, 'journey/release/mode-browser-windows-20261114.json')));
    const names = filesIn(path.join(viewer, 'src')).map(file => path.relative(viewer, file).replaceAll('\\', '/'));
    assert.deepEqual(Object.keys(native.normalizedProductionSource).sort(), names.sort());
    for (const [name, digest] of Object.entries(native.normalizedProductionSource))
        assert.equal(hash(read(path.join(viewer, name)).replaceAll('\r\n', '\n')), digest, name);
    assert.equal(browser.inputs.length, names.length);
    for (const input of browser.inputs)
        assert.equal(input.sha256, native.productionSource[input.path.replace('session_viewer/', '')], input.path);
    assert.equal(hash(read(path.join(viewer, '../session_tests/scripts/course/stage-browser.cjs')).replaceAll('\r\n', '\n')), browser.normalizedChecker);
    assert.match(native.log, /18 passed; 0 failed/);
    for (const resource of [native.resources, ...Object.values<any>(native.applicationChecks).map(check => check.resources)]) {
        assert.equal(resource.exitCode, 0);
        assert.equal(resource.timedOut, false);
    }
    assert.equal(browser.status, 'failed-history-assertions');
    assert.equal(browser.completedComparisons, Object.keys(browser.cases).length);
    assert(browser.completedComparisons < browser.requiredComparisons);
    for (const [name, record] of Object.entries<any>(browser.cases)) {
        assert.equal(record.status, 'failed');
        assert.equal(record.comparisonComplete, true);
        assert.equal(record.roundTripFailures.length, 2);
        assert.equal(browser.commands[name].resources.exitCode, 1);
        assert.deepEqual(record.errors, []);
        assert.equal(record.frames.length, 23);
        for (const frame of record.frames) {
            assert.equal(frame.pair.length, 2);
            assert.deepEqual(frame.pair[0].state, frame.pair[1].state);
            assert.equal(frame.pair[0].pixels, frame.pair[1].pixels);
            assert.deepEqual(frame.pair[0].state.errors, []);
            assert.deepEqual(frame.pair[0].state.overlaps, []);
        }
    }
});

test('force-density comparison evidence preserves every failed history assertion', () => {
    const receipt = JSON.parse(read(path.join(docs, 'journey/release/mode-force-density-windows-20261114.json')));
    const original = JSON.parse(read(path.join(docs, 'journey/release/mode-browser-windows-20261114.json')));
    assert.deepEqual(receipt.inputs, original.inputs);
    assert.equal(receipt.normalizedChecker, original.normalizedChecker);
    assert.equal(receipt.status, 'failed-history-assertions');
    assert.equal(receipt.completedComparisons, 6);
    assert.equal(Object.keys(receipt.cases).length, 6);
    let pairs = 0;
    for (const chapter of [1, 2, 3]) for (const viewport of ['desktop', 'upright']) {
        const name = `mode-force-density-${chapter}-${viewport}`;
        const record = receipt.cases[name];
        assert.equal(record.status, 'failed');
        assert.equal(record.comparisonComplete, true);
        assert.equal(record.roundTripFailures.length, 2);
        assert.equal(record.frames.length, [16, 15, 29][chapter - 1]);
        assert.equal(receipt.commands[name].resources.exitCode, 1);
        assert.equal(receipt.commands[name].resources.timedOut, false);
        assert.deepEqual(record.errors, []);
        for (const frame of record.frames) {
            assert.equal(frame.pair.length, 2);
            assert.deepEqual(frame.pair[0].state, frame.pair[1].state);
            assert.equal(frame.pair[0].pixels, frame.pair[1].pixels);
            assert.deepEqual(frame.pair[0].state.errors, []);
            assert.deepEqual(frame.pair[0].state.overlaps, []);
            pairs++;
        }
    }
    assert.equal(pairs, 120);
    assert.equal(receipt.comparedFramePairs, pairs);
});
