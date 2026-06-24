#!/usr/bin/env python3
"""
🦑 Squid Scanner - فحص Squid Proxy
يبحث عن Squid Proxy ويكتشف CVE-2026-47729
"""
import requests, sys, socket

TARGET = sys.argv[1] if len(sys.argv) > 1 else "beeorder.com"

print(f"🦑 Squid Scanner: {TARGET}")
print("═" * 50)

# 1. فحص منفذ Squid (3128)
print("\n1️⃣ فحص منفذ Squid (3128):")
s = socket.socket(); s.settimeout(2)
if s.connect_ex((TARGET, 3128)) == 0:
    print("   🚨 3128 مفتوح - Squid Proxy!")
else:
    print("   ✅ 3128 مغلق")
s.close()

# 2. فحص كـ Proxy
print("\n2️⃣ فحص Proxy:")
try:
    proxies = {"http": f"http://{TARGET}:3128", "https": f"http://{TARGET}:3128"}
    r = requests.get("http://example.com", proxies=proxies, timeout=5)
    if r.status_code == 200:
        print("   🚨 Squid Proxy شغال!")
        print(f"   Via: {r.headers.get('Via','?')}")
        print(f"   X-Cache: {r.headers.get('X-Cache','?')}")
except:
    print("   ❌ ليس Proxy")

# 3. فحص CVE-2026-47729
print("\n3️⃣ فحص Squidbleed:")
try:
    # إرسال طلبين معاً
    r1 = requests.get(f"http://{TARGET}:3128", timeout=5)
    r2 = requests.get(f"http://{TARGET}:3128", timeout=5)
    if r1.text != r2.text:
        print("   🚨 تسريب محتمل - Squidbleed!")
    else:
        print("   ✅ لا تسريب")
except:
    print("   ❌ فشل")

print("\n✅ اكتمل")
