#!/usr/bin/env python3
"""
🛡️ Containment Scanner - فحص الاحتواء
يفحص إذا كان الهدف يطبق مبادئ Zero Trust
"""
import requests, sys, json, re

TARGET = sys.argv[1] if len(sys.argv) > 1 else "https://beeorder.com"

print(f"🛡️ Containment Scanner: {TARGET}")
print("═" * 50)

score = 100

# 1. HTTPS
print("\n1️⃣ HTTPS:")
r = requests.get(TARGET.replace("https://","http://"), timeout=5, allow_redirects=False)
if r.status_code in [301,302]:
    print("   ✅ HTTPS enforced")
else:
    print("   🚨 HTTP allowed!")
    score -= 20

# 2. Security Headers
print("\n2️⃣ Security Headers:")
r = requests.get(TARGET, timeout=5)
headers = r.headers

checks = {
    "Strict-Transport-Security": "HSTS",
    "X-Frame-Options": "Clickjacking",
    "X-Content-Type-Options": "MIME sniffing",
    "Content-Security-Policy": "XSS protection",
    "X-Permitted-Cross-Domain-Policies": "Cross-domain",
    "Referrer-Policy": "Referrer leak",
}

for header, name in checks.items():
    if header in headers:
        print(f"   ✅ {name}")
    else:
        print(f"   🚨 Missing: {name}")
        score -= 10

# 3. Cookies
print("\n3️⃣ Cookies:")
cookies = r.headers.get("Set-Cookie", "")
if "Secure" in cookies:
    print("   ✅ Secure flag")
else:
    print("   🚨 No Secure flag")
    score -= 10
if "HttpOnly" in cookies:
    print("   ✅ HttpOnly flag")
else:
    print("   🚨 No HttpOnly flag")
    score -= 10
if "SameSite" in cookies:
    print("   ✅ SameSite flag")
else:
    print("   🚨 No SameSite flag")
    score -= 10

# 4. CORS
print("\n4️⃣ CORS:")
acao = headers.get("Access-Control-Allow-Origin", "")
if acao == "*":
    print("   🚨 CORS: * - أي موقع يقدر يقرأ!")
    score -= 20
elif acao:
    print(f"   ✅ CORS: {acao}")
else:
    print("   ✅ No CORS")

# 5. Server Info
print("\n5️⃣ Server Info:")
server = headers.get("Server", "")
if server:
    print(f"   🚨 Server: {server} - يفضح التكنولوجيا")
    score -= 10
else:
    print("   ✅ Server hidden")

print(f"\n📊 Score: {max(score, 0)}/100")
print(f"   {'🔴 خطر' if score < 50 else '🟡 متوسط' if score < 80 else '🟢 آمن'}")
