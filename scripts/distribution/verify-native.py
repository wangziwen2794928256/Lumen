"""Static native package acceptance. Does not launch the app or bypass OS trust."""
from pathlib import Path
import hashlib, json, platform, plistlib, struct, subprocess, tempfile

def command(*args, cwd=None):
    return subprocess.check_output(args, cwd=cwd, stderr=subprocess.STDOUT, timeout=120).decode(errors='replace').strip()

results = []
for build in Path('downloaded').rglob('BUILD.json'):
    metadata = json.loads(build.read_text())
    for entry in metadata['files']:
        package = (build.parent / entry['file']).resolve()
        if not package.is_relative_to(build.parent.resolve()):
            raise SystemExit('Package path escapes artifact')
        if hashlib.sha256(package.read_bytes()).hexdigest() != entry['sha256']:
            raise SystemExit('Package hash mismatch')
        result = {'file': package.name, 'target': metadata['target'], 'commit': metadata['commit'], 'sha256': entry['sha256'], 'checks': []}
        if package.suffix == '.dmg' and platform.system() == 'Darwin':
            command('hdiutil', 'verify', str(package))
            with tempfile.TemporaryDirectory(prefix='lumen-image-') as mount:
                command('hdiutil', 'attach', '-readonly', '-nobrowse', '-noautoopen', '-mountpoint', mount, str(package))
                try:
                    app = next(Path(mount).glob('*.app'))
                    info = plistlib.loads((app / 'Contents/Info.plist').read_bytes())
                    binary = app / 'Contents/MacOS' / info['CFBundleExecutable']
                    arch = command('lipo', '-archs', str(binary))
                    expected = 'arm64' if metadata['target'].startswith('aarch64') else 'x86_64'
                    assert expected in arch.split(), 'Mach-O architecture mismatch'
                    assert info['CFBundleIdentifier'] == 'com.lumen.desktop'
                    assert info['CFBundleShortVersionString'] == '0.1.0'
                    assert info['LSMinimumSystemVersion'] == '14.0'
                    assert not list(app.rglob('*.db')), 'Database embedded in app'
                    result['checks'] = ['hdiutil checksum', 'readonly image mount', 'bundle identifier/version/minimum OS', 'Mach-O architecture', 'no SQLite payload']
                    result['architecture'] = arch
                finally:
                    command('hdiutil', 'detach', mount)
        elif package.suffix in ('.deb', '.AppImage') and platform.system() == 'Linux':
            with tempfile.TemporaryDirectory(prefix='lumen-package-') as extract:
                if package.suffix == '.deb':
                    assert command('dpkg-deb', '--field', str(package), 'Architecture') == 'amd64'
                    assert command('dpkg-deb', '--field', str(package), 'Version') == '0.1.0'
                    command('dpkg-deb', '--extract', str(package), extract)
                    base = Path(extract)
                    result['checks'].append('deb metadata and extraction')
                else:
                    package.chmod(package.stat().st_mode | 0o100)
                    command(str(package), '--appimage-extract', cwd=extract)
                    base = Path(extract) / 'squashfs-root'
                    assert (base / 'AppRun').exists(), 'AppImage entry missing'
                    result['checks'].append('AppImage extraction and AppRun')
                binary = next(p for p in (base / 'usr/bin').iterdir() if p.name.lower() == 'lumen')
                header = binary.read_bytes()[:20]
                assert header[:5] == b'\x7fELF\x02', 'Not ELF64'
                assert struct.unpack('<H', header[18:20])[0] == 62, 'Not x86_64'
                assert not list(base.rglob('*.db')), 'Database embedded in package'
                result['checks'] += ['ELF64 x86_64', 'no SQLite payload']
        else:
            continue
        result['result'] = 'PASS'
        result['nativeLaunchVerified'] = False
        result['osIntegrationVerified'] = False
        results.append(result)
expected_targets = {'aarch64-apple-darwin', 'x86_64-apple-darwin'} if platform.system() == 'Darwin' else {'x86_64-unknown-linux-gnu'}
if len(results) != 2 or {r['target'] for r in results} != expected_targets:
    raise SystemExit('Expected two native packages and complete target coverage')
Path('verification').mkdir(exist_ok=True)
Path('verification/native-package-verification.json').write_text(json.dumps({'result': 'PASS', 'packages': results}, indent=2))
print(json.dumps({'result': 'PASS', 'packages': [r['file'] for r in results], 'scope': 'static package validation only'}))
