import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {course, docs, edited, expected, hash, snippetSource, viewer} from './model.ts';

const core = 'actions check deck draw eval font geometry layout matrix morph net object paint run tex tween view presentation math_mode'.split(' ');
const production = ['src/app/mode.rs', ...core.map(name => `src/app/stage/${name}.rs`), 'src/app/stage/tests.rs', 'src/app/stage/presentation_tests.rs'];
const source = (text: string) => text.replaceAll('\r\n', '\n').split('\n').filter(line => !line.includes('--8<-- [')).join('\n');

test('cumulative mathematics follows first painting and teaches the complete production implementations', () => {
    const steps = course().steps;
    const index = steps.findIndex(step => step.id === '03a-mathematics');
    assert.equal(index, steps.findIndex(step => step.id === '03-triangle') + 1);
    assert(index > 0);
    const files = expected('03a-mathematics');
    const declarations = (text: string) => text.split(/\r?\n/).filter(line => /^(pub )?mod \w+;$/.test(line));
    assert.deepEqual(declarations(files['src/app/stage/mod.rs']).filter(line => line !== 'mod motion_tests;'),
        declarations(fs.readFileSync(path.join(viewer, 'src/app/stage/mod.rs'), 'utf8')).filter(line => !['mod mode;', 'pub mod spatial;'].includes(line)));
    for (const name of production) {
        let lesson = source(files[name]);
        for (const logo of ['logo-eth.png', 'logo-brg.png']) {
            lesson = lesson.replace(`include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stage/${logo}"))`, `include_bytes!("../../../assets/stage/${logo}")`);
        }
        assert.equal(lesson, source(fs.readFileSync(path.join(viewer, name), 'utf8')), name);
    }
});

test('later subjects preserve finished mathematics implementations and the fixed startup dispatcher', () => {
    const finished = expected('03a-mathematics');
    const retained = new Set([...production, 'src/math_tests.rs', 'src/app/stage/motion_tests.rs', 'src/app/stage/tests.rs', 'src/app/stage/presentation_tests.rs', 'examples/course/fonts.rs', 'examples/course/mathematics.rs', 'examples/course/entry.rs']);
    const files = {...finished};
    const steps = course().steps;
    for (const step of steps.slice(steps.findIndex(step => step.id === '03a-mathematics') + 1)) {
        for (const edit of step.edits) {
            files[edit.path] = edited(files[edit.path], edit, snippetSource(edit.snippet), step.id);
            if (retained.has(edit.path)) assert.equal(files[edit.path], finished[edit.path], `${step.id}: finished ${edit.path} changed`);
        }
    }
});

test('cumulative mathematics browser evidence binds its implemented modules and published original images', () => {
    const receipt = JSON.parse(fs.readFileSync(path.join(docs, 'journey/release/cumulative-mathematics-registration-windows-20261114.json'), 'utf8'));
    const files = expected('03a-mathematics');
    for (const name of Object.keys(files)) {
        assert.equal(hash(files[name].replaceAll('\r\n', '\n')), receipt.normalizedSourceSha256[name], name);
    }
    for (const image of receipt.publishedImages) assert.equal(hash(fs.readFileSync(path.join(docs, image.file))), image.sha256, image.file);
    assert.equal(receipt.native.tests, 27);
    assert.equal(receipt.native.failed, 0);
    assert.equal(receipt.native.ignored, 0);
});
