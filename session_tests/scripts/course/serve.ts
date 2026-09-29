import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import {evidence} from './verify.ts';

export function serve(port = 8781) {
    if (!Number.isInteger(port) || port < 1 || port > 65535) throw Error('Choose a port between 1 and 65535.');
    const types: Record<string, string> = {'.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm',
        '.css': 'text/css', '.png': 'image/png', '.svg': 'image/svg+xml', '.json': 'application/json'};
    const server = http.createServer((request, response) => {
        try {
            const url = new URL(request.url || '/', 'http://localhost');
            if (url.pathname === '/favicon.ico') {
                response.writeHead(204).end();
                return;
            }
            let file = path.resolve(evidence, '.' + decodeURIComponent(url.pathname));
            if (!file.startsWith(evidence + path.sep)) throw Error('Unknown path');
            if (fs.statSync(file).isDirectory()) {
                if (!url.pathname.endsWith('/')) {
                    response.writeHead(302, {Location: url.pathname + '/'}).end();
                    return;
                }
                file = path.join(file, 'index.html');
            }
            const body = fs.readFileSync(file);
            response.writeHead(200, {'Content-Type': types[path.extname(file)] || 'application/octet-stream', 'Cache-Control': 'no-store'});
            response.end(body);
        } catch {
            response.writeHead(404).end('Not found');
        }
    });
    server.on('error', error => { console.error(error.message); process.exitCode = 1; });
    server.listen(port, '127.0.0.1', () => console.log(`Course bundles: http://127.0.0.1:${port}/LESSON-ID/dist/`));
}
