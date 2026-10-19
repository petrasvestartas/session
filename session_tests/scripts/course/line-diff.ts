export const lines = (text: string) => text.match(/[^\n]*\n|[^\n]+$/g) || [];

export function lineContext(file: string, before: string, after: string) {
    const at = file.indexOf(before);
    if (!before || at < 0 || file.indexOf(before, at + before.length) >= 0) throw Error('Code context requires one exact replacement');
    const start = at === 0 ? 0 : file.lastIndexOf('\n', at - 1) + 1;
    const end = at + before.length;
    const newline = file.indexOf('\n', end);
    const stop = before.endsWith('\n') && after.endsWith('\n') ? end : newline < 0 ? file.length : newline + 1;
    const prefix = file.slice(start, at), suffix = file.slice(end, stop);
    return {before: prefix + before + suffix, after: prefix + after + suffix};
}

type Change = {kind: 'same' | 'add' | 'remove'; text: string};

export function lineChanges(before: string, after: string): Change[] {
    const a = lines(before), b = lines(after);
    const common = Array.from({length: a.length + 1}, () => new Uint32Array(b.length + 1));
    for (let i = a.length - 1; i >= 0; i--) for (let j = b.length - 1; j >= 0; j--) {
        common[i][j] = a[i] === b[j] ? common[i + 1][j + 1] + 1 : Math.max(common[i + 1][j], common[i][j + 1]);
    }
    const changes: Change[] = [];
    let i = 0, j = 0;
    while (i < a.length || j < b.length) {
        if (i < a.length && j < b.length && a[i] === b[j]) {
            changes.push({kind: 'same', text: a[i++]}); j++;
        }
        else if (i < a.length && (j === b.length || common[i + 1][j] > common[i][j + 1])) changes.push({kind: 'remove', text: a[i++]});
        else changes.push({kind: 'add', text: b[j++]});
    }
    return changes;
}

export function changedBlocks(before: string, after: string, file: string) {
    const changes = lineChanges(before, after);
    const text = (start: number, end: number, excluded: Change['kind']) => changes.slice(start, end).filter(line => line.kind !== excluded).map(line => line.text).join('');
    let spans = changes.flatMap((line, index) => line.kind === 'same' ? [] : [{start: Math.max(0, index - 3), end: Math.min(changes.length, index + 4)}]);
    let repeat = true;
    while (repeat) {
        repeat = false;
        const merged: typeof spans = [];
        for (const span of spans) {
            const last = merged.at(-1);
            if (last && span.start <= last.end) {last.end = Math.max(last.end, span.end); repeat = true;}
            else merged.push({...span});
        }
        spans = merged;
        for (const span of spans) {
            while (!text(span.start, span.end, 'add') || file.split(text(span.start, span.end, 'add')).length !== 2) {
                if (span.start === 0 && span.end === changes.length) return [{before, after}];
                span.start = Math.max(0, span.start - 1);
                span.end = Math.min(changes.length, span.end + 1);
                repeat = true;
            }
        }
    }
    return spans.reverse().map(span => ({before: text(span.start, span.end, 'add'), after: text(span.start, span.end, 'remove')}));
}
