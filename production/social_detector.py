#!/usr/bin/env python3
"""
🎭 Social Engineering Detector
يكتشف حملات التصيد عبر البريد، SMS، ووسائل التواصل
"""
import re, sys

TEXT = sys.argv[1] if len(sys.argv) > 1 else "تهانينا! لقد ربحت جائزة. اضغط هنا: http://fake.com"

print(f"🎭 Social Detector")
print("═" * 50)
print(f"📝 النص: {TEXT[:100]}")
print()

# 1. كشف الروابط المشبوهة
urls = re.findall(r'https?://[^\s]+', TEXT)
if urls:
    print(f"🔗 {len(urls)} روابط:")
    for url in urls:
        # فحص النطاق
        domain = re.findall(r'https?://([^/]+)', url)[0]
        suspicious = ['.tk','.cf','.ml','.ga','.xyz','.top','.info','.click','.win','.loan']
        if any(domain.endswith(s) for s in suspicious):
            print(f"   🚨 نطاق مشبوه: {domain}")
        elif len(domain) > 30:
            print(f"   🚨 نطاق طويل: {domain}")
        else:
            print(f"   ✅ {domain}")

# 2. كشف كلمات التصيد
phishing_words = [
    'ربحت','جائزة','تهانينا','عاجل','تأكيد','حسابك','تم تعليق',
    'won','prize','urgent','verify','account','suspended','click here',
    'confirm','password','login','limited','offer','free','gift'
]

found = [w for w in phishing_words if w.lower() in TEXT.lower()]
if found:
    print(f"\n🎣 كلمات تصيد: {', '.join(found)}")
    print(f"   ⚠️ احتمالية تصيد: HIGH")

# 3. كشف الإلحاح
urgency = ['الآن','فوراً','ساعة','دقائق','أيام','now','immediately','hour','minutes','limited']
urgent = [w for w in urgency if w.lower() in TEXT.lower()]
if urgent:
    print(f"\n⏰ إلحاح: {', '.join(urgent)}")

# 4. Score
score = 0
if urls: score += 30
if found: score += 40
if urgent: score += 30

print(f"\n📊 Score: {min(score, 100)}/100")
if score > 50:
    print("   🚨 تحذير: هذا النص يبدو تصيداً!")
elif score > 20:
    print("   ⚠️ حذر: يحتاج مراجعة")
else:
    print("   ✅ يبدو آمناً")
