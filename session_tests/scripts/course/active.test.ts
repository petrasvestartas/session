import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {activeCourse, type ActiveCourse} from '../../src/activeCourse.ts';
import {completionFiles, completionSubjects} from './completion.ts';
import {docs} from './model.ts';
import {auditCompletion} from './completion-audit.ts';
import {latestCodingLesson} from '../../src/courseOrder.ts';

const load = (): ActiveCourse => JSON.parse(fs.readFileSync(path.join(docs, 'journey/active-course.json'), 'utf8'));

test('the active route selects every actual cumulative assembly exactly once', () => {
    const route = activeCourse(load());
    assert.deepEqual(route.subjects.map(subject => subject.id), [...completionSubjects]);
    for (const [index, subject] of route.subjects.entries()) {
        assert.deepEqual(subject.prerequisites, index ? [route.subjects[index - 1].id] : []);
        const files = completionFiles(subject.id);
        assert(files['Cargo.toml'] && files['src/lib.rs']);
        for (const page of [subject.page, ...subject.chapters]) {
            const text = fs.readFileSync(path.join(docs, page), 'utf8');
            for (const heading of ['Prerequisites', 'Build', 'Run and observe', 'Predict and try', 'Question and answer', 'Validation'])
                assert(text.includes(heading), `${page}: ${heading}`);
        }
    }
    const legacy = JSON.parse(fs.readFileSync(path.join(docs, route.historicalManifest), 'utf8'));
    assert(legacy.steps.length > route.subjects.length);
    assert(legacy.steps.every((step: {edits?: unknown[]}) => Array.isArray(step.edits)));
});

test('active routing rejects duplicates, forward prerequisites and unsafe page paths', () => {
    for (const mutate of [
        (route: ActiveCourse) => { route.subjects[1].id = route.subjects[0].id; },
        (route: ActiveCourse) => { route.subjects[0].prerequisites = ['application']; },
        (route: ActiveCourse) => { route.subjects[0].page = '../outside.md'; },
        (route: ActiveCourse) => { route.subjects[1].page = route.subjects[0].page; },
    ]) {
        const route = load(); mutate(route);
        assert.throws(() => activeCourse(route));
    }
});

test('the production checkpoint link cannot select an unrelated historical endpoint', () => {
    assert.equal(latestCodingLesson({
        'journey/old-last': {prev: 'journey/old-first'},
        'journey/completion-application': {prev: 'journey/completion-decoding', next: 'journey/completion-documents'},
    }), 'journey/completion-application');
});

test('active audit rejects rewritten implementation and a missing endpoint source', () => {
    assert(auditCompletion().passed);
    const rewritten = auditCompletion(subject => {
        const files = completionFiles(subject);
        if (subject === 'painting') files['src/app/launch.rs'] += '\nfn hidden_rewrite() {}\n';
        return files;
    });
    assert(!rewritten.passed && rewritten.failures.some(failure => failure.includes('launch.rs')));
    const missing = auditCompletion(subject => {
        const files = completionFiles(subject);
        if (subject === 'application') delete files['src/app/runtime.rs'];
        return files;
    });
    assert(!missing.passed && missing.missing.includes('src/app/runtime.rs'));
});
