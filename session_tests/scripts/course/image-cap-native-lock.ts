import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const ids = new Set(['96zza-connect-section-primitive-images', '96zzc-share-section-addresses-with-outlines-and-ambient', '96zzd-pack-surface-position-normal-and-color']);
const hash = (text: string | Buffer) => createHash('sha256').update(text).digest('hex');
const normalizedHash = (text: string) => hash(text.replaceAll('\r\n', '\n'));

export function validateCapNativeLock(cargo: string, lock: string, data: any, manifest: string, seed: string) {
    assert.equal(normalizedHash(cargo), data.beforeCargoSha256, 'Exact image manifest preimage');
    assert.equal(hash(lock), data.beforeLockSha256, 'Exact original image lock preimage');
    assert.equal(normalizedHash(manifest), data.maintainedManifestSha256, 'Maintained viewer manifest changed');
    assert.equal(hash(seed), data.afterLockSha256, 'Reviewed image lock changed');
    assert.equal(data.existingPackageVersionsChanged, false);
    assert.deepEqual(data.removedPackages, []);
    return seed;
}

export function capNativeLockIdentity(id: string) {
    if (!ids.has(id)) return null;
    const metadata = path.join(here, 'image-cap-native-lock.json'), lock = path.join(here, 'image-cap-native.lock');
    const manifest = fileURLToPath(new URL('../../../session_viewer/Cargo.toml', import.meta.url));
    const data = JSON.parse(fs.readFileSync(metadata, 'utf8'));
    assert.deepEqual(new Set(data.ids), ids);
    assert.equal(hash(fs.readFileSync(lock)), data.afterLockSha256);
    assert.equal(normalizedHash(fs.readFileSync(manifest, 'utf8')), data.maintainedManifestSha256);
    return {data, files: [fileURLToPath(import.meta.url), metadata, lock, manifest], identity: {metadataSha256: hash(fs.readFileSync(metadata)), lockSha256: data.afterLockSha256, maintainedManifestSha256: data.maintainedManifestSha256}};
}

export function capNativeLockOverlay(id: string, cargo: string, lock: string) {
    const selected = capNativeLockIdentity(id);
    if (!selected) return {lock, proof: null};
    const seed = fs.readFileSync(selected.files[2], 'utf8'), manifest = fs.readFileSync(selected.files[3], 'utf8');
    return {lock: validateCapNativeLock(cargo, lock, selected.data, manifest, seed), proof: selected.identity};
}
