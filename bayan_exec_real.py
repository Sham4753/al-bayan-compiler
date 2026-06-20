#!/usr/bin/env python3
# 📜 محرك البيان الشامل - ٢٧ سلاح
import sys, os

ROOTS = {
    "فَتَحَ": "qaysar_ultimate.py",
    "اِحتَسَبَ": "qaysar_fingerprint.py",
    "حَلَّلَ": "qaysar_ai.py",
    "اِختَرَقَ": "qaysar_autoexploit.py",
    "أَمَّنَ": "qaysar_shield.py",
    "رَاقَبَ": "qaysar_network.py",
    "سَتَرَ": "qaysar_shabah.py",
    "بَحَثَ": "qaysar_subdomains.py",
    "حَفِظَ": "qaysar_ssl.py",
    "خَزَنَ": "qaysar_cookies.py",
    "رَسَمَ": "qaysar_sitemap.py",
    "فَصَلَ": "qaysar_idor.py",
    "جَمَعَ": "qaysar_jwt.py",
    "نَشَرَ": "qaysar_api.py",
    "هَدَى": "qaysar_destroyer.py",
    "طَوَّرَ": "qaysar_deep.py",
    "صَمَّمَ": "qaysar_kamil.py",
    "عَلِمَ": "qaysar_muhaqiq.py",
    "قَوِيَ": "qaysar_full_arsenal.py",
    "شَغَّلَ": "qaysar_exploit.py",
    "زَرَعَ": "qaysar_fuzz.py",
    "حَصَّنَ": "qaysar_stealth.py",
    "نَصَرَ": "qaysar_poc.py",
    "حَمَلَ": "qaysar_apk.py",
    "وَصَلَ": "qaysar_web.py",
    "سَمِعَ": "qaysar_shares.py",
    "عَمِلَ": "qaysar_engine.py",
}

BASE = "/www/wwwroot/syria-platform.org/public"
target = sys.argv[1] if len(sys.argv) > 1 else "scanme.nmap.org"
code = sys.stdin.read() if not sys.argv[2:] else open(sys.argv[2]).read()
roots = [r.strip() for r in code.split('\n') if r.strip() in ROOTS]

if not roots:
    print("❌ الجذور المتاحة (٢٧):")
    for r in ROOTS: print(f"   {r} → {ROOTS[r]}")
    sys.exit()

print(f"🎯 {target} | 📜 {len(roots)} جذر")
for root in roots:
    cmd = f"python3 {BASE}/{ROOTS[root]} {target} 2>/dev/null"
    print(f"\n⚡ {root}:")
    print(os.popen(cmd).read()[:600])
