import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {docs, viewer, reference, json, read, hash, lines, filesIn} from './model.ts';
import type {Files} from './model.ts';

function snapshot(id: string, names: string[]): Files {
    const files: Files = {};
    for (const name of names) {
        const file = path.join(docs, 'lessons', id, name);
        if (!fs.existsSync(file)) continue;
        let text = lines(read(file)).filter(line => !/^\s*(\/\/|#|<!--|\/\*)\s*--8<-- \[(start|end):/.test(line)).join('');
        text = text.replaceAll('../../../../session_rust', '../../../session_rust').replace(/Checkpoint \d\d[a-z]?\b/g, 'Course viewer');
        if (name === '.gitignore' && !text.split('\n').includes('target/')) text = 'target/\n' + text;
        if (name === 'Trunk.toml') text = text.replace(/dist = "[^"\n]+"/, 'dist = "dist"').replace('port = 8770', 'port = 8780')
            .replace(/watch = \[.*\]/, 'watch = ["src", "Cargo.toml", "index.html", "assets", "../../../session_rust/src"]');
        if (name === '.cargo/config.toml') text = text.replace('RUST_TEST_THREADS = "4"', 'RUST_TEST_THREADS = "1"');
        if (name === 'assets/view_local.yaml') text = 'name: Course specimen\nitems:\n  - file: pb/course-boxes.pb\n    name: Boxes\n';
        if (name === 'index.html') {
            for (const asset of ['assets/text-quality.html', 'assets/view_local.yaml']) {
                if (!fs.existsSync(path.join(docs, 'lessons', id, asset))) text = lines(text).filter(line => !line.includes(`href="${asset}"`)).join('');
            }
        }
        files[name] = text;
    }
    return files;
}

export function referenceStructure() {
    const data = reference();
    const notes = json(path.join(docs, 'chapters.json'));
    const files: Files = {};
    const pages = new Map<string, string>();
    const images = new Set<string>();
    const names = Object.keys(data.files);
    const last = new Map(data.chapters.map((chapter: any) => [chapter.last, chapter.id]));
    for (const chapter of data.chapters) {
        const page = read(path.join(docs, chapter.page));
        pages.set(chapter.id, page);
        assert(/^# [^\n]+\n\n\*\*Estimated study time: about \d+–\d+ hours\./.test(page), chapter.id);
        for (const key of ['local', 'global', 'trace', 'question', 'answer']) assert(page.includes(notes[chapter.id].guide[key]), `${chapter.id}: ${key}`);
        assert(page.includes(`reference-check ${chapter.id}`));
        assert(page.includes('cargo build --lib --locked -j4'));
        for (const match of page.matchAll(/!\[[^\]]*\]\(([^)]+)\)/g)) {
            const digest = hash(fs.readFileSync(path.join(docs, match[1])));
            assert(!images.has(digest), `Repeated reference image: ${match[1]}`);
            images.add(digest);
        }
    }
    for (const edit of data.lessons) {
        const old = files[edit.path] || '';
        assert.equal(hash(old), edit.before, `${edit.id}: input`);
        const code = read(path.join(docs, 'typing', edit.snippet));
        assert.equal(code, edit.code, `${edit.id}: listing`);
        const text = lines(old);
        assert(edit.at >= 0 && edit.at <= text.length, `${edit.id}: insertion position`);
        text.splice(edit.at, 0, ...lines(code));
        files[edit.path] = text.join('');
        assert.equal(hash(files[edit.path]), edit.after, `${edit.id}: output`);
        const page = pages.get(edit.chapter)!;
        const sections = page.split(`<span id="code-${edit.id}"></span>`);
        assert.equal(sections.length, 2, `${edit.id}: unique displayed section`);
        if (code) assert(sections[1].split('<span id="code-')[0].includes(`--8<-- "typing/${edit.snippet}"`), edit.id);
        const chapter = last.get(edit.id);
        if (chapter) assert.deepEqual(files, snapshot(chapter as string, names), `Checkpoint ${chapter}`);
    }
    assert.deepEqual(Object.fromEntries(Object.entries(files).map(([name, text]) => [name, hash(text)])), data.files);
    const production = filesIn(path.join(viewer, 'src')).map(file => path.relative(viewer, file)).sort();
    assert.deepEqual(names.filter(name => name.startsWith('src/')).sort(), production);
    assert(!data.external.some((name: string) => /^(src|examples|tests)\//.test(name)), 'Implementation is hidden in supplied assets');
    console.log(`${data.chapters.length} reference checkpoints match all displayed edits and ${names.length} final files.`);
}
