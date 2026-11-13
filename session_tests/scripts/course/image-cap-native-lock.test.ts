import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {createHash} from 'node:crypto';
import {capNativeLockIdentity, capNativeLockOverlay, validateCapNativeLock} from './image-cap-native-lock.ts';

const hash = (s: string) => createHash('sha256').update(s).digest('hex');
const data = {beforeCargoSha256: hash('cargo\n'), beforeLockSha256: hash('old lock'), maintainedManifestSha256: hash('viewer\n'), afterLockSha256: hash('new lock'), existingPackageVersionsChanged: false, removedPackages: []};
test('unrelated images retain their exact lock without an overlay', () => {
    assert.equal(capNativeLockIdentity('48d-input'), null);
    assert.deepEqual(capNativeLockOverlay('48d-input', 'any manifest', 'exact old bytes'), {lock: 'exact old bytes', proof: null});
});
test('reviewed graph accepts only its manifest, original lock, maintained manifest and seed', () => {
    assert.equal(validateCapNativeLock('cargo\r\n', 'old lock', data, 'viewer\r\n', 'new lock'), 'new lock');
});
for (const [name, cargo, lock, manifest, seed] of [
    ['image manifest', 'changed', 'old lock', 'viewer\n', 'new lock'],
    ['original lock', 'cargo\n', 'changed', 'viewer\n', 'new lock'],
    ['maintained viewer', 'cargo\n', 'old lock', 'changed', 'new lock'],
    ['reviewed seed', 'cargo\n', 'old lock', 'viewer\n', 'changed'],
]) test('rejects a changed ' + name, () => {
    assert.throws(() => validateCapNativeLock(cargo, lock, data, manifest, seed));
});
test('rejects package version changes or removals', () => {
    assert.throws(() => validateCapNativeLock('cargo\n', 'old lock', {...data, existingPackageVersionsChanged: true}, 'viewer\n', 'new lock'));
    assert.throws(() => validateCapNativeLock('cargo\n', 'old lock', {...data, removedPackages: ['unreviewed']}, 'viewer\n', 'new lock'));
});
test('three production cap selectors share the exact pinned source-only proposal', () => {
    const ids = ['96zza-connect-section-primitive-images', '96zzc-share-section-addresses-with-outlines-and-ambient', '96zzd-pack-surface-position-normal-and-color'];
    const selected = ids.map(id => capNativeLockIdentity(id)!);
    assert(selected.every(x => x.identity.lockSha256 === selected[0].identity.lockSha256));
    assert.equal(selected[0].data.addedPackages.length, 9);
    assert.equal(fs.readFileSync(selected[0].files[2], 'utf8').includes('name = "image_incremental"'), true);
});
