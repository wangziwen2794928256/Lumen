// Compile the actual documented example in an isolated source copy.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { spawnSync } from 'node:child_process';
const root = process.cwd();
const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'lumen-module-tutorial-'));
for (const name of ['src', 'tests', 'website', 'fixtures']) {
  if (fs.existsSync(name)) fs.cpSync(name, path.join(directory, name), {
    recursive: true,
    filter: (item) => !item.split(path.sep).includes('screenshots'),
  });
}
for (const name of ['package.json', 'tsconfig.json', 'vite.config.ts', 'playwright.config.ts'])
  fs.copyFileSync(name, path.join(directory, name));
fs.symlinkSync(path.join(root, 'node_modules'), path.join(directory, 'node_modules'), 'junction');
const source = fs.readFileSync('docs/en/MODULE_TUTORIAL.md', 'utf8');
const chinese = fs.readFileSync('docs/zh-CN/MODULE_TUTORIAL.md', 'utf8');
const blocks = [...source.matchAll(/```tsx\n([\s\S]*?)\n```/g)].map((m) => m[1]);
const zhBlocks = [...chinese.matchAll(/```tsx\n([\s\S]*?)\n```/g)].map((m) => m[1]);
if (blocks.length !== 3 || JSON.stringify(blocks) !== JSON.stringify(zhBlocks))
  throw Error('Bilingual tutorial examples differ or are incomplete');
const manifest = JSON.parse(source.match(/```json\n([\s\S]*?)\n```/)[1]);
const entry = path.join(directory, 'src/modules/official/study-lab/index.tsx');
fs.mkdirSync(path.dirname(entry), { recursive: true });
fs.writeFileSync(entry, blocks[0]);
const catalogue = path.join(directory, 'src/kernel/modules/catalogue.json');
const modules = JSON.parse(fs.readFileSync(catalogue, 'utf8'));
if (modules.some((m) => m.id === manifest.id || m.route === manifest.route))
  throw Error('Example already conflicts with bundled modules');
fs.writeFileSync(catalogue, JSON.stringify([...modules, manifest], null, 2));
const router = path.join(directory, 'src/app/router.tsx');
let routes = fs.readFileSync(router, 'utf8');
if (!routes.includes('<Routes>')) throw Error('Route insertion point missing');
routes = blocks[1] + '\n' + routes.replace('<Routes>', '<Routes>\n' + blocks[2]);
fs.writeFileSync(router, routes);
const checked = spawnSync(process.execPath, [path.join(root, 'node_modules/typescript/bin/tsc'), '--noEmit'], { cwd: directory, encoding: 'utf8' });
if (checked.status !== 0) {
  process.stderr.write(checked.stdout + checked.stderr);
  process.exit(checked.status ?? 1);
}
console.log(JSON.stringify({ result: 'PASS', languages: 2, documentedTsxBlocks: blocks.length, module: manifest.id, isolatedDirectory: directory, productionDataAccessed: false }));
