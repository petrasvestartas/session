import fs from 'node:fs';
import path from 'node:path';
import {completionFiles, completionSubjects} from './completion.ts';
import {assertRegistrationExtension} from './registration.ts';
import {viewer, docs, hash, type Files} from './model.ts';
import {activeCourse, type ActiveCourse} from '../../src/activeCourse.ts';

export function auditAcceptance(course: ActiveCourse, assemble: (subject: string) => Files,
    read: (name: string) => string) {
    const failures: string[] = [];
    for (const subject of activeCourse(course).subjects) {
        if (subject.acceptance !== 'accepted') continue;
        try {
            const raw = read(subject.review!.path).replaceAll('\r\n', '\n');
            if (hash(raw) !== subject.review!.sha256) throw Error('review digest changed');
            const review = JSON.parse(raw);
            if (review.status !== 'accepted' || review.subject !== subject.id)
                throw Error('review does not accept this subject');
            const files = assemble(subject.id);
            if (!review.files || Object.keys(review.files).length !== Object.keys(files).length
                || Object.entries(files).some(([name, text]) => review.files[name] !== hash(text)))
                throw Error('reviewed checkpoint source changed');
            if (review.lesson?.path !== subject.page
                || review.lesson.sha256 !== hash(read(subject.page).replaceAll('\r\n', '\n')))
                throw Error('reviewed lesson changed');
            const chapters = review.chapters ?? {};
            if (Object.keys(chapters).length !== subject.chapters.length
                || subject.chapters.some(name => chapters[name] !== hash(read(name).replaceAll('\r\n', '\n'))))
                throw Error('reviewed chapter changed or missing');
            if (!Array.isArray(review.evidence) || !review.evidence.length)
                throw Error('review has no supporting evidence');
            for (const proof of review.evidence) {
                if (!/^journey\/release\/[a-z0-9-]+\.json$/.test(proof.path)
                    || hash(read(proof.path).replaceAll('\r\n', '\n')) !== proof.sha256)
                    throw Error('supporting evidence changed or missing');
            }
        } catch (error) { failures.push(`${subject.id}: ${String(error)}`); }
    }
    return failures;
}

const registrations = new Set(['src/lib.rs', 'viewer-core/src/lib.rs', 'src/app/mod.rs', 'src/engine/mod.rs',
    'src/engine/gpu/mod.rs', 'src/engine/gpu/objects.rs', 'src/engine/gpu/hull.rs',
    'src/engine/gpu/instance.rs', 'src/engine/pipelines/mod.rs', 'src/app/stage/mod.rs', 'examples/course/registration.rs']);

function implementation(text: string) {
    return text.split('\n').filter(line => {
        if (!/\/\/ register:(?:application-tests|loading)/.test(line)) return true;
        if (!/^(?:#\[(?:cfg|path).*\]|(?:pub )?mod \w+;|pub use stream::\{[\w, ]+\};) \/\/ register:(?:application-tests|loading)$/.test(line))
            throw Error('Implementation hidden in a conditional registration');
        return false;
    }).join('\n');
}

export function auditCompletion(assemble: (subject: string) => Files = completionFiles) {
    const course = JSON.parse(fs.readFileSync(path.join(docs, 'journey/active-course.json'), 'utf8'));
    const failures = auditAcceptance(course, assemble, name => fs.readFileSync(path.join(docs, name), 'utf8'));
    const checkpoints: {subject: string; files: number}[] = [];
    let previous: Files = {};
    for (const subject of completionSubjects) {
        const current = assemble(subject);
        for (const [name, before] of Object.entries(previous)) {
            try {
                if (!(name in current)) throw Error('removed');
                if (registrations.has(name)) assertRegistrationExtension(name, before, current[name]);
                else if (implementation(before) !== implementation(current[name])) throw Error('implementation rewritten');
            } catch (error) { failures.push(`${subject}: ${name}: ${String(error)}`); }
        }
        checkpoints.push({subject, files: Object.keys(current).length});
        previous = current;
    }
    const production: Files = {};
    for (const relative of fs.readdirSync(path.join(viewer, 'src'), {recursive:true})) {
        const name = 'src/' + String(relative).replaceAll('\\', '/');
        if (fs.statSync(path.join(viewer, name)).isFile()) production[name] = fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n');
    }
    const missing = Object.keys(production).filter(name => !(name in previous));
    const extra = Object.keys(previous).filter(name => name.startsWith('src/') && !(name in production));
    const changed = Object.keys(production).filter(name => name in previous && production[name] !== previous[name]);
    return {scope:'Active cumulative assemblies only; historical checks and teaching/runtime acceptance remain separate',
        checkpoints, failures, productionFiles:Object.keys(production).length, missing, extra, changed,
        passed: !failures.length && !missing.length && !extra.length && !changed.length};
}
