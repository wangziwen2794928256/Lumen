"""Verify the two-line upstream glib backport against the original crate archive."""
from pathlib import Path
import hashlib, io, json, tarfile, urllib.request

root = Path(__file__).resolve().parents[2]
original_checksum = '233daaf6e83ae6a12a52055f568f9d7cf4671dabb78ff9560ab6da230ce00ee5'
url = 'https://static.crates.io/crates/glib/glib-0.18.5.crate'
with urllib.request.urlopen(url, timeout=45) as response:
    data = response.read()
if hashlib.sha256(data).hexdigest() != original_checksum:
    raise SystemExit('Original crate checksum mismatch')
vendor = root / 'src-tauri/vendor/glib'
checked = 0
with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as archive:
    for member in archive.getmembers():
        if not member.isfile(): continue
        relative = Path(member.name).relative_to('glib-0.18.5')
        original = archive.extractfile(member).read()
        target = vendor / relative
        # Cargo creates these registry bookkeeping files locally; none is shipped.
        if relative.name in ['.cargo_vcs_info.json', '.cargo-checksum.json', '.cargo-ok']: continue
        if relative.as_posix() == 'src/variant_iter.rs':
            original = original.replace(b'let p: *mut libc::c_char = std::ptr::null_mut();',
                b'let mut p: *mut libc::c_char = std::ptr::null_mut();').replace(b'                &p,', b'                &mut p,')
        # Git normalizes text line endings; binary data is otherwise byte-identical.
        if not target.is_file() or target.read_bytes().replace(b'\r\n', b'\n') != original.replace(b'\r\n', b'\n'):
            raise SystemExit('Vendor source differs beyond declared patch: ' + str(relative))
        checked += 1
print(json.dumps({'result': 'PASS', 'crate': 'glib@0.18.5', 'archiveSha256': original_checksum,
    'sourceFilesChecked': checked, 'upstreamPatch': 'gtk-rs/gtk-rs-core#1343', 'productionLinesChanged': 2}))
