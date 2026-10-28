import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {course, docs, viewer, work, expected, referenceFiles, compare, materialize, referenceAssets, safeOutput, lockFor, dependencies, assetsFor} from './course/model.ts';
import {save, restore} from './course/checkpoints.ts';
import {generate} from './course/render.ts';
import {structure, verify, runCommand, nativeTarget, removeGenerated} from './course/verify.ts';
import {serve} from './course/serve.ts';
import {verifyReference, parity} from './course/reference.ts';
import {diagrams} from './course/diagrams.ts';
import {foundations} from './course/foundations.ts';
import {dockPreview} from './course/dock.ts';
import {referenceStructure} from './course/reference-checks.ts';
import {courseBook, readingBook} from '../src/courseBook.ts';

function book() {
    return courseBook(course().steps, JSON.parse(fs.readFileSync(path.join(docs, 'journey/book.json'), 'utf8')));
}

function lessonId(name: string) {
    if (/^\d+\.\d+$/.test(name)) {
        const plan = JSON.parse(fs.readFileSync(path.join(docs, 'journey/book.json'), 'utf8'));
        const chapter = readingBook(course().steps, plan).chapters[Number(name.split('.')[0]) - 1];
        const lesson = chapter?.sections.flatMap(section => section.lessons).find(step => step.number === name);
        const endpoint = plan.chapters[Number(name.split('.')[0]) - 1]?.consolidated?.find((step: {id: string}) => step.id === lesson?.id)?.checkpoint;
        if (!endpoint) throw Error(`Unknown consolidated lesson number: ${name}`);
        return endpoint;
    }
    if (!/^\d+\.\d+\.\d+$/.test(name)) return name;
    const lesson = [...book().byId.values()].find(step => step.number === name);
    if (!lesson) throw Error(`Unknown lesson number: ${name}`);
    return lesson.id;
}

function main() {
    const [action, selected, ...args] = process.argv.slice(2);
    const name = ['check', 'dependencies', 'reference'].includes(action) ? lessonId(selected) : selected;
    const lock = path.join(docs, 'journey/release/Cargo.lock');
    if (action === 'init') {
        fs.mkdirSync(path.dirname(work), {recursive: true});
        fs.mkdirSync(work);
        fs.copyFileSync(lock, path.join(work, 'Cargo.lock'));
        console.log('Created workspace/journey with the dependency lock only. Type the implementation yourself.');
    } else if (action === 'check') {
        const failures = compare(expected(name), work);
        if (failures.length) throw Error(failures.join('\n\n'));
        console.log('Source matches. Build and run the lesson experiment to check behavior.');
    } else if (action === 'dependencies') {
        dependencies(name);
        console.log('Dependency lock and supplied binary assets are current. Any previous lock was preserved; implementation files were not changed.');
    } else if (action === 'save') {
        console.log(`Saved your files in ${save(name)}`);
    } else if (action === 'restore') {
        const backup = restore(name);
        console.log(`Restored ${name}.${backup ? ` Your previous work is preserved in ${backup}.` : ''}`);
    } else if (action === 'reference') {
        const at = args.indexOf('--output');
        if (at < 0 || !args[at + 1]) throw Error('Choose a new --output directory.');
        const output = path.resolve(process.env.INIT_CWD || process.cwd(), args[at + 1]);
        safeOutput(output);
        materialize(expected(name), output, lockFor(name), assetsFor(name));
        console.log(`Reference assembled in ${output}. Your project was not changed.`);
    } else if (action === 'reference-check') {
        const failures = compare(referenceFiles(name), path.join(viewer, 'workspace/handwritten'));
        if (failures.length) throw Error(failures.join('\n\n'));
        console.log(`${name}: reference source matches.`);
    } else if (action === 'reference-init') {
        const target = path.join(viewer, 'workspace/handwritten');
        fs.mkdirSync(target);
        referenceAssets(target);
        console.log('Created the reference workspace with external assets only.');
    } else if (action === 'dock-preview') {
        dockPreview();
    } else if (action === 'generate') {
        generate();
    } else if (action === 'structure') {
        structure();
        referenceStructure();
    } else if (action === 'diagrams') {
        diagrams(name === '--check');
    } else if (action === 'foundations') {
        foundations();
    } else if (action === 'parity') {
        parity();
    } else if (action === 'verify-reference') {
        const ids = [name, ...args].filter(Boolean);
        verifyReference(ids.filter(id => id !== '--stored' && id !== '--all'), ids.includes('--stored'), ids.includes('--all'));
    } else if (action === 'verify') {
        const ids = [name, ...args].filter(Boolean);
        const stored = ids.includes('--stored');
        const all = ids.includes('--all');
        verify(ids.filter(id => id !== '--stored' && id !== '--all').map(lessonId), stored, all);
    } else if (action === 'capture') {
        const rawIds = [name, ...args].filter(Boolean);
        const all = rawIds.includes('--all');
        const ids = rawIds.filter(id => id !== '--all').map(lessonId);
        if (!ids.length && !all) throw Error('Choose capture checkpoint IDs or --all');
        verify(ids, true, all);
        // Materialize and clean up each checkpoint one at a time so target/course-* never accumulates.
        for (const step of course().steps.filter(step => !ids.length || ids.includes(step.id))) {
            const directory = path.join(viewer, 'target', `course-${step.id}`);
            const bundles = path.join(viewer, 'target/course-checks');
            const bundle = path.join(bundles, step.id, 'dist');
            try {
                removeGenerated(directory, path.join(viewer, 'target'));
                materialize(expected(step.id), directory, lockFor(step.id), assetsFor(step.id));
                const uploads = (step.browser_actions || []).filter(action => 'file' in action);
                if (uploads.some(action => 'file' in action && action.file !== 'sample.pb')) {
                    throw Error(`${step.id}: unsupported browser specimen`);
                }
                removeGenerated(bundle, bundles);
                runCommand(`${step.id}-capture-web`, ['trunk', 'build', '--release', '--public-url', './'], directory);
                fs.mkdirSync(path.dirname(bundle), {recursive: true});
                fs.cpSync(path.join(directory, 'dist'), bundle, {recursive: true});
                if (fs.existsSync(path.join(directory, 'examples/sample.rs'))) {
                    runCommand(`${step.id}-sample`, ['cargo', 'run', '--example', 'sample', '--locked',
                        '--target', nativeTarget(), '-j1'], directory);
                } else if (uploads.length) {
                    throw Error(`${step.id}: missing original sample generator`);
                }
                const capture = step.browser_check === 'journey/checks/browser/final.cjs' ? 'capture_current.cjs' : 'capture_journey.cjs';
                runCommand(`${step.id}-browser`, [process.execPath, path.join(docs, capture), step.id], viewer,
                    {NODE_PATH: path.join(viewer, 'target/course-tools/node_modules'), COURSE_SAMPLES_PREPARED: '1',
                        COURSE_MEMORY_MIB: process.env.COURSE_BROWSER_MEMORY_MIB || '6144'});
            } finally {
                if (process.env.COURSE_KEEP_BUNDLES !== '1') removeGenerated(bundle, bundles);
                if (process.env.COURSE_KEEP_WORKSPACES !== '1') {
                    removeGenerated(directory, path.join(viewer, 'target'));
                }
            }
        }
        generate();
    } else if (action === 'serve') {
        serve(name ? Number(name) : 8781);
    } else if (action === 'list') {
        const selectedId = name && name !== '--ids' ? lessonId(name) : undefined;
        const lessons = course().steps.filter(step => !selectedId || step.id === selectedId);
        if (!lessons.length) throw Error(`Unknown lesson: ${name}`);
        const numbers = book().byId;
        for (const step of lessons) console.log(`${name === '--ids' ? step.id : numbers.get(step.id)?.number || 'Reference comparison'}  ${step.title}`);
    } else {
        console.log('npm run course -- init | check LESSON | dependencies LESSON | save NAME | restore NAME | reference LESSON --output NEW_FOLDER');
        console.log('Use a book number such as 22.2.3 or an existing lesson ID. List shows book numbers; list --ids shows internal IDs.');
        console.log('Authoring: dock-preview | generate | structure | verify ID|--all [--stored] | verify-reference ID|--all [--stored] | parity | foundations | diagrams [--check] | serve [PORT] | capture ID|--all | list');
    }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    try {
        main();
    } catch (error) {
        console.error(error instanceof Error ? error.message : error);
        process.exitCode = 1;
    }
}
