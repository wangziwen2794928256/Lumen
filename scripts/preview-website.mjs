// Usage: node scripts/preview-website.mjs <export-directory>. No database dependencies.
import { createServer } from 'node:http';
import { readFile, realpath } from 'node:fs/promises';
import { resolve } from 'node:path';
const root = await realpath(resolve(process.argv[2] || 'website'));
const files = new Map();
for (const [name, mime] of [
  ['index.html', 'text/html'],
  ['site.js', 'text/javascript'],
  ['site.css', 'text/css'],
  ['public-profile.json', 'application/json'],
])
  files.set('/' + name, { body: await readFile(resolve(root, name)), mime });
const server = createServer((request, response) => {
  if (request.method !== 'GET' || request.headers.host !== `127.0.0.1:${server.address().port}`) {
    response.writeHead(404).end();
    return;
  }
  const file = files.get(request.url === '/' ? '/index.html' : request.url);
  if (!file) {
    response.writeHead(404).end();
    return;
  }
  response
    .writeHead(200, {
      'Content-Type': file.mime + '; charset=utf-8',
      'Cache-Control': 'no-store',
      'X-Content-Type-Options': 'nosniff',
      'Content-Security-Policy':
        "default-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'",
    })
    .end(file.body);
});
server.listen(0, '127.0.0.1', () =>
  console.log(`Lumen local preview: http://127.0.0.1:${server.address().port}/`),
);
