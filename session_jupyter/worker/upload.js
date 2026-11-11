// POST /upload?view=iso&grid=0   body: the session as protobuf bytes (gzip or plain)
// -> { "scene": "tmp/<hash>.yaml", "url": "<viewer>?scene=tmp/<hash>.yaml" }
//
// Each file is stored under the hash of its content, so the same scene is stored once.
// Everything is written below tmp/, which the bucket deletes after one day.

const VIEWS = ['top', 'bottom', 'front', 'back', 'left', 'right', 'iso', 'isoback'];
const CORS = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'POST, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type',
};

const answer = (status, body) =>
  new Response(JSON.stringify(body), { status, headers: { ...CORS, 'Content-Type': 'application/json' } });

const hex = async (bytes) =>
  [...new Uint8Array(await crypto.subtle.digest('SHA-256', bytes))].map((b) => b.toString(16).padStart(2, '0')).join('');

const gzipped = (bytes) => bytes.length > 2 && bytes[0] === 0x1f && bytes[1] === 0x8b;

export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (request.method === 'OPTIONS') return new Response(null, { status: 204, headers: CORS });
    if (request.method !== 'POST' || url.pathname !== '/upload') return answer(404, { error: 'POST /upload' });

    const view = url.searchParams.get('view') || 'iso';
    if (!VIEWS.includes(view)) return answer(400, { error: `view must be one of ${VIEWS.join(', ')}` });
    const grid = url.searchParams.get('grid') === '1';

    const body = new Uint8Array(await request.arrayBuffer());
    if (body.length === 0) return answer(400, { error: 'empty body' });
    if (body.length > Number(env.MAX_BYTES)) return answer(413, { error: `larger than ${env.MAX_BYTES} bytes` });

    // The key is the hash of the uncompressed scene, so equal scenes share one key however they were sent.
    const unzip = (bytes) => new Response(new Blob([bytes]).stream().pipeThrough(new DecompressionStream('gzip'))).arrayBuffer();
    const zip = (bytes) => new Response(new Blob([bytes]).stream().pipeThrough(new CompressionStream('gzip'))).arrayBuffer();
    let raw;
    try {
      raw = gzipped(body) ? new Uint8Array(await unzip(body)) : body;
    } catch {
      return answer(400, { error: 'body is not valid gzip' });
    }
    if (raw.length > Number(env.MAX_BYTES)) return answer(413, { error: `larger than ${env.MAX_BYTES} bytes` });
    const stored = gzipped(body) ? body : new Uint8Array(await zip(raw));
    const size = raw.length;

    const pb = `tmp/${await hex(raw)}.pb`;
    // Written again even when it exists: no second copy, and the day starts over.
    await env.BUCKET.put(pb, stored, {
      httpMetadata: { contentType: 'application/octet-stream', contentEncoding: 'gzip', cacheControl: 'public, max-age=86400' },
    });

    const scene = JSON.stringify({ name: 'notebook', live: false, view, grid, items: [{ file: pb, name: 'notebook', at: [0, 0, 0], encoding: 'gzip', size }] }, null, 2) + '\n';
    const key = `tmp/${await hex(new TextEncoder().encode(scene))}.yaml`;
    await env.BUCKET.put(key, scene, { httpMetadata: { contentType: 'application/yaml', cacheControl: 'public, max-age=86400' } });

    return answer(200, { scene: key, url: `${env.VIEWER.replace(/\/$/, '')}/?scene=${key}` });
  },
};
