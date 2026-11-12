import {lines} from './line-diff.ts';
import type {Step} from './model.ts';

export type OwnershipFinding = {
    path: string; owner: string; subject: string; checkpoint: string;
    kind: 'registration' | 'rewrite';
};

export function auditOwnership(steps: Pick<Step, 'id' | 'edits'>[], subjects: Map<string, string>, snippet: (name: string) => string) {
    const owners = new Map<string, string>();
    const findings: OwnershipFinding[] = [];
    for (const step of steps) {
        const subject = subjects.get(step.id) || step.id;
        for (const edit of step.edits) {
            const owner = owners.get(edit.path);
            if (!owner) { owners.set(edit.path, subject); continue; }
            if (owner === subject || edit.before === null) continue;
            const after = snippet(edit.snippet);
            if (edit.before === after) continue;
            const original = lines(edit.before);
            let cursor = 0;
            const additions: string[] = [];
            for (const line of lines(after)) {
                if (line === original[cursor]) cursor++;
                else if (line.trim()) additions.push(line.trim());
            }
            const registration = edit.path.endsWith('.rs') && cursor === original.length
                && additions.some(line => /^(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;$/.test(line))
                && additions.every(line => /^(?:(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;|#\[cfg\([^\n]+\)\])$/.test(line));
            findings.push({path: edit.path, owner, subject, checkpoint: step.id, kind: registration ? 'registration' : 'rewrite'});
        }
    }
    return {owners, findings};
}
