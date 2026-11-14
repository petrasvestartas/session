import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {viewer, docs, hash, read, resolveJourneyImage} from './model.ts';

test('the merged course preserves every original unchanged image alias', () => {
    const file = path.join(docs, 'journey/release/viewer-image-goal-inventory-1763-20261113.json');
    const inventory = JSON.parse(read(file));
    assert.equal(inventory.explicitlyUnchanged.length, 429);
    for (const entry of inventory.explicitlyUnchanged) {
        assert.equal(hash(fs.readFileSync(resolveJourneyImage(path.join(viewer, entry.destination)))),
            entry.originalSha256, entry.destination);
    }
});

test('image reconciliation preserves both source versions without changing accepted hashes', () => {
    const receipt = JSON.parse(read(path.join(docs, 'journey/release/image-merge-reconciliation-20261114.json')));
    const inventory = path.join(docs, 'journey/release', receipt.inventory);
    assert.equal(hash(read(inventory).replaceAll('\r\n', '\n')), receipt.inventoryNormalizedSha256);
    assert.equal(hash(fs.readFileSync(path.join(viewer, receipt.retainedPhysicalFile))), receipt.originalImageSha256);
    assert.equal(hash(fs.readFileSync(path.join(viewer, receipt.currentConsolidatedImage))), receipt.currentConsolidatedSha256);
    assert.notEqual(receipt.originalImageSha256, receipt.currentConsolidatedSha256);
    assert.equal(receipt.logicalIndices.length, 25);
    assert.equal(receipt.newAcceptances, 0);
});
