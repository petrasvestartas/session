import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {read, json, hash, docs, viewer, filesIn, referenceFiles, reference, write, materialize, referenceAssets} from './model.ts';
import type {Files} from './model.ts';
import {runCommand, evidence} from './verify.ts';
import {ppm} from './png.ts';

function originalFingerprint(files: Files) {
    const entries = Object.keys(files).sort().map(name => `${JSON.stringify(name)}: ${JSON.stringify(files[name])}`);
    const text = `{${entries.join(', ')}}`.replace(/[^\x00-\x7f]/g, char => `\\u${char.charCodeAt(0).toString(16).padStart(4, '0')}`);
    return hash(text);
}

export function verifyReference(ids: string[], stored: boolean) {
    const known = reference().chapters.map((step: {id: string}) => step.id);
    assert(ids.every(id => known.includes(id)), 'Unknown reference checkpoint');
    const report = path.join(viewer, 'target/course-steps/results.json');
    const previous = fs.existsSync(report) ? json(report) : {};
    const checker = path.join(docs, 'journey/reference_pixels.rs');
    const executable = path.join(evidence, 'reference-pixels');
    if (!stored) runCommand('reference-pixels-compile', ['rustc', '--edition', '2024', checker, '-o', executable], viewer);
    for (const id of ids.length ? ids : known) {
        const files = referenceFiles(id);
        if (stored) {
            const record = previous[id];
            assert(record, `${id}: no stored evidence`);
            if (record.checker) assert.equal(record.checker, hash(read(checker)), 'Pixel checker changed');
            assert.equal(record.source, originalFingerprint(files), `${id}: reference evidence is stale`);
            assert(record.commands.every((command: {returncode: number}) => command.returncode === 0));
            assert(record.commands.some((command: {name: string}) => command.name === 'build'));
            assert(record.commands.some((command: {name: string}) => command.name === 'tests'));
            if (id !== '00') assert(record.frame.width === 640 && record.frame.height === 480);
            if (id === '01') assert.notEqual(record.grey.sha256, record.frame.sha256);
            if (id === '37') assert(record.scene.visible_objects >= 2 && record.scene.ink > 100 && record.scene.ink < 900 * 700 * 0.9);
            console.log(`Reference ${id}: stored build and rendering evidence matches`);
            continue;
        }
        delete previous[id];
        write(report, JSON.stringify(previous, null, 2) + '\n');
        const target = path.join(viewer, 'target/course-reference');
        fs.rmSync(target, {recursive: true, force: true});
        materialize(files, target, path.join(docs, 'lessons/37/Cargo.lock'));
        referenceAssets(target);
        const record: any = {source: originalFingerprint(files), checker: hash(read(checker)), commands: []};
        const run = (name: string, args: string[], env = {}) => {
            record.commands.push({...runCommand(`reference-${id}-${name}`, args, target, env), name});
        };
        const inspect = (mode: string, file: string, extra: string[] = []) => {
            const output = path.join(evidence, `reference-${id}-${mode}.json`);
            run(`pixels-${mode}`, [executable, mode, path.join(target, file), output, ...extra]);
            return {...json(output), sha256: hash(ppm(path.join(target, file)).pixels)};
        };
        run('build', ['cargo', 'build', '--lib', '--locked', '-j4']);
        run('tests', ['cargo', 'xtest', '--lib', '--locked', '-j4']);
        const frame = ['cargo', 'run', '--example', 'course_frame', '--locked', '--target', 'x86_64-unknown-linux-gnu', '-j4'];
        if (id !== '00') {
            run('frame', frame);
            record.frame = inspect('white', 'lesson.ppm');
        }
        if (id === '01') {
            const shader = path.join(target, 'src/shaders/background.wgsl');
            const original = read(shader);
            assert(original.includes('select(1.0,0.94,'));
            try {
                write(shader, original.replace('select(1.0,0.94,', 'select(0.5,0.94,'));
                run('grey', frame);
                record.grey = inspect('grey', 'lesson.ppm');
                assert.notEqual(record.frame.sha256, record.grey.sha256);
            } finally {
                write(shader, original);
            }
        }
        if (id === '37') {
            const ids = path.join(evidence, 'reference-37.ids');
            run('scene', ['cargo', 'run', '--example', 'selftest', '--locked', '--target', 'x86_64-unknown-linux-gnu', '-j4',
                '--', 'final.ppm', 'assets/view_local.yaml'], {VIEWER_IDS: ids});
            const objects = Object.values(json(ids + '.json')) as {object_id: number}[];
            record.scene = inspect('scene', 'final.ppm', [ids, objects.map(object => object.object_id).join(',')]);
        }
        previous[id] = record;
        write(report, JSON.stringify(previous, null, 2) + '\n');
        console.log(`Reference ${id}: compiled, native tests and rendering assertions passed`);
    }
}

export function rustCode(text: string): string {
    let out = '', i = 0;
    while (i < text.length) {
        if (text.startsWith('//', i)) {
            const end = text.indexOf('\n', i);
            i = end < 0 ? text.length : end;
        } else if (text.startsWith('/*', i)) {
            let depth = 1;
            i += 2;
            while (depth && i < text.length) {
                if (text.startsWith('/*', i)) { depth++; i += 2; }
                else if (text.startsWith('*/', i)) { depth--; i += 2; }
                else i++;
            }
        } else {
            const raw = text.slice(i).match(/^r(#+)?"/);
            if (raw && (i === 0 || !/[\w]/.test(text[i - 1]))) {
                const closing = '"' + (raw[1] || '');
                const end = text.indexOf(closing, i + raw[0].length);
                assert(end >= 0, 'Unclosed raw string');
                out += text.slice(i, end + closing.length);
                i = end + closing.length;
            } else if (text[i] === '"') {
                let end = i + 1;
                while (end < text.length && text[end] !== '"') end += text[end] === '\\' ? 2 : 1;
                out += text.slice(i, end + 1);
                i = end + 1;
            } else {
                const char = text.slice(i).match(/^'(?:\\(?:u\{[\da-fA-F]+\}|x[\da-fA-F]{2}|.)|[^'\\\n])'/);
                if (char) { out += char[0]; i += char[0].length; }
                else out += text[i++];
            }
        }
    }
    return out.split('\n').map(line => line.trimEnd()).filter(line => line.trim()).join('\n');
}

export function parity() {
    const production = path.join(viewer, 'src');
    const final = path.join(docs, 'lessons/37/src');
    const names = filesIn(production).map(file => path.relative(production, file)).sort();
    assert.deepEqual(names, filesIn(final).map(file => path.relative(final, file)).sort());
    for (const name of names) {
        assert.equal(rustCode(read(path.join(final, name))), rustCode(read(path.join(production, name))), `Final source differs: ${name}`);
    }
    console.log(`All ${names.length} Rust/WGSL source files match the current viewer after teaching comments are removed.`);
}
