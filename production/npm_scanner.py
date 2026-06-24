#!/usr/bin/env python3
"""
📦 NPM Malware Scanner - مستوحى من PostCSS Attack
يفحص package.json بحثاً عن حزم مشبوهة
"""
import requests, sys, json, re, os

TARGET = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"

print(f"📦 NPM Scanner: {TARGET}")
print("═" * 50)

# 1. فحص package.json
print("\n1️⃣ فحص package.json:")
try:
    r = requests.get(f"{TARGET}/package.json", timeout=5)
    if r.status_code == 200:
        try:
            data = r.json()
            deps = data.get("dependencies", {})
            devDeps = data.get("devDependencies", {})
            print(f"   📦 {len(deps)} dependencies, {len(devDeps)} devDependencies")
            
            # فحص حزم مشبوهة
            suspicious = []
            for name, version in {**deps, **devDeps}.items():
                if any(w in name.lower() for w in ['postcss','build','tool','util','helper','core','plugin']):
                    if '0.0.' in version or version.startswith('1.0.0'):
                        suspicious.append(f"{name}@{version}")
            
            if suspicious:
                print(f"   🚨 {len(suspicious)} حزم مشبوهة:")
                for s in suspicious:
                    print(f"      {s}")
        except:
            print(f"   📄 {len(r.text)} bytes (not JSON)")
    else:
        print(f"   ❌ Status: {r.status_code}")
except:
    print("   ❌ فشل")

# 2. فحص node_modules
print("\n2️⃣ فحص node_modules:")
for path in ["/node_modules/.package-lock.json", "/node_modules/postcss/package.json"]:
    r = requests.get(f"{TARGET}{path}", timeout=5)
    if r.status_code == 200:
        print(f"   🚨 {path} مكشوف!")
    else:
        print(f"   ✅ {path}")

print("\n✅ اكتمل")
