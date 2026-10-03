// Test-only reader for the exact source-coordinate fields in the checked-in protobuf schema.
const assert = require('node:assert/strict');
function fields(bytes) {
    let at = 0; const out = [];
    const varint = () => {
        let value = 0n;
        for (let shift = 0n; shift < 70n; shift += 7n) {
            assert(at < bytes.length, 'Truncated varint'); const byte = bytes[at++];
            value |= BigInt(byte & 127) << shift;
            if (!(byte & 128)) return value;
        }
        throw new Error('Oversized varint');
    };
    while (at < bytes.length) {
        const tag = Number(varint()), field = tag >>> 3, wire = tag & 7; assert(field > 0);
        if (wire === 0) out.push({field, wire, value: varint()});
        else {
            const length = wire === 1 ? 8 : wire === 5 ? 4 : wire === 2 ? Number(varint()) : -1;
            assert(length >= 0 && Number.isSafeInteger(length) && at + length <= bytes.length, 'Invalid field length');
            out.push({field, wire, value: bytes.subarray(at, at + length)}); at += length;
        }
    }
    return out;
}
function coordinates(bytes) {
    const objects = fields(bytes).find(row => row.field === 3 && row.wire === 2); assert(objects);
    return fields(objects.value).filter(row => row.field === 9 && row.wire === 2).map(row => {
        const mesh = fields(row.value); const name = mesh.find(row => row.field === 2)?.value.toString() || '';
        const vertices = mesh.filter(row => row.field === 3 && row.wire === 2).map(row => {
            const entry = fields(row.value), key = entry.find(row => row.field === 1)?.value || 0n;
            const vertex = entry.find(row => row.field === 2 && row.wire === 2); assert(vertex);
            const values = fields(vertex.value);
            const xyz = [1, 2, 3].map(field => {
                const encoded = values.find(row => row.field === field && row.wire === 1);
                const value = encoded ? encoded.value.readDoubleLE() : 0;
                assert(Number.isFinite(value)); return value;
            });
            return [key.toString(), xyz];
        }).sort((a, b) => a[0].localeCompare(b[0]));
        return {name, vertices};
    }).sort((a, b) => a.name.localeCompare(b.name));
}
module.exports = coordinates;
