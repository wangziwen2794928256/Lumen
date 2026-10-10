"""Collect only CI-built distributables and their SHA-256; never user data."""
from pathlib import Path
import hashlib,json,shutil,sys,subprocess
target=sys.argv[1]
root=Path("src-tauri/target")/target/"release/bundle"
dest=Path("native-output");dest.mkdir(exist_ok=True)
files=[]
for p in root.rglob("*"):
 if p.is_file() and p.suffix in (".deb",".AppImage",".dmg"):
  q=dest/p.name;shutil.copy2(p,q)
  files.append({"file":q.name,"bytes":q.stat().st_size,"sha256":hashlib.sha256(q.read_bytes()).hexdigest()})
if not files: raise SystemExit("No native packages produced")
(dest/"SHA256SUMS").write_text("".join(f"{f['sha256']}  {f['file']}\n" for f in files))
(dest/"BUILD.json").write_text(json.dumps({"target":target,"commit":subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip(),"channel":"experimental-unsigned","runtimeParityVerified":False,"files":files},indent=2))
print(json.dumps(files))
