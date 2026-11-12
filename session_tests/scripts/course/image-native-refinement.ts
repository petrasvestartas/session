import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';

const ids = ['78m-exercise-profiles-exact-surfaces-and-one-undo-transaction', '79zze-fit-the-original-gordon-surface'];
const sha = (text: string) => createHash('sha256').update(text).digest('hex');
const changes = [
    ['pub const SURFACE_STEPS: usize = 128;', 'pub const SURFACE_STEPS: usize = 256;'],
    ['pub const TRANSIENT_BYTES: usize = 32 * 1024 * 1024;', 'pub const TRANSIENT_BYTES: usize = 64 * 1024 * 1024;'],
];

export function nativeRefinementBudget(id: string, requested = process.env.COURSE_NATIVE_REFINEMENT_BUDGET || 'default') {
    assert(['default', 'surface256-upload64'].includes(requested), 'Unknown Native refinement budget');
    if (requested === 'default') return null;
    assert(ids.includes(id), 'Native refinement budget is restricted to the two original curved-surface fixtures');
    return {name: 'surface256-upload64', surfaceIntervals: 256, transientBytes: 64 * 1024 * 1024};
}

export function nativeRefinementDensity(profile: any, density: number) {
    if (profile) assert.equal(density, 4, 'Expanded Native refinement is raw DPR4 only');
}

export function nativeRefinementFiles(files: Record<string, string>, profile: any) {
    if (!profile) return files;
    assert.deepEqual({name: profile.name, surfaceIntervals: profile.surfaceIntervals, transientBytes: profile.transientBytes},
        {name: 'surface256-upload64', surfaceIntervals: 256, transientBytes: 64 * 1024 * 1024});
    const name = 'src/math_quality.rs', before = files[name];
    assert(typeof before === 'string', 'Original mathematical quality module required');
    let after = before;
    for (const [from, to] of changes) {
        assert.equal(after.split(from).length, 2, 'Exact finite budget preimage required');
        after = after.replace(from, to);
    }
    let restored = after;
    for (const [from, to] of [...changes].reverse()) restored = restored.replace(to, from);
    assert.equal(restored, before, 'Only the two finite ceilings may change');
    if (profile.originalMathSha256) assert.equal(sha(before), profile.originalMathSha256);
    if (profile.adaptedMathSha256) assert.equal(sha(after), profile.adaptedMathSha256);
    return {...files, [name]: after};
}
