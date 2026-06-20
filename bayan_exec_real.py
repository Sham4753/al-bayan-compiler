#!/usr/bin/env python3
import sys, os, concurrent.futures

ROOTS = {
    "فَتَحَ": "python3 /www/wwwroot/syria-platform.org/public/qaysar_ultimate.py",
    "اِحتَسَبَ": "python3 /www/wwwroot/syria-platform.org/public/qaysar_fingerprint.py",
    "حَلَّلَ": "python3 /www/wwwroot/syria-platform.org/public/qaysar_ai.py",
    "اِختَرَقَ": "python3 /www/wwwroot/syria-platform.org/public/qaysar_autoexploit.py",
}

target = sys.argv[1] if len(sys.argv) > 1 else "sana.sy"
code = sys.stdin.read() if not sys.argv[2:] else open(sys.argv[2]).read()
roots = [r.strip() for r in code.split('\n') if r.strip() in ROOTS]

def run_root(root):
    cmd = f"{ROOTS[root]} {target} 2>/dev/null"
    return f"⚡ {root}:\n{os.popen(cmd).read()[:500]}"

print(f"🎯 {target}")
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
    results = ex.map(run_root, roots)
    for r in results:
        print(r)
