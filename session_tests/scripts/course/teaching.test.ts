import assert from 'node:assert/strict';
import test from 'node:test';
import {hash} from './model.ts';
import {auditTeaching, type Teaching} from './teaching.ts';

const source = 'pub fn prepare() {}\n';
const paragraph = 'The cache retains shaped glyphs when placement changes because placement does not change the glyph sequence. Replacing fonts invalidates the atlas because its glyph identifiers belong to the previous font database.';
const entry: Teaching = {path: 'src/text.rs', owner: 'geometry', sourceSha256: hash(source),
    explanation: {page: 'journey/text.md', heading: '### Cache ownership', paragraph}};
const sources = {'src/text.rs': source};
const owners = new Map([['src/text.rs', 'geometry']]);
const audit = (entries: Teaching[], page = `### Cache ownership\n\n${paragraph}\n`) =>
    auditTeaching(entries, sources, {'journey/text.md': page}, owners);

test('teaching requires a source-bound explanation, not inventory membership', () => {
    assert.equal(audit([entry]).explained, 1);
    const {explanation, ...inventory} = entry;
    assert.deepEqual(audit([inventory]).missing, ['src/text.rs']);
    assert.match(audit([]).errors.join('\n'), /Unaccounted/);
});

test('code, links, wrong sections, changed source and duplicate owners cannot establish coverage', () => {
    for (const page of [`### Cache ownership\n\n\`\`\`rust\n${paragraph}\n\`\`\``,
        `### Cache ownership\n\n[${paragraph}](source.rs)`,
        `### Cache ownership\n\n### Other section\n\n${paragraph}`])
        assert.match(audit([entry], page).errors.join('\n'), /Missing substantive/);
    assert.match(audit([{...entry, sourceSha256: hash(source + 'changed')}]).errors.join('\n'), /changed source/);
    assert.match(audit([{...entry, owner: 'application'}]).errors.join('\n'), /Wrong introduction owner/);
    assert.match(audit([entry, entry]).errors.join('\n'), /Duplicate/);
});

test('a source link may accompany substantive prose but cannot supply it', () => {
    const mixed = `Read [the source](source.rs). ${paragraph}`;
    const withLink = {...entry, explanation: {...entry.explanation!, paragraph: mixed}};
    assert.equal(audit([withLink], `### Cache ownership\n\n${mixed}\n`).explained, 1);
    const onlyLink = {...entry, explanation: {...entry.explanation!, paragraph: `[${paragraph}](source.rs)`}};
    assert.equal(audit([onlyLink], `### Cache ownership\n\n[${paragraph}](source.rs)`).explained, 0);
    assert.match(auditTeaching([entry], sources, {'journey/text.md': `### Cache ownership\n\n${paragraph}`}, new Map()).errors.join('\n'), /Wrong introduction owner/);
});
