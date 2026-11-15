import fs from 'node:fs';
import path from 'node:path';
import {execFileSync} from 'node:child_process';
import {docs, viewer} from './model.ts';

export interface Destination {
    commit: string;
    status: 'planned';
    subjects: {id: string; title: string; prerequisites: string[]}[];
    files: {path: string; blob: string; subject: string; teaching: null}[];
    inputs: {path: string; blob: string}[];
}

export function auditDestination(plan: Destination, tree: Map<string, string>, changed: string[]) {
    const errors: string[] = [];
    if (!/^[a-f0-9]{40}$/.test(plan.commit)) errors.push('Destination must name a full commit');
    if (plan.status !== 'planned') errors.push('This contract records planned ownership, not acceptance');
    const subjects = new Set<string>();
    for (const subject of plan.subjects) {
        if (!subject.id || !subject.title || subjects.has(subject.id)) errors.push(`Invalid subject: ${subject.id}`);
        for (const prerequisite of subject.prerequisites)
            if (!subjects.has(prerequisite)) errors.push(`${subject.id}: prerequisite must precede it: ${prerequisite}`);
        subjects.add(subject.id);
    }
    const paths = new Set<string>();
    for (const file of [...plan.files, ...plan.inputs]) {
        if (paths.has(file.path)) errors.push(`Duplicate destination path: ${file.path}`);
        paths.add(file.path);
        if (tree.get(file.path) !== file.blob) errors.push(`Pinned source mismatch: ${file.path}`);
    }
    for (const file of plan.files) {
        if (!file.path.startsWith('src/') || !subjects.has(file.subject)) errors.push(`Missing source owner: ${file.path}`);
        if (file.teaching !== null) errors.push(`Planned ownership cannot establish teaching: ${file.path}`);
    }
    for (const name of tree.keys())
        if (name.startsWith('src/') && !plan.files.some(file => file.path === name)) errors.push(`Unassigned production source: ${name}`);
    for (const name of changed) errors.push(`Production reconciliation required: ${name}`);
    return {commit: plan.commit, sources: plan.files.length, subjects: subjects.size, acceptedSubjects: 0, errors};
}

export function destination() {
    const plan: Destination = JSON.parse(fs.readFileSync(path.join(docs, 'journey/completion-batch.json'), 'utf8'));
    const git = (...args: string[]) => execFileSync('git', args, {cwd: viewer, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024});
    const tree = new Map<string, string>();
    for (const entry of git('ls-tree', '-r', '-z', plan.commit, '--', '.').split('\0').filter(Boolean)) {
        const match = /^\d+ blob ([a-f0-9]{40})\t(.+)$/.exec(entry);
        if (match) tree.set(match[2], match[1]);
    }
    const tracked = ['src', ...plan.inputs.map(file => file.path)];
    const changed = git('diff', '--name-only', '--relative', '-z', plan.commit, '--', ...tracked).split('\0').filter(Boolean);
    changed.push(...git('ls-files', '--others', '--exclude-standard', '-z', '--', 'src').split('\0').filter(Boolean));
    return auditDestination(plan, tree, changed);
}
