import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { registerLinkedAsset } from './course-source-assets.ts';

const viewer = process.env.COURSE_SOURCE_TEST_VIEWER || fileURLToPath(new URL('../../session_viewer/', import.meta.url));

test('Rust source links receive plain-text filenames and retain the original exact download bytes', () => {
  const assets = new Map<string, string>();
  const file = path.join(viewer, 'src/app/scene_sync/notes.rs');
  const before = fs.readFileSync(file);
  const key = registerLinkedAsset(assets, 'src/app/scene_sync/notes.rs', file, 'history notes');
  assert.equal(key, 'src/app/scene_sync/notes.rs.txt');
  assert.deepEqual(fs.readFileSync(assets.get(key)!), before);
  assert.deepEqual(fs.readFileSync(assets.get('src/app/scene_sync/notes.rs')!), before);
});

test('an explicitly labelled HTML source link exposes exact source without running the original HTML path', () => {
  const assets = new Map<string, string>();
  const file = path.join(viewer, 'assets/text-quality.html');
  const key = registerLinkedAsset(assets, 'assets/text-quality.html', file, 'maintained text-quality page source');
  assert.equal(key, 'assets/text-quality.html.txt');
  assert.match(fs.readFileSync(assets.get(key)!, 'utf8'), /<script type="module">/);
  assert.equal(assets.get('assets/text-quality.html'), file);
  assert.deepEqual(fs.readFileSync(assets.get(key)!), fs.readFileSync(file));
});

test('ordinary runnable HTML and images retain their existing asset URLs', () => {
  const assets = new Map<string, string>();
  assert.equal(registerLinkedAsset(assets, 'assets/text-quality.html', '/fixture.html', 'Open text-quality comparison'), 'assets/text-quality.html');
  assert.equal(registerLinkedAsset(assets, 'assets/diagram.svg', '/diagram.svg', 'source diagram'), 'assets/diagram.svg');
  assert.equal(registerLinkedAsset(assets, 'docs/screenshots/a.png', '/a.png'), 'docs/screenshots/a.png');
  assert.equal(assets.size, 3);
});

test('non-link registrations stay raw while repeated source links share one generated preview', () => {
  const assets = new Map<string, string>();
  assert.equal(registerLinkedAsset(assets, 'src/example.rs', '/example.rs'), 'src/example.rs');
  assert.equal(registerLinkedAsset(assets, 'src/example.rs', '/example.rs', 'example'), 'src/example.rs.txt');
  assert.equal(registerLinkedAsset(assets, 'src/example.rs', '/example.rs', 'example source'), 'src/example.rs.txt');
  assert.equal(assets.size, 2);
});
