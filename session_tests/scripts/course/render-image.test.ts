import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {resultImage} from './render.ts';
import {docs, hash} from './model.ts';

test('equal-byte checkpoint images retain their own reader filename and caption', t => {
    const files = ['33b-cache-browser.png', '60b-coverage.png'];
    const bytes = Buffer.from('identical recorded image bytes');
    const reads: string[] = [];
    t.mock.method(fs, 'existsSync', (file: string) => files.some(name => file === path.join(docs, 'screenshots/journey', name)));
    t.mock.method(fs, 'readFileSync', (file: string) => {
        assert(files.some(name => file === path.join(docs, 'screenshots/journey', name)));
        reads.push(file);
        return bytes;
    });
    assert.equal(resultImage(files[0], 'Chapter four owner'),
        '![Chapter four owner](../screenshots/journey/33b-cache-browser.png)');
    assert.equal(resultImage(files[1], 'Chapter ten labels'),
        '![Chapter ten labels](../screenshots/journey/60b-coverage.png)');
    assert.equal(resultImage(files[0], 'Repeated owner within its chapter'),
        '![Repeated owner within its chapter](../screenshots/journey/33b-cache-browser.png)');
    assert.deepEqual(reads, [files[0], files[1], files[0]].map(file => path.join(docs, 'screenshots/journey', file)));
});

test('a missing requested image still fails instead of borrowing a previous image', t => {
    const missing = path.join(docs, 'screenshots/journey', 'missing-reader-owner.png');
    const error = Object.assign(new Error('missing requested image'), {code: 'ENOENT'});
    t.mock.method(fs, 'existsSync', (file: string) => file === path.join(docs, 'screenshots/journey', '33b-cache-browser.png'));
    t.mock.method(fs, 'readFileSync', (file: string) => {
        if (file === missing) throw error;
        return Buffer.from('same image bytes');
    });
    resultImage('33b-cache-browser.png', 'Available');
    assert.throws(() => resultImage('missing-reader-owner.png', 'Missing'), actual => actual === error);
});

test('an explicit verified alias renders the canonical URL and preserves the requested caption', t => {
    const root = path.join(docs, 'screenshots/journey'), canonical = path.join(root, 'kept.png');
    const bytes = Buffer.from('canonical image');
    t.mock.method(fs, 'existsSync', (file: string) => [canonical, path.join(root, 'image-aliases.json')].includes(file));
    t.mock.method(fs, 'realpathSync', (file: string) => file);
    t.mock.method(fs, 'lstatSync', () => ({isFile: () => true}));
    t.mock.method(fs, 'readFileSync', (file: string) => file === canonical ? bytes : JSON.stringify({version: 1, aliases: {
        'removed.png': {canonical: 'kept.png', sha256: hash(bytes)},
    }}));
    assert.equal(resultImage('removed.png', 'Requested subject'), '![Requested subject](../screenshots/journey/kept.png)');
});
