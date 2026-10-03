const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const assert = require('node:assert/strict');
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

module.exports = function fingerprint(check, root = path.join(__dirname, 'browser')) {
    assert(/^journey\/checks\/browser\/[a-z0-9-]+\.cjs$/.test(check), 'Invalid checkpoint browser check');
    root = path.resolve(root);
    const inputs = new Map([['fingerprint.cjs', hash(fs.readFileSync(__filename))]]);
    const visit = file => {
        const name = path.relative(root, file);
        assert(!name.startsWith('..') && !path.isAbsolute(name), 'Browser dependency must stay inside its check folder');
        if (inputs.has(name)) return;
        const code = fs.readFileSync(file, 'utf8');
        inputs.set(name, hash(code));
        // Check modules use literal relative requires; package versions are pinned separately.
        for (const match of code.matchAll(/require\(\s*(['"])(\.{1,2}\/[^'"]+)\1\s*\)/g)) {
            let dependency = path.resolve(path.dirname(file), match[2]);
            if (!path.extname(dependency)) dependency += '.cjs';
            visit(dependency);
        }
    };
    visit(path.join(root, path.basename(check)));
    return hash(JSON.stringify([...inputs].sort(([a], [b]) => a.localeCompare(b))));
};
