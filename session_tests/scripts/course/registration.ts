import assert from 'node:assert/strict';

/** A module index may add declarations and reexports, never an implementation. */
export function assertRegistrationExtension(name: string, before: string, after: string) {
    const original = before.split('\n').filter(line => line.trim());
    const added: string[] = [];
    let cursor = 0;
    for (const line of after.split('\n')) {
        if (line === original[cursor]) cursor++;
        else if (line.trim()) added.push(line.trim());
    }
    assert.equal(cursor, original.length, `Registration changed instead of appending: ${name}`);
    for (const line of added) {
        const code = line.replace(/\s*\/\/.*$/, '');
        if (!code) continue;
        const declaration = /^(?:pub(?:\(crate\))? )?mod \w+;$/.test(code);
        const reexport = /^pub(?:\(crate\))? use [\w:*{}, ]+;$/.test(code);
        const attribute = /^#\[(?:cfg\([\w\s=",()]+\)|path = "[\w./-]+")\]$/.test(code);
        const entry = /^\("(frame|painting|mathematics|dock|camera|rows|geometry|decoding)", \1::start\),$/.test(code)
            || code === '("application", crate::run_web),';
        assert(declaration || reexport || attribute
            || (name === 'examples/course/registration.rs' && entry),
            `Implementation added through registration interface ${name}: ${line}`);
    }
}
