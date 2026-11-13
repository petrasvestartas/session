import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {course, docs} from './model.ts';
import {generate} from './render.ts';

test('generation preserves authored subject explanations and named source listings', () => {
    const subjects = course().steps.filter(step => step.authored);
    assert(subjects.length >= 3);
    const before = subjects.map(step => fs.readFileSync(path.join(docs, step.page)));
    generate(subjects.map(step => step.id));
    subjects.forEach((step, i) => assert.deepEqual(fs.readFileSync(path.join(docs, step.page)), before[i], step.id));
});
