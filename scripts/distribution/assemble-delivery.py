"""Assemble verified distribution files from allowlisted source and native CI receipts.

No credentials, production AppData, local Git history or personal captures are read.
Run only after downloading build and static-verification artifacts.
"""
from pathlib import Path
import argparse, hashlib, json, shutil, subprocess, sys, zipfile

parser = argparse.ArgumentParser()
parser.add_argument('--mac-run', required=True)
parser.add_argument('--linux-run', required=True)
parser.add_argument('--verification-run', action='append', required=True)
args = parser.parse_args()
root = Path(__file__).resolve().parents[2]
delivery = root / 'release/Lumen-Phase17'
assets = delivery / 'assets'
assets.mkdir(parents=True, exist_ok=True)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def read(path):
    return json.loads(path.read_text(encoding='utf-8'))

def archive(path, entries):
    with zipfile.ZipFile(path, 'w', zipfile.ZIP_DEFLATED, compresslevel=9) as output:
        for name, data in sorted(entries):
            info = zipfile.ZipInfo(name, (2000, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            output.writestr(info, data)

def artifact_root(run):
    base = delivery / 'native-artifacts'
    return base if run == '38080545648' else base / run

packages = []
for run in (args.mac_run, args.linux_run):
    metadata = read(root / f'docs/testing/phase17/artifacts-{run}.json')
    for artifact in metadata:
        for package in artifact['packages']:
            if run == args.mac_run and not package['target'].endswith('apple-darwin'):
                continue
            if run == args.linux_run and package['target'] != 'x86_64-unknown-linux-gnu':
                continue
            source = artifact_root(run) / artifact['name'] / 'native-output' / package['file']
            if sha(source) != package['sha256'] or source.stat().st_size != package['bytes']:
                raise SystemExit('Package receipt mismatch: ' + package['file'])
            shutil.copy2(source, assets / source.name)
            packages.append(package | {'run': run})
expected = {'Lumen_0.1.0_aarch64.dmg', 'Lumen_0.1.0_x64.dmg',
            'Lumen_0.1.0_amd64.deb', 'Lumen_0.1.0_amd64.AppImage'}
if {p['file'] for p in packages} != expected or len(packages) != 4:
    raise SystemExit('Incomplete native package set')

verification = []
evidence_entries = []
for run in args.verification_run:
    for path in artifact_root(run).rglob('native-package-verification.json'):
        receipt = read(path)
        if receipt['result'] != 'PASS':
            raise SystemExit('Native package verification failed')
        verification.extend(receipt['packages'])
        evidence_entries.append((f'verification/{run}/{path.parent.parent.name}.json', path.read_bytes()))
if len(verification) != 4 or {v['file'] for v in verification} != expected:
    raise SystemExit('Missing native package validation receipts')
for package in packages:
    matches = [v for v in verification if v['file'] == package['file']]
    if len(matches) != 1 or matches[0]['sha256'] != package['sha256']:
        raise SystemExit('Verified package hash differs from build')

stable = 'bcee0483ae5c072e49307dada2bee479a830ef99'
for relative, name, checksum in [
    ('builds/Lumen.exe', 'Lumen-Windows-x64.exe', 'ac9a016d9c5212b6c4433aa3e0fadb8fb3244398ebd4ee280dc6c552c663dd59'),
    ('builds/Lumen_0.1.0_x64-setup.exe', 'Lumen_0.1.0_x64-setup.exe', '5e9ee74802718fb077816d47d3a204b3f3a3261999686ff1d7a6ec2300ed3c6d'),
    ('docs/presentations/Lumen-Product-Architecture-zh-CN.pptx', 'Lumen-Product-Architecture-zh-CN.pptx', '47a3f41b02c611ccdfa65434249b72ab6458e6fc2f65faf1c0e9d7b7765909e8')]:
    source = root / relative
    if sha(source) != checksum:
        raise SystemExit('Stable deliverable changed: ' + name)
    shutil.copy2(source, assets / name)

checkout = root / 'work/phase17-publish'
subprocess.run([sys.executable, str(root / 'scripts/distribution/verify-source.py'), str(checkout)], check=True)
subprocess.run([sys.executable, str(root / 'scripts/distribution/check-docs.py'), str(checkout)], check=True)
publication = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=checkout, text=True).strip()
source_archive = assets / f'Lumen-Source-{publication[:12]}.zip'
subprocess.run(['git', 'archive', '--format=zip', '--prefix=Lumen/', '-o', str(source_archive), 'HEAD'], cwd=checkout, check=True)
with zipfile.ZipFile(source_archive) as source_zip:
    source_names = source_zip.namelist()
    if any(Path(name).suffix.lower() in ('.db', '.sqlite', '.sqlite3', '.log', '.pptx') or '/.git/' in name for name in source_names):
        raise SystemExit('Forbidden source archive payload')

docs_entries = []
for directory in ('en', 'zh-CN', 'contracts', 'data', 'distribution'):
    for path in (checkout / 'docs' / directory).rglob('*'):
        if path.is_file():
            docs_entries.append((path.relative_to(checkout).as_posix(), path.read_bytes()))
archive(assets / 'Lumen-Technical-Documentation.zip', docs_entries)
for run in (args.mac_run, args.linux_run):
    for path in artifact_root(run).glob('Lumen-experimental-*/evidence/*'):
        if path.is_file():
            evidence_entries.append((f'build/{run}/{path.parent.parent.name}/{path.name}', path.read_bytes()))
    path = root / f'docs/testing/phase17/artifacts-{run}.json'
    evidence_entries.append((path.name, path.read_bytes()))
archive(assets / 'Lumen-Native-CI-Evidence.zip', evidence_entries)

allowed_assets = expected | {source_archive.name, 'Lumen-Technical-Documentation.zip',
    'Lumen-Native-CI-Evidence.zip', 'Lumen-Windows-x64.exe', 'Lumen_0.1.0_x64-setup.exe',
    'Lumen-Product-Architecture-zh-CN.pptx', 'BUILD_MANIFEST.json', 'SHA256SUMS'}
if any(not p.is_file() or p.name not in allowed_assets for p in assets.iterdir()):
    raise SystemExit('Unexpected existing asset; inspect before release upload')

manifest = {
    'schemaVersion': 1, 'release': 'v0.1.0-distribution-rc1',
    'repository': 'https://github.com/wangziwen2794928256/Lumen', 'visibility': 'private',
    'publicationCommit': publication,
    'localImplementationCommit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(),
    'windowsStableCommit': stable, 'nativePackages': packages,
    'nativePackageVerification': verification, 'verificationRuns': args.verification_run,
    'platformParityVerified': False, 'macLinuxNativeLaunchVerified': False,
    'signedOrNotarized': False, 'productionDataIncluded': False,
    'sourceArchive': source_archive.name,
    'limitations': ['macOS/Linux session events and Today/focus unavailable',
                    'native macOS/Linux install/upgrade/uninstall not run',
                    'media/attachment/preview/deep-link integration unsupported on macOS/Linux',
                    'PowerPoint native open/edit not run'],
    'files': [{'file': p.name, 'bytes': p.stat().st_size, 'sha256': sha(p)}
              for p in sorted(assets.iterdir()) if p.is_file() and p.name not in ('BUILD_MANIFEST.json', 'SHA256SUMS')]
}
(assets / 'BUILD_MANIFEST.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
(assets / 'SHA256SUMS').write_text(''.join(f'{sha(p)}  {p.name}\n' for p in sorted(assets.iterdir())
                                        if p.is_file() and p.name != 'SHA256SUMS'), encoding='utf-8')
print(json.dumps({'result': 'PASS', 'assets': len(list(assets.iterdir())),
                  'publicationCommit': publication, 'nativePackagesVerified': 4}))
