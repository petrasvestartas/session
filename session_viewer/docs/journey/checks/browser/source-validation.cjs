const assert = require('node:assert/strict');
module.exports = async page => {
    const result = await page.evaluate(() => {
        const canvas = document.querySelector('canvas');
        const snapshot = () => ['data-object-count', 'data-row-placements', 'data-row-metadata', 'data-cpu-usage', 'data-gpu-stats']
            .map(name => canvas.getAttribute(name));
        const before = snapshot();
        const validate = window.wasmBindings.validate_source_json;
        if (typeof validate !== 'function') throw Error('Browser source validator export is missing');
        validate('{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}');
        const errors = [];
        for (const text of [
            '{"control_points":[[0,0,0],[1,0,0]],"cv_count":3,"dimension":3,"order":2,"nurbsknots":[0,1]}',
            '{"control_points":[],"cv_count":18446744073709551615,"dimension":3}',
        ]) {
            try { validate(text); errors.push(null); }
            catch (error) { errors.push(String(error)); }
        }
        return {before, after: snapshot(), errors};
    });
    assert(result.before.every(value => value !== null), 'Source inspection attributes must exist');
    assert.match(result.errors[0], /declared count or stride/);
    assert.match(result.errors[1], /supported count/);
    assert.deepEqual(result.after, result.before, 'Pure source validation leaves original drawing, history and GPU uploads unchanged');
};
