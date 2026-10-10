"""Check publication Git blobs against the allowlisted source manifest."""
from pathlib import Path
import hashlib, json, subprocess, sys
root = Path(sys.argv[1]).resolve()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=root)
manifest = json.loads(git('show', 'HEAD:UPSTREAM_CHECKPOINT.json'))
for entry in manifest['files']:
    data = git('show', 'HEAD:' + entry['file'])
    if hashlib.sha256(data).hexdigest() != entry['sha256']:
        raise SystemExit('Publication blob checksum mismatch: ' + entry['file'])
names = set(git('ls-tree', '-r', '--name-only', 'HEAD').decode().splitlines())
allowed = {f['file'] for f in manifest['files']} | {'README.md', 'UPSTREAM_CHECKPOINT.json'}
if names != allowed:
    raise SystemExit('Publication tree does not match allowlist')
if git('show', 'HEAD:README.md') != git('show', 'HEAD:docs/distribution/REPOSITORY_README.md'):
    raise SystemExit('Root README differs from verified publication template')
print(json.dumps({'result': 'PASS', 'commit': git('rev-parse', 'HEAD').decode().strip(),
    'verifiedSourceBlobs': len(manifest['files']), 'productionDataIncluded': False}))
