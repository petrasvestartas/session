import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {hash, resolveJourneyImage} from './model.ts';

function fixture(t: any) {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'journey-image-alias-'));
    t.after(() => { assert(path.dirname(root) === path.resolve(os.tmpdir())); fs.rmSync(root, {recursive: true}); });
    const bytes = Buffer.from('retained PNG bytes'), digest = hash(bytes), canonical = path.join(root, 'kept.png');
    fs.writeFileSync(canonical, bytes);
    const aliases: Record<string, any> = {'removed.png': {canonical: 'kept.png', sha256: digest}};
    const save = () => fs.writeFileSync(path.join(root, 'image-aliases.json'), JSON.stringify({version: 1, aliases}));
    save();
    return {root, bytes, digest, canonical, aliases, save,
        resolve: (file: string, expected?: string) => resolveJourneyImage(file, expected, root)};
}

test('missing relative and absolute names resolve to the same verified physical bytes', t => {
    const f = fixture(t);
    assert.equal(f.resolve('removed.png', f.digest), f.canonical);
    assert.equal(f.resolve(path.join(f.root, 'removed.png')), f.canonical);
    assert.deepEqual(fs.readFileSync(f.resolve('removed.png')), f.bytes);
    assert(!fs.existsSync(path.join(f.root, 'removed.png')));
});

test('fresh physical recapture wins, while an expected hash is still enforced', t => {
    const f = fixture(t), fresh = path.join(f.root, 'removed.png');
    fs.writeFileSync(fresh, 'fresh image');
    assert.equal(f.resolve('removed.png'), fresh);
    assert.equal(f.resolve('removed.png', hash('fresh image')), fresh);
    assert.throws(() => f.resolve('removed.png', f.digest), /hash mismatch/);
});

test('changed canonical bytes or mismatched immutable receipt hash fail closed', t => {
    const f = fixture(t);
    assert.throws(() => f.resolve('removed.png', hash('other receipt')), /alias hash mismatch/);
    fs.writeFileSync(f.canonical, 'changed canonical');
    assert.throws(() => f.resolve('removed.png'), /hash mismatch/);
});

test('missing canonical file is not replaced by another alias or arbitrary file', t => {
    const f = fixture(t);
    fs.unlinkSync(f.canonical);
    assert.throws(() => f.resolve('removed.png'), {code: 'ENOENT'});
});

test('alias targets reject traversal, absolute paths, backslashes and non-PNG names', t => {
    const f = fixture(t);
    for (const canonical of ['../outside.png', '/outside.png', 'C:/outside.png', 'nested\\other.png', 'nested/../kept.png', './kept.png', 'kept.json', 'kept.png?x']) {
        f.aliases['removed.png'].canonical = canonical; f.save();
        assert.throws(() => f.resolve('removed.png'), /Invalid journey image alias/);
    }
    for (const requested of ['../outside.png', './removed.png', 'nested/../removed.png'])
        assert.throws(() => f.resolve(requested), /Invalid journey image name/);
    assert.throws(() => f.resolve(path.join(f.root, '..', 'missing-outside.png')), /outside journey/);
});

test('chains and malformed SHA declarations are rejected', t => {
    const f = fixture(t);
    f.aliases['kept.png'] = {canonical: 'other.png', sha256: f.digest}; f.save();
    assert.throws(() => f.resolve('removed.png'), /Invalid journey image alias/);
    delete f.aliases['kept.png']; f.aliases['removed.png'].sha256 = 'bad'; f.save();
    assert.throws(() => f.resolve('removed.png'), /Invalid journey image alias/);
});

test('nested journey aliases resolve without changing logical filenames', t => {
    const f = fixture(t);
    f.aliases['review/evidence.png'] = {canonical: 'kept.png', sha256: f.digest}; f.save();
    assert.equal(f.resolve('review/evidence.png', f.digest), f.canonical);
});

test('existing evidence outside journey remains physical and hash checked', t => {
    const f = fixture(t), elsewhere = path.join(f.root, 'not-a-png.json');
    fs.writeFileSync(elsewhere, 'proof');
    assert.equal(f.resolve(elsewhere, hash('proof')), elsewhere);
    assert.throws(() => f.resolve(elsewhere, hash('changed')), /hash mismatch/);
});
