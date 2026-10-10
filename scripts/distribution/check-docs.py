"""Check local Markdown links in the distributed bilingual documentation."""
from pathlib import Path
from urllib.parse import unquote, urlsplit
import argparse, json, re

parser = argparse.ArgumentParser()
parser.add_argument('root', nargs='?', default='.')
args = parser.parse_args()
root = Path(args.root).resolve()
files = []
for directory in ('en', 'zh-CN', 'contracts', 'data', 'distribution'):
    files.extend((root / 'docs' / directory).rglob('*.md'))
missing = []
checked = 0
for path in files:
    # This reviewed template is copied to the repository root on publication.
    base = root if path.name == 'REPOSITORY_README.md' else path.parent
    for link in re.findall(r'\[[^\]]+\]\(([^)]+)\)', path.read_text(encoding='utf-8-sig')):
        parsed = urlsplit(link)
        if parsed.scheme or parsed.netloc or not parsed.path:
            continue
        checked += 1
        target = (base / unquote(parsed.path)).resolve()
        if not target.is_relative_to(root) or not target.exists():
            missing.append({'file': path.relative_to(root).as_posix(), 'link': link})
result = {'result': 'FAIL' if missing else 'PASS', 'documents': len(files),
          'localLinksChecked': checked, 'missing': missing}
print(json.dumps(result, indent=2))
raise SystemExit(1 if missing else 0)
