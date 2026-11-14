import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {course, docs, hash} from './model.ts';
import {generate} from './render.ts';

test('generation preserves authored subject explanations and named source listings', () => {
    const subjects = course().steps.filter(step => step.authored);
    assert(subjects.length >= 3);
    const before = subjects.map(step => fs.readFileSync(path.join(docs, step.page)));
    subjects.forEach((step, i) => assert(before[i].toString('utf8').startsWith('# '), `${step.id}: visible Markdown title`));
    generate(subjects.map(step => step.id));
    subjects.forEach((step, i) => assert.deepEqual(fs.readFileSync(path.join(docs, step.page)), before[i], step.id));
});

test('navigation generation preserves every active lesson and consolidated reader', () => {
    const plan = JSON.parse(fs.readFileSync(path.join(docs, 'journey/book.json'), 'utf8'));
    const pages = [...new Set([
        ...course().steps.map(step => step.page),
        ...plan.chapters.flatMap(chapter => (chapter.consolidated || []).map(subject => subject.page)),
    ])];
    const before = new Map(pages.map(page => [page, hash(fs.readFileSync(path.join(docs, page)))]));
    generate(undefined, true);
    for (const [page, digest] of before) assert.equal(hash(fs.readFileSync(path.join(docs, page))), digest, page);
    const contents = fs.readFileSync(path.join(docs, 'journey/contents.md'), 'utf8');
    assert(contents.includes(course().steps.length.toLocaleString('en-US') + ' active runnable checkpoints'));
    const overview = fs.readFileSync(path.join(docs, 'journey.md'), 'utf8');
    assert(overview.includes(course().steps.length.toLocaleString('en-US') + ' active runnable checkpoints'));
});
