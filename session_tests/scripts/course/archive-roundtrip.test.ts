import assert from 'node:assert/strict';
import test from 'node:test';
import {fields, verifyArchive} from './archive-roundtrip.ts';

test('wire reader preserves opaque bytes and rejects truncated or overflowing fields', () => {
    assert.deepEqual(fields(Uint8Array.from([10, 3, 0, 255, 1])).get(1), [Uint8Array.from([0, 255, 1])]);
    for (const bytes of [[0], [10, 4, 1], [128], [11], [9, 1],
        [128, 128, 128, 128, 128, 128, 128, 128, 128, 2]])
        assert.throws(() => fields(Uint8Array.from(bytes)));
});

test('archive comparison rejects changed opaque component bytes', () => {
    const original = Buffer.from([26, 4, 130, 1, 1, 255]);
    const archive = (document: Buffer) => Buffer.concat([Buffer.from('SESSION-VIEWER\x01\n'),
        Buffer.from([10, document.length]), document, Buffer.from([18, 2, 123, 125])]);
    assert.equal(verifyArchive(original, archive(original)).components, 1);
    const changed = Buffer.from(original); changed[5] = 254;
    assert.throws(() => verifyArchive(original, archive(changed)), /payload changed/);
});
