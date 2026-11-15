import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

// Read wire fields without decoding/re-encoding opaque component or interaction bytes.
export function fields(bytes: Uint8Array): Map<number, Uint8Array[]> {
    let at = 0;
    const result = new Map<number, Uint8Array[]>();
    function varint() {
        let value = 0n;
        for (let shift = 0n; shift < 70n; shift += 7n) {
            assert(at < bytes.length, 'Truncated varint');
            const byte = bytes[at++];
            assert(shift !== 63n || byte <= 1, 'Varint overflow');
            value |= BigInt(byte & 127) << shift;
            if (!(byte & 128)) return value;
        }
        throw Error('Varint overflow');
    }
    while (at < bytes.length) {
        const key = varint(), number = Number(key >> 3n), wire = Number(key & 7n);
        assert(number > 0 && number < 2 ** 29, 'Invalid field number');
        if (wire === 0) { varint(); continue; }
        const length = wire === 1 ? 8n : wire === 5 ? 4n : wire === 2 ? varint() : -1n;
        assert(length >= 0n && length <= BigInt(bytes.length - at), 'Invalid wire type or truncated field');
        const end = at + Number(length);
        if (wire === 2) {
            const values = result.get(number) || [];
            values.push(bytes.slice(at, end)); result.set(number, values);
        }
        at = end;
    }
    return result;
}

const one = (map: Map<number, Uint8Array[]>, number: number) => {
    const values = map.get(number) || [];
    assert.equal(values.length, 1, `Expected one field ${number}`);
    return values[0];
};

export function verifyArchive(original: Uint8Array, archive: Uint8Array) {
    const header = Buffer.from('SESSION-VIEWER\x01\n');
    assert.deepEqual(Buffer.from(archive.slice(0, header.length)), header);
    const saved = fields(archive.slice(header.length));
    const documents = saved.get(1) || [];
    assert.equal(documents.length, 1, 'Fixture archive must retain one source document');
    JSON.parse(Buffer.from(one(saved, 2)).toString('utf8'));
    const before = fields(original), after = fields(documents[0]);
    const beforeObjects = fields(one(before, 3)), afterObjects = fields(one(after, 3));
    for (const field of [15, 16, 18])
        assert.deepEqual(afterObjects.get(field) || [], beforeObjects.get(field) || [],
            `Element/component/instance payload changed: Objects field ${field}`);
    for (const field of [8, 9])
        assert.deepEqual(after.get(field) || [], before.get(field) || [],
            `Definition/interaction payload changed: Session field ${field}`);
    return {documents: documents.length, elements: (beforeObjects.get(15) || []).length,
        components: (beforeObjects.get(16) || []).length, instances: (beforeObjects.get(18) || []).length,
        definitions: (before.get(8) || []).length, interactions: (before.get(9) || []).length};
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    const [original, archive] = process.argv.slice(2);
    assert(original && archive, 'Usage: archive-roundtrip.ts ORIGINAL.pb BROWSER-SAVED.session');
    console.log(JSON.stringify(verifyArchive(fs.readFileSync(original), fs.readFileSync(archive))));
}
