import {lines, lineChanges} from './line-diff.ts';
export {lines} from './line-diff.ts';
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {fileURLToPath} from 'node:url';

export const viewer = fileURLToPath(new URL('../../../session_viewer/', import.meta.url));
export const docs = path.join(viewer, 'docs');
export const site = fileURLToPath(new URL('../../', import.meta.url));
export const work = path.join(viewer, 'workspace/journey');
export type Files = Record<string, string>;
export interface Edit { path: string; snippet: string; before: string | null; why: string; copy?: boolean; reuse?: string; supplied?: 'validation' | 'example' | 'move'; rename_field?: boolean }
export interface Step {
    id: string; title: string; page: string; hours: [number, number]; goal: string;
    trace: string; question: string; answer: string; edits: Edit[]; story: string;
    result: string; experiment: string; production: string; browser_status?: string;
    checks?: string;
    image_same_as?: string;
    browser_result_status?: string;
    browser_caption?: string;
    browser_check?: string;
    frame?: string;
    frame_constructor?: string;
    frame_size?: [number, number];
    tests?: boolean;
    browser_actions?: ({command: string} | {canvas: [number, number]} | {viewport: [number, number]} | {drag: [[number, number], [number, number]]} | {wheel: number} | {key: string} | {file: string; fit?: boolean})[];
    image_result?: string;
    lock?: string;
    assets?: string[];
}
export interface Course { release: string; inventory?: string; steps: Step[] }

export const read = (file: string) => fs.readFileSync(file, 'utf8');
export const json = (file: string) => JSON.parse(read(file));
export const hash = (value: string | Uint8Array) => crypto.createHash('sha256').update(value).digest('hex');
export const course = (): Course => sequence?.data || json(path.join(docs, 'journey/course.json'));
export const reference = () => json(path.join(docs, 'typing/manifest.json'));
export const sorted = (files: Files) => Object.fromEntries(Object.keys(files).sort().map(name => [name, files[name]]));
export const fingerprint = (files: Files) => hash(JSON.stringify(sorted(files)));

export function addedLines(before: string, after: string): string[] {
    return lineChanges(before, after).filter(line => line.kind === 'add').map(line => line.text);
}

export function edited(original: string | undefined, edit: Edit, code: string, label = 'checkpoint'): string {
    const where = `${label}: ${edit.path}`;
    if (edit.rename_field) {
        const field = /^self(?:\.[A-Za-z_]\w*)+$/;
        if (edit.copy || edit.supplied || edit.reuse || edit.before === null || edit.before !== edit.before.trim() || code !== code.trim()
            || !field.test(edit.before) || !field.test(code)) throw Error(`${where}: field rename requires two field references and must be typed`);
        const pattern = new RegExp('\\b' + edit.before.replaceAll('.', '\\.') + '\\b', 'g');
        if (original === undefined || !pattern.test(original)) throw Error(`${where}: field reference not found`);
        return original.replace(pattern, () => code);
    }
    if (edit.before === null) {
        if (original !== undefined) throw Error(`${where} already exists`);
        return code;
    }
    if (original === undefined) throw Error(`${where} does not exist`);
    if (original.split(edit.before).length !== 2) throw Error(`${where}: replacement must match exactly once`);
    return original.replace(edit.before, () => code);
}

export function typingLoad(step: Step) {
    const added = step.edits.filter(edit => !edit.copy).flatMap(edit => {
        const code = read(path.join(docs, edit.snippet));
        return edit.rename_field ? [edit.before || '', code] : addedLines(edit.before || '', code);
    });
    const characters = [...added.join('')].length;
    return {lines: added.length, characters, minutes: [Math.ceil(characters / 100), Math.ceil(characters / 50)]};
}

export function lockFor(id: string, steps: Step[] = course().steps): string {
    let lock = 'journey/release/Cargo.lock';
    for (const step of steps) {
        lock = step.lock || lock;
        if (step.id === id) return path.join(docs, lock);
    }
    throw Error(`Unknown lesson: ${id}`);
}

export function assetsFor(id: string): string[] {
    const assets = new Set<string>();
    for (const step of course().steps) {
        for (const name of step.assets || []) assets.add(name);
        if (step.id === id) return [...assets];
    }
    throw Error(`Unknown lesson: ${id}`);
}

export function installAssets(names: string[], destination: string) {
    for (const name of names) {
        const source = fs.readFileSync(path.join(viewer, name));
        const target = path.join(destination, name);
        if (fs.existsSync(target)) {
            if (!fs.readFileSync(target).equals(source)) throw Error(`Asset differs: ${target}. Preserve your changed file before installing the course asset.`);
        } else write(target, source);
    }
}

export function dependencies(id: string, project = work) {
    const failures = compare({'Cargo.toml': expected(id)['Cargo.toml']}, project);
    if (failures.length) throw Error(failures.join('\n'));
    installAssets(assetsFor(id), project);
    const lock = path.join(project, 'Cargo.lock');
    const wanted = read(lockFor(id));
    if (read(lock) === wanted) return;
    fs.copyFileSync(lock, `${lock}.before-${id}`, fs.constants.COPYFILE_EXCL);
    write(lock, wanted);
}

export function write(file: string, text: string | Uint8Array) {
    fs.mkdirSync(path.dirname(file), {recursive: true});
    fs.writeFileSync(file, text);
}

export function filesIn(directory: string): string[] {
    return fs.readdirSync(directory, {withFileTypes: true}).flatMap(entry => {
        const file = path.join(directory, entry.name);
        return entry.isDirectory() ? filesIn(file) : [file];
    });
}

let sequence: {data: Course; at: number; files: Files} | undefined;

export function withExpectedSequence<T>(run: () => T): T {
    // Scope a synchronous pass that does not edit course inputs.
    const previous = sequence;
    sequence = {data: json(path.join(docs, 'journey/course.json')), at: -1, files: {}};
    try { return run(); }
    finally { sequence = previous; }
}

export function expected(id: string): Files {
    const state = sequence || {data: course(), at: -1, files: {}};
    const end = state.data.steps.findIndex(step => step.id === id);
    if (end < 0) throw Error(`Unknown lesson: ${id}`);
    if (end < state.at) { state.at = -1; state.files = {}; }
    const files = state.files;
    for (let index = state.at + 1; index <= end; index++) {
        const step = state.data.steps[index];
        for (const edit of step.edits) {
            const code = read(path.join(docs, edit.snippet));
            files[edit.path] = edited(files[edit.path], edit, code, step.id);
        }
        state.at = index;
    }
    return {...files};
}

export function referenceFiles(id: string): Files {
    const data = reference();
    const stop = data.chapters.find((step: {id: string}) => step.id === id)?.last || id;
    const files: Files = {};
    for (const edit of data.lessons) {
        const text = lines(files[edit.path] || '');
        text.splice(edit.at, 0, ...lines(edit.code));
        files[edit.path] = text.join('');
        if (edit.id === stop) return files;
    }
    throw Error(`Unknown reference checkpoint: ${id}`);
}

export function compare(files: Files, target: string): string[] {
    const failures: string[] = [];
    for (const [name, wanted] of Object.entries(files)) {
        const file = path.join(target, name);
        if (!fs.existsSync(file)) {
            failures.push(`Missing: ${name}`);
            continue;
        }
        const actual = read(file);
        if (actual === wanted) continue;
        const a = lines(actual), b = lines(wanted);
        let i = 0;
        while (i < a.length && i < b.length && a[i] === b[i]) i++;
        failures.push(`${name}:${i + 1}\nExpected:\n${b.slice(i, i + 5).join('')}\nYour file:\n${a.slice(i, i + 5).join('')}`);
    }
    return failures;
}

export function materialize(files: Files, destination: string, lock: string, assets: string[] = []) {
    fs.mkdirSync(path.dirname(destination), {recursive: true});
    fs.mkdirSync(destination, {recursive: false});
    const kernel = path.relative(destination, path.join(viewer, '../session_rust')).split(path.sep).join('/');
    for (const [name, text] of Object.entries(files)) {
        write(path.join(destination, name), name === 'Cargo.toml' ? text.replaceAll('../../../session_rust', kernel) : text);
    }
    fs.copyFileSync(lock, path.join(destination, 'Cargo.lock'));
    installAssets(assets, destination);
}

export function referenceAssets(destination: string) {
    for (const name of reference().external) {
        const file = path.join(destination, name);
        if (fs.existsSync(file)) continue;
        write(file, fs.readFileSync(path.join(docs, 'lessons/37', name)));
    }
    const scene = path.join(destination, 'assets/pb/course-boxes.pb');
    if (!fs.existsSync(scene)) write(scene, fs.readFileSync(path.join(docs, 'lessons/37/assets/course-boxes.pb')));
    const link = path.join(destination, 'target/docs/site');
    fs.mkdirSync(path.dirname(link), {recursive: true});
    if (!fs.existsSync(link)) fs.symlinkSync(path.join(viewer, 'dist/docs'), link, 'dir');
}

export function safeOutput(destination: string, project = work) {
    const relative = path.relative(canonical(project), canonical(destination));
    if (!relative || (!relative.startsWith('..' + path.sep) && relative !== '..' && !path.isAbsolute(relative))) {
        throw Error('Choose a new reference folder outside your handwritten project.');
    }
    if (fs.existsSync(destination)) throw Error(`Refusing to overwrite ${destination}`);
}

function canonical(file: string) {
    const full = path.resolve(file);
    let parent = full;
    while (!fs.existsSync(parent)) parent = path.dirname(parent);
    return path.join(fs.realpathSync(parent), path.relative(parent, full));
}
