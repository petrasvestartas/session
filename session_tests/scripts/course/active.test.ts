import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {activeCourse, type ActiveCourse} from '../../src/activeCourse.ts';
import {completionFiles, completionSubjects} from './completion.ts';
import {docs, hash} from './model.ts';
import {auditCompletion, auditAcceptance} from './completion-audit.ts';
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

test('accepted subjects require a pinned review and accepted prerequisites', () => {
    const route = load();
    for (const subject of route.subjects) { subject.acceptance = 'pending'; delete subject.review; }
    route.subjects[0].acceptance = 'accepted';
    assert.throws(() => activeCourse(route), /Missing review/);
    route.subjects[0].review = {path: 'journey/release/frame-review.json', sha256: 'a'.repeat(64)};
    assert.equal(activeCourse(route).subjects[0].acceptance, 'accepted');
    route.subjects[2].acceptance = 'accepted';
    route.subjects[2].review = route.subjects[0].review;
    assert.throws(() => activeCourse(route), /accepted prerequisite/);
    route.subjects[2].acceptance = 'pending'; delete route.subjects[2].review;
    route.subjects[0].review.path = '../outside.json';
    assert.throws(() => activeCourse(route), /Missing review/);
});

test('acceptance audit rejects changed review, source and teaching inputs', () => {
    const course = load(); course.subjects = [course.subjects[0]];
    const subject = course.subjects[0], files = {'src/frame.rs': 'pub fn draw() {}\n'};
    const lesson = '# Frame\nExplain submission and presentation.\n';
    const proofPath = 'journey/release/browser-review.json', proof = '{"observed":"white frame"}';
    const raw = JSON.stringify({status: 'accepted', subject: subject.id,
        files: {'src/frame.rs': hash(files['src/frame.rs'])},
        lesson: {path: subject.page, sha256: hash(lesson)}, evidence: [{path: proofPath, sha256: hash(proof)}]});
    subject.acceptance = 'accepted';
    subject.review = {path: 'journey/release/frame-review.json', sha256: hash(raw)};
    const read = (name: string) => name === subject.page ? lesson : name === proofPath ? proof : raw;
    assert.deepEqual(auditAcceptance(course, () => files, read), []);
    assert.match(auditAcceptance(course, () => files, name => read(name) + ' ').join(), /review digest changed/);
    assert.match(auditAcceptance(course, () => ({'src/frame.rs': 'changed'}), read).join(), /checkpoint source changed/);
    assert.match(auditAcceptance(course, () => files, name => name === subject.page ? 'changed' : raw).join(), /lesson changed/);
    assert.match(auditAcceptance(course, () => files, name => name === proofPath ? 'changed' : read(name)).join(), /supporting evidence changed/);
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
