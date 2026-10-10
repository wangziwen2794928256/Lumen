"""Create/update an allowlisted publication checkout, without rewriting local history.

Never copies Git history, production data, logs, screenshots or native QA evidence.
The destination must be a dedicated checkout outside the source root or under work/.
"""
from pathlib import Path
import argparse, hashlib, json, re, shutil, subprocess

parser = argparse.ArgumentParser()
parser.add_argument('destination')
args = parser.parse_args()
root = Path(__file__).resolve().parents[2]
dest = Path(args.destination).resolve()
if dest == root or (root in dest.parents and root / 'work' not in dest.parents):
    raise SystemExit('Destination must be an isolated publication checkout')
dest.mkdir(parents=True, exist_ok=True)
dirs = ['src', 'public', 'tests/e2e', 'tests/unit', 'tests/fixtures', 'website', 'skills', 'src-tauri/src', 'src-tauri/migrations',
        'src-tauri/capabilities', 'src-tauri/icons', 'src-tauri/tests', 'src-tauri/vendor',
        'docs/en', 'docs/zh-CN', 'docs/contracts', 'docs/data', 'docs/distribution',
        '.github', 'scripts/distribution', 'scripts/release']
files = ['.gitattributes', '.gitignore', '.prettierrc.json', '.prettierignore', 'LICENSE', 'SECURITY.md',
         'CONTRIBUTING.md', 'components.json', 'eslint.config.js', 'index.html', 'package.json',
         'playwright.config.ts', 'pnpm-lock.yaml', 'pnpm-workspace.yaml', 'rust-toolchain.toml',
         'tsconfig.json', 'vite.config.ts', 'src-tauri/build.rs', 'src-tauri/Cargo.toml',
         'src-tauri/Cargo.lock', 'src-tauri/tauri.conf.json', 'src-tauri/tauri.release.conf.json',
         'src-tauri/tauri.experimental.conf.json', 'scripts/check-contracts.mjs', 'scripts/environment.ps1',
         'scripts/preview-website.mjs', 'scripts/install-desktop-shortcut.ps1']
for directory in dirs:
    p = root / directory
    if p.exists(): files.extend(str(f.relative_to(root)).replace('\\', '/') for f in p.rglob('*')
                               if f.is_file() and '__pycache__' not in f.parts)
blocked = re.compile(r'(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}|-----BEGIN (?:RSA |OPENSSH |EC )?PRIVATE KEY-----|sk-proj-[A-Za-z0-9_-]{40,})')
manifest = []
for relative in sorted(set(files)):
    p = root / relative
    if not p.exists(): continue
    if not p.resolve().is_relative_to(root) or p.is_symlink() or p.suffix.lower() in ['.db', '.sqlite', '.sqlite3', '.log', '.pptx', '.pyc'] or '.env' in p.name:
        raise SystemExit('Forbidden publication file: ' + relative)
    data = p.read_bytes()
    if len(data) > 25 * 1024 * 1024: raise SystemExit('Oversized publication file: ' + relative)
    if blocked.search(data.decode('utf-8', errors='ignore')): raise SystemExit('Potential credential in: ' + relative)
    # Match the Git blob's LF normalization, so remote checkout hashes are verifiable.
    if b'\0' not in data: data = data.replace(b'\r\n', b'\n')
    q = dest / relative; q.parent.mkdir(parents=True, exist_ok=True); q.write_bytes(data)
    manifest.append({'file': relative, 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()})
sha = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip()
(dest / 'UPSTREAM_CHECKPOINT.json').write_text(json.dumps({'upstreamCommit': sha,
    'stablePhase16Commit': 'bcee0483ae5c072e49307dada2bee479a830ef99',
    'historyPolicy': 'Allowlisted source snapshot. Full phase history and local acceptance evidence stay on the owner workstation.',
    'workingTreeMayIncludeUncommittedChanges': True, 'hashEncoding': 'Git blob LF for text; binary bytes unchanged',
    'productionDataIncluded': False, 'files': manifest}, indent=2), encoding='utf-8')
readme = root / 'docs/distribution/REPOSITORY_README.md'
if readme.exists(): shutil.copy2(readme, dest / 'README.md')
allowed = {entry['file'] for entry in manifest} | {'README.md', 'UPSTREAM_CHECKPOINT.json'}
unexpected = [str(p.relative_to(dest)).replace('\\', '/') for p in dest.rglob('*')
    if p.is_file() and '.git' not in p.relative_to(dest).parts
    and str(p.relative_to(dest)).replace('\\', '/') not in allowed]
if unexpected:
    raise SystemExit('Unapproved existing publication files; inspect before staging: ' + ', '.join(unexpected[:10]))
print(json.dumps({'destination': str(dest), 'files': len(manifest), 'sourceCommit': sha,
    'productionDataIncluded': False, 'credentialPatternFindings': 0}))
