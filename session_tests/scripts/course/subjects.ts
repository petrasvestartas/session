import {edited, type Files, type Step} from './model.ts';
import {lines} from './line-diff.ts';

export interface SubjectEdit { path: string; before: string | null; code: string }

function replacement(before: string, after: string) {
    const a = lines(before), b = lines(after);
    let start = 0, end = a.length, tail = b.length;
    while (start < end && start < tail && a[start] === b[start]) start++;
    while (end > start && tail > start && a[end - 1] === b[tail - 1]) { end--; tail--; }
    let old = a.slice(start, end).join('');
    while (!old || before.split(old).length !== 2) {
        if (start) start--;
        else if (end < a.length) { end++; tail++; }
        else throw Error('An empty existing file cannot be replaced by the current course edit format');
        old = a.slice(start, end).join('');
    }
    return {before: old, code: b.slice(start, tail).join('')};
}

/** Finish each touched file once, preserving the subject's exact cumulative endpoint. */
export function collapseSubject(before: Files, steps: Pick<Step, 'id' | 'edits'>[], load: (name: string) => string) {
    const files = {...before};
    const touched = new Set<string>();
    for (const step of steps) for (const edit of step.edits) {
        files[edit.path] = edited(files[edit.path], edit, load(edit.snippet), step.id);
        touched.add(edit.path);
    }
    const edits: SubjectEdit[] = [];
    for (const path of touched) {
        if (before[path] === files[path]) continue;
        const change = before[path] === undefined
            ? {before: null, code: files[path]} : replacement(before[path], files[path]);
        const rebuilt = edited(before[path], {path, before: change.before, snippet: '', why: ''}, change.code);
        if (rebuilt !== files[path]) throw Error(`${path}: consolidated edit changes the endpoint`);
        edits.push({path, ...change});
    }
    return {files, edits};
}
