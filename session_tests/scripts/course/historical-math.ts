import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {canonicalCheckpoint, docs, edited, expected, fingerprint, hash, type Files} from './model.ts';

const bytes = fs.readFileSync(fileURLToPath(new URL('./fixtures/cumulative-math-historical-delta.json', import.meta.url)));
assert.equal(hash(bytes), '06125eaa86ecf17e115e7b3296835fbcbc28e6c8cba5c144c06f61a40754b26e');
const delta = JSON.parse(bytes.toString('utf8'));
const recordFor = (id: string) => {
    const record = delta.records.find(row => row.id === canonicalCheckpoint(id));
    assert(record, `No reviewed historical delta for ${id}`);
    return record;
};

// Reconstruct historical assertions without transferring their runtime acceptance.
export function historicalExpected(id: string, current: Files = expected(id)): Files {
    const record = recordFor(id), files = {...current};
    for (const index of record.changes) {
        const change = delta.changes[index];
        if (change.remove) files[change.path] = files[change.path]?.replaceAll('\r\n', '\n');
    }
    assert.equal(fingerprint(files), record.after, `${id}: unreviewed cumulative source change`);
    for (const index of record.changes) {
        const change = delta.changes[index];
        assert.equal(hash(files[change.path]), change.currentSha256, change.path);
        if (change.remove) delete files[change.path];
        else {
            for (const edit of change.edits) {
                files[change.path] = edited(files[change.path],
                    {path: change.path, before: edit.before, snippet: '', why: ''}, edit.after);
            }
            assert.equal(hash(files[change.path]), change.previousSha256, change.path);
        }
    }
    assert.equal(fingerprint(files), record.before, `${id}: historical fingerprint changed`);
    return files;
}

export function historicalLockFor(id: string): string {
    const record = recordFor(id), file = path.join(docs, record.lock);
    assert.equal(hash(fs.readFileSync(file)), record.lockSha256, `${id}: historical lock changed`);
    return file;
}
